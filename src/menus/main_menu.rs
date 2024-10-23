use bevy::prelude::*;

use crate::{rendering::PIXEL_PERFECT_LAYERS, states::GameState};

use super::{ButtonTag, UiElementsHandles};

#[derive(Component)]
pub struct MainMenuTag;

pub fn setup(mut commands: Commands, buttons_images: Res<UiElementsHandles>) {
    let style = Style {
        width: Val::Px(32. * 6.),
        height: Val::Px(17. * 6.),
        ..default()
    };

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
                        style: style.clone(),
                        ..default()
                    },
                    ButtonTag::Play,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            style,
                            image: UiImage::new(
                                buttons_images.0.get("play").unwrap().image.clone(),
                            ),
                            ..default()
                        },
                        TextureAtlas::from(
                            buttons_images.0.get("play").unwrap().atlas.clone().unwrap(),
                        ),
                    ));
                });

            // Don't put a quit button if it's web.
            // Seems dirty to do a return on a cfg...
            #[cfg(target_family = "wasm")]
            return;

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
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            style,
                            image: UiImage::new(
                                buttons_images.0.get("quit").unwrap().image.clone(),
                            ),
                            ..default()
                        },
                        TextureAtlas::from(
                            buttons_images.0.get("quit").unwrap().atlas.clone().unwrap(),
                        ),
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
    interaction_query: Query<(&Interaction, &ButtonTag), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, tag) in &interaction_query {
        match *interaction {
            Interaction::Pressed => match tag {
                ButtonTag::Play => {
                    next_state.set(GameState::Playing);
                }
                ButtonTag::Quit => {
                    exit.send(AppExit::Success);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
