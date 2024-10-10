use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};

use crate::{
    config::{MAX_RUN_AWAY_RADIUS, MIN_RUN_AWAY_RADIUS, RUNNING_SPEED},
    environment::Tile,
    grid::{Grid, GridLocation},
    pathfinding::Path,
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
    query: Query<(Entity, &Transform, &RunAway), Added<RunAway>>,
    grid: Res<Grid<Tile>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    // IMPROVEME: While running away and seeing an Investigator, will switch to going to the investigator and tell him where the villager seen the player.
    for (entity, transform, run_away) in &query {
        if let Some(entity_grid_location) = GridLocation::from_world(transform.translation.xy()) {
            if let Ok(target) = grid.find_away_from(
                &entity_grid_location,
                &run_away.player_last_seen,
                &[MIN_RUN_AWAY_RADIUS, MAX_RUN_AWAY_RADIUS],
                rng.as_mut(),
            ) {
                if let Ok(path) = grid.path_to(&entity_grid_location, &target) {
                    commands.entity(entity).insert(path);
                    commands.entity(entity).insert(MovementSpeed(RUNNING_SPEED));
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
