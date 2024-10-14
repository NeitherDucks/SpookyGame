use bevy::math::IVec2;

use crate::ldtk::animation::AnimationConfig;

/// AI

pub const INVESTIGATOR_VIEW_RANGE: f32 = 5. * 16.; // In world units
pub const INVESTIGATOR_VIEW_HALF_ANGLE: f32 = 35.; // In degrees

pub const VILLAGERS_VIEW_RANGE: f32 = 3. * 16.; // In world units
pub const VILLAGERS_VIEW_HALF_ANGLE: f32 = 50.; // In degrees

pub const NORMAL_SPEED: f32 = 3. * 16.; // In world unites per seconds
pub const RUNNING_SPEED: f32 = 5. * 16.; // In world unites per seconds
pub const CHASE_SPEED: f32 = 6. * 16.; // In world unites per seconds

pub const INVESTIGATING_RADIUS: u32 = 10; // In seconds
pub const INVESTIGATING_TIME: u64 = 10; // In seconds

pub const WANDERING_RADIUS: u32 = 32; // In grid units

pub const MIN_RUN_AWAY_RADIUS: u32 = 32; // In grid units
pub const MAX_RUN_AWAY_RADIUS: u32 = 64; // In grid units
pub const MAX_RUN_AWAY_ANGLE: f32 = 45.; // In degrees

pub const IDLING_TIME: u64 = 5; // In seconds

pub const FIND_NEARBY_MAX_TRIES: u32 = 10;

// AI & PLAYER

pub const PLAYER_SPEED: f32 = 5. * 16.; // In world unites per seconds

pub const INTERACTION_DISTANCE: f32 = 1.1 * 16.; // In world units

// MAP

pub const GRID_SIZE: IVec2 = IVec2::new(30, 30); // Defined in the ldtk file
pub const TILE_SIZE: IVec2 = IVec2::new(16, 16); // Defined in the ldtk file

// ANIMATIONS
//// PLAYER
pub const PLAYER_ANIMATION_IDLE: AnimationConfig = AnimationConfig::new(0, 0, 8);

//// INVESTIGATOR
pub const INVESTIGATOR_ANIMATION_IDLE: AnimationConfig = AnimationConfig::new(0, 0, 8);

//// VILLAGER
pub const VILLAGER_ANIMATION_IDLE: AnimationConfig = AnimationConfig::new(0, 0, 8);
