use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_rand::prelude::{GlobalEntropy, WyRand};

use crate::{
    config::{NORMAL_SPEED, WANDERING_RADIUS},
    grid::{Grid, Tile},
    pathfinding::Path,
};

use super::MovementSpeed;

#[derive(Clone, Default, Component)]
#[component(storage = "SparseSet")]
pub struct Wander;

/// When [`Wander`] is added, generate a target and a [`Path`].
pub fn wander_on_enter(
    mut commands: Commands,
    query: Query<(Entity, &GridCoords), Added<Wander>>,
    grid: Res<Grid<Tile>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for (entity, coords) in &query {
        if let Ok(target) = grid.find_nearby(&coords, WANDERING_RADIUS, rng.as_mut()) {
            if let Ok(path) = grid.path_to(&coords, &target) {
                commands.entity(entity).insert(path);
                commands.entity(entity).insert(MovementSpeed(NORMAL_SPEED));
            }
        }
    }
}

/// When [`Wander`] is removed, remove any [`Path`] and [`MovementSpeed`].
pub fn wander_on_exit(mut commands: Commands, mut query: RemovedComponents<Wander>) {
    for entity in query.read() {
        commands.entity(entity).remove::<Path>();
        commands.entity(entity).remove::<MovementSpeed>();
    }
}
