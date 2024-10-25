use bevy::prelude::*;

use super::{AudioControllerTag, ButtonTag, UiElementsHandles, UiFocus, UiFocusOrder};
use crate::{audio::AudioSetting, states::GameState};

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct WinMenuTag;

pub fn setup(mut commands: Commands, ui_elements: Res<UiElementsHandles>) {
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
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(41. * 10.),
                    height: Val::Px(10. * 10.),
                    margin: UiRect::bottom(Val::Px(100.0)),
                    ..default()
                },
                image: UiImage::new(ui_elements.0.get("success").unwrap().image.clone()),
                ..default()
            });

            let style = Style {
                width: Val::Px(51. * 3.),
                height: Val::Px(17. * 3.),
                ..default()
            };

            parent
                .spawn((
                    ButtonBundle {
                        style: style.clone(),
                        ..default()
                    },
                    ButtonTag::Reset,
                    UiFocusOrder(0),
                    UiFocus::Focused,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            style,
                            image: UiImage::new(
                                ui_elements.0.get("restart").unwrap().image.clone(),
                            ),
                            ..default()
                        },
                        TextureAtlas::from(
                            ui_elements.0.get("restart").unwrap().atlas.clone().unwrap(),
                        ),
                    ));
                });

            if !cfg!(target_family = "wasm") {
                let style = Style {
                    width: Val::Px(29. * 3.),
                    height: Val::Px(17. * 3.),
                    ..default()
                };

                parent
                    .spawn((
                        ButtonBundle {
                            style: style.clone(),
                            ..default()
                        },
                        ButtonTag::Quit,
                        UiFocusOrder(1),
                        UiFocus::None,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            ImageBundle {
                                style,
                                image: UiImage::new(
                                    ui_elements.0.get("quit").unwrap().image.clone(),
                                ),
                                ..default()
                            },
                            TextureAtlas::from(
                                ui_elements.0.get("quit").unwrap().atlas.clone().unwrap(),
                            ),
                        ));
                    });
            }
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::End,
                    ..default()
                },
                ..default()
            },
            WinMenuTag,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(16. * 3.),
                            height: Val::Px(16. * 3.),
                            margin: UiRect::all(Val::Px(10. * 3.)),
                            ..Default::default()
                        },
                        ..default()
                    },
                    ButtonTag::Audio,
                    UiFocusOrder(-1),
                    UiFocus::None,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                ..Default::default()
                            },
                            image: UiImage::new(ui_elements.0.get("audio").unwrap().image.clone()),
                            ..default()
                        },
                        TextureAtlas::from(
                            ui_elements.0.get("audio").unwrap().atlas.clone().unwrap(),
                        ),
                        AudioControllerTag,
                    ));
                });
        });
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<WinMenuTag>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&mut Interaction, &ButtonTag),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut audio_settings: ResMut<AudioSetting>,
    mut exit: EventWriter<AppExit>,
) {
    for (mut interaction, tag) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match tag {
                ButtonTag::Quit => {
                    exit.send(AppExit::Success);
                }
                ButtonTag::Reset => {
                    next_state.set(GameState::Reset);
                }
                ButtonTag::Audio => {
                    audio_settings.next_audio_level();
                    *interaction = Interaction::Hovered;
                }
                _ => {}
            },
            _ => {}
        }
    }
}
