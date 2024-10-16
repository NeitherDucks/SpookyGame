use bevy::prelude::*;

use crate::states::PlayingState;

#[derive(Component)]
struct PauseMenuTag;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingState::Pause), setup)
            .add_systems(OnExit(PlayingState::Pause), cleanup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.5)),
                ..default()
            },
            PauseMenuTag,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Paused",
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            ));
        });
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<PauseMenuTag>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
