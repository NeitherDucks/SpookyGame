use bevy::prelude::*;

use crate::{grid::GridLocation, pathfinding::Path};

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct RunAway {
    pub target: GridLocation,
    pub speed: f32,
    pub path: Path,
}

pub fn run_away(
    mut transform: Query<&mut Transform>,
    chasing: Query<(Entity, &RunAway)>,
    time: Res<Time>,
) {
    // Move to a specified location
    // TODO
}
