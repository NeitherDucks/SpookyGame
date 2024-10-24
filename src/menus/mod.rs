use bevy::{prelude::*, utils::HashMap};
use iyes_progress::{prelude::AssetsLoading, ProgressPlugin};

use crate::states::{GameState, PlayingState};

mod lose_menu;
mod main_menu;
mod pause_menu;
mod ui;
mod win_menu;

pub use ui::{PlayerLivesUiTag, VillagerKilledUiTag, VillagerTotalUiTag};

#[derive(Component)]
enum ButtonTag {
    Play,
    Quit,
    Reset,
    Resume,
}

struct UiElementHandles {
    image: Handle<Image>,
    atlas: Option<Handle<TextureAtlasLayout>>,
}

#[derive(Resource)]
struct UiElementsHandles(HashMap<String, UiElementHandles>);

#[derive(Component)]
struct UiFocusOrder(i32);

#[derive(Component, PartialEq, Eq)]
enum UiFocus {
    Pressed,
    Focused,
    None,
}

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            ProgressPlugin::new(GameState::Loading)
                .continue_to(GameState::MainMenu)
                .track_assets(),
        )
        .add_systems(OnEnter(GameState::Loading), setup)
        .add_systems(OnEnter(PlayingState::Lose), lose_menu::setup)
        .add_systems(OnExit(PlayingState::Lose), lose_menu::cleanup)
        .add_systems(OnEnter(GameState::MainMenu), main_menu::setup)
        .add_systems(OnExit(GameState::MainMenu), main_menu::cleanup)
        .add_systems(OnEnter(PlayingState::Pause), pause_menu::setup)
        .add_systems(OnExit(PlayingState::Pause), pause_menu::cleanup)
        .add_systems(OnEnter(PlayingState::Win), win_menu::setup)
        .add_systems(OnExit(PlayingState::Win), win_menu::cleanup)
        .add_systems(OnEnter(GameState::Playing), ui::setup)
        .add_systems(
            Update,
            (
                lose_menu::button_system.run_if(in_state(PlayingState::Lose)),
                main_menu::button_system.run_if(in_state(GameState::MainMenu)),
                pause_menu::button_system.run_if(in_state(PlayingState::Pause)),
                win_menu::button_system.run_if(in_state(PlayingState::Win)),
                (button_interaction, change_focus, focus_press_button).run_if(
                    in_state(PlayingState::Lose)
                        .or_else(in_state(GameState::MainMenu))
                        .or_else(in_state(PlayingState::Pause))
                        .or_else(in_state(PlayingState::Win)),
                ),
            ),
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut loading: ResMut<AssetsLoading>,
) {
    let buttons = vec![
        ("play", "_button", 32, 17, 3),
        ("quit", "_button", 29, 17, 3),
        ("restart", "_button", 50, 17, 3),
        ("resume", "_button", 50, 17, 3),
        ("numbers", "", 8, 10, 11),
        ("others", "", 16, 17, 2),
    ];

    let mut store: HashMap<String, UiElementHandles> = HashMap::new();

    for (name, second, width, height, cols) in buttons {
        let image_handle: Handle<Image> = asset_server.load(format!("ui/{}{}.png", name, second));
        let texture = TextureAtlasLayout::from_grid(UVec2::new(width, height), cols, 1, None, None);
        let texture_handle = texture_atlases.add(texture);
        loading.add(&image_handle);

        store.insert(
            name.into(),
            UiElementHandles {
                image: image_handle,
                atlas: Some(texture_handle),
            },
        );
    }

    let elements = vec!["paused", "failed", "success", "title"];

    for name in elements {
        let image_handle: Handle<Image> = asset_server.load(format!("ui/{}.png", name));
        store.insert(
            name.into(),
            UiElementHandles {
                image: image_handle,
                atlas: None,
            },
        );
    }

    commands.insert_resource(UiElementsHandles(store));
}

fn button_interaction(
    interaction_query: Query<
        (&Interaction, &UiFocus, &Children),
        (Or<(Changed<Interaction>, Changed<UiFocus>)>, With<Button>),
    >,
    mut images: Query<&mut TextureAtlas>,
) {
    for (interaction, focus, children) in &interaction_query {
        for child in children {
            let Ok(mut atlas) = images.get_mut(*child) else {
                continue;
            };

            if *interaction == Interaction::Pressed || *focus == UiFocus::Pressed {
                atlas.index = 2;
            } else if *interaction == Interaction::Hovered || *focus == UiFocus::Focused {
                atlas.index = 1;
            } else {
                atlas.index = 0;
            }
        }
    }
}

fn change_focus(
    mut buttons: Query<(&UiFocusOrder, &mut UiFocus), With<Button>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction: Option<i32> = None;

    if input.just_pressed(KeyCode::KeyS)
        || input.just_pressed(KeyCode::ArrowDown)
        || input.just_pressed(KeyCode::Tab)
    {
        direction = Some(1);
    }

    if input.just_pressed(KeyCode::KeyW) || input.just_pressed(KeyCode::ArrowUp) {
        direction = Some(-1);
    }

    if let Some(direction) = direction {
        // Find current focused item. Also get min/max index.
        let mut current: Option<i32> = None;
        let mut min_index: i32 = 0;
        let mut max_index: i32 = 0;

        for (order, focus) in &buttons {
            if *focus == UiFocus::Focused {
                current = Some(order.0);
            }

            min_index = min_index.min(order.0);
            max_index = max_index.max(order.0);
        }

        // If none, set focus on item with index 0, else, set focus on next item, wrapping as needed.
        let next_id = match current {
            Some(current) => {
                if current + direction > max_index {
                    min_index
                } else if current + direction < min_index {
                    max_index
                } else {
                    current + direction
                }
            }
            None => 0,
        };

        for (order, mut focus) in &mut buttons {
            if order.0 == next_id {
                *focus = UiFocus::Focused;
            } else {
                *focus = UiFocus::None;
            }
        }
    }
}

fn focus_press_button(
    mut buttons: Query<(&mut Interaction, &mut UiFocus), With<Button>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Enter) || input.just_pressed(KeyCode::Space) {
        for (mut interaction, focus) in &mut buttons {
            if *focus == UiFocus::Focused {
                // Piggy back on the Mouse Interaction to trigger the pressed in different menus.
                *interaction = Interaction::Pressed;
            }
        }
    }
}
