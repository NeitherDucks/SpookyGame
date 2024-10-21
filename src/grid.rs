// Grid for pathfinding
// "Stolen" from https://www.youtube.com/watch?v=QTUEyAZmdv4

use std::{
    f32::consts::PI,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

use crate::{
    config::{FIND_NEARBY_MAX_TRIES, GRID_SIZE, MAX_RUN_AWAY_ANGLE, TILE_SIZE},
    utils::remap_rand_f32,
};

#[derive(Component, Default, Debug)]
pub struct Tile;

pub struct GridFindError;

#[derive(Default)]
pub struct GridPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T: Component> Plugin for GridPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid<T>>()
            .add_systems(PreUpdate, (add_to_grid::<T>, remove_from_grid::<T>));
    }
}

#[derive(Resource)]
pub struct Grid<T> {
    pub entities: [[Option<Entity>; GRID_SIZE.y as usize]; GRID_SIZE.x as usize],
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
            entities: [[None; GRID_SIZE.y as usize]; GRID_SIZE.x as usize],
            _marker: Default::default(),
        }
    }
}

impl<T> Grid<T> {
    pub fn occupied(&self, location: &GridLocation) -> bool {
        Grid::<T>::valid_index(location) && self[location].is_some()
    }

    pub fn valid_index(location: &GridLocation) -> bool {
        location.x >= 0 && location.y >= 0 && location.x < GRID_SIZE.x && location.y < GRID_SIZE.y
    }

    pub fn find_nearby(
        &self,
        location: &GridCoords,
        radius: u32,
        rng: &mut GlobalEntropy<WyRand>,
    ) -> Result<GridCoords, GridFindError> {
        let location = GridLocation::from(*location);

        for _ in 0..FIND_NEARBY_MAX_TRIES {
            let angle = remap_rand_f32(rng.next_u32(), 0., 2. * PI);
            let dist = remap_rand_f32(rng.next_u32(), 0., radius as f32 * 16.);

            let new_world_pos =
                location.to_world() + Vec2::new(angle.cos() * dist, angle.sin() * dist);

            if let Some(nearby) = GridLocation::from_world(new_world_pos) {
                if Grid::<T>::valid_index(&nearby) && !self.occupied(&nearby) {
                    return Ok(nearby.into());
                }
            }
        }

        Err(GridFindError)
    }

    pub fn find_away_from(
        &self,
        location: &GridCoords,
        away_from: &GridCoords,
        radius: &[u32; 2],
        rng: &mut GlobalEntropy<WyRand>,
    ) -> Result<GridCoords, GridFindError> {
        let location = GridLocation::from(*location);
        let away_from = GridLocation::from(*away_from);

        for _ in 0..FIND_NEARBY_MAX_TRIES {
            let away_dir = (location.to_world() - away_from.to_world()).normalize_or_zero();

            let angle = away_dir.y.atan2(away_dir.x)
                + remap_rand_f32(
                    rng.next_u32(),
                    -MAX_RUN_AWAY_ANGLE.to_radians(),
                    MAX_RUN_AWAY_ANGLE.to_radians(),
                );

            let dist = remap_rand_f32(
                rng.next_u32(),
                (radius[0] * TILE_SIZE.x as u32) as f32,
                (radius[1] * TILE_SIZE.y as u32) as f32,
            );

            let new_world_pos =
                away_from.to_world() + Vec2::new(angle.cos() * dist, angle.sin() * dist);

            if let Some(away) = GridLocation::from_world(new_world_pos) {
                if Grid::<T>::valid_index(&away) && !self.occupied(&away) {
                    return Ok(away.into());
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

impl From<GridCoords> for GridLocation {
    fn from(value: GridCoords) -> Self {
        GridLocation::new(value.x, value.y)
    }
}

impl From<GridLocation> for GridCoords {
    fn from(value: GridLocation) -> Self {
        GridCoords::new(value.x, value.y)
    }
}

impl GridLocation {
    pub fn new(x: i32, y: i32) -> Self {
        GridLocation(IVec2::new(x, y))
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
                    GridLocation::new(i as i32 / GRID_SIZE.x, i as i32 % GRID_SIZE.y),
                )
            })
    }
}

fn remove_from_grid<T: Component>(mut grid: ResMut<Grid<T>>, mut query: RemovedComponents<T>) {
    for removed_entity in query.read() {
        let removed = grid.iter().find(|(entity, _)| *entity == removed_entity);
        if let Some((_, location)) = removed {
            grid[&location] = None;
        }
    }
}

fn add_to_grid<T: Component>(
    mut grid: ResMut<Grid<T>>,
    query: Query<(Entity, &GridLocation), (Added<GridLocation>, With<T>)>,
) {
    for (entity, location) in &query {
        if let Some(existing) = grid[location] {
            if existing != entity {
                warn!("Over-writing entity in grid");
                grid[location] = Some(entity);
            }
        } else {
            grid[location] = Some(entity);
        }
    }
}
