use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

use crate::{
    config::{MAX_RUN_AWAY_RADIUS, MIN_RUN_AWAY_RADIUS},
    environment::Tile,
    grid::{Grid, GridLocation},
    pathfinding::Path,
    utils::remap_rand_u32,
};

use super::MovementSpeed;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct RunAway {
    pub player_last_seen: GridLocation,
}

/// When [`RunAway`] is added, generate [`Path`].
pub fn run_away_on_enter(
    mut commands: Commands,
    query: Query<(Entity, &Transform), Added<RunAway>>,
    grid: Res<Grid<Tile>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    // TODO: Actually run away from the player, not in a random direction, but the oposite.
    // IMPROVEME: While running away and seeing an Investigator, will switch to going to the investigator and tell him where the villager seen the player.
    for (entity, transform) in &query {
        if let Some(entity_grid_location) = GridLocation::from_world(transform.translation.xy()) {
            let radius = remap_rand_u32(rng.next_u32(), MIN_RUN_AWAY_RADIUS, MAX_RUN_AWAY_RADIUS);

            if let Ok(target) = grid.find_nearby(&entity_grid_location, radius, rng.as_mut()) {
                if let Ok(path) = grid.path_to(&entity_grid_location, &target) {
                    commands.entity(entity).insert(path);
                }
            }
        };
    }
}

/// When [`RunAway`] is removed, remove any [`Path`] and [`MovementSpeed`].
pub fn run_away_on_exit(mut commands: Commands, mut query: RemovedComponents<RunAway>) {
    for entity in query.read() {
        commands.entity(entity).remove::<Path>();
        commands.entity(entity).remove::<MovementSpeed>();
    }
}
