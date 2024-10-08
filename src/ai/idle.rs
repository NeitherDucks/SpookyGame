use std::time::{Duration, Instant};

use bevy::prelude::*;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct Idle {
    pub start: Instant,
    pub duration: Duration,
}
