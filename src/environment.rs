use bevy::prelude::*;

use crate::states::{GameState, PlayingState};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingState::Loading), setup)
            .add_systems(OnExit(GameState::Playing), cleanup);
    }
}

fn setup() {}

fn cleanup() {}
