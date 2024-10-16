pub use bevy::{prelude::*, render::view::RenderLayers};
pub use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::ldtk_grid_coords_to_translation;

use crate::ldtk::{UnresolvedEntityRef, GRID_SIZE};
pub use crate::rendering::PIXEL_PERFECT_LAYERS;

use super::TILE_SIZE;

#[derive(Default, Component)]
pub struct HiddingSpotExit(pub Vec2);

#[derive(Bundle, LdtkEntity)]
pub struct HiddingSpotBundle {
    render_layer: RenderLayers,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[with(UnresolvedEntityRef::from_ref_field)]
    unresolved_ref: UnresolvedEntityRef,
    #[with(exit_from_field)]
    exit: HiddingSpotExit,
}

impl Default for HiddingSpotBundle {
    fn default() -> Self {
        HiddingSpotBundle {
            render_layer: PIXEL_PERFECT_LAYERS,
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
            unresolved_ref: UnresolvedEntityRef::default(),
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
