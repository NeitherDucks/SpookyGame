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
pub struct InvestigatorBundle {
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

impl Default for InvestigatorBundle {
    fn default() -> Self {
        InvestigatorBundle {
            collider: ColliderBundle::default(),
            animation: INVESTIGATOR_ANIMATION_IDLE,
            animation_timer: AnimationTimer::new(INVESTIGATOR_ANIMATION_IDLE),
            tag: EnemyTag::Investigator,
            render_layer: PIXEL_PERFECT_LAYERS,
            name: Name::new("Investigator"),
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
            grid_coords: GridCoords::default(),
            aim: Aim::default(),
        }
    }
}
