use bevy::prelude::*;

use crate::pathfinding::Path;

#[derive(Clone, Default, Component)]
#[component(storage = "SparseSet")]
pub struct Wander {
    pub path: Path,
}
