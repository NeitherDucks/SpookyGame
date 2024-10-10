use std::time::Instant;

use bevy::prelude::*;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct Idle {
    pub start: Instant,
}

impl Default for Idle {
    fn default() -> Self {
        Idle {
            start: Instant::now(),
        }
    }
}
