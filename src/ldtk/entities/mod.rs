pub mod collision_tile;
pub mod enemies;
pub mod hidding_spot;
pub mod interactible;
pub mod player;

use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkIntCell;
use bevy_rapier2d::prelude::*;

pub use collision_tile::CollisionTileBundle;
pub use enemies::*;
pub use hidding_spot::HiddingSpotBundle;
pub use interactible::InteractibleBundle;
pub use player::PlayerBundle;

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
