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

pub fn test_collision(
    position1: Vec2,
    collider1: &Collider,
    position2: Vec2,
    collider2: &Collider,
) -> bool {
    let p1 = position1 + collider1.center;
    let p2 = position2 + collider2.center;

    match collider1.shape {
        ColliderShape::Circle => match collider2.shape {
            ColliderShape::Circle => p1.distance(p2) <= collider1.extent + collider2.extent,
            ColliderShape::Square => {
                let coll_pos = p1 + (p1 - p2).normalize() * collider1.extent;
                coll_pos.x < p2.x + collider2.extent
                    && coll_pos.x > p2.x - collider2.extent
                    && coll_pos.y < p2.y + collider2.extent
                    && coll_pos.y > p2.y - collider2.extent
            }
        },
        ColliderShape::Square => match collider2.shape {
            ColliderShape::Circle => {
                let coll_pos = p2 + (p2 - p1).normalize() * collider2.extent;
                coll_pos.x < p1.x + collider1.extent
                    && coll_pos.x > p1.x - collider1.extent
                    && coll_pos.y < p1.y + collider1.extent
                    && coll_pos.y > p1.y - collider1.extent
            }
            ColliderShape::Square => {
                p1.x < p2.x + collider2.extent
                    && p1.x + collider1.extent > p2.x
                    && p1.y < p2.y + collider2.extent
                    && p1.y + collider1.extent > p2.y
            }
        },
    }
}
