use bevy::prelude::*;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct Chase {
    pub target: Entity,
    pub speed: f32,
}

pub fn chase(
    mut transform: Query<&mut Transform>,
    chasing: Query<(Entity, &Chase)>,
    time: Res<Time>,
) {
    for (entity, chase) in &chasing {
        let target_translation = transform.get(chase.target).unwrap().translation;
        let follow_transform = &mut transform.get_mut(entity).unwrap();
        let follow_translation = follow_transform.translation;

        follow_transform.translation += (target_translation - follow_translation)
            .normalize_or_zero()
            * chase.speed
            * time.delta_seconds();
    }
}
