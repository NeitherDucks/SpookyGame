use bevy::prelude::*;

use crate::{audio::AudioSetting, rendering::PIXEL_PERFECT_LAYERS, states::GameState};

use super::{AudioControllerTag, ButtonTag, UiElementsHandles, UiFocus, UiFocusOrder};

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct MainMenuTag;

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
                    ..default()
                },
                ..default()
            },
            MainMenuTag,
            PIXEL_PERFECT_LAYERS,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(121. * 4.),
                    height: Val::Px(70. * 4.),
                    margin: UiRect::bottom(Val::Px(100.0)),
                    ..default()
                },
                image: UiImage::new(ui_elements.0.get("title").unwrap().image.clone()),
                ..default()
            });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(32. * 6.),
                            height: Val::Px(17. * 6.),
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                        ..default()
                    },
                    ButtonTag::Play,
                    UiFocusOrder(0),
                    UiFocus::Focused,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Px(32. * 6.),
                                height: Val::Px(17. * 6.),
                                ..default()
                            },
                            image: UiImage::new(ui_elements.0.get("play").unwrap().image.clone()),
                            ..default()
                        },
                        TextureAtlas::from(
                            ui_elements.0.get("play").unwrap().atlas.clone().unwrap(),
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
            MainMenuTag,
            PIXEL_PERFECT_LAYERS,
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

pub fn cleanup(mut commands: Commands, main_menu: Query<Entity, With<MainMenuTag>>) {
    for entity in &main_menu {
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
        if *interaction == Interaction::Pressed {
            match tag {
                ButtonTag::Play => {
                    next_state.set(GameState::Playing);
                }
                ButtonTag::Quit => {
                    exit.send(AppExit::Success);
                }
                ButtonTag::Audio => {
                    audio_settings.next_audio_level();
                    *interaction = Interaction::Hovered;
                }
                _ => {}
            }
        }
    }
}
