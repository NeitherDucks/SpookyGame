use std::collections::HashMap;

use bevy::prelude::*;
use bevy_dev_tools::states::log_transitions;

use crate::animated_sprite::{animate_sprite, Animations};
use crate::states::{GameState, PlayingState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PlayingState>()
            .insert_resource(Animations(HashMap::new()))
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(OnEnter(PlayingState::Loading), load)
            .add_systems(OnEnter(PlayingState::IntroScene), intro_scene_setup)
            .add_systems(
                Update,
                intro_scene_update.run_if(in_state(PlayingState::IntroScene)),
            )
            .add_systems(Update, log_transitions::<PlayingState>)
            .add_systems(Update, animate_sprite);
    }
}

fn setup(mut next_state: ResMut<NextState<PlayingState>>) {
    // Extra setup if needed

    next_state.set(PlayingState::Loading);
}

pub fn load(mut next_state: ResMut<NextState<PlayingState>>) {
    // Wait for everything to load

    // Trigger intro scene
    next_state.set(PlayingState::IntroScene);
}

pub fn intro_scene_setup(mut next_state: ResMut<NextState<PlayingState>>) {
    // Setup necessary stuff for the intro_scene

    // For now, skip to the next state
    // This will change if I have time to add the intro cut scene
    next_state.set(PlayingState::Playing);
}

pub fn intro_scene_update() {}
