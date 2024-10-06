use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

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
