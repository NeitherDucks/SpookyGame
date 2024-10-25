// use std::time::Instant;

use bevy::prelude::*;

use crate::ldtk::{animation::new_animation, entities::EnemyTag};

use super::{IDLING_TIME, INVESTIGATOR_ANIMATION_IDLE, VILLAGER_ANIMATION_IDLE};

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct Idle {
    // pub start: Instant,
    pub timer: Timer,
}

impl Default for Idle {
    fn default() -> Self {
        Idle {
            // start: Instant::now(),
            timer: Timer::from_seconds(IDLING_TIME as f32, TimerMode::Once),
        }
    }
}

pub fn idle_update(mut query: Query<&mut Idle>, time: Res<Time>) {
    for mut idle in &mut query {
        idle.timer.tick(time.delta());
    }
}

pub fn idle_on_enter(mut commands: Commands, query: Query<(Entity, &EnemyTag), Added<Idle>>) {
    for (entity, tag) in &query {
        commands.entity(entity).insert(new_animation(match tag {
            EnemyTag::Investigator => INVESTIGATOR_ANIMATION_IDLE,
            EnemyTag::Villager => VILLAGER_ANIMATION_IDLE,
        }));
    }
}
