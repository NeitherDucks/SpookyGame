use bevy::prelude::*;

use crate::{rendering::PIXEL_PERFECT_LAYERS, states::GameState};

#[derive(Component)]
struct MainMenuTag;

pub struct MainMenuPlugin;

#[derive(Component)]
enum ButtonTag {
    Play,
    Quit,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(OnExit(GameState::MainMenu), cleanup)
            .add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)));
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
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            MainMenuTag,
            PIXEL_PERFECT_LAYERS,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        ..default()
                    },
                    ButtonTag::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 30.0,
                            ..default()
                        },
                    ));
                });

            // Don't put a quit button if it's web.
            // Seems dirty to do a return on a cfg...
            #[cfg(target_family = "wasm")]
            return;

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(2.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::top(Val::Px(30.)),
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        ..default()
                    },
                    ButtonTag::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font_size: 20.0,
                            ..default()
                        },
                    ));
                });
        });
}

fn cleanup(mut commands: Commands, main_menu: Query<Entity, With<MainMenuTag>>) {
    for entity in &main_menu {
        commands.entity(entity).despawn_recursive();
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ButtonTag,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, mut border_color, tag) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.5, 0.5, 0.5).into();
                border_color.0 = Color::WHITE;
                match tag {
                    ButtonTag::Play => {
                        next_state.set(GameState::Playing);
                    }
                    ButtonTag::Quit => {
                        exit.send(AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.3, 0.3, 0.3).into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::srgb(0.2, 0.2, 0.2).into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
