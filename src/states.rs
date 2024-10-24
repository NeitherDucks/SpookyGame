use bevy::prelude::*;

// Different states of the game.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Playing,
    Reset,
}

// Different states of the playing the game.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates, Reflect)]
#[source(GameState = GameState::Playing)]
pub enum PlayingState {
    #[default]
    Setup,
    Loading,
    IntroScene,
    Playing,
    Pause,
    Death,
    Respawning,
    Win,
    Lose,
}
