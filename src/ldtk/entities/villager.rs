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
    game_mode::Score,
    ldtk::{animation::AnimationOffset, EnemyLights, Light, VillagerSpritesheetHandles},
    rendering::LIGHTS_LAYERS,
    utils::remap_rand_f32,
};

use super::{Aim, EnemyTag};

#[derive(Bundle, LdtkEntity)]
pub struct VillagerBundle {
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

impl Default for VillagerBundle {
    fn default() -> Self {
        VillagerBundle {
            collider: ColliderBundle::default(),
            animation: VILLAGER_ANIMATION_IDLE,
            animation_offset: AnimationOffset::default(),
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

pub fn villager_added(
    mut commands: Commands,
    query: Query<(Entity, &EnemyTag), Added<EnemyTag>>,
    mut score: ResMut<Score>,
    villagers_handles: Res<VillagerSpritesheetHandles>,
    lights: Res<EnemyLights>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for (entity, tag) in &query {
        if *tag != EnemyTag::Villager {
            continue;
        }

        // Randomize Villager spritesheet
        let index: usize =
            remap_rand_f32(rng.next_u32(), 0., villagers_handles.0.len() as f32 - 1.) as usize;
        if let Some(handle) = villagers_handles.0.get(index) {
            commands.entity(entity).insert(handle.clone());
        }

        // Add light
        let light = commands
            .spawn((
                SpriteBundle {
                    texture: lights.villager_light.clone(),
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

        score.villager_spawned();

        let new_child = commands
            .spawn((
                TransformBundle::default(),
                InteractibleTag::Villager,
                ColliderBundle {
                    collider: Collider::cuboid(8. * 3., 8. * 3.),
                    ..Default::default()
                },
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::STATIC_STATIC,
                InteractibleEntityRef(entity),
            ))
            .id();

        commands.entity(entity).add_child(new_child);
    }
}
