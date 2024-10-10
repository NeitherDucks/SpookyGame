use bevy::{ecs::query, prelude::*};

use crate::{
    config::CHASE_SPEED,
    environment::Tile,
    grid::{Grid, GridLocation},
    pathfinding::Path,
};

use super::MovementSpeed;

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Chase {
    pub target: Entity,
    pub player_last_seen: GridLocation,
}

pub fn chase_on_enter(mut commands: Commands, query: Query<Entity, Added<Chase>>) {
    for entity in &query {
        commands.entity(entity).insert(MovementSpeed(CHASE_SPEED));
    }
}

/// While [`Chase`], update [`Path`] to reflect target new position
pub fn chase_update(
    mut commands: Commands,
    transform: Query<&Transform, Without<Chase>>,
    mut query: Query<(Entity, &Transform, &mut Chase)>,
    grid: Res<Grid<Tile>>,
) {
    for (entity, entity_transform, mut chase) in &mut query {
        let entity_position = entity_transform.translation.xy();

        let Ok(target_transform) = transform.get(chase.target) else {
            continue;
        };

        let target_position = target_transform.translation.xy();

        // Get current position on the grid
        let start = GridLocation::from_world(entity_position);
        // Get target position on the grid
        let goal = GridLocation::from_world(target_position);

        // If both are actually in the grid
        if start.is_some() && goal.is_some() {
            let start = start.unwrap();
            let goal = goal.unwrap();

            // Store last known position
            chase.player_last_seen = goal.clone();

            // Calculate path to target
            let path = grid.path_to(&start, &goal);

            // if found a valid path to target
            if let Ok(path) = path {
                commands.entity(entity).insert(path);
            } else {
                warn!("Could not find a Path from {} to {}", entity, chase.target);
            }
        } else {
            warn!(
                "Could not find a GridLocation for {} or {}",
                entity, chase.target
            );
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
