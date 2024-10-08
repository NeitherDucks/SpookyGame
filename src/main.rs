mod ai;
mod animated_sprite;
mod collider;
mod enemies;
mod environment;
mod game_mode;
mod grid;
mod interactibles;
mod main_menu;
mod pathfinding;
mod pause_menu;
mod player;
mod rendering;
mod states;

use bevy::prelude::*;
use bevy_dev_tools::states::log_transitions;
use enemies::EnemiesPlugin;
use environment::{EnvironmentPlugin, Tile};
use game_mode::GamePlugin;
use grid::GridPlugin;
use interactibles::InteractiblesPlugin;
use pause_menu::PauseMenuPlugin;
use player::PlayerPlugin;
use rendering::RenderingPlugin;

use crate::main_menu::MainMenuPlugin;
use crate::states::GameState;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RenderingPlugin,
            GridPlugin::<Tile>::default(),
            MainMenuPlugin,
            PauseMenuPlugin,
            GamePlugin,
            EnvironmentPlugin,
            InteractiblesPlugin,
            EnemiesPlugin,
            PlayerPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Update, log_transitions::<GameState>)
        .run();
}
