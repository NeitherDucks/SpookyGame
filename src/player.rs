use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;

use crate::{
    collisions::{test_ray, Collider},
    config::PLAYER_SPEED,
    ldtk::entities::Aim,
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
    mut player: Query<&mut KinematicCharacterController, With<PlayerTag>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut controller) = player.get_single_mut() else {
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

    controller.translation = Some(move_delta);
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

pub fn is_player_visible(
    player_location: Vec2,
    other_location: Vec2,
    other_aim: Aim,
    max_distance: f32,
    max_angle: f32,
    player_collider: &Collider,
) -> bool {
    // Check if player is withing range.
    if other_location.distance(player_location) < max_distance {
        let dir = (other_location - player_location).normalize();

        // Check if player is roughly in front.
        if other_aim.0.dot(dir) < max_angle.cos() {
            // Check of player is not behind wall.
            if let Ok(dir) = Dir2::new(dir) {
                let ray = Ray2d {
                    origin: other_location,
                    direction: dir,
                };

                return !test_ray(
                    ray,
                    other_location.distance(player_location) * 1.1,
                    player_collider,
                );
            }
        }
    }

    false
}
