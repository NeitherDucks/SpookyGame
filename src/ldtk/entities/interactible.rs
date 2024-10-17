use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{ColliderBundle, UnresolvedEntityRef};

#[derive(Debug, Component, Reflect, Copy, Clone)]
pub enum InteractibleTag {
    HiddingSpot,
    NoiseMaker,
    Villager,
}

impl InteractibleTag {
    pub fn from_str(s: &str) -> InteractibleTag {
        match s {
            "HiddingSpot" => InteractibleTag::HiddingSpot,
            "NoiseMaker" => InteractibleTag::NoiseMaker,
            "Villager" => InteractibleTag::Villager,
            _ => InteractibleTag::NoiseMaker,
        }
    }

    pub fn from_field(entity_instance: &EntityInstance) -> InteractibleTag {
        InteractibleTag::from_str(
            entity_instance
                .get_enum_field("interaction_type")
                .expect("Expected entity to have non-nullable interaction_type enum field."),
        )
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct InteractibleBundle {
    #[with(InteractibleTag::from_field)]
    tag: InteractibleTag,
    collider: ColliderBundle,
    sensor: Sensor,
    active_events: ActiveEvents,
    active_collisions: ActiveCollisionTypes,
    #[with(UnresolvedEntityRef::from_ref_field)]
    unresolved_ref: UnresolvedEntityRef,
}

impl Default for InteractibleBundle {
    fn default() -> Self {
        InteractibleBundle {
            tag: InteractibleTag::HiddingSpot,
            collider: ColliderBundle {
                collider: Collider::cuboid(7., 7.),
                ..Default::default()
            },
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
            active_collisions: ActiveCollisionTypes::STATIC_STATIC,
            unresolved_ref: UnresolvedEntityRef::default(),
        }
    }
}
