use bevy::prelude::*;
use bevy_dev_tools::states::log_transitions;

use crate::states::{GameState, PlayingState};

#[derive(Default, Resource)]
pub struct Score {
    total_villagers: u8,
    villagers_killed: u8,
}

impl Score {
    pub fn villager_spawned(&mut self) {
        self.total_villagers += 1;
    }

    pub fn villager_killed(&mut self) {
        self.villagers_killed += 1;
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PlayingState>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(OnEnter(PlayingState::Loading), load)
            .add_systems(OnEnter(PlayingState::IntroScene), intro_scene_setup)
            .add_systems(
                Update,
                intro_scene_update.run_if(in_state(PlayingState::IntroScene)),
            )
            .add_systems(Update, log_transitions::<PlayingState>);
    }
}

fn setup(mut commands: Commands, mut next_state: ResMut<NextState<PlayingState>>) {
    // Extra setup if needed
    commands.init_resource::<Score>();

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
