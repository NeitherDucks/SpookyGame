use bevy::{math::bounding::BoundingCircle, prelude::*};

use crate::{
    animated_sprite::{AnimatedSprite, AnimationIndices, AnimationTimer, Animations},
    collider::{Collider, ColliderOffset},
    rendering::PIXEL_PERFECT_LAYERS,
    states::{GameState, PlayingState},
};

#[derive(Component)]
pub struct EnvironmentTag;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingState::Loading), setup)
            .add_systems(OnExit(GameState::Playing), cleanup);
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
    commands.spawn((
        AnimatedSprite {
            sprite: SpriteBundle {
                texture: player_texture,
                transform: Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
                ..default()
            },
            atlas: TextureAtlas {
                layout: player_texture_atlas_layout,
                index: player_idle_animation_indices.first,
            },
            animation: *animations.0.get("player_idle").unwrap(),
            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        },
        ColliderOffset::ZERO,
        Collider::Circle(BoundingCircle {
            center: Vec2::new(0.0, 50.0),
            circle: Circle { radius: 16.0 },
        }),
        EnvironmentTag,
        PIXEL_PERFECT_LAYERS,
    ));
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<EnvironmentTag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
