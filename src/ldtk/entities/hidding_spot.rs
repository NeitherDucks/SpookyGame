pub use bevy::{prelude::*, render::view::RenderLayers};
pub use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::ldtk_grid_coords_to_translation;

use crate::ldtk::GRID_SIZE;
pub use crate::rendering::PIXEL_PERFECT_LAYERS;

use super::TILE_SIZE;

#[derive(Reflect, Clone, Component, Default)]
#[reflect(Component)]
pub struct HiddingSpotExit(pub Vec2);

#[derive(Bundle, LdtkEntity)]
pub struct HiddingSpotBundle {
    render_layer: RenderLayers,
    #[with(exit_from_field)]
    exit: HiddingSpotExit,
}

impl Default for HiddingSpotBundle {
    fn default() -> Self {
        HiddingSpotBundle {
            render_layer: PIXEL_PERFECT_LAYERS,
            exit: HiddingSpotExit::default(),
        }
    }
}

fn exit_from_field(entity_instance: &EntityInstance) -> HiddingSpotExit {
    let point = entity_instance
        .get_point_field("exit")
        .expect("expected entity to have an exit tile field");

    HiddingSpotExit(ldtk_grid_coords_to_translation(
        *point,
        GRID_SIZE.y,
        TILE_SIZE,
    ))
}
