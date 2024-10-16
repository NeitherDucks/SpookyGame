pub use bevy::{prelude::*, render::view::RenderLayers};
pub use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::ldtk_grid_coords_to_grid_coords;

pub use crate::rendering::PIXEL_PERFECT_LAYERS;

use super::{player::PlayerTag, InteractibleEntityRef, InteractionPossible, GRID_SIZE};

#[derive(Component)]
pub struct NoiseMakerTriggered(pub GridCoords);

#[derive(Component)]
pub struct NoiseMakerTriggerable;

#[derive(Component)]
pub struct NoiseMakerReTriggerable;

#[derive(Default, Component)]
pub struct NoiseMakerInvestigateTarget(pub GridCoords);

#[derive(Bundle, LdtkEntity)]
pub struct NoiseMakerBundle {
    render_layer: RenderLayers,
    triggerable: NoiseMakerTriggerable,
    #[with(target_from_field)]
    investigate_target: NoiseMakerInvestigateTarget,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

impl Default for NoiseMakerBundle {
    fn default() -> Self {
        NoiseMakerBundle {
            render_layer: PIXEL_PERFECT_LAYERS,
            triggerable: NoiseMakerTriggerable,
            investigate_target: NoiseMakerInvestigateTarget::default(),
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
        }
    }
}

pub fn noise_maker_trigger_removed(
    mut commands: Commands,
    mut noise_makers: RemovedComponents<NoiseMakerTriggerable>,
    query: Query<(Entity, &InteractibleEntityRef)>,
    player: Query<(Entity, &InteractionPossible), With<PlayerTag>>,
) {
    for noise_maker in noise_makers.read() {
        // Remove any Interactible linked to this Noise Maker.
        for (entity, reference) in &query {
            if reference.0 == noise_maker {
                commands.entity(entity).despawn_recursive();
            }
        }

        // Rmove any InteractionPossible linked to this Noise Maker.
        for (entity, interaction) in &player {
            if interaction.entity == noise_maker {
                commands.entity(entity).remove::<InteractionPossible>();
            }
        }

        // Remove any interaction indicator
        commands.entity(noise_maker).despawn_descendants();
    }
}

pub fn target_from_field(entity_instance: &EntityInstance) -> NoiseMakerInvestigateTarget {
    NoiseMakerInvestigateTarget(ldtk_grid_coords_to_grid_coords(
        *entity_instance
            .get_point_field("investigate")
            .expect("Except to have a investigate point field"),
        GRID_SIZE.y,
    ))
}
