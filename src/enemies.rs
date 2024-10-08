use bevy::prelude::*;

use crate::states::{GameState, PlayingState};

#[derive(Component)]
pub enum EnemyTag {
    Investigator,
    Villager,
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingState::Loading), setup)
            .add_systems(OnExit(GameState::Playing), cleanup);
    }
}

fn setup() {}

fn cleanup() {}
