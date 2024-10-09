use std::time::{Duration, Instant};

// Very simple "AI"
// Execute different Actions based on which component exists on the Entity
use bevy::prelude::*;

mod chase;
pub mod idle;
mod investigate;
mod run_away;
mod wander;

use chase::{chase, Chase};
use idle::Idle;
use investigate::{investigate, Investigate};
use run_away::{run_away, RunAway};
use wander::{wander, Wander};

use crate::{
    enemies::EnemyTag, grid::GridLocation, pathfinding::Path, player::PlayerTag,
    states::PlayingState,
};

const PLAYER_VISIBLE_DISTANCE_INVESTIGATOR: f32 = 5. * 16.;
const PLAYER_VISIBLE_DISTANCE_VILLAGERS: f32 = 3. * 16.;
const KILLING_DISTANCE: f32 = 1. * 16.;

const NORMAL_SPEED: f32 = 75.0;
const RUNNING_SPEED: f32 = 90.0;
const CHASE_SPEED: f32 = 110.0;

const INVESTIGATING_RADIUS: u32 = 10;
const INVESTIGATING_TIME: u64 = 10;

const WANDERING_RADIUS: u32 = 32;

const IDLING_TIME: u64 = 5;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
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
                update_player_position_chase,
            )
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            Update,
            (chase, investigate, run_away, wander).run_if(in_state(PlayingState::Playing)),
        );
    }
}

// IMPROVEME: Probably not a good idea, since it has to check every frame
fn nothing_to_idle(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            Added<EnemyTag>,
            Without<Idle>,
            Without<Investigate>,
            Without<Wander>,
        ),
    >,
) {
    for entity in &query {
        commands.entity(entity).insert(Idle {
            duration: Duration::from_secs(IDLING_TIME),
            start: Instant::now(),
        });
    }
}

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

            // TODO: Check if player is roughly in front
            if transform
                .translation
                .xy()
                .distance(player_transform.translation.xy())
                < distance_threshold
            {
                let mut entity_cmd = commands.entity(entity);

                // Removing inexisting component seems fine (nothing is screaming at me)
                entity_cmd.remove::<Idle>();
                entity_cmd.remove::<Investigate>();
                entity_cmd.remove::<Wander>();

                match tag {
                    // TODO: Pathfind to player
                    EnemyTag::Investigator => {
                        entity_cmd.insert(Chase {
                            target: player,
                            speed: CHASE_SPEED,
                            path: Path::default(),
                        });
                    }
                    EnemyTag::Villager => {
                        // TODO: Choose proper target to run away and pathfind to it
                        // IMPROVEME: Run away to the nearest investigator, which will trigger them to investigate where the player was seen
                        entity_cmd.insert(RunAway {
                            target: GridLocation::new(5, 5),
                            speed: RUNNING_SPEED,
                            path: Path::default(),
                        });
                    }
                }
            }
        }
    }
}

fn update_player_position_chase(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Chase)>,
    player: Query<(Entity, &Transform), (With<PlayerTag>, Changed<Transform>)>,
) {
    // If player is still visible, update pathfinding
    // TODO
}

fn idle_to_wandering(mut commands: Commands, query: Query<(Entity, &Idle)>) {
    for (entity, idle) in &query {
        if idle.start.elapsed() >= idle.duration {
            // TODO: Pick random location and pathfind to it
            commands.entity(entity).remove::<Idle>();
            commands.entity(entity).insert(Wander::default());
        }
    }
}

fn wandering_to_idle(mut commands: Commands, query: Query<(Entity, &Wander)>) {
    for (entity, wander) in &query {
        if wander.path.steps.is_empty() {
            commands.entity(entity).remove::<Wander>();
            commands.entity(entity).insert(Idle {
                duration: Duration::from_secs(IDLING_TIME),
                start: Instant::now(),
            });
        }
    }
}

fn run_away_to_idle(mut commands: Commands, query: Query<(Entity, &RunAway)>) {
    for (entity, run_away) in &query {
        if run_away.path.steps.is_empty() {
            commands.entity(entity).remove::<Wander>();
            commands.entity(entity).insert(Idle {
                duration: Duration::from_secs(IDLING_TIME),
                start: Instant::now(),
            });
        }
    }
}

fn idle_or_wandering_to_investigating(
    mut commands: Commands,
    query: Query<(Entity, AnyOf<(&Idle, &Wander)>)>,
) {
    // If the player triggered an interactible nearby, go investigate.
}

fn chasing_to_investigating(mut commands: Commands, query: Query<(Entity, &Chase)>) {
    // If lost visual on player during chase, go investigate last known location.
}

fn chasing_to_killing(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Chase>>,
    player: Query<(Entity, &Transform), With<PlayerTag>>,
) {
    // If within range of the player, trigger death and end the run.
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
            }
        }
    }
}

fn investigating_to_idle(mut commands: Commands, query: Query<(Entity, &Investigate)>) {
    // After theh investigation timer ran out, switch back to idle.
    for (entity, investigate) in &query {
        if investigate.start.elapsed() >= investigate.duration {
            commands.entity(entity).remove::<Investigate>();
            commands.entity(entity).insert(Idle {
                start: Instant::now(),
                duration: Duration::from_secs(IDLING_TIME),
            });
        }
    }
}
