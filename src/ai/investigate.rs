use std::time::Instant;

use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};

use crate::{
    config::INVESTIGATING_RADIUS,
    environment::Tile,
    grid::{Grid, GridLocation},
    pathfinding::Path,
};

use super::MovementSpeed;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct Investigate {
    pub target: GridLocation,
    pub start: Instant,
}

/// When entity doens't have a [`Path`], pick a location around the [`Investigate`].target, within [`Investigate`].range
pub fn investigate_update(
    mut commands: Commands,
    investigate: Query<(Entity, &Transform, &Investigate), Without<Path>>,
    grid: Res<Grid<Tile>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for (entity, transform, investigate) in &investigate {
        if let Some(current_grid_position) = GridLocation::from_world(transform.translation.xy()) {
            if let Ok(new_target) =
                grid.find_nearby(&investigate.target, INVESTIGATING_RADIUS, rng.as_mut())
            {
                if let Ok(path) = grid.path_to(&current_grid_position, &new_target) {
                    commands.entity(entity).insert(path);
                }
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
