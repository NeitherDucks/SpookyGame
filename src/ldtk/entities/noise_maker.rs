pub use bevy::{prelude::*, render::view::RenderLayers};
pub use bevy_ecs_ldtk::prelude::*;

pub use crate::rendering::PIXEL_PERFECT_LAYERS;

use super::InteractibleSpotTag;

#[derive(Component)]
pub struct NoiseMakerTriggered {
    pub location: GridCoords,
}

// IMPROVEME
// Could have a list of possible Entity interactions and counters.
// And a Key press to switch.
#[derive(Component, Reflect)]
pub struct InteractionPossible {
    pub entity: Entity,
    pub counter: u32,
}

#[derive(Bundle, LdtkEntity)]
pub struct NoiseMakerBundle {
    tag: InteractibleSpotTag,
    render_layer: RenderLayers,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

impl Default for NoiseMakerBundle {
    fn default() -> Self {
        NoiseMakerBundle {
            tag: InteractibleSpotTag::NoiseMaker,
            render_layer: PIXEL_PERFECT_LAYERS,
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
        }
    }
}
