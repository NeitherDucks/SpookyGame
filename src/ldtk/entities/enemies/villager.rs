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

use super::{Aim, EnemyTag};

#[derive(Bundle, LdtkEntity)]
pub struct VillagerBundle {
    collider: ColliderBundle,
    animation: AnimationConfig,
    animation_timer: AnimationTimer,
    tag: EnemyTag,
    render_layer: RenderLayers,
    name: Name,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    aim: Aim,
}

impl Default for VillagerBundle {
    fn default() -> Self {
        VillagerBundle {
            collider: ColliderBundle::default(),
            animation: VILLAGER_ANIMATION_IDLE,
            animation_timer: AnimationTimer::new(VILLAGER_ANIMATION_IDLE),
            tag: EnemyTag::Villager,
            render_layer: PIXEL_PERFECT_LAYERS,
            name: Name::new("Villager"),
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
            grid_coords: GridCoords::default(),
            aim: Aim::default(),
        }
    }
}
