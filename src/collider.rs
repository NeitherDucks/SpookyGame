use bevy::prelude::*;

pub enum ColliderShape {
    Circle,
    Square,
}

#[derive(Component)]
pub struct Collider {
    pub shape: ColliderShape,
    pub center: Vec2,
    pub extent: f32,
}
