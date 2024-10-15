pub use bevy::{prelude::*, render::view::RenderLayers};
pub use bevy_ecs_ldtk::prelude::*;

pub use crate::rendering::PIXEL_PERFECT_LAYERS;

#[derive(Component)]
pub struct InteractibleTriggered {
    pub location: GridCoords,
}

#[derive(Bundle, LdtkEntity)]
pub struct InteractibleBundle {
    render_layer: RenderLayers,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

impl Default for InteractibleBundle {
    fn default() -> Self {
        InteractibleBundle {
            render_layer: PIXEL_PERFECT_LAYERS,
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
        }
    }
}
