// Very simple "AI"
// Execute different Actions based on which component exists on the Entity
use std::time::{Duration, Instant};

use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};
use bevy_ecs_ldtk::{utils::grid_coords_to_translation, GridCoords};

mod chase;
mod idle;
mod investigate;
mod run_away;
mod talk_to_investigator;
mod wander;

use bevy_rapier2d::plugin::RapierContext;
use chase::*;
use idle::*;
use investigate::*;
use run_away::*;
use talk_to_investigator::*;
use wander::*;

use crate::{
    config::*,
    ldtk::entities::{Aim, EnemyTag, InteractibleTriggered},
    pathfinding::Path,
    player::{is_player_visible, PlayerTag},
    states::PlayingState,
};

// All the logic for transitioning from different Tasks will be executed during this schedule.
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct AiTransition;

// All logic when entering a task will be executed during this schedule.
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct AiOnEnter;

// All logic when leaving a task will be executed during this schedule.
// This is executed before AiOnEnter, so we can clean up nicely and not interfer with components added in the AiOnEnter phase.
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct AiOnExit;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementSpeed(f32);

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(AiTransition);
        app.init_schedule(AiOnEnter);
        app.init_schedule(AiOnExit);
        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_before(Update, AiTransition);
        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_before(Update, AiOnExit);
        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_before(Update, AiOnEnter);

        app.add_systems(
            AiTransition,
            (
                notice_player,
                idle_to_wandering,
                wandering_to_idle,
                idle_or_wandering_to_investigating,
                chasing_to_investigating,
                chasing_to_killing,
                investigating_to_idle,
                run_away_to_idle,
                running_away_to_talk_to_investigator,
                talk_to_investigator_to_running_away,
            )
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            AiOnExit,
            (
                chase_on_exit,
                investigate_on_exit,
                run_away_on_exit,
                wander_on_exit,
                talk_to_investigator_on_exit,
            )
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            AiOnEnter,
            (
                chase_on_enter,
                investigate_on_enter,
                run_away_on_enter,
                wander_on_enter,
                talk_to_investigator_on_enter,
            )
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            Update,
            (
                chase_update,
                investigate_update,
                talk_to_investigator_update,
                follow_path,
            )
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            PostUpdate,
            (check_empty_path, nothing_to_idle).run_if(in_state(PlayingState::Playing)),
        )
        .register_type::<Chase>();
    }
}

/// Default [`Idle`] if no AI taks found for enemy entity.
fn nothing_to_idle(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            With<EnemyTag>,
            Without<Chase>,
            Without<Idle>,
            Without<Investigate>,
            Without<RunAway>,
            Without<Wander>,
        ),
    >,
) {
    for entity in &query {
        commands.entity(entity).insert(Idle::default());
    }
}

/// In any [`Idle`], [`Investigate`] or [`Wander`], and the player is nearby and in the field of vision of an Enemy, either [`Chase`] or [`RunAway`].
fn notice_player(
    mut commands: Commands,
    transforms: Query<&Transform, (Without<PlayerTag>, Without<EnemyTag>)>,
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
fn running_away_to_talk_to_investigator(
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
fn talk_to_investigator_to_running_away(
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
                    start: Instant::now(),
                    target: talk.player_last_seen,
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
fn idle_to_wandering(mut commands: Commands, query: Query<(Entity, &Idle)>) {
    for (entity, idle) in &query {
        if idle.start.elapsed() >= Duration::from_secs(IDLING_TIME) {
            commands.entity(entity).remove::<Idle>();

            commands.entity(entity).insert(Wander);
        }
    }
}

/// After reaching the [`Wander`].target, switch to [`Idle`].
fn wandering_to_idle(mut commands: Commands, query: Query<Entity, (With<Wander>, Without<Path>)>) {
    for entity in &query {
        commands.entity(entity).remove::<Wander>();

        commands.entity(entity).insert(Idle::default());
    }
}

/// After reaching the [`RunAway`].target, switch to [`Idle`].
fn run_away_to_idle(mut commands: Commands, query: Query<Entity, (With<RunAway>, Without<Path>)>) {
    for entity in &query {
        commands.entity(entity).remove::<RunAway>();

        commands.entity(entity).insert(Idle::default());
    }
}

/// If the player triggered an interactible nearby, go [`Investigate`].
fn idle_or_wandering_to_investigating(
    mut commands: Commands,
    query: Query<(Entity, &InteractibleTriggered), Or<(With<Idle>, With<Wander>)>>,
) {
    for (entity, interactible) in &query {
        commands.entity(entity).remove::<Idle>();
        commands.entity(entity).remove::<Wander>();

        commands.entity(entity).insert(Investigate {
            target: interactible.location,
            start: Instant::now(),
        });
    }
}

/// If lost visual on player during [`Chase`], go [`Investigate`] last known location.
fn chasing_to_investigating(mut commands: Commands, query: Query<(Entity, &Chase), Without<Path>>) {
    for (entity, chase) in &query {
        let last_kown_location = chase.player_last_seen.clone();

        commands.entity(entity).remove::<Chase>();

        commands.entity(entity).insert(Investigate {
            target: last_kown_location,
            start: Instant::now(),
        });
    }
}

/// If within range of the player while [`Chase`], trigger death and end the run.
fn chasing_to_killing(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Chase>>,
    player: Query<(Entity, &Transform), With<PlayerTag>>,
) {
    if let Ok((player, player_transform)) = player.get_single() {
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
fn investigating_to_idle(mut commands: Commands, query: Query<(Entity, &Investigate)>) {
    for (entity, investigate) in &query {
        if investigate.start.elapsed() >= Duration::from_secs(INVESTIGATING_TIME) {
            commands.entity(entity).remove::<Investigate>();

            commands.entity(entity).insert(Idle::default());
        }
    }
}

/// If has a [`Path`], move the entity along.
fn follow_path(
    mut query: Query<(&mut Transform, &mut Path, &MovementSpeed, &mut Aim)>,
    time: Res<Time>,
) {
    for (mut transform, mut path, speed, mut aim) in &mut query {
        if let Some(next_target) = path.steps.front() {
            let delta =
                grid_coords_to_translation(*next_target, TILE_SIZE) - transform.translation.xy();
            let travel_amount = time.delta_seconds() * speed.0;

            if delta.length() > travel_amount * 1.1 {
                let direction = delta.normalize_or_zero();
                let travel = direction.extend(0.) * travel_amount;
                transform.translation += travel;
                aim.0 = direction;
            } else {
                path.steps.pop_front();
            }
        }
    }
}

/// If [`Path`] is empty, remove the component
fn check_empty_path(mut commands: Commands, query: Query<(Entity, &Path)>) {
    for (entity, path) in &query {
        if path.steps.is_empty() {
            commands.entity(entity).remove::<Path>();
        }
    }
}
