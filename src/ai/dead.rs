use bevy::prelude::*;

use crate::{
    game_mode::Score,
    ldtk::{
        animation::new_animation,
        entities::{AnimationConfig, EnemyTag},
    },
};

use super::{Idle, RunAway, TalkToInvestigator, Wander, VILLAGER_ANIMATION_DEATH};

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Dead;

pub fn dead_on_enter(
    mut commands: Commands,
    query: Query<(Entity, &EnemyTag, &AnimationConfig), Added<Dead>>,
    mut score: ResMut<Score>,
) {
    for (entity, tag, animation) in &query {
        // Only villagers should have the Dead state.
        if *tag != EnemyTag::Villager {
            commands.entity(entity).remove::<Dead>();
            continue;
        }

        score.villager_killed();

        // Remove any states the villager might be in.
        commands
            .entity(entity)
            .remove::<Idle>()
            .remove::<Wander>()
            .remove::<RunAway>()
            .remove::<TalkToInvestigator>();

        // Remove any interaction prompt or interaction sensors.
        commands.entity(entity).despawn_descendants();

        // Play death animation
        commands.entity(entity).insert(new_animation(
            VILLAGER_ANIMATION_DEATH.with_offset(animation.get_offset()),
        ));
    }
}
