pub use bevy::{prelude::*, render::view::RenderLayers};
pub use bevy_ecs_ldtk::prelude::*;

pub use crate::rendering::PIXEL_PERFECT_LAYERS;

#[derive(Bundle, LdtkEntity)]
pub struct HiddingSpotBundle {
    render_layer: RenderLayers,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

impl Default for HiddingSpotBundle {
    fn default() -> Self {
        HiddingSpotBundle {
            render_layer: PIXEL_PERFECT_LAYERS,
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
        }
    }
}
