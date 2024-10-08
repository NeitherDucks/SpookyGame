use std::time::{Duration, Instant};

// Very simple "Ai"
// Execute different Actions based on which component exists on the Entity
use bevy::{ecs::query, prelude::*};

mod chase;
pub mod idle;
mod investigate;
mod run_away;
mod wander;

use chase::Chase;
use idle::Idle;
use investigate::Investigate;
use wander::Wander;

use crate::{enemies::EnemyTag, player::PlayerTag, states::PlayingState};

const PLAYER_VISIBLE_DISTANCE_INVESTIGATOR: f32 = 5. * 16.;
const PLAYER_VISIBLE_DISTANCE_VILLAGERS: f32 = 3.0;

const NORMAL_SPEED: f32 = 75.0;
const CHASE_SPEED: f32 = 150.0;

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
                anything_to_chase,
                idle_to_wandering,
                wandering_to_idle,
            )
                .run_if(in_state(PlayingState::Playing)),
        );
    }
}

// Probably not a good idea, since it had to check every frame
// IMPROVEME
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

fn anything_to_chase(
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
            if transform
                .translation
                .xy()
                .distance(player_transform.translation.xy())
                < PLAYER_VISIBLE_DISTANCE_INVESTIGATOR
            {
                let mut entity_cmd = commands.entity(entity);

                // Removing inexisting component seems fine (nothing is screaming at me)
                entity_cmd.remove::<Idle>();
                entity_cmd.remove::<Investigate>();
                entity_cmd.remove::<Wander>();

                match tag {
                    EnemyTag::Investigator => {
                        entity_cmd.insert(Chase {
                            target: player,
                            speed: CHASE_SPEED,
                        });
                    }
                    EnemyTag::Villager => {
                        // TODO
                    }
                }
            }
        }
    }
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
