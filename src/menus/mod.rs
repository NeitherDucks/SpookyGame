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
                button_interaction.run_if(
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
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut images: Query<&mut TextureAtlas>,
) {
    for (interaction, children) in &interaction_query {
        for child in children {
            let Ok(mut atlas) = images.get_mut(*child) else {
                continue;
            };

            match *interaction {
                Interaction::Pressed => {
                    atlas.index = 2;
                }
                Interaction::Hovered => {
                    atlas.index = 1;
                }
                Interaction::None => {
                    atlas.index = 0;
                }
            }
        }
    }
}
