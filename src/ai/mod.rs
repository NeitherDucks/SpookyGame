use std::time::{Duration, Instant};

// Very simple "AI"
// Execute different Actions based on which component exists on the Entity
use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

mod chase;
pub mod idle;
mod investigate;
mod run_away;
mod wander;

use chase::{chase_on_enter, chase_on_exit, chase_update, Chase};
use idle::Idle;
use investigate::{investigate_on_enter, investigate_on_exit, investigate_update, Investigate};
use run_away::{run_away_on_enter, run_away_on_exit, RunAway};
use wander::{wander_on_enter, wander_on_exit, Wander};

use crate::{
    config::{
        CHASE_SPEED, IDLING_TIME, INVESTIGATING_TIME, KILLING_DISTANCE, NORMAL_SPEED,
        PLAYER_VISIBLE_DISTANCE_INVESTIGATOR, PLAYER_VISIBLE_DISTANCE_VILLAGERS, RUNNING_SPEED,
    },
    enemies::EnemyTag,
    grid::GridLocation,
    interactibles::InteractibleTriggered,
    pathfinding::Path,
    player::PlayerTag,
    states::PlayingState,
};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct AiTransition;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct AiOnEnter;

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
                nothing_to_idle,
                notice_player,
                idle_to_wandering,
                wandering_to_idle,
                idle_or_wandering_to_investigating,
                chasing_to_investigating,
                chasing_to_killing,
                investigating_to_idle,
                run_away_to_idle,
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
            )
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            Update,
            (chase_update, investigate_update, follow_path).run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            PostUpdate,
            check_empty_path.run_if(in_state(PlayingState::Playing)),
        )
        .register_type::<Chase>();
    }
}

/// When Enemies are created, add a default [`Idle`] task.
fn nothing_to_idle(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            Added<EnemyTag>,
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
    query: Query<(
        Entity,
        &Transform,
        &EnemyTag,
        AnyOf<(&Idle, &Investigate, &Wander)>,
    )>,
    player: Query<(Entity, &Transform), With<PlayerTag>>,
) {
    if let Ok((player, player_transform)) = player.get_single() {
        for (entity, transform, tag, _) in &query {
            let distance_threshold = match tag {
                EnemyTag::Investigator => PLAYER_VISIBLE_DISTANCE_INVESTIGATOR,
                EnemyTag::Villager => PLAYER_VISIBLE_DISTANCE_VILLAGERS,
            };

            // TODO: Check if player is roughly in front.
            if transform
                .translation
                .xy()
                .distance(player_transform.translation.xy())
                < distance_threshold
            {
                let mut entity_cmd = commands.entity(entity);

                // Removing inexisting component seems fine (nothing is screaming at me).
                entity_cmd.remove::<Idle>();
                entity_cmd.remove::<Investigate>();
                entity_cmd.remove::<Wander>();

                // Get player position on grid, if fails, idles.
                if let Some(player_grid_position) =
                    GridLocation::from_world(player_transform.translation.xy())
                {
                    match tag {
                        // If Enemy is an Investigator, chase the player.
                        EnemyTag::Investigator => {
                            entity_cmd.insert(Chase {
                                target: player,
                                player_last_seen: player_grid_position,
                            });
                        }
                        // If enemy is a Villager, run away from player.
                        EnemyTag::Villager => {
                            // TODO: Choose proper target to run away to and pathfind to it
                            entity_cmd.insert(RunAway {
                                player_last_seen: player_grid_position,
                            });
                        }
                    }
                } else {
                    entity_cmd.insert(Idle::default());
                    continue;
                };
            }
        }
    }
}

/// After the timer for [`Idle`] expires swtich to [`Idle`].
fn idle_to_wandering(mut commands: Commands, query: Query<(Entity, &Idle)>) {
    for (entity, idle) in &query {
        if idle.start.elapsed() >= Duration::from_secs(IDLING_TIME) {
            // TODO: Pick random location and pathfind to it
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
            target: interactible.location.clone(),
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
                <= KILLING_DISTANCE
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
fn follow_path(mut paths: Query<(&mut Transform, &mut Path, &MovementSpeed)>, time: Res<Time>) {
    for (mut transform, mut path, speed) in &mut paths {
        if let Some(next_target) = path.steps.front() {
            let delta = next_target.to_world() - transform.translation.xy();
            let travel_amount = time.delta_seconds() * speed.0;

            if delta.length() > travel_amount * 1.1 {
                let direction = delta.normalize_or_zero().extend(0.) * travel_amount;
                transform.translation += direction;
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
