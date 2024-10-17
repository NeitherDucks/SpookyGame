use animation::update_animations;
use bevy::prelude::*;

pub mod animation;
pub mod entities;

use crate::ai::Chased;
use bevy_ecs_ldtk::{
    app::{LdtkEntityAppExt, LdtkIntCellAppExt},
    assets::LdtkProject,
    utils::translation_to_grid_coords,
    GridCoords, LdtkPlugin, LdtkWorldBundle, LevelSelection,
};
use bevy_rapier2d::prelude::*;
use entities::{InteractionPossible, NoiseMakerBundle};
use hidding_spot::HiddingSpotBundle;
use player::PlayerTag;

use crate::{
    config::TILE_SIZE,
    grid::{GridLocation, Tile},
    ldtk::entities::*,
    states::{GameState, PlayingState},
};

#[derive(Resource)]
pub struct SpaceBarSpriteHandle(Handle<Image>);

pub struct MyLdtkPlugin;

impl Plugin for MyLdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<InvestigatorBundle>("Investigator")
            .register_ldtk_entity::<VillagerBundle>("Villager")
            .register_ldtk_entity::<HiddingSpotBundle>("HiddingSpot")
            .register_ldtk_entity::<NoiseMakerBundle>("NoiseMaker")
            .register_ldtk_entity::<InteractibleBundle>("Interactible")
            .register_ldtk_int_cell::<CollisionTileBundle>(1)
            .register_type::<InteractionPossible>()
            .register_type::<InteractibleEntityRef>()
            .register_type::<ActiveCollisionTypes>()
            .register_type::<ActiveEvents>()
            .register_type::<EnemyTag>()
            .add_systems(OnEnter(PlayingState::Loading), setup)
            .add_systems(OnExit(GameState::Playing), cleanup)
            .add_systems(
                Update,
                (
                    add_grid_location_to_wall,
                    resolve_entity_references,
                    update_animations,
                    update_grid_coords,
                    interaction_events,
                    noise_maker_trigger_removed,
                    villager_added,
                )
                    .run_if(in_state(PlayingState::Playing)),
            )
            .insert_resource(LevelSelection::index(0));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ldtk_file: Handle<LdtkProject> = asset_server.load("ldtk/spooky_game.ldtk");

    let spacebar_sprite: Handle<Image> = asset_server.load("2d/space_bar.png");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: ldtk_file,
        ..default()
    });

    commands.insert_resource(SpaceBarSpriteHandle(spacebar_sprite));
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

pub fn interaction_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player: Query<
        (Entity, Option<&mut InteractionPossible>),
        (With<PlayerTag>, Without<Chased>),
    >,
    interactibles: Query<(&InteractibleTag, &InteractibleEntityRef)>,
    spacebar_sprite_handle: Res<SpaceBarSpriteHandle>,
) {
    let Ok((player, mut current_interaction)) = player.get_single_mut() else {
        return;
    };

    let mut events: i32 = 0;
    let mut entity: Option<(Entity, InteractibleTag)> = None;

    for collision_event in collision_events.read() {
        let (add, from, to) = match collision_event {
            CollisionEvent::Started(entity_from, entity_to, _) => (true, *entity_from, *entity_to),
            CollisionEvent::Stopped(entity_from, entity_to, _) => (false, *entity_from, *entity_to),
        };

        if to != player && from != player {
            continue;
        }

        let other = match to == player {
            true => from,
            false => to,
        };

        let Ok((tag, reference)) = interactibles.get(other) else {
            continue;
        };

        if let Some((entity, _)) = entity {
            if entity == reference.0 {
                if add {
                    events += 1;
                } else {
                    events -= 1;
                }
            }
        } else {
            entity = Some((reference.0, *tag));
            if add {
                events += 1;
            } else {
                events -= 1;
            }
        }
    }

    if let Some((entity, tag)) = entity {
        if let Some(current_interaction) = current_interaction.as_deref_mut() {
            if current_interaction.entity == entity {
                let counter = current_interaction.counter as i32 + events;
                if counter > 0 {
                    current_interaction.counter = counter as u32;
                } else {
                    commands.entity(player).remove::<InteractionPossible>();
                    commands.entity(entity).despawn_descendants();
                }
            }
        } else {
            commands.entity(player).insert(InteractionPossible {
                entity: entity,
                counter: events as u32,
                interactibe_type: tag,
            });

            let child = commands
                .spawn((
                    ShowInteractionButtonTag,
                    SpriteBundle {
                        texture: spacebar_sprite_handle.0.clone(),
                        transform: Transform::from_translation(Vec3::new(0., 16., 0.)),
                        ..Default::default()
                    },
                ))
                .id();
            commands.entity(entity).add_child(child);
        }
    }
}
