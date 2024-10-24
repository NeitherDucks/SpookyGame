use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game_mode::Score;

#[derive(Reflect, Clone, Component, Default)]
#[reflect(Component)]
pub struct PlayerRespawnPointTag;

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerRespawnPointBundle {
    transform: TransformBundle,
    tag: PlayerRespawnPointTag,
}

pub fn on_respawn_point_added(
    mut score: ResMut<Score>,
    query: Query<Entity, Added<PlayerRespawnPointTag>>,
) {
    for _ in &query {
        score.player_respawn_point_added();
    }
}
