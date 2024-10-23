use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_rand::prelude::{GlobalEntropy, WyRand};

use crate::{
    config::{NORMAL_SPEED, WANDERING_RADIUS},
    grid::{Grid, Tile},
    ldtk::{animation::new_animation, entities::EnemyTag},
    pathfinding::Path,
};

use super::{MovementSpeed, INVESTIGATOR_ANIMATION_WALK, VILLAGER_ANIMATION_WALK};

#[derive(Clone, Default, Component)]
#[component(storage = "SparseSet")]
pub struct Wander;

/// When [`Wander`] is added, generate a target and a [`Path`].
pub fn wander_on_enter(
    mut commands: Commands,
    query: Query<(Entity, &GridCoords, &EnemyTag), Added<Wander>>,
    grid: Res<Grid<Tile>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for (entity, coords, tag) in &query {
        if let Ok(target) = grid.find_nearby(&coords, WANDERING_RADIUS, rng.as_mut()) {
            if let Ok(path) = grid.path_to(&coords, &target) {
                commands.entity(entity).insert(path);
                commands.entity(entity).insert(MovementSpeed(NORMAL_SPEED));

                commands.entity(entity).insert(new_animation(match tag {
                    EnemyTag::Investigator => INVESTIGATOR_ANIMATION_WALK,
                    EnemyTag::Villager => VILLAGER_ANIMATION_WALK,
                }));
            }
        }
    }
}

/// When [`Wander`] is removed, remove any [`Path`] and [`MovementSpeed`].
pub fn wander_on_exit(mut commands: Commands, mut query: RemovedComponents<Wander>) {
    for entity in query.read() {
        commands.entity(entity).remove::<Path>();
        commands.entity(entity).remove::<MovementSpeed>();
    }
}
