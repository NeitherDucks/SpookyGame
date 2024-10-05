use bevy::prelude::*;

use crate::states::PlayingState;

pub fn load(mut commands: Commands, mut next_state: ResMut<NextState<PlayingState>>) {
    // Spawn environment

    // Spawn player

    // Trigger cut scene // If enough time

    // Onto next state
    next_state.set(PlayingState::IntroScene);
}
