use bevy::math::IVec2;

use crate::ldtk::animation::{AnimationConfig, ANIMATIONS};

/// GLOBALS

pub const PIXEL_PER_TILE: f32 = 16.;

/// AI

pub const INVESTIGATOR_VIEW_RANGE: f32 = 5. * PIXEL_PER_TILE; // In world units
pub const INVESTIGATOR_VIEW_HALF_ANGLE: f32 = 90.; // In degrees
pub const INVESTIGATOR_HEARING_RANGE: f32 = 8. * PIXEL_PER_TILE; // In world units

pub const VILLAGERS_VIEW_RANGE: f32 = 3. * PIXEL_PER_TILE; // In world units
pub const VILLAGERS_VIEW_HALF_ANGLE: f32 = 50.; // In degrees

pub const NORMAL_SPEED: f32 = 3. * PIXEL_PER_TILE; // In world unites per seconds
pub const RUNNING_SPEED: f32 = 5. * PIXEL_PER_TILE; // In world unites per seconds
pub const CHASE_SPEED: f32 = 6. * PIXEL_PER_TILE; // In world unites per seconds

pub const INVESTIGATING_RADIUS: u32 = 10; // In seconds
pub const INVESTIGATING_TIME: u64 = 10; // In seconds

pub const WANDERING_RADIUS: u32 = 32; // In grid units

pub const MIN_RUN_AWAY_RADIUS: u32 = 8; // In grid units
pub const MAX_RUN_AWAY_RADIUS: u32 = 16; // In grid units
pub const MAX_RUN_AWAY_ANGLE: f32 = 45.; // In degrees

pub const IDLING_TIME: u64 = 5; // In seconds

pub const FIND_NEARBY_MAX_TRIES: u32 = 10;

// AI & PLAYER

pub const PLAYER_SPEED: f32 = 7. * PIXEL_PER_TILE; // In world unites per seconds

pub const INTERACTION_DISTANCE: f32 = 1.1 * PIXEL_PER_TILE; // In world units

// MAP

pub const GRID_SIZE: IVec2 = IVec2::new(54, 40); // Defined in the ldtk file
pub const TILE_SIZE: IVec2 = IVec2::splat(PIXEL_PER_TILE as i32); // Defined in the ldtk file

// ANIMATIONS

//// PLAYER
pub const PLAYER_ANIMATION_IDLE: AnimationConfig =
    AnimationConfig::new(ANIMATIONS::PlayerIdle, 0, 5, 8).repeats();
pub const PLAYER_ANIMATION_RUN: AnimationConfig =
    AnimationConfig::new(ANIMATIONS::PlayerRun, 14, 25, 8).repeats();
pub const PLAYER_ANIMATION_ATTACK: AnimationConfig =
    AnimationConfig::new(ANIMATIONS::PlayerAttack, 28, 41, 12).resets();
pub const PLAYER_ANIMATION_DEATH: AnimationConfig =
    AnimationConfig::new(ANIMATIONS::PlayerDeath, 42, 50, 8);
pub const PLAYER_ANIMATION_HIDDING: AnimationConfig =
    AnimationConfig::new(ANIMATIONS::PlayerHidding, 10, 10, 8);

//// INVESTIGATOR
pub const INVESTIGATOR_ANIMATION_IDLE: AnimationConfig =
    AnimationConfig::new(ANIMATIONS::InvestigatorIdle, 0, 3, 8).repeats();
pub const INVESTIGATOR_ANIMATION_WALK: AnimationConfig =
    AnimationConfig::new(ANIMATIONS::InvestigatorWalk, 16, 19, 8).repeats();
pub const INVESTIGATOR_ANIMATION_RUN: AnimationConfig =
    AnimationConfig::new(ANIMATIONS::InvestigatorRun, 32, 35, 8).repeats();

//// VILLAGER
pub const VILLAGER_ANIMATION_IDLE: AnimationConfig =
    AnimationConfig::new(ANIMATIONS::VillagerIdle, 0, 3, 8).repeats();
pub const VILLAGER_ANIMATION_WALK: AnimationConfig =
    AnimationConfig::new(ANIMATIONS::VillagerWalk, 16, 19, 8).repeats();
pub const VILLAGER_ANIMATION_FLEE: AnimationConfig =
    AnimationConfig::new(ANIMATIONS::VillagerFlee, 48, 51, 8).repeats();
pub const VILLAGER_ANIMATION_DEATH: AnimationConfig =
    AnimationConfig::new(ANIMATIONS::VillagerDeath, 64, 67, 8);

/// NOISE MAKER
pub const NOISE_MAKER_ANIMATION: AnimationConfig =
    AnimationConfig::new(ANIMATIONS::NoiseMaker, 0, 3, 8).resets();
