mod animated_sprite;
mod main_menu;
mod player;
mod playing;
mod states;

use bevy::prelude::*;
use bevy_dev_tools::states::log_transitions;
use playing::PlayingPlugin;

use crate::main_menu::MainMenuPlugin;
use crate::states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainMenuPlugin)
        .add_plugins(PlayingPlugin)
        .init_state::<GameState>()
        .add_systems(Update, log_transitions::<GameState>)
        .run();
}
