mod ai;
mod animated_sprite;
mod collisions;
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
mod utils;

use ai::AiPlugin;
use bevy::prelude::*;
use bevy_dev_tools::states::log_transitions;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rand::plugin::EntropyPlugin;
use bevy_rand::prelude::WyRand;
use collisions::CollisionsPlugin;
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
            WorldInspectorPlugin::new(), // for debug
            EntropyPlugin::<WyRand>::default(),
            RenderingPlugin,
            GridPlugin::<Tile>::default(),
            CollisionsPlugin,
            MainMenuPlugin,
            PauseMenuPlugin,
            GamePlugin,
            EnvironmentPlugin,
            InteractiblesPlugin,
            EnemiesPlugin,
            PlayerPlugin,
            AiPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Update, log_transitions::<GameState>)
        .run();
}
