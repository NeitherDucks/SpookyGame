use std::time::{Duration, Instant};

use bevy::prelude::*;

use crate::grid::GridLocation;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct Investigate {
    pub target: GridLocation,
    pub start: Instant,
    pub duration: Duration,
    pub range: u32,
}

pub fn investigate(
    mut transform: Query<&mut Transform>,
    chasing: Query<(Entity, &Investigate)>,
    time: Res<Time>,
) {
    // Move to specified location, once reached, pick a new location within range. Do until timer runs out.
    // TODO
}
