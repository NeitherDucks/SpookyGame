use bevy::{math::bounding::Bounded2d, prelude::*};

use crate::{
    collisions::{test_collision, Collider, ColliderOffset, ColliderShape},
    config::PLAYER_SPEED,
    rendering::InGameCamera,
    states::{GameState, PlayingState},
};

#[derive(Component, Default)]
pub struct PlayerTag;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Playing), cleanup)
            .add_systems(
                Update,
                (setup_camera, move_player).run_if(in_state(PlayingState::Playing)),
            )
            .add_systems(
                Update,
                escape_pressed
                    .run_if(in_state(PlayingState::Playing).or_else(in_state(PlayingState::Pause))),
            );
    }
}

fn setup_camera(
    mut commands: Commands,
    player: Query<Entity, Added<PlayerTag>>,
    camera: Query<Entity, With<InGameCamera>>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    let Ok(camera) = camera.get_single() else {
        return;
    };

    // Attach camera to player
    commands.entity(player).push_children(&[camera]);
}

fn cleanup(
    mut commands: Commands,
    player: Query<Entity, With<PlayerTag>>,
    camera: Query<Entity, With<InGameCamera>>,
) {
    let Ok(camera) = camera.get_single() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    commands.entity(player).remove_children(&[camera]);
}

fn move_player(
    mut player: Query<(&mut Transform, &ColliderShape, &ColliderOffset), With<PlayerTag>>,
    colliders: Query<&Collider, Without<PlayerTag>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut player, shape, offset)) = player.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }

    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();

    if move_delta.length() > 0.0 {
        let new_position = player.translation.xy() + move_delta + offset.0;

        let mut collide: bool = false;

        // Obviously very slow, need some space partitioning algo like Quadtree, or KD-tree to query only things near the player.
        // But should be fine for this small game
        for collider2 in colliders.iter() {
            collide |= match shape {
                ColliderShape::Circle(c) => test_collision(
                    &Collider::Circle(c.bounding_circle(new_position, 0.)),
                    collider2,
                ),
                &ColliderShape::Rectangle(r) => {
                    test_collision(&Collider::Rectangle(r.aabb_2d(new_position, 0.)), collider2)
                }
            };
        }

        if !collide {
            player.translation += move_delta.extend(0.);
        }
    }
}

fn escape_pressed(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<PlayingState>>,
    mut next_state: ResMut<NextState<PlayingState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        if *current_state.get() == PlayingState::Playing {
            next_state.set(PlayingState::Pause);
        } else if *current_state.get() == PlayingState::Pause {
            next_state.set(PlayingState::Playing);
        }
    }
}
