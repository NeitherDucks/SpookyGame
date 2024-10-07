mod animated_sprite;
mod collider;
mod environment;
mod game_mode;
mod interactibles;
mod investigators;
mod main_menu;
mod pause_menu;
mod player;
mod rendering;
mod states;
mod villagers;

use bevy::prelude::*;
use bevy_dev_tools::states::log_transitions;
use environment::EnvironmentPlugin;
use game_mode::GamePlugin;
use interactibles::InteractiblesPlugin;
use investigators::InvestigatorsPlugin;
use pause_menu::PauseMenuPlugin;
use player::PlayerPlugin;
use rendering::RenderingPlugin;
use villagers::VillagersPlugin;

use crate::main_menu::MainMenuPlugin;
use crate::states::GameState;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RenderingPlugin,
            MainMenuPlugin,
            PauseMenuPlugin,
            GamePlugin,
            EnvironmentPlugin,
            InteractiblesPlugin,
            InvestigatorsPlugin,
            PlayerPlugin,
            VillagersPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Update, log_transitions::<GameState>)
        .run();
}
