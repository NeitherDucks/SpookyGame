use std::time::Instant;

use bevy::prelude::*;

use crate::ldtk::{
    animation::new_animation,
    entities::{AnimationConfig, EnemyTag},
};

use super::{INVESTIGATOR_ANIMATION_IDLE, VILLAGER_ANIMATION_IDLE};

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

pub fn idle_on_enter(
    mut commands: Commands,
    query: Query<(Entity, &AnimationConfig, &EnemyTag), Added<Idle>>,
) {
    for (entity, animation, tag) in &query {
        commands.entity(entity).insert(new_animation(
            match tag {
                EnemyTag::Investigator => INVESTIGATOR_ANIMATION_IDLE,
                EnemyTag::Villager => VILLAGER_ANIMATION_IDLE,
            }
            .with_offset(animation.get_offset()),
        ));
    }
}
