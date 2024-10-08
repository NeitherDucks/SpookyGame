// Very simple "Ai"
// Execute different Actions based on which component exists on the Entity

use std::time::Duration;

use bevy::prelude::*;

use crate::{
    enemies::EnemyTag,
    environment::Tile,
    grid::{Grid, GridLocation},
    pathfinding::Path,
    player::PlayerTag,
    states::PlayingState,
};

const PLAYER_VISIBLE_DISTANCE_INVESTIGATOR: f32 = 5.0;
const PLAYER_VISIBLE_DISTANCE_VILLAGERS: f32 = 3.0;

const ENEMY_NORMAL_SPEED: f32 = 75.0;
const ENEMY_CHASE_SPEED: f32 = 150.0;

const INVESTIGATING_RADIUS: u32 = 10;
const INVESTIGATING_TIME: u64 = 10;

const WANDERING_RADIUS: u32 = 32;

const IDLING_TIME: u64 = 5;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_for_player.run_if(in_state(PlayingState::Playing)),
                (action_idle, action_move, action_investigate)
                    .run_if(in_state(PlayingState::Playing)),
                no_action.run_if(in_state(PlayingState::Playing)),
            )
                .chain(),
        );
    }
}

#[derive(Component)]
pub struct ActionIdle(Duration);

#[derive(Component)]
pub struct ActionMove(GridLocation);

#[derive(Component)]
pub struct ActionChase;

#[derive(Component)]
pub struct ActionInvestigate {
    location: GridLocation,
    duration: Duration,
}

fn check_for_player(
    mut command: Commands,
    query: Query<(Entity, &Transform, &EnemyTag)>,
    player: Query<&Transform, With<PlayerTag>>,
) {
    for (entity, entity_transform, enemy_tag) in &query {
        if let Ok(player) = player.get_single() {
            // Check distance to the player
            let allowed_distance = match enemy_tag {
                EnemyTag::Investigator => PLAYER_VISIBLE_DISTANCE_INVESTIGATOR,
                EnemyTag::Villager => PLAYER_VISIBLE_DISTANCE_VILLAGERS,
            };

            if player
                .translation
                .xy()
                .distance(entity_transform.translation.xy())
                <= allowed_distance
            {
                //TODO: if close enough, check if visible, and in front of entity
                let player_location = match GridLocation::from_world(player.translation.xy()) {
                    Some(val) => val,
                    None => {
                        warn!("Player not in grid ...");
                        continue;
                    }
                };

                command
                    .entity(entity)
                    .insert((ActionChase, ActionMove(player_location)));
            }
        }
    }
}

/// Timer for [`ActionIdle`]
fn action_idle(
    mut command: Commands,
    mut query: Query<
        (Entity, &Transform, &mut ActionIdle),
        (Without<ActionChase>, Without<ActionInvestigate>),
    >,
    grid: Res<Grid<Tile>>,
    time: Res<Time>,
) {
    for (entity, transform, mut idle) in &mut query {
        idle.0 -= time.delta();

        if idle.0.is_zero() {
            command.entity(entity).remove::<ActionIdle>();

            if let Some(location) = GridLocation::from_world(transform.translation.xy()) {
                let nearby = grid.find_nearby(&location, WANDERING_RADIUS);
                command.entity(entity).insert(ActionMove(nearby));
            }
        }
    }
}

/// Handle the movement logic for AIs
fn action_move(
    mut command: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &ActionMove,
        Option<&mut Path>,
        Option<&ActionChase>,
        Option<&ActionInvestigate>,
    )>,
    grid: Res<Grid<Tile>>,
    time: Res<Time>,
) {
    for (entity, mut transform, action, path, chasing, investigating) in &mut query {
        let chasing = chasing.is_some();
        let move_speed = match chasing {
            true => ENEMY_CHASE_SPEED,
            false => ENEMY_NORMAL_SPEED,
        };

        if let Some(mut path) = path {
            if let Some(target) = path.steps.pop_front() {
                let location = target.to_world();
                let direction = transform.translation.xy() - location;
                transform.translation +=
                    (direction.normalize_or_zero() * move_speed * time.delta_seconds()).extend(0.);

                if path.steps.is_empty() {
                    command.entity(entity).remove::<ActionMove>();
                    command.entity(entity).remove::<Path>();

                    match chasing {
                        true => {
                            command.entity(entity).insert(ActionInvestigate {
                                location: target,
                                duration: Duration::from_secs(INVESTIGATING_TIME),
                            });
                        }
                        false => match investigating {
                            Some(investigating) => {
                                let nearby =
                                    grid.find_nearby(&investigating.location, INVESTIGATING_RADIUS);
                                command.entity(entity).insert(ActionMove(nearby));
                            }
                            None => {
                                command
                                    .entity(entity)
                                    .insert(ActionIdle(Duration::from_secs(IDLING_TIME)));
                            }
                        },
                    }
                }
            };
        } else {
            let current_location = match GridLocation::from_world(transform.translation.xy()) {
                Some(val) => val,
                None => {
                    warn!("Entity not in grid...");
                    continue;
                }
            };

            let path = grid.path_to(&current_location, &action.0);
            if let Ok(mut path) = path {
                if let Some(target) = path.steps.pop_front() {
                    let target = target.to_world();
                    let direction = transform.translation.xy() - target;
                    transform.translation +=
                        (direction.normalize_or_zero() * move_speed * time.delta_seconds())
                            .extend(0.);
                };

                if !path.steps.is_empty() {
                    command.entity(entity).insert(path);
                }
            } else {
                warn!("Could not find a path for entity.");
                continue;
            }
        }
    }
}

/// Tick the 'timer' for [`ActionInvestigate`]
fn action_investigate(
    mut command: Commands,
    mut query: Query<(Entity, &mut ActionInvestigate)>,
    time: Res<Time>,
) {
    for (entity, mut investigating) in &mut query {
        investigating.duration -= time.delta();

        if investigating.duration.is_zero() {
            command.entity(entity).remove::<ActionInvestigate>();
        }
    }
}

fn no_action(
    mut command: Commands,
    query: Query<
        Entity,
        (
            Without<ActionChase>,
            Without<ActionIdle>,
            Without<ActionInvestigate>,
            Without<ActionMove>,
        ),
    >,
) {
    for entity in &query {
        command
            .entity(entity)
            .insert(ActionIdle(Duration::from_secs(IDLING_TIME)));
    }
}
