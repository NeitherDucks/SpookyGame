use bevy::prelude::*;

const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct PlayerTag;

#[derive(Component)]
pub struct PlayerCameraTag;

pub fn move_player(
    mut player: Query<&mut Transform, With<PlayerTag>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }

    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
    player.translation += move_delta.extend(0.);
}
