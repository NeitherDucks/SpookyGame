// use std::time::Instant;

use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_rand::prelude::{GlobalEntropy, WyRand};

use crate::{
    config::{INVESTIGATING_RADIUS, RUNNING_SPEED},
    grid::{Grid, Tile},
    ldtk::animation::new_animation,
    pathfinding::Path,
};

use super::{MovementSpeed, INVESTIGATING_TIME, INVESTIGATOR_ANIMATION_RUN};

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct Investigate {
    pub target: GridCoords,
    // pub start: Instant,
    pub reached_area: bool,
    pub timer: Timer,
}

impl Default for Investigate {
    fn default() -> Self {
        Investigate {
            target: GridCoords::default(),
            // start: Instant::now(),
            reached_area: false,
            timer: Timer::from_seconds(INVESTIGATING_TIME as f32, TimerMode::Once),
        }
    }
}

pub fn investigate_on_enter(mut commands: Commands, query: Query<Entity, Added<Investigate>>) {
    for entity in &query {
        commands.entity(entity).insert(MovementSpeed(RUNNING_SPEED));

        commands
            .entity(entity)
            .insert(new_animation(INVESTIGATOR_ANIMATION_RUN));
    }
}

/// When entity doens't have a [`Path`], pick a location around the [`Investigate`].target, within [`Investigate`].range
pub fn investigate_update(
    mut commands: Commands,
    mut investigate: Query<(Entity, &GridCoords, &mut Investigate), Without<Path>>,
    grid: Res<Grid<Tile>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    time: Res<Time>,
) {
    for (entity, coords, mut investigate) in &mut investigate {
        investigate.timer.tick(time.delta());

        if *coords == investigate.target {
            investigate.reached_area = true;
        }

        if investigate.reached_area {
            if let Ok(new_target) =
                grid.find_nearby(&investigate.target, INVESTIGATING_RADIUS, rng.as_mut())
            {
                if let Ok(path) = grid.path_to(&coords, &new_target) {
                    commands.entity(entity).insert(path);
                }
            }
        } else {
            if let Ok(path) = grid.path_to(&coords, &investigate.target) {
                commands.entity(entity).insert(path);
            }
        }
    }
}

/// When [`Investigate`] is removed, remove any [`Path`] and [`MovementSpeed`].
pub fn investigate_on_exit(mut commands: Commands, mut query: RemovedComponents<Investigate>) {
    for entity in query.read() {
        commands.entity(entity).remove::<Path>();
        commands.entity(entity).remove::<MovementSpeed>();
    }
}
