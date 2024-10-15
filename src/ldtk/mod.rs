use animation::update_animations;
use bevy::prelude::*;

pub mod animation;
pub mod entities;

use bevy_ecs_ldtk::{
    app::{LdtkEntityAppExt, LdtkIntCellAppExt},
    assets::LdtkProject,
    utils::translation_to_grid_coords,
    GridCoords, LdtkPlugin, LdtkWorldBundle, LevelSelection,
};

use crate::{
    config::TILE_SIZE,
    grid::{GridLocation, Tile},
    ldtk::entities::*,
    states::{GameState, PlayingState},
};

pub struct MyLdtkPlugin;

impl Plugin for MyLdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<InvestigatorBundle>("Investigator")
            .register_ldtk_entity::<VillagerBundle>("Villager")
            .register_ldtk_entity::<HiddingSpotBundle>("HiddingSpot")
            .register_ldtk_entity::<InteractibleBundle>("Interactible")
            .register_ldtk_int_cell::<CollisionTileBundle>(1)
            .add_systems(OnEnter(PlayingState::Loading), setup)
            .add_systems(OnExit(GameState::Playing), cleanup)
            .add_systems(
                Update,
                (
                    add_grid_location_to_wall,
                    update_animations,
                    update_grid_coords,
                )
                    .run_if(in_state(PlayingState::Playing)),
            )
            .insert_resource(LevelSelection::index(0));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ldtk_file: Handle<LdtkProject> = asset_server.load("ldtk/spooky_game.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: ldtk_file,
        ..default()
    });
}

fn cleanup() {}

fn update_grid_coords(
    mut commands: Commands,
    query: Query<(Entity, Ref<GridCoords>, Ref<Transform>)>,
) {
    for (entity, coords, transform) in &query {
        if !coords.is_changed() && transform.is_changed() {
            let new_coords = translation_to_grid_coords(transform.translation.xy(), TILE_SIZE);

            commands.entity(entity).insert(new_coords);
        }
    }
}

fn add_grid_location_to_wall(
    mut commands: Commands,
    query: Query<(Entity, &GridCoords), (With<Tile>, Without<GridLocation>)>,
) {
    for (entity, coords) in &query {
        commands.entity(entity).insert(GridLocation::from(*coords));
    }
}
