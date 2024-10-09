use bevy::prelude::*;

use crate::{grid::GridLocation, pathfinding::Path};

#[derive(Clone, Default, Component)]
#[component(storage = "SparseSet")]
pub struct Wander {
    pub target: GridLocation,
    pub speed: f32,
    pub path: Path,
}

pub fn wander(
    mut transform: Query<&mut Transform>,
    chasing: Query<(Entity, &Wander)>,
    time: Res<Time>,
) {
    // Move to target.
    // TODO
}
