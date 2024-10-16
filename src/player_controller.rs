use bevy::prelude::*;
use bevy_rapier2d::{plugin::RapierContext, prelude::*};

use crate::{
    ai::{Chased, Dead},
    config::{INVESTIGATOR_HEARING_RANGE, NOISE_MAKER_ANIMATION, PLAYER_SPEED},
    ldtk::{
        animation::new_animation,
        entities::{
            hidding_spot::HiddingSpotExit, player::PlayerTag, Aim, EnemyTag, InteractibleEntityRef,
            InteractibleTag, InteractionPossible, NoiseMakerInvestigateTarget,
            NoiseMakerReTriggerable, NoiseMakerTriggerable, NoiseMakerTriggered,
        },
    },
    rendering::InGameCamera,
    states::{GameState, PlayingState},
};

#[derive(Component)]
pub struct PlayerIsHidding(pub Vec2);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Playing), cleanup)
            .add_systems(
                Update,
                (
                    setup_camera,
                    move_player,
                    spacebar_pressed,
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

fn spacebar_pressed(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<
        (
            Entity,
            &mut Transform,
            Option<&InteractionPossible>,
            Option<&PlayerIsHidding>,
        ),
        (With<PlayerTag>, Without<Chased>),
    >,
    enemies: Query<(Entity, &Transform, &EnemyTag), Without<PlayerTag>>,
    hidding_spots: Query<
        (&Transform, &InteractibleEntityRef, &HiddingSpotExit),
        Without<PlayerTag>,
    >,
    noise_makers: Query<
        (
            Entity,
            &Transform,
            &NoiseMakerInvestigateTarget,
            Option<&NoiseMakerReTriggerable>,
        ),
        (With<NoiseMakerTriggerable>, Without<PlayerTag>),
    >,
) {
    // If the Space bar was just pressed
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    // and we can query the player
    let Ok((player, mut player_transform, player_interaction, player_hidding)) =
        player.get_single_mut()
    else {
        return;
    };

    // If already hidding
    if let Some(hidding) = player_hidding {
        // Get exit position
        // Move player to exit
        // IMPROVEME: Tweening between positions
        *player_transform = Transform::from_translation(hidding.0.extend(0.));
        // Change collision to Fixed and remove PlayerIsHidding tag.
        commands
            .entity(player)
            .insert(RigidBody::Fixed)
            .remove::<PlayerIsHidding>();
    } else {
        // If there is a possible interaction
        let Some(interaction) = player_interaction else {
            return;
        };

        match interaction.interactibe_type {
            InteractibleTag::HiddingSpot => {
                // If we can get the position of hiding spot from the interaction
                let Ok((hidding_spot_transform, _, exit_location)) =
                    hidding_spots.get(interaction.entity)
                else {
                    return;
                };

                // Change player collision to KinematicPosition, so it stops colliding and add PlayerIsHidding tag.
                commands
                    .entity(player)
                    .insert(RigidBody::KinematicPositionBased)
                    .insert(PlayerIsHidding(exit_location.0));

                // Move player to hidding spot
                // IMPROVEME: Tweening between positions
                *player_transform = *hidding_spot_transform;
            }
            InteractibleTag::NoiseMaker => {
                // If we can get the linked noise maker from the interaction.
                let Ok((
                    noise_maker,
                    noise_maker_transform,
                    noise_maker_investigate_coords,
                    noise_maker_retrigger,
                )) = noise_makers.get(interaction.entity)
                else {
                    return;
                };

                // Store the noise maker's location in 2d
                let noise_maker_location = noise_maker_transform.translation.xy();

                // Trigger animation on noise maker entity
                // IMPROVEME: There is probably a better place for this
                commands
                    .entity(interaction.entity)
                    .insert(new_animation(NOISE_MAKER_ANIMATION));

                // If the noise maker can't be re-triggered, remove the triggerable component.
                if noise_maker_retrigger.is_none() {
                    commands
                        .entity(noise_maker)
                        .remove::<NoiseMakerTriggerable>();
                }

                // Add NoiseTriggered to all Investigators in range
                for (investigator, transform, tag) in &enemies {
                    if *tag == EnemyTag::Investigator {
                        if noise_maker_location.distance(transform.translation.xy())
                            <= INVESTIGATOR_HEARING_RANGE
                        {
                            commands
                                .entity(investigator)
                                .insert(NoiseMakerTriggered(noise_maker_investigate_coords.0));
                        }
                    }
                }
            }
            InteractibleTag::Villager => {
                // Set dead state (this also handle animation and cleanup).
                commands.entity(interaction.entity).insert(Dead);

                // Add points to player?

                // Play player kill animation
                // TODO
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
