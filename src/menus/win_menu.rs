use bevy::prelude::*;

use super::{ButtonTag, UiElementsHandles};
use crate::states::GameState;

#[derive(Component)]
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
                            image: UiImage::new(ui_elements.0.get("quit").unwrap().image.clone()),
                            ..default()
                        },
                        TextureAtlas::from(
                            ui_elements.0.get("quit").unwrap().atlas.clone().unwrap(),
                        ),
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
    interaction_query: Query<(&Interaction, &ButtonTag), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, tag) in &interaction_query {
        match *interaction {
            Interaction::Pressed => match tag {
                ButtonTag::Quit => {
                    exit.send(AppExit::Success);
                }
                ButtonTag::Reset => {
                    next_state.set(GameState::Reset);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
