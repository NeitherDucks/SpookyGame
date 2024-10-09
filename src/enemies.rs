use bevy::prelude::*;

use crate::{
    collisions::{ColliderOffset, ColliderShape},
    config::GRID_SIZE,
    environment::Tile,
    grid::GridLocation,
    rendering::PIXEL_PERFECT_LAYERS,
    states::{GameState, PlayingState},
};

#[derive(Component)]
pub enum EnemyTag {
    Investigator,
    Villager,
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingState::Loading), setup)
            .add_systems(OnExit(GameState::Playing), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemy_texture: Handle<Image> = asset_server.load("2d/enemy_placeholder.png");

    commands.spawn((
        SpriteBundle {
            texture: enemy_texture,
            transform: Transform::from_translation(Vec3::new(
                (GRID_SIZE / 2) as f32 * 16.,
                (GRID_SIZE / 2) as f32 * 16.,
                0.,
            )),
            ..default()
        },
        GridLocation::new(10, 10),
        ColliderShape::Circle(Circle { radius: 8.0 }),
        ColliderOffset::ZERO,
        PIXEL_PERFECT_LAYERS,
        Tile,
        Name::new("Enemy"),
        EnemyTag::Investigator,
    ));
}

fn cleanup() {}
