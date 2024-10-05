use bevy::prelude::*;

use crate::states::PlayingState;

pub fn intro_scene_setup(mut commands: Commands, mut next_state: ResMut<NextState<PlayingState>>) {
    // Setup necessary stuff for the intro_scene

    // For now, skip to the next state
    // This will change if I have time to add the intro cut scene
    next_state.set(PlayingState::Playing);
}

pub fn intro_scene_update(mut commands: Commands) {}
