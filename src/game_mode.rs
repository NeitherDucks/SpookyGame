use bevy::prelude::*;
use bevy_dev_tools::states::log_transitions;

use crate::states::{GameState, PlayingState};

#[derive(Resource)]
pub struct Score {
    total_villagers: u8,
    villagers_killed: u8,
    player_lives: u8,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            total_villagers: 0,
            villagers_killed: 0,
            player_lives: 1,
        }
    }
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
            .add_systems(OnEnter(GameState::Reset), reset)
            .add_systems(
                Update,
                intro_scene_update.run_if(in_state(PlayingState::IntroScene)),
            )
            .add_systems(
                Update,
                check_win_lose_condition.run_if(in_state(PlayingState::Playing)),
            )
            .add_systems(Update, log_transitions::<PlayingState>);
    }
}

fn setup(mut commands: Commands, mut next_state: ResMut<NextState<PlayingState>>) {
    // Extra setup if needed
    commands.init_resource::<Score>();

    next_state.set(PlayingState::Loading);
}

/// Most of the loading happens in [`ldtk::setup()`].
/// But this can also be used if needed.
/// Don't forget to add it to the [`AssetLoading`] resource.
fn load() {
    // Wait for everything to load
}

fn intro_scene_setup(mut next_state: ResMut<NextState<PlayingState>>) {
    // Setup necessary stuff for the intro_scene

    // For now, skip to the next state
    // This will change if I have time to add the intro cut scene
    next_state.set(PlayingState::Playing);
}

fn intro_scene_update() {}

fn check_win_lose_condition(score: Res<Score>, mut next_state: ResMut<NextState<PlayingState>>) {
    // score.total_villagers != 0 is a cheap way of not triggering the win condition before everthing is setup
    // IMPROVEME: Proper loading flow, so that everything is setup (especially LDtk stuff) before switching to PlayingState::Playing
    if score.villagers_killed == score.total_villagers && score.total_villagers != 0 {
        next_state.set(PlayingState::Win);
        return;
    }

    if score.player_lives == 0 {
        next_state.set(PlayingState::Lose);
    }
}

fn reset(mut score: ResMut<Score>, mut next_state: ResMut<NextState<GameState>>) {
    *score = Score::default();

    next_state.set(GameState::Playing);
}
