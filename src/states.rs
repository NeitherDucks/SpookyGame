use bevy::prelude::*;

// Different states of the game.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
#[allow(dead_code)]
pub enum GameState {
    #[default]
    MainMenu,
    PauseMenu,
    Playing,
}

// Different states of the playing the game.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Playing)]
#[allow(dead_code)]
pub enum PlayingState {
    #[default]
    Setup,
    Loading,
    IntroScene,
    Playing,
    Death,
    Respawning,
    Win,
    Lose,
}
