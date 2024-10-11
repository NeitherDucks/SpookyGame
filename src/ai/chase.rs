use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{
    config::CHASE_SPEED,
    grid::{Grid, Tile},
    pathfinding::Path,
};

use super::MovementSpeed;

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Chase {
    pub target: Entity,
    pub player_last_seen: GridCoords,
}

pub fn chase_on_enter(mut commands: Commands, query: Query<Entity, Added<Chase>>) {
    for entity in &query {
        commands.entity(entity).insert(MovementSpeed(CHASE_SPEED));
    }
}

/// While [`Chase`], update [`Path`] to reflect target new position
pub fn chase_update(
    mut commands: Commands,
    coords: Query<&GridCoords, Without<Chase>>,
    mut query: Query<(Entity, &GridCoords, &mut Chase)>,
    grid: Res<Grid<Tile>>,
) {
    for (entity, entity_coords, mut chase) in &mut query {
        let Ok(target_coords) = coords.get(chase.target) else {
            continue;
        };

        // Store last known position
        chase.player_last_seen = target_coords.clone();

        // Calculate path to target
        let path = grid.path_to(&entity_coords, &target_coords);

        // if found a valid path to target
        if let Ok(path) = path {
            commands.entity(entity).insert(path);
        } else {
            warn!("Could not find a Path from {} to {}", entity, chase.target);
        }
    }
}

/// When [`Chase`] is removed, remove any [`Path`] and [`MovementSpeed`].
pub fn chase_on_exit(mut commands: Commands, mut query: RemovedComponents<Chase>) {
    for entity in query.read() {
        commands.entity(entity).remove::<Path>();
        commands.entity(entity).remove::<MovementSpeed>();
    }
}
