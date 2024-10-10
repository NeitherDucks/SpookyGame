use bevy::prelude::*;

use crate::{
    config::RUNNING_SPEED,
    environment::Tile,
    grid::{Grid, GridLocation},
    pathfinding::Path,
};

use super::{run_away::RunAway, MovementSpeed};

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct TalkToInvestigator {
    pub investigator: Entity,
    pub player_last_seen: GridLocation,
}

pub fn talk_to_investigator_on_enter(
    mut commands: Commands,
    query: Query<Entity, Added<TalkToInvestigator>>,
) {
    for entity in &query {
        commands.entity(entity).insert(MovementSpeed(RUNNING_SPEED));
    }
}

pub fn talk_to_investigator_update(
    mut commands: Commands,
    transform: Query<&Transform, Without<TalkToInvestigator>>,
    mut query: Query<(Entity, &Transform, &TalkToInvestigator)>,
    grid: Res<Grid<Tile>>,
) {
    for (entity, entity_transform, talk) in &mut query {
        let entity_position = entity_transform.translation.xy();

        let Ok(target_transform) = transform.get(talk.investigator) else {
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

            // Calculate path to target
            let path = grid.path_to(&start, &goal);

            // if found a valid path to target
            if let Ok(path) = path {
                commands.entity(entity).insert(path);
                continue;
            }
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
