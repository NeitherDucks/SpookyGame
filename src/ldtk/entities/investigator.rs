pub use bevy::{prelude::*, render::view::RenderLayers};
pub use bevy_ecs_ldtk::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

pub use crate::{
    config::*,
    ldtk::{
        animation::{AnimationConfig, AnimationTimer},
        entities::*,
    },
    rendering::PIXEL_PERFECT_LAYERS,
};
use crate::{
    ldtk::{animation::AnimationOffset, EnemyLights, Light},
    rendering::LIGHTS_LAYERS,
    utils::remap_rand_f32,
};

use super::{Aim, EnemyTag};

#[derive(Bundle, LdtkEntity)]
pub struct InvestigatorBundle {
    collider: ColliderBundle,
    animation: AnimationConfig,
    animation_offset: AnimationOffset,
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
            animation_offset: AnimationOffset::default(),
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

pub fn investigator_added(
    mut commands: Commands,
    query: Query<(Entity, &EnemyTag), Added<EnemyTag>>,
    lights: Res<EnemyLights>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for (entity, tag) in &query {
        if *tag != EnemyTag::Investigator {
            continue;
        }

        // Add light
        let light = commands
            .spawn((
                SpriteBundle {
                    texture: lights.investigator_light.clone(),
                    transform: Transform::from_translation(Vec3::new(48., 0., 0.)),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: lights.atlas.clone(),
                    index: remap_rand_f32(rng.next_u32(), 0., 4.) as usize,
                },
                Light,
                LIGHTS_LAYERS,
            ))
            .id();

        commands.entity(entity).add_child(light);
    }
}
