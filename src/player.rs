use bevy::{math::bounding::Bounded2d, prelude::*};

use crate::{
    animated_sprite::{AnimatedSprite, AnimationIndices, AnimationTimer, Animations},
    collisions::{test_collision, Collider, ColliderOffset, ColliderShape},
    environment::Tile,
    grid::GridLocation,
    rendering::{InGameCamera, PIXEL_PERFECT_LAYERS},
    states::{GameState, PlayingState},
};

const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct PlayerTag;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingState::Loading), setup)
            .add_systems(OnExit(GameState::Playing), cleanup)
            .add_systems(Update, move_player.run_if(in_state(PlayingState::Playing)))
            .add_systems(
                Update,
                escape_pressed
                    .run_if(in_state(PlayingState::Playing).or_else(in_state(PlayingState::Pause))),
            );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Animations>,
    camera: Query<Entity, With<InGameCamera>>,
) {
    // Load Textures and Animations
    let player_texture: Handle<Image> = asset_server.load("2d/player_placeholder.png");
    // let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
    // let player_texture_atlas_layout = texture_atlas_layouts.add(player_layout);

    // let player_idle_animation_indices = AnimationIndices { first: 0, last: 0 };
    // let player_movment_animation_indices = AnimationIndices { first: 0, last: 3 };

    // animations
    //     .0
    //     .insert("player_idle".to_string(), player_idle_animation_indices);
    // animations.0.insert(
    //     "player_movement".to_string(),
    //     player_movment_animation_indices,
    // );

    // // Spawn player
    // let player = commands
    //     .spawn((
    //         AnimatedSprite {
    //             sprite: SpriteBundle {
    //                 texture: player_texture,
    //                 ..default()
    //             },
    //             atlas: TextureAtlas {
    //                 layout: player_texture_atlas_layout,
    //                 index: player_idle_animation_indices.first,
    //             },
    //             animation: *animations.0.get("player_idle").unwrap(),
    //             timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    //         },
    //         ColliderShape::Circle(Circle { radius: 16.0 }),
    //         ColliderOffset::ZERO,
    //         PlayerTag,
    //         PIXEL_PERFECT_LAYERS,
    //     ))
    //     .id();

    let player = commands
        .spawn((
            SpriteBundle {
                texture: player_texture,
                transform: Transform::from_translation(Vec3::new(32., 32., 0.)),
                ..default()
            },
            ColliderShape::Circle(Circle { radius: 8.0 }),
            ColliderOffset::ZERO,
            PlayerTag,
            PIXEL_PERFECT_LAYERS,
            Name::new("Player"),
        ))
        .id();

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
    commands.entity(player).despawn_recursive();
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
