use bevy::prelude::*;

use crate::states::{GameState, PlayingState};

#[derive(Component)]
struct WinMenuTag;

#[derive(Component)]
enum ButtonTag {
    Reset,
    Quit,
}

pub struct WinMenuPlugin;

impl Plugin for WinMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingState::Win), setup)
            .add_systems(OnExit(PlayingState::Win), cleanup)
            .add_systems(
                Update,
                (button_system, update).run_if(in_state(PlayingState::Win)),
            );
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
                    align_content: AlignContent::SpaceBetween,
                    ..default()
                },
                background_color: BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.75)),
                ..default()
            },
            WinMenuTag,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "You Win!",
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            ));

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
                    ButtonTag::Reset,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play again",
                        TextStyle {
                            font_size: 20.0,
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

fn cleanup(mut commands: Commands, query: Query<Entity, With<WinMenuTag>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn update() {}

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
                    ButtonTag::Quit => {
                        exit.send(AppExit::Success);
                    }
                    ButtonTag::Reset => {
                        next_state.set(GameState::Reset);
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
