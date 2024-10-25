use animation::{
    animation_changed, animation_offset_changed, update_animations, update_animations_during_death,
    AnimationFinishedEvent,
};
use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_ecs_tilemap::tiles::TileTextureIndex;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use collision_tile::AICollisionTileBundle;
use iyes_progress::prelude::*;
use rand_core::RngCore;

pub mod animation;
pub mod entities;

use crate::{
    ai::Chased, player_controller::PlayerIsHidding, rendering::HEIGHT_LAYERS, utils::remap_rand_f32,
};
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

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct SpaceBarSpriteHandle(Handle<Image>);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct DeadPlayerSpriteHandle(pub Handle<Image>);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct VillagerSpritesheetHandles(pub Vec<Handle<Image>>);

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct AnimatedLdtkLayer;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct AnimatedLdtkLayerTimer(pub Timer);

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct ConstantAnimatedLdtkLayer;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ConstantAnimatedLdtkLayerTimer(pub Timer);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct EnemyLights {
    investigator_light: Handle<Image>,
    villager_light: Handle<Image>,
    atlas: Handle<TextureAtlasLayout>,
    timer: Timer,
}

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct Light;

pub struct MyLdtkPlugin;

impl Plugin for MyLdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LdtkPlugin,
            ProgressPlugin::new(PlayingState::Loading)
                .continue_to(PlayingState::IntroScene)
                .track_assets(),
        ))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<InvestigatorBundle>("Investigator")
        .register_ldtk_entity::<VillagerBundle>("Villager")
        .register_ldtk_entity::<HiddingSpotBundle>("HiddingSpot")
        .register_ldtk_entity::<NoiseMakerBundle>("NoiseMaker")
        .register_ldtk_entity::<InteractibleBundle>("Interactible")
        .register_ldtk_entity::<PlayerRespawnPointBundle>("PlayerRespawnPoint")
        .register_ldtk_int_cell::<CollisionTileBundle>(1)
        .register_ldtk_int_cell::<AICollisionTileBundle>(2)
        .register_type::<InteractionPossible>()
        .register_type::<InteractibleEntityRef>()
        .register_type::<ActiveCollisionTypes>()
        .register_type::<ActiveEvents>()
        .register_type::<EnemyTag>()
        .register_type::<Aim>()
        .register_type::<AnimationConfig>()
        .add_event::<AnimationFinishedEvent>()
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
                investigator_added,
                villager_added,
                on_respawn_point_added,
                animation_changed,
                animation_offset_changed,
                modify_ldtk_layers,
                update_layer_animations,
                update_layer_animations_constant,
                update_lights_animation,
            )
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            Update,
            (update_animations_during_death, animation_changed)
                .run_if(in_state(PlayingState::Death)),
        )
        .insert_resource(LevelSelection::index(0));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut loading: ResMut<AssetsLoading>,
) {
    let ldtk_file: Handle<LdtkProject> = asset_server.load("ldtk/spooky_game.ldtk");
    let spacebar_sprite: Handle<Image> = asset_server.load("2d/space_bar.png");
    let deadplayer_sprite: Handle<Image> = asset_server.load("2d/dead_player.png");

    let mut villager_handles: Vec<Handle<Image>> = Vec::new();
    let villager_names = vec![
        "Artun", "Grym", "Hana", "Hark", "Janik", "Julz", "Khali", "Meza", "Nel", "Nyro", "Reza",
        "Serek", "Seza", "Vash",
    ];

    for name in villager_names {
        villager_handles.push(asset_server.load(format!("2d/villagers/{}.png", name)));
    }

    commands.insert_resource(VillagerSpritesheetHandles(villager_handles));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: ldtk_file.clone(),
        ..default()
    });

    commands.insert_resource(SpaceBarSpriteHandle(spacebar_sprite.clone()));
    commands.insert_resource(DeadPlayerSpriteHandle(deadplayer_sprite.clone()));

    loading.add(&ldtk_file);
    loading.add(&spacebar_sprite);
    loading.add(&deadplayer_sprite);

    commands.insert_resource(AnimatedLdtkLayerTimer(Timer::from_seconds(
        1. / 2., // 2 fps
        TimerMode::Repeating,
    )));

    commands.insert_resource(ConstantAnimatedLdtkLayerTimer(Timer::from_seconds(
        1. / 8., // 8 fps
        TimerMode::Repeating,
    )));

    let investigator_light_handle: Handle<Image> = asset_server.load("2d/light_investigator.png");
    let villager_light_handle: Handle<Image> = asset_server.load("2d/light_villager.png");

    let atlas = TextureAtlasLayout::from_grid(UVec2::new(112, 192), 5, 1, None, None);
    let atlas_handle = texture_atlases.add(atlas);

    loading.add(&investigator_light_handle);
    loading.add(&villager_light_handle);

    commands.insert_resource(EnemyLights {
        atlas: atlas_handle,
        investigator_light: investigator_light_handle,
        villager_light: villager_light_handle,
        timer: Timer::from_seconds(0.25, TimerMode::Repeating),
    });
}

fn cleanup(mut commands: Commands, ldtk_world: Query<Entity, With<Handle<LdtkProject>>>) {
    for entity in &ldtk_world {
        commands.entity(entity).despawn_recursive();
    }
}

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
        (With<PlayerTag>, Without<Chased>, Without<PlayerIsHidding>),
    >,
    interaction_button: Query<Entity, With<ShowInteractionButtonTag>>,
    interactibles: Query<(&InteractibleTag, &InteractibleEntityRef)>,
    spacebar_sprite_handle: Res<SpaceBarSpriteHandle>,
) {
    let Ok((player, mut current_interaction)) = player.get_single_mut() else {
        return;
    };

    let mut events: HashMap<Entity, (i32, &InteractibleTag)> = HashMap::new();

    // let mut events: i32 = 0;
    // let mut entity: Option<(Entity, InteractibleTag)> = None;

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

        // if let Some((entity, _)) = entity {
        //     if entity == reference.0 {
        //         if add {
        //             events += 1;
        //         } else {
        //             events -= 1;
        //         }
        //     }
        // } else {
        //     entity = Some((reference.0, *tag));
        //     if add {
        //         events += 1;
        //     } else {
        //         events -= 1;
        //     }
        // }

        if let Some((count, _)) = events.get_mut(&reference.0) {
            if add {
                *count += 1;
            } else {
                *count -= 1;
            }
        } else {
            if add {
                events.insert(reference.0, (1, tag));
            } else {
                events.insert(reference.0, (-1, tag));
            }
        }
    }

    if let Some((entity, (count, tag))) = events.into_iter().last() {
        if let Some(current_interaction) = current_interaction.as_deref_mut() {
            if current_interaction.entity == entity {
                let counter = current_interaction.counter as i32 + count;
                if counter > 0 {
                    current_interaction.counter = counter as u32;
                } else {
                    commands.entity(player).remove::<InteractionPossible>();

                    for button in &interaction_button {
                        commands.entity(button).despawn_recursive();
                    }
                }
            }
        } else {
            commands.entity(player).insert(InteractionPossible {
                entity: entity,
                counter: count as u32,
                interactibe_type: *tag,
            });

            let child = commands
                .spawn((
                    ShowInteractionButtonTag,
                    SpriteBundle {
                        texture: spacebar_sprite_handle.0.clone(),
                        transform: Transform::from_translation(Vec3::new(0., 16., 100.)),
                        ..Default::default()
                    },
                ))
                .id();
            commands.entity(entity).add_child(child);
        }
    }
}

fn modify_ldtk_layers(
    mut commands: Commands,
    mut query: Query<(Entity, &LayerMetadata, &mut Visibility), Added<LayerMetadata>>,
) {
    for (entity, layer, mut visibility) in &mut query {
        match layer.identifier.as_str() {
            "HEIGHT" => {
                commands.entity(entity).insert(HEIGHT_LAYERS);
            }
            "IntGrid" => {
                *visibility = Visibility::Hidden;
            }
            "AnimatedTilesTop" => {
                commands
                    .entity(entity)
                    .insert((PIXEL_PERFECT_LAYERS, ConstantAnimatedLdtkLayer));
            }
            "AnimatedTilesBelow" => {
                commands
                    .entity(entity)
                    .insert((PIXEL_PERFECT_LAYERS, AnimatedLdtkLayer));
            }
            _ => {
                commands.entity(entity).insert(PIXEL_PERFECT_LAYERS);
            }
        }
    }
}

fn update_layer_animations(
    query: Query<&Children, With<AnimatedLdtkLayer>>,
    mut query_children: Query<&mut TileTextureIndex, Without<AnimatedLdtkLayer>>,
    mut timer: ResMut<AnimatedLdtkLayerTimer>,
    time: Res<Time>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        for children in &query {
            for child in children {
                if let Ok(mut index) = query_children.get_mut(*child) {
                    // 10% chance to animate the tile
                    if remap_rand_f32(rng.next_u32(), 0., 1.) < 0.1 {
                        let x = index.0.rem_euclid(4);
                        let y = index.0 / 4;

                        index.0 = (x + 1).rem_euclid(4) + (y * 4);
                    }
                }
            }
        }
    }
}

fn update_layer_animations_constant(
    query: Query<&Children, With<ConstantAnimatedLdtkLayer>>,
    mut query_children: Query<&mut TileTextureIndex, Without<ConstantAnimatedLdtkLayer>>,
    mut timer: ResMut<ConstantAnimatedLdtkLayerTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        for children in &query {
            for child in children {
                if let Ok(mut index) = query_children.get_mut(*child) {
                    let x = index.0.rem_euclid(4);
                    let y = index.0 / 4;

                    index.0 = (x + 1).rem_euclid(4) + (y * 4);
                }
            }
        }
    }
}

fn update_lights_animation(
    mut lights: Query<&mut TextureAtlas, With<Light>>,
    mut timer: ResMut<EnemyLights>,
    time: Res<Time>,
) {
    timer.timer.tick(time.delta());

    if timer.timer.finished() {
        for mut light in &mut lights {
            light.index = light.index.rem_euclid(5);
        }
    }
}
