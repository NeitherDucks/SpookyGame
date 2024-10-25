use bevy::{audio::Volume, prelude::*};

use crate::states::GameState;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GameMusic;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct FadeIn;

#[derive(Default, Reflect)]
pub enum AudioLevels {
    Mute,
    Low,
    #[default]
    Normal,
    High,
}

#[derive(Resource, Reflect, Default)]
pub struct AudioSetting(pub AudioLevels);

impl AudioSetting {
    pub fn next_audio_level(&mut self) {
        self.0 = match self.0 {
            AudioLevels::High => AudioLevels::Mute,
            AudioLevels::Mute => AudioLevels::Low,
            AudioLevels::Low => AudioLevels::Normal,
            AudioLevels::Normal => AudioLevels::High,
        }
    }
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioSetting>()
            .add_systems(OnEnter(GameState::Loading), setup)
            .add_systems(Update, (fadein, audio_settings_changed));
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

fn audio_settings_changed(
    music_controller: Query<&AudioSink, With<GameMusic>>,
    audio_settings: Res<AudioSetting>,
) {
    if audio_settings.is_changed() {
        if let Ok(sink) = music_controller.get_single() {
            sink.set_volume(match audio_settings.0 {
                AudioLevels::High => 3.16,
                AudioLevels::Normal => 1.0,
                AudioLevels::Low => 0.316,
                AudioLevels::Mute => 0.,
            });
        }
    }
}
