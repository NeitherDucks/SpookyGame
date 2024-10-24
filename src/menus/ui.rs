use bevy::prelude::*;

use super::UiElementsHandles;

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct UiTag;

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct VillagerKilledUiTag;

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct VillagerTotalUiTag;

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct PlayerLivesUiTag;

pub fn setup(mut commands: Commands, ui_elements: Res<UiElementsHandles>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    align_content: AlignContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            UiTag,
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        width: Val::Px(42.0 * 3.),
                        height: Val::Px(18.0 * 3.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        flex_direction: FlexDirection::Row,
                        align_content: AlignContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    let mut atlas = TextureAtlas::from(
                        ui_elements.0.get("others").unwrap().atlas.clone().unwrap(),
                    );
                    atlas.index = 0;

                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Px(16. * 3.),
                                height: Val::Px(17. * 3.),
                                margin: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            image: UiImage::new(ui_elements.0.get("others").unwrap().image.clone()),

                            ..default()
                        },
                        atlas,
                    ));

                    let mut atlas = TextureAtlas::from(
                        ui_elements.0.get("numbers").unwrap().atlas.clone().unwrap(),
                    );
                    atlas.index = 0;

                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Px(8. * 3.),
                                height: Val::Px(10. * 3.),
                                margin: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            image: UiImage::new(
                                ui_elements.0.get("numbers").unwrap().image.clone(),
                            ),

                            ..default()
                        },
                        atlas,
                        VillagerKilledUiTag,
                    ));

                    let mut atlas = TextureAtlas::from(
                        ui_elements.0.get("numbers").unwrap().atlas.clone().unwrap(),
                    );
                    atlas.index = 10;

                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Px(8. * 3.),
                                height: Val::Px(10. * 3.),
                                margin: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            image: UiImage::new(
                                ui_elements.0.get("numbers").unwrap().image.clone(),
                            ),

                            ..default()
                        },
                        atlas,
                    ));

                    let mut atlas = TextureAtlas::from(
                        ui_elements.0.get("numbers").unwrap().atlas.clone().unwrap(),
                    );
                    atlas.index = 0;

                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Px(8. * 3.),
                                height: Val::Px(10. * 3.),
                                margin: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            image: UiImage::new(
                                ui_elements.0.get("numbers").unwrap().image.clone(),
                            ),

                            ..default()
                        },
                        atlas,
                        VillagerTotalUiTag,
                    ));
                });

            parent
                .spawn((NodeBundle {
                    style: Style {
                        width: Val::Px(28.0 * 3.),
                        height: Val::Px(18.0 * 3.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        flex_direction: FlexDirection::Row,
                        align_content: AlignContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    let mut atlas = TextureAtlas::from(
                        ui_elements.0.get("others").unwrap().atlas.clone().unwrap(),
                    );
                    atlas.index = 1;

                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Px(16. * 3.),
                                height: Val::Px(17. * 3.),
                                margin: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            image: UiImage::new(ui_elements.0.get("others").unwrap().image.clone()),

                            ..default()
                        },
                        atlas,
                    ));

                    let mut atlas = TextureAtlas::from(
                        ui_elements.0.get("numbers").unwrap().atlas.clone().unwrap(),
                    );
                    atlas.index = 0;

                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Px(8. * 3.),
                                height: Val::Px(10. * 3.),
                                margin: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            image: UiImage::new(
                                ui_elements.0.get("numbers").unwrap().image.clone(),
                            ),

                            ..default()
                        },
                        atlas,
                        PlayerLivesUiTag,
                    ));
                });
        });
}
