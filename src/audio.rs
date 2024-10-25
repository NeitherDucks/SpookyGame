use bevy::{audio::Volume, prelude::*};

use crate::states::GameState;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GameMusic;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct FadeIn;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup)
            .add_systems(Update, fadein);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/music/bg_music.ogg"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: Volume::new(0.),
                ..Default::default()
            },
        },
        GameMusic,
        FadeIn,
    ));
}

fn fadein(
    mut commands: Commands,
    music_controller: Query<(Entity, &AudioSink), (With<GameMusic>, With<FadeIn>)>,
    time: Res<Time>,
) {
    if let Ok((entity, sink)) = music_controller.get_single() {
        sink.set_volume(sink.volume() + time.elapsed_seconds() / 400.);

        if sink.volume() >= 1. {
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}
