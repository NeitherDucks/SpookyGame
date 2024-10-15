pub use bevy::prelude::*;
pub use bevy_ecs_ldtk::prelude::*;

pub use crate::{grid::*, ldtk::entities::*};

#[derive(Bundle, LdtkIntCell)]
pub struct CollisionTileBundle {
    tile: Tile,
    collider: ColliderBundle,
}

impl Default for CollisionTileBundle {
    fn default() -> Self {
        CollisionTileBundle {
            tile: Tile,
            collider: ColliderBundle::default(),
        }
    }
}
