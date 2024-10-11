mod ai;
mod collisions;
mod game_mode;
mod grid;
mod ldtk;
mod main_menu;
mod pathfinding;
mod pause_menu;
mod player;
mod rendering;
mod states;
mod utils;

mod config;

use ai::AiPlugin;
use bevy::prelude::*;
use bevy_dev_tools::states::log_transitions;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rand::plugin::EntropyPlugin;
use bevy_rand::prelude::WyRand;
use collisions::CollisionsPlugin;
use game_mode::GamePlugin;
use grid::{collision_gizmos, GridPlugin, Tile};
use ldtk::MyLdtkPlugin;
use pathfinding::pathfinding_gizmos;
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
            MyLdtkPlugin,
            PlayerPlugin,
            AiPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Update, log_transitions::<GameState>)
        .add_systems(Update, (pathfinding_gizmos, collision_gizmos))
        .run();
}
