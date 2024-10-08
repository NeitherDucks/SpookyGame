use bevy::{
    math::bounding::{Aabb2d, Bounded2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

use crate::states::PlayingState;

#[derive(Component)]
pub enum ColliderShape {
    Circle(Circle),
    Rectangle(Rectangle),
}

#[derive(Component)]
pub enum Collider {
    Circle(BoundingCircle),
    Rectangle(Aabb2d),
}

#[derive(Component)]
pub struct ColliderOffset(pub Vec2);

impl ColliderOffset {
    pub const ZERO: Self = ColliderOffset(Vec2::ZERO);
}

pub fn test_collision(collider1: &Collider, collider2: &Collider) -> bool {
    match collider1 {
        Collider::Circle(c) => match collider2 {
            Collider::Circle(c2) => c.intersects(c2),
            Collider::Rectangle(r) => c.intersects(r),
        },
        &Collider::Rectangle(r) => match collider2 {
            Collider::Circle(c) => r.intersects(c),
            Collider::Rectangle(r2) => r.intersects(r2),
        },
    }
}

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_colliders.run_if(in_state(PlayingState::Playing)),
        );
    }
}

fn update_colliders(
    mut commands: Commands,
    query: Query<
        (Entity, &ColliderShape, &ColliderOffset, &Transform),
        Or<(Changed<Transform>, Added<ColliderShape>)>,
    >,
) {
    for (entity, collider_shape, collider_offset, transform) in query.iter() {
        let translation = transform.translation.xy() + collider_offset.0;
        match collider_shape {
            ColliderShape::Circle(c) => {
                let bounding = c.bounding_circle(translation, 0.);
                commands.entity(entity).insert(Collider::Circle(bounding));
            }
            ColliderShape::Rectangle(s) => {
                let aabb = s.aabb_2d(translation, 0.);
                commands.entity(entity).insert(Collider::Rectangle(aabb));
            }
        }
    }
}
