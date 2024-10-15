use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_rapier2d::plugin::RapierContext;

use crate::{
    ldtk::entities::{Aim, EnemyTag, InteractibleTriggered},
    pathfinding::Path,
    player::{is_player_visible, PlayerTag},
};

use super::{Chase, Idle, Investigate, RunAway, TalkToInvestigator, Wander};

use crate::config::*;

/// In any [`Idle`], [`Investigate`] or [`Wander`], and the player is nearby and in the field of vision of an Enemy, either [`Chase`] or [`RunAway`].
pub fn notice_player(
    mut commands: Commands,
    player: Query<(Entity, &GridCoords, &Transform), With<PlayerTag>>,
    query: Query<(
        Entity,
        &Transform,
        &Aim,
        &EnemyTag,
        AnyOf<(&Idle, &Investigate, &Wander)>,
    )>,
    rapier_context: Res<RapierContext>,
    mut gizmos: Gizmos,
) {
    if let Ok((player, player_coords, player_transform)) = player.get_single() {
        for (entity, entity_transform, aim, tag, _) in &query {
            let distance_threshold = match tag {
                EnemyTag::Investigator => INVESTIGATOR_VIEW_RANGE,
                EnemyTag::Villager => VILLAGERS_VIEW_RANGE,
            };

            let angle_threshold = match tag {
                EnemyTag::Investigator => INVESTIGATOR_VIEW_HALF_ANGLE,
                EnemyTag::Villager => VILLAGERS_VIEW_HALF_ANGLE,
            };

            let player_location = player_transform.translation.xy();
            let enemy_location = entity_transform.translation.xy();

            // Check if player is visible.
            let result = is_player_visible(
                player,
                entity,
                player_location,
                enemy_location,
                *aim,
                distance_threshold,
                angle_threshold,
                &rapier_context,
                &mut gizmos,
            );

            if result {
                // Removing inexisting component seems fine (nothing is screaming at me).
                commands.entity(entity).remove::<Idle>();
                commands.entity(entity).remove::<Investigate>();
                commands.entity(entity).remove::<Wander>();

                match tag {
                    // If Enemy is an Investigator, chase the player.
                    EnemyTag::Investigator => {
                        commands.entity(entity).insert(Chase {
                            target: player,
                            player_last_seen: *player_coords,
                        });
                    }
                    // If enemy is a Villager, run away from player.
                    EnemyTag::Villager => {
                        commands.entity(entity).insert(RunAway {
                            player_last_seen: *player_coords,
                        });
                    }
                }
            }
        }
    }
}

/// If any [`RunAway`] find an investigator on their path, swtich to going to talk to them.
pub fn running_away_to_talk_to_investigator(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Aim, &RunAway)>,
    query2: Query<(Entity, &Transform, &EnemyTag), Without<Chase>>,
) {
    for (entity, transform, aim, run_away) in &query {
        for (investigator, investigator_transform, tag) in &query2 {
            if *tag == EnemyTag::Investigator {
                let investigator_location = investigator_transform.translation.xy();
                let entity_locaton = transform.translation.xy();

                // Check if investigator is withing range.
                if investigator_location.distance(entity_locaton) < VILLAGERS_VIEW_RANGE {
                    // Check if investigator is within view.
                    if aim
                        .0
                        .dot((investigator_location - entity_locaton).normalize())
                        < VILLAGERS_VIEW_HALF_ANGLE.cos()
                    {
                        commands.entity(entity).remove::<RunAway>();

                        commands.entity(entity).insert(TalkToInvestigator {
                            investigator,
                            player_last_seen: run_away.player_last_seen,
                        });
                    }
                }
            }
        }
    }
}

/// If [`TalkToInvestigator`] and in reach of said investigator, switch to [`RunAway`], and swtich that investigator to [`Investigate`] the player's last seen location.
pub fn talk_to_investigator_to_running_away(
    mut commands: Commands,
    transforms: Query<&Transform, Without<TalkToInvestigator>>,
    query: Query<(Entity, &Transform, &TalkToInvestigator)>,
) {
    for (entity, entity_transform, talk) in &query {
        if let Ok(investigator_transform) = transforms.get(talk.investigator) {
            if investigator_transform
                .translation
                .xy()
                .distance(entity_transform.translation.xy())
                <= INTERACTION_DISTANCE
            {
                commands.entity(talk.investigator).remove::<Idle>();
                commands.entity(talk.investigator).remove::<Investigate>();
                commands.entity(talk.investigator).remove::<Wander>();

                commands.entity(talk.investigator).insert(Investigate {
                    target: talk.player_last_seen,
                    ..Default::default()
                });

                commands.entity(entity).remove::<TalkToInvestigator>();

                commands.entity(entity).insert(RunAway {
                    player_last_seen: talk.player_last_seen,
                });
            }
        }
    }
}

/// After the timer for [`Idle`] expires switch to [`Idle`].
pub fn idle_to_wandering(mut commands: Commands, query: Query<(Entity, &Idle)>) {
    for (entity, idle) in &query {
        if idle.start.elapsed() >= Duration::from_secs(IDLING_TIME) {
            commands.entity(entity).remove::<Idle>();

            commands.entity(entity).insert(Wander);
        }
    }
}

/// After reaching the [`Wander`].target, switch to [`Idle`].
pub fn wandering_to_idle(
    mut commands: Commands,
    query: Query<Entity, (With<Wander>, Without<Path>)>,
) {
    for entity in &query {
        commands.entity(entity).remove::<Wander>();

        commands.entity(entity).insert(Idle::default());
    }
}

/// After reaching the [`RunAway`].target, switch to [`Idle`].
pub fn run_away_to_idle(
    mut commands: Commands,
    query: Query<Entity, (With<RunAway>, Without<Path>)>,
) {
    for entity in &query {
        commands.entity(entity).remove::<RunAway>();

        commands.entity(entity).insert(Idle::default());
    }
}

/// If the player triggered an interactible nearby, go [`Investigate`].
pub fn idle_or_wandering_to_investigating(
    mut commands: Commands,
    query: Query<(Entity, &InteractibleTriggered), Or<(With<Idle>, With<Wander>)>>,
) {
    for (entity, interactible) in &query {
        commands.entity(entity).remove::<Idle>();
        commands.entity(entity).remove::<Wander>();

        commands.entity(entity).insert(Investigate {
            target: interactible.location,
            ..Default::default()
        });
    }
}

/// If lost visual on player during [`Chase`], go [`Investigate`] last known location.
pub fn chasing_to_investigating(
    mut commands: Commands,
    player: Query<(Entity, &GridCoords, &Transform), With<PlayerTag>>,
    query: Query<(Entity, &Transform, &Aim), With<Chase>>,
    rapier_context: Res<RapierContext>,
    mut gizmos: Gizmos,
) {
    for (entity, entity_transform, aim) in &query {
        let Ok((player, target_coords, target_transform)) = player.get_single() else {
            continue;
        };

        // Check if player is visible
        let entity_translate = entity_transform.translation.xy();
        let target_translate = target_transform.translation.xy();

        let result = is_player_visible(
            player,
            entity,
            target_translate,
            entity_translate,
            *aim,
            INVESTIGATOR_VIEW_RANGE * 1.3,
            INVESTIGATOR_VIEW_HALF_ANGLE,
            &rapier_context,
            &mut gizmos,
        );

        // If still visible, update last seen coordinates, otherwise, swtich to Investigate.
        if !result {
            commands.entity(entity).remove::<Chase>();

            commands.entity(entity).insert(Investigate {
                target: *target_coords,
                ..Default::default()
            });
        }
    }
}

/// If within range of the player while [`Chase`], trigger death and end the run.
pub fn chasing_to_killing(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Chase>>,
    player: Query<(Entity, &Transform), With<PlayerTag>>,
) {
    if let Ok((_, player_transform)) = player.get_single() {
        for (entity, transform) in &query {
            if transform
                .translation
                .xy()
                .distance(player_transform.translation.xy())
                <= INTERACTION_DISTANCE
            {
                commands.entity(entity).remove::<Chase>();

                // TODO: Add death
                warn!("Player died!");
            }
        }
    }
}

/// After the [`Investigate`] timer ran out, switch back to [`Idle`].
pub fn investigating_to_idle(mut commands: Commands, query: Query<(Entity, &Investigate)>) {
    for (entity, investigate) in &query {
        if investigate.start.elapsed() >= Duration::from_secs(INVESTIGATING_TIME) {
            commands.entity(entity).remove::<Investigate>();

            commands.entity(entity).insert(Idle::default());
        }
    }
}

/// If [`Path`] is empty, remove the component
pub fn check_empty_path(mut commands: Commands, query: Query<(Entity, &Path)>) {
    for (entity, path) in &query {
        if path.steps.is_empty() {
            commands.entity(entity).remove::<Path>();
        }
    }
}
