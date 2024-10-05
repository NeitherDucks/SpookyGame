use bevy::prelude::*;

use crate::animated_sprite::*;
use crate::player::*;
use crate::states::PlayingState;

pub fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<PlayingState>>,
) {
    // Spawn environment

    // Spawn player
    let texture: Handle<Image> = asset_server.load("2d/player_placeholder.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 1, last: 4 };

    let player = commands
        .spawn((
            SpriteBundle {
                texture,
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
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
