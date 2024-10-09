use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};

use crate::{
    config::WANDERING_RADIUS,
    environment::Tile,
    grid::{Grid, GridLocation},
    pathfinding::Path,
};

use super::MovementSpeed;

#[derive(Clone, Default, Component)]
#[component(storage = "SparseSet")]
pub struct Wander;

/// When [`Wander`] is added, generate a target and a [`Path`].
pub fn wander_on_enter(
    mut commands: Commands,
    query: Query<(Entity, &Transform), Added<Wander>>,
    grid: Res<Grid<Tile>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for (entity, transform) in &query {
        if let Some(entity_grid_location) = GridLocation::from_world(transform.translation.xy()) {
            if let Ok(target) =
                grid.find_nearby(&entity_grid_location, WANDERING_RADIUS, rng.as_mut())
            {
                if let Ok(path) = grid.path_to(&entity_grid_location, &target) {
                    commands.entity(entity).insert(path);
                }
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
