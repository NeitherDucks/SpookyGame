mod ai;
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
use bevy_rapier2d::plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode};
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use config::PIXEL_PER_TILE;
use game_mode::GamePlugin;
use grid::{GridPlugin, Tile};
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
            RapierDebugRenderPlugin::default(), // for debug
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXEL_PER_TILE),
            WorldInspectorPlugin::new(), // for debug
            EntropyPlugin::<WyRand>::default(),
            RenderingPlugin,
            GridPlugin::<Tile>::default(),
            MainMenuPlugin,
            PauseMenuPlugin,
            GamePlugin,
            MyLdtkPlugin,
            PlayerPlugin,
            AiPlugin,
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            physics_pipeline_active: true,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Variable {
                max_dt: 1. / 60.,
                time_scale: 1.,
                substeps: 1,
            },
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: true,
        })
        .init_state::<GameState>()
        .add_systems(Update, log_transitions::<GameState>)
        .add_systems(Update, pathfinding_gizmos)
        .run();
}
