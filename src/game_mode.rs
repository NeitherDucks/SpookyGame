use bevy::prelude::*;
use bevy_dev_tools::states::log_transitions;

use crate::{
    ai::Chased,
    config::{PLAYER_ANIMATION_DEATH, PLAYER_ANIMATION_IDLE},
    ldtk::{
        animation::{new_animation_during_death, AnimationFinishedEvent, ANIMATIONS},
        entities::{
            dead_player::{DeadPlayerBundle, DeadPlayerTag},
            player_respawn_point::PlayerRespawnPointTag,
            PlayerTag,
        },
        DeadPlayerSpriteHandle,
    },
    rendering::InGameCamera,
    states::{GameState, PlayingState},
};

#[derive(Resource)]
pub struct Score {
    total_villagers: u8,
    villagers_killed: u8,
    player_lives: u8,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            total_villagers: 0,
            villagers_killed: 0,
            player_lives: 1,
        }
    }
}

impl Score {
    pub fn villager_spawned(&mut self) {
        self.total_villagers += 1;
    }

    pub fn villager_killed(&mut self) {
        self.villagers_killed += 1;
    }

    pub fn player_respawn_point_added(&mut self) {
        self.player_lives += 1;
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PlayingState>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(OnEnter(PlayingState::Loading), load)
            .add_systems(OnEnter(PlayingState::IntroScene), intro_scene_setup)
            .add_systems(OnEnter(PlayingState::Death), player_death)
            .add_systems(OnEnter(PlayingState::Respawning), player_respawn)
            .add_systems(OnEnter(GameState::Reset), reset)
            .add_systems(
                Update,
                intro_scene_update.run_if(in_state(PlayingState::IntroScene)),
            )
            .add_systems(
                Update,
                check_win_condition.run_if(in_state(PlayingState::Playing)),
            )
            .add_systems(Update, player_died.run_if(in_state(PlayingState::Death)))
            .add_systems(Update, log_transitions::<PlayingState>);
    }
}

fn setup(mut commands: Commands, mut next_state: ResMut<NextState<PlayingState>>) {
    // Extra setup if needed
    commands.init_resource::<Score>();

    next_state.set(PlayingState::Loading);
}

/// Most of the loading happens in [`ldtk::setup()`].
/// But this can also be used if needed.
/// Don't forget to add it to the [`AssetLoading`] resource.
fn load() {
    // Wait for everything to load
}

fn intro_scene_setup(mut next_state: ResMut<NextState<PlayingState>>) {
    // Setup necessary stuff for the intro_scene

    // For now, skip to the next state
    // This will change if I have time to add the intro cut scene
    next_state.set(PlayingState::Playing);
}

fn intro_scene_update() {}

fn check_win_condition(score: Res<Score>, mut next_state: ResMut<NextState<PlayingState>>) {
    // score.total_villagers != 0 is a cheap way of not triggering the win condition before everthing is setup
    // IMPROVEME: Proper loading flow, so that everything is setup (especially LDtk stuff) before switching to PlayingState::Playing
    if score.villagers_killed == score.total_villagers && score.total_villagers != 0 {
        next_state.set(PlayingState::Win);
    }
}

fn player_death(
    mut commands: Commands,
    mut player: Query<Entity, With<PlayerTag>>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<PlayingState>>,
) {
    score.player_lives -= 1;

    let Ok(player) = player.get_single_mut() else {
        next_state.set(PlayingState::Lose);
        return;
    };

    // Play player death animation
    commands
        .entity(player)
        .insert(new_animation_during_death(PLAYER_ANIMATION_DEATH));
}

fn player_died(
    mut animation_finished: EventReader<AnimationFinishedEvent>,
    score: Res<Score>,
    mut next_state: ResMut<NextState<PlayingState>>,
) {
    // Check if animation is finished
    for event in animation_finished.read() {
        if event.0 == ANIMATIONS::PlayerDeath {
            if score.player_lives == 0 {
                // If no more lives, trigger lose condition.
                next_state.set(PlayingState::Lose);
            } else {
                // Otherwise, respawn the player.
                next_state.set(PlayingState::Respawning);
            }
        }
    }
}

fn player_respawn(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Transform, &mut Visibility), With<PlayerTag>>,
    camera: Query<Entity, With<InGameCamera>>,
    respawn_points: Query<(Entity, &Transform), (With<PlayerRespawnPointTag>, Without<PlayerTag>)>,
    dead_player_handle: Res<DeadPlayerSpriteHandle>,
    mut next_state: ResMut<NextState<PlayingState>>,
) {
    // Just in case we can't find what we need, trigger lose condition.
    let Ok((respawn_entity, respawn_transform)) = respawn_points.get_single() else {
        next_state.set(PlayingState::Lose);
        return;
    };

    let Ok((player, mut player_transform, mut visibility)) = player.get_single_mut() else {
        next_state.set(PlayingState::Lose);
        return;
    };

    let Ok(camera) = camera.get_single() else {
        next_state.set(PlayingState::Lose);
        return;
    };

    // Spawn dead "player", so it's visible later
    commands.spawn(DeadPlayerBundle::new(
        &Transform::from_translation(player_transform.translation.with_z(2.)),
        dead_player_handle.0.clone(),
    ));

    // Hide player
    *visibility = Visibility::Hidden;

    // Detach camera
    commands.entity(player).remove_children(&[camera]);

    // Remove chased tag
    commands.entity(player).remove::<Chased>();

    // Move player to respawn point
    *player_transform = *respawn_transform;

    // Player Idle animation
    commands
        .entity(player)
        .insert(new_animation_during_death(PLAYER_ANIMATION_IDLE));

    // Despawn respawn point
    commands.entity(respawn_entity).despawn_recursive();

    // Show player
    *visibility = Visibility::Inherited;

    // Attach camera
    commands.entity(player).add_child(camera);

    // Switch to playing state
    next_state.set(PlayingState::Playing);
}

fn reset(
    mut commands: Commands,
    dead_players: Query<Entity, With<DeadPlayerTag>>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    *score = Score::default();

    for entity in &dead_players {
        commands.entity(entity).despawn_recursive();
    }

    next_state.set(GameState::Playing);
}
