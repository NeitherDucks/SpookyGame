use bevy_rapier2d::prelude::KinematicCharacterController;

pub use bevy::{prelude::*, render::view::RenderLayers};
pub use bevy_ecs_ldtk::prelude::*;

pub use crate::{
    config::*,
    ldtk::{
        animation::{AnimationConfig, AnimationTimer},
        entities::*,
    },
    rendering::PIXEL_PERFECT_LAYERS,
};

#[derive(Reflect, Clone, Component, Default)]
#[reflect(Component)]
pub struct PlayerTag;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    collider: ColliderBundle,
    active_collisions: ActiveCollisionTypes,
    controller: KinematicCharacterController,
    animation: AnimationConfig,
    animation_timer: AnimationTimer,
    tag: PlayerTag,
    render_layer: RenderLayers,
    name: Name,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            collider: ColliderBundle {
                collider: Collider::cuboid(5.0, 5.0),
                ..Default::default()
            },
            active_collisions: ActiveCollisionTypes::STATIC_STATIC,
            controller: KinematicCharacterController::default(),
            animation: PLAYER_ANIMATION_IDLE,
            animation_timer: AnimationTimer::new(PLAYER_ANIMATION_IDLE),
            tag: PlayerTag,
            render_layer: PIXEL_PERFECT_LAYERS,
            name: Name::new("Player"),
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
            grid_coords: GridCoords::default(),
        }
    }
}
