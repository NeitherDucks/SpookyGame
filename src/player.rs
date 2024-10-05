use bevy::prelude::*;

use crate::{
    animated_sprite::{AnimatedSprite, AnimationIndices, AnimationTimer, Animations},
    collider::{Collider, ColliderShape},
    states::{GameState, PlayingState},
};

const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct PlayerTag;

#[derive(Component)]
pub struct PlayerCameraTag;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingState::Loading), setup)
            .add_systems(OnExit(GameState::Playing), cleanup)
            .add_systems(Update, move_player.run_if(in_state(PlayingState::Playing)));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Animations>,
) {
    // Load Textures and Animations
    let player_texture: Handle<Image> = asset_server.load("2d/player_placeholder.png");
    let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
    let player_texture_atlas_layout = texture_atlas_layouts.add(player_layout);

    let player_idle_animation_indices = AnimationIndices { first: 0, last: 0 };
    let player_movment_animation_indices = AnimationIndices { first: 0, last: 3 };

    animations
        .0
        .insert("player_idle".to_string(), player_idle_animation_indices);
    animations.0.insert(
        "player_movement".to_string(),
        player_movment_animation_indices,
    );

    // Spawn player
    let player = commands
        .spawn((
            AnimatedSprite {
                sprite: SpriteBundle {
                    texture: player_texture,
                    ..default()
                },
                atlas: TextureAtlas {
                    layout: player_texture_atlas_layout,
                    index: player_idle_animation_indices.first,
                },
                animation: *animations.0.get("player_idle").unwrap(),
                timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            },
            Collider {
                shape: ColliderShape::Circle,
                center: Vec2::ZERO,
                extent: 32.0,
            },
            PlayerTag,
        ))
        .id();

    // Spawn camera
    let camera = commands
        .spawn((Camera2dBundle::default(), PlayerCameraTag))
        .id();

    // Attach camera to player
    commands.entity(player).push_children(&[camera]);
}

fn cleanup(
    mut commands: Commands,
    player: Query<Entity, With<PlayerTag>>,
    camera: Query<Entity, With<PlayerCameraTag>>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    commands.entity(player).despawn_recursive();

    let Ok(camera) = camera.get_single() else {
        return;
    };

    commands.entity(camera).despawn_recursive();
}

fn move_player(
    mut player: Query<&mut Transform, With<PlayerTag>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut player) = player.get_single_mut() else {
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
    player.translation += move_delta.extend(0.);
}
