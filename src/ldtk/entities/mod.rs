pub mod collision_tile;
pub mod dead_player;
pub mod hidding_spot;
pub mod interactible;
pub mod investigator;
pub mod noise_maker;
pub mod player;
pub mod player_respawn_point;
pub mod villager;

use bevy_ecs_ldtk::{prelude::LdtkFields, EntityIid, EntityInstance, LdtkIntCell};
use bevy_rapier2d::prelude::*;

pub use collision_tile::CollisionTileBundle;
pub use hidding_spot::*;
pub use interactible::*;
pub use investigator::*;
pub use noise_maker::*;
pub use player::*;
pub use player_respawn_point::*;
pub use villager::*;

// IMPROVEME
// Could have a list of possible Entity interactions and counters.
// And a Key press to switch.
#[derive(Component, Reflect)]
pub struct InteractionPossible {
    pub entity: Entity,
    pub counter: u32,
    pub interactibe_type: InteractibleTag,
}

#[derive(Component)]
pub struct ShowInteractionButtonTag;

#[derive(Reflect, Component, PartialEq, Eq)]
pub enum EnemyTag {
    Investigator,
    Villager,
}

#[derive(Component, Clone, Copy)]
pub struct Aim(pub Vec2);

impl Default for Aim {
    fn default() -> Self {
        Aim(Vec2::new(1., 0.))
    }
}

#[derive(Clone, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl Default for ColliderBundle {
    fn default() -> Self {
        ColliderBundle {
            collider: Collider::cuboid(8., 8.),
            rigid_body: RigidBody::Fixed,
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            density: ColliderMassProperties::default(),
            gravity_scale: GravityScale::default(),
            velocity: Velocity::default(),
        }
    }
}

#[derive(Debug, Default, Deref, DerefMut, Component)]
pub struct UnresolvedEntityRef(Option<EntityIid>);

impl UnresolvedEntityRef {
    pub fn from_ref_field(entity_instance: &EntityInstance) -> UnresolvedEntityRef {
        UnresolvedEntityRef(
            entity_instance
                .get_maybe_entity_ref_field("entity")
                .expect("expected entity to have an entity reference field")
                .as_ref()
                .map(|entity_ref| EntityIid::new(entity_ref.entity_iid.clone())),
        )
    }
}

#[derive(Debug, Deref, DerefMut, Component, Reflect)]
pub struct InteractibleEntityRef(pub Entity);

pub fn resolve_entity_references(
    mut commands: Commands,
    unresolved: Query<(Entity, &UnresolvedEntityRef), Added<UnresolvedEntityRef>>,
    ldtk_entities: Query<(Entity, &EntityIid)>,
) {
    for (entity, unresolved_ref) in &unresolved {
        if let Some(ref_iid) = unresolved_ref.0.as_ref() {
            let (ref_entity, _) = ldtk_entities
                .iter()
                .find(|(_, iid)| *iid == ref_iid)
                .expect("Reference entity should exist");

            commands
                .entity(entity)
                .insert(InteractibleEntityRef(ref_entity));
        }
        commands.entity(entity).remove::<UnresolvedEntityRef>();
    }
}
