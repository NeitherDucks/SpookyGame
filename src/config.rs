/// AI

pub const INVESTIGATOR_VIEW_RANGE: f32 = 5. * 16.; // In world units
pub const INVESTIGATOR_VIEW_HALF_ANGLE: f32 = 35.; // In degrees

pub const VILLAGERS_VIEW_RANGE: f32 = 3. * 16.; // In world units
pub const VILLAGERS_VIEW_HALF_ANGLE: f32 = 50.; // In degrees

pub const NORMAL_SPEED: f32 = 75.0;
pub const RUNNING_SPEED: f32 = 90.0;
pub const CHASE_SPEED: f32 = 110.0;

pub const INVESTIGATING_RADIUS: u32 = 10; // In seconds
pub const INVESTIGATING_TIME: u64 = 10; // In seconds

pub const WANDERING_RADIUS: u32 = 32; // In grid units

pub const MIN_RUN_AWAY_RADIUS: u32 = 32; // In grid units
pub const MAX_RUN_AWAY_RADIUS: u32 = 64; // In grid units
pub const MAX_RUN_AWAY_ANGLE: f32 = 45.; // In degrees

pub const IDLING_TIME: u64 = 5; // In seconds

pub const FIND_NEARBY_MAX_TRIES: u32 = 10;

// AI & PLAYER

pub const INTERACTION_DISTANCE: f32 = 1.1 * 16.; // In world units

// MAP

pub const GRID_SIZE: usize = 20;
