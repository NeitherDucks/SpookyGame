use bevy::prelude::*;

use crate::{
    grid::GridLocation,
    states::{GameState, PlayingState},
};

#[derive(Component)]
pub struct InteractibleTriggered {
    pub location: GridLocation,
}

pub struct InteractiblesPlugin;

impl Plugin for InteractiblesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingState::Loading), setup)
            .add_systems(OnExit(GameState::Playing), cleanup);
    }
}

fn setup() {}

fn cleanup() {}
