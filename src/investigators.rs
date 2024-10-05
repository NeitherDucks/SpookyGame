use bevy::prelude::*;

use crate::states::{GameState, PlayingState};

pub struct InvestigatorsPlugin;

impl Plugin for InvestigatorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingState::Loading), setup)
            .add_systems(OnExit(GameState::Playing), cleanup);
    }
}

fn setup() {}

fn cleanup() {}
