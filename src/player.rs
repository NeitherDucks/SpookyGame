use bevy::prelude::*;
use bevy_rapier2d::{plugin::RapierContext, prelude::*};

use crate::{
    ai::Chased,
    config::PLAYER_SPEED,
    ldtk::entities::{
        hidding_spot::HiddingSpotTag, player::PlayerTag, Aim, InteractibleEntityRef,
        InteractibleSpotTag, InteractionPossible,
    },
    rendering::InGameCamera,
    states::{GameState, PlayingState},
};

#[derive(Component)]
pub struct PlayerIsHidding(pub Entity);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Playing), cleanup)
            .add_systems(
                Update,
                (
                    setup_camera,
                    move_player,
                    spacebar_pressed_hidding,
                    player_is_chased,
                )
                    .run_if(in_state(PlayingState::Playing)),
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
    mut player: Query<
        &mut KinematicCharacterController,
        (With<PlayerTag>, Without<PlayerIsHidding>),
    >,
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

fn spacebar_pressed_hidding(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            Option<&InteractionPossible>,
            Option<&PlayerIsHidding>,
        ),
        (With<PlayerTag>, Without<Chased>),
    >,
    transforms: Query<
        (&Transform, &InteractibleEntityRef),
        (
            Or<(With<HiddingSpotTag>, With<InteractibleSpotTag>)>,
            Without<PlayerTag>,
        ),
    >,
) {
    if input.just_pressed(KeyCode::Space) {
        if let Ok((entity, mut transform, interaction, hidding)) = query.get_single_mut() {
            // If already hidding
            if let Some(hidding) = hidding {
                // Get exit position
                if let Ok((exit_transform, _)) = transforms.get(hidding.0) {
                    // Move player to exit
                    // IMPROVEME: Tweening between positions
                    *transform = *exit_transform;
                    // Change collision to Fixed and remove PlayerIsHidding tag.
                    commands
                        .entity(entity)
                        .insert(RigidBody::Fixed)
                        .remove::<PlayerIsHidding>();
                }
            } else if let Some(interaction) = interaction {
                // Get position of hiding spot
                if let Ok((target_transform, reference)) = transforms.get(interaction.entity) {
                    // Change player collision to KinematicPosition, so it stops colliding and add PlayerIsHidding tag.
                    commands
                        .entity(entity)
                        .insert(RigidBody::KinematicPositionBased)
                        .insert(PlayerIsHidding(reference.0));

                    // Move player to hidding spot
                    // IMPROVEME: Tweening between positions
                    *transform = *target_transform;
                }
            }
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

pub fn player_is_chased(
    mut commands: Commands,
    player: Query<Entity, (With<PlayerTag>, Added<Chased>)>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    commands.entity(player).remove::<InteractionPossible>();
}

pub fn is_player_visible(
    player: Entity,
    other: Entity,
    player_location: Vec2,
    other_location: Vec2,
    other_aim: Aim,
    max_distance: f32,
    max_angle: f32,
    rapier_context: &Res<RapierContext>,
    gizmos: &mut Gizmos,
) -> bool {
    let max_angle = max_angle.to_radians();
    // Check if player is within range.
    gizmos.arc_2d(
        other_location,
        other_aim.0.to_angle() - 90f32.to_radians(),
        max_angle * 2.,
        max_distance,
        Color::srgb(1.0, 1.0, 1.0),
    );

    if other_location.distance(player_location) < max_distance {
        let dir = (player_location - other_location).normalize();

        // Check if player is within field of view.
        if other_aim.0.angle_between(dir).abs() < max_angle {
            // Check of player is not behind wall.
            let filter = QueryFilter::exclude_dynamic()
                .exclude_sensors()
                .exclude_rigid_body(other);

            gizmos.line_2d(other_location, player_location, Color::srgb(1.0, 1.0, 0.0));

            let result =
                rapier_context.cast_ray(other_location, dir, max_distance + 8., true, filter);

            if let Some((entity, rio)) = result {
                let color = match entity == player {
                    true => Color::srgb(1.0, 0.0, 1.0),
                    false => Color::srgb(1.0, 1.0, 0.0),
                };

                let hit_pos = other_location + (dir * rio);
                gizmos.circle_2d(hit_pos, 5.0, color);

                return entity == player;
            }
        }
    }

    false
}
