use bevy::prelude::*;

use crate::states::GameState;

#[derive(Component)]
struct MainMenuTag;

#[derive(Component)]
struct MainMenuCameraTag;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(OnExit(GameState::MainMenu), cleanup)
            .add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera3dBundle { ..default() }, MainMenuCameraTag));
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
                ..default()
            },
            MainMenuTag,
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
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
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 30.0,
                            ..default()
                        },
                    ));
                });
        });
}

fn cleanup(
    mut commands: Commands,
    main_menu: Query<Entity, With<MainMenuTag>>,
    camera: Query<Entity, With<MainMenuCameraTag>>,
) {
    for entity in &main_menu {
        commands.entity(entity).despawn_recursive();
    }

    for entity in &camera {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.5, 0.5, 0.5).into();
                border_color.0 = Color::WHITE;
                next_state.set(GameState::Playing);
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
