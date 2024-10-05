mod intro_scene;
mod loading;
mod playing;

use bevy::prelude::*;
use bevy_dev_tools::states::log_transitions;

use crate::animated_sprite::animate_sprite;
use crate::playing::intro_scene::{intro_scene_setup, intro_scene_update};
use crate::playing::loading::load;
use crate::states::{GameState, PlayingState};

#[derive(Component)]
pub struct PlayingTag;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PlayingState>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(OnExit(GameState::Playing), cleanup)
            .add_systems(OnEnter(PlayingState::Loading), load)
            .add_systems(OnEnter(PlayingState::IntroScene), intro_scene_setup)
            .add_systems(
                Update,
                intro_scene_update.run_if(in_state(PlayingState::IntroScene)),
            )
            .add_systems(Update, log_transitions::<PlayingState>)
            .add_systems(Update, animate_sprite);
    }
}

fn setup(mut next_state: ResMut<NextState<PlayingState>>) {
    // Extra setup if needed

    println!("SETUP PLAYING STATE");
    next_state.set(PlayingState::Loading);
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<PlayingTag>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
