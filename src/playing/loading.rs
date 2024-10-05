use bevy::prelude::*;

use crate::animated_sprite::*;
use crate::player::*;
use crate::states::PlayingState;

pub fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<PlayingState>>,
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

    // Spawn environment

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
            PlayerTag,
        ))
        .id();

    // Spawn camera
    let camera = commands
        .spawn((Camera2dBundle::default(), PlayerCameraTag))
        .id();

    // Attach camera to player
    commands.entity(player).push_children(&[camera]);

    // Trigger cut scene // If enough time

    // Onto next state
    next_state.set(PlayingState::IntroScene);
}
