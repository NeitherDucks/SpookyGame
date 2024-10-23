use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_rand::prelude::{GlobalEntropy, WyRand};

use crate::{
    config::{MAX_RUN_AWAY_RADIUS, MIN_RUN_AWAY_RADIUS, RUNNING_SPEED},
    grid::{Grid, Tile},
    ldtk::animation::new_animation,
    pathfinding::Path,
};

use super::{MovementSpeed, VILLAGER_ANIMATION_FLEE};

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct RunAway {
    pub player_last_seen: GridCoords,
}

/// When [`RunAway`] is added, generate [`Path`].
pub fn run_away_on_enter(
    mut commands: Commands,
    query: Query<(Entity, &GridCoords, &RunAway), Added<RunAway>>,
    grid: Res<Grid<Tile>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for (entity, coords, run_away) in &query {
        if let Ok(target) = grid.find_away_from(
            &coords,
            &run_away.player_last_seen,
            &[MIN_RUN_AWAY_RADIUS, MAX_RUN_AWAY_RADIUS],
            rng.as_mut(),
        ) {
            if let Ok(path) = grid.path_to(&coords, &target) {
                commands.entity(entity).insert(path);
                commands.entity(entity).insert(MovementSpeed(RUNNING_SPEED));
            }
        }

        commands
            .entity(entity)
            .insert(new_animation(VILLAGER_ANIMATION_FLEE));
    }
}

/// When [`RunAway`] is removed, remove any [`Path`] and [`MovementSpeed`].
pub fn run_away_on_exit(mut commands: Commands, mut query: RemovedComponents<RunAway>) {
    for entity in query.read() {
        commands.entity(entity).remove::<Path>();
        commands.entity(entity).remove::<MovementSpeed>();
    }
}
