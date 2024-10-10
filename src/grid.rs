// Grid for pathfinding
// "Stolen" from https://www.youtube.com/watch?v=QTUEyAZmdv4

use std::{
    f32::consts::PI,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

use crate::{
    config::{FIND_NEARBY_MAX_TRIES, GRID_SIZE, MAX_RUN_AWAY_ANGLE},
    utils::remap_rand_f32,
};

pub struct GridFindError;

#[derive(Default)]
pub struct GridPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T: Component> Plugin for GridPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid<T>>()
            // .add_systems(Update, lock_to_grid::<T>)
            .add_event::<DirtyGridEvent<T>>()
            .add_systems(PreUpdate, (add_to_grid::<T>, remove_from_grid::<T>));
    }
}

#[derive(Event)]
pub struct DirtyGridEvent<T>(pub GridLocation, PhantomData<T>);

#[derive(Resource)]
pub struct Grid<T> {
    pub entities: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
    _marker: PhantomData<T>,
}

impl<T> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            entities: self.entities,
            _marker: self._marker,
        }
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            entities: [[None; GRID_SIZE]; GRID_SIZE],
            _marker: Default::default(),
        }
    }
}

impl<T> Grid<T> {
    pub fn occupied(&self, location: &GridLocation) -> bool {
        Grid::<T>::valid_index(location) && self[location].is_some()
    }

    pub fn valid_index(location: &GridLocation) -> bool {
        location.x >= 0
            && location.y >= 0
            && location.x < GRID_SIZE as i32
            && location.y < GRID_SIZE as i32
    }

    pub fn find_nearby(
        &self,
        location: &GridLocation,
        radius: u32,
        rng: &mut GlobalEntropy<WyRand>,
    ) -> Result<GridLocation, GridFindError> {
        for _ in 0..FIND_NEARBY_MAX_TRIES {
            let angle = remap_rand_f32(rng.next_u32(), 0., 2. * PI);
            let dist = remap_rand_f32(rng.next_u32(), 0., radius as f32 * 16.);

            let new_world_pos =
                location.to_world() + Vec2::new(angle.cos() * dist, angle.sin() * dist);

            if let Some(nearby) = GridLocation::from_world(new_world_pos) {
                if Grid::<T>::valid_index(&nearby) && !self.occupied(&nearby) {
                    return Ok(nearby);
                }
            }
        }

        Err(GridFindError)
    }

    pub fn find_away_from(
        &self,
        location: &GridLocation,
        away_from: &GridLocation,
        radius: &[u32; 2],
        rng: &mut GlobalEntropy<WyRand>,
    ) -> Result<GridLocation, GridFindError> {
        for _ in 0..FIND_NEARBY_MAX_TRIES {
            let away_dir = (location.to_world() - away_from.to_world()).normalize_or_zero();

            let angle = away_dir.y.atan2(away_dir.x)
                + remap_rand_f32(
                    rng.next_u32(),
                    -MAX_RUN_AWAY_ANGLE.to_radians(),
                    MAX_RUN_AWAY_ANGLE.to_radians(),
                );
            let dist = remap_rand_f32(rng.next_u32(), radius[0] as f32, radius[1] as f32);

            let new_world_pos =
                away_from.to_world() + Vec2::new(angle.cos() * dist, angle.sin() * dist);

            if let Some(away) = GridLocation::from_world(new_world_pos) {
                if Grid::<T>::valid_index(&away) && !self.occupied(&away) {
                    return Ok(away);
                }
            }
        }

        Err(GridFindError)
    }
}

#[derive(Component, Reflect, Default, Eq, PartialEq, Hash, Clone, Copy, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub struct GridLocation(pub IVec2);

impl<T> Index<&GridLocation> for Grid<T> {
    type Output = Option<Entity>;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        &self.entities[index.x as usize][index.y as usize]
    }
}

impl<T> IndexMut<&GridLocation> for Grid<T> {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        &mut self.entities[index.x as usize][index.y as usize]
    }
}

impl GridLocation {
    pub fn new(x: u32, y: u32) -> Self {
        GridLocation(IVec2::new(x as i32, y as i32))
    }

    pub fn from_world(position: Vec2) -> Option<Self> {
        let position = (position / 16.0) + Vec2::splat(0.5);
        let location = GridLocation(IVec2::new(position.x as i32, position.y as i32));
        if Grid::<()>::valid_index(&location) {
            Some(location)
        } else {
            None
        }
    }

    pub fn to_world(&self) -> Vec2 {
        Vec2::new(self.x as f32 * 16., self.y as f32 * 16.)
    }

    pub fn distance(&self, other: &GridLocation) -> usize {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as usize
    }
}

impl<T> Grid<T> {
    pub fn iter(&self) -> impl Iterator<Item = (Entity, GridLocation)> + '_ {
        self.entities
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_i, entity)| entity.is_some())
            .map(|(i, entity)| {
                (
                    entity.unwrap(),
                    GridLocation::new(i as u32 / GRID_SIZE as u32, i as u32 % GRID_SIZE as u32),
                )
            })
    }
}

fn remove_from_grid<T: Component>(
    mut grid: ResMut<Grid<T>>,
    mut query: RemovedComponents<T>,
    mut dirty: EventWriter<DirtyGridEvent<T>>,
) {
    for removed_entity in query.read() {
        let removed = grid.iter().find(|(entity, _)| *entity == removed_entity);
        if let Some((_, location)) = removed {
            dirty.send(DirtyGridEvent::<T>(
                location.clone(),
                PhantomData::default(),
            ));
            grid[&location] = None;
        }
    }
}

fn add_to_grid<T: Component>(
    mut grid: ResMut<Grid<T>>,
    query: Query<(Entity, &GridLocation), Added<T>>,
    mut dirty: EventWriter<DirtyGridEvent<T>>,
) {
    for (entity, location) in &query {
        if let Some(existing) = grid[location] {
            if existing != entity {
                warn!("Over-writing entity in grid");
                dirty.send(DirtyGridEvent::<T>(
                    location.clone(),
                    PhantomData::default(),
                ));
                grid[location] = Some(entity);
            }
        } else {
            dirty.send(DirtyGridEvent::<T>(
                location.clone(),
                PhantomData::default(),
            ));
            grid[location] = Some(entity);
        }
    }
}
