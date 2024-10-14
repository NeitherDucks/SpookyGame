use std::time::Instant;

use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{
    collisions::Collider,
    config::CHASE_SPEED,
    grid::{Grid, Tile},
    ldtk::entities::Aim,
    pathfinding::Path,
    player::{is_player_visible, PlayerTag},
};

use super::{Investigate, MovementSpeed, INVESTIGATOR_VIEW_HALF_ANGLE, INVESTIGATOR_VIEW_RANGE};

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
    player: Query<(&GridCoords, &Transform, &Collider), With<PlayerTag>>,
    mut query: Query<(Entity, &GridCoords, &Transform, &Aim, &mut Chase)>,
    grid: Res<Grid<Tile>>,
) {
    for (entity, entity_coords, entity_transform, aim, mut chase) in &mut query {
        let Ok((target_coords, target_transform, target_collider)) = player.get(chase.target)
        else {
            continue;
        };

        // Check if player is visible
        let entity_translate = entity_transform.translation.xy();
        let target_translate = target_transform.translation.xy();

        if is_player_visible(
            target_translate,
            entity_translate,
            *aim,
            INVESTIGATOR_VIEW_RANGE,
            INVESTIGATOR_VIEW_HALF_ANGLE,
            target_collider,
        ) {
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
        } else {
            commands.entity(entity).remove::<Chase>();

            commands.entity(entity).insert(Investigate {
                start: Instant::now(),
                target: chase.player_last_seen,
            });
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
