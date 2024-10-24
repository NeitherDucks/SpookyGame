use bevy::{prelude::*, render::view::RenderLayers};

use crate::rendering::PIXEL_PERFECT_LAYERS;

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct DeadPlayerTag;

#[derive(Bundle)]
pub struct DeadPlayerBundle {
    render_layer: RenderLayers,
    tag: DeadPlayerTag,
    sprite_bundle: SpriteBundle,
}

impl DeadPlayerBundle {
    pub fn new(transform: &Transform, texture: Handle<Image>) -> Self {
        Self {
            render_layer: PIXEL_PERFECT_LAYERS,
            tag: DeadPlayerTag,
            sprite_bundle: SpriteBundle {
                transform: *transform,
                texture,
                ..Default::default()
            },
        }
    }
}
