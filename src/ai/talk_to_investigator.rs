use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{
    config::RUNNING_SPEED,
    grid::{Grid, Tile},
    ldtk::animation::new_animation,
    pathfinding::Path,
};

use super::{run_away::RunAway, MovementSpeed, VILLAGER_ANIMATION_FLEE};

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct TalkToInvestigator {
    pub investigator: Entity,
    pub player_last_seen: GridCoords,
}

pub fn talk_to_investigator_on_enter(
    mut commands: Commands,
    query: Query<Entity, Added<TalkToInvestigator>>,
) {
    for entity in &query {
        commands.entity(entity).insert(MovementSpeed(RUNNING_SPEED));
        commands
            .entity(entity)
            .insert(new_animation(VILLAGER_ANIMATION_FLEE));
    }
}

pub fn talk_to_investigator_update(
    mut commands: Commands,
    coords: Query<&GridCoords, Without<TalkToInvestigator>>,
    mut query: Query<(Entity, &GridCoords, &TalkToInvestigator)>,
    grid: Res<Grid<Tile>>,
) {
    for (entity, entity_coords, talk) in &mut query {
        let Ok(target_coords) = coords.get(talk.investigator) else {
            continue;
        };

        // Calculate path to target
        let path = grid.path_to(&entity_coords, &target_coords);

        // if found a valid path to target
        if let Ok(path) = path {
            commands.entity(entity).insert(path);
            continue;
        }

        // Could not find a path to the investigator, abandon trying and go back to running away.
        commands.entity(entity).remove::<TalkToInvestigator>();
        commands.entity(entity).insert(RunAway {
            player_last_seen: talk.player_last_seen,
        });
    }
}

pub fn talk_to_investigator_on_exit(
    mut commands: Commands,
    mut query: RemovedComponents<TalkToInvestigator>,
) {
    for entity in query.read() {
        commands.entity(entity).remove::<Path>();
        commands.entity(entity).remove::<MovementSpeed>();
    }
}
