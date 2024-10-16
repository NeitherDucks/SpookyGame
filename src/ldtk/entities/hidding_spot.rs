pub use bevy::{prelude::*, render::view::RenderLayers};
pub use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ldtk::{ColliderBundle, UnresolvedEntityRef};
pub use crate::rendering::PIXEL_PERFECT_LAYERS;

use super::InteractibleSpotTag;

#[derive(Component)]
pub struct HiddingSpotTag;

#[derive(Bundle, LdtkEntity)]
pub struct HiddingSpotBundle {
    render_layer: RenderLayers,
    tag: HiddingSpotTag,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[with(UnresolvedEntityRef::from_ref_field)]
    unresolved_ref: UnresolvedEntityRef,
}

impl Default for HiddingSpotBundle {
    fn default() -> Self {
        HiddingSpotBundle {
            render_layer: PIXEL_PERFECT_LAYERS,
            tag: HiddingSpotTag,
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
            unresolved_ref: UnresolvedEntityRef::default(),
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct HiddingSpotInteractionBundle {
    tag: InteractibleSpotTag,
    collider: ColliderBundle,
    sensor: Sensor,
    active_events: ActiveEvents,
    active_collisions: ActiveCollisionTypes,
    #[with(UnresolvedEntityRef::from_ref_field)]
    unresolved_ref: UnresolvedEntityRef,
    render_layer: RenderLayers,
}

impl Default for HiddingSpotInteractionBundle {
    fn default() -> Self {
        HiddingSpotInteractionBundle {
            tag: InteractibleSpotTag::HiddingSpot,
            collider: ColliderBundle {
                collider: Collider::cuboid(7., 7.),
                ..Default::default()
            },
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
            active_collisions: ActiveCollisionTypes::STATIC_STATIC,
            unresolved_ref: UnresolvedEntityRef::default(),
            render_layer: PIXEL_PERFECT_LAYERS,
        }
    }
}
