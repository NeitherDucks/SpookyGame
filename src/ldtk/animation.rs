use std::time::Duration;

use bevy::prelude::*;

use super::{
    INVESTIGATOR_ANIMATION_IDLE, INVESTIGATOR_ANIMATION_RUN, INVESTIGATOR_ANIMATION_WALK,
    VILLAGER_ANIMATION_DEATH, VILLAGER_ANIMATION_FLEE, VILLAGER_ANIMATION_IDLE,
    VILLAGER_ANIMATION_WALK,
};

// Enum of all animations
#[derive(Reflect, Clone, Copy, Eq, PartialEq)]
pub enum ANIMATIONS {
    PlayerIdle,
    PlayerRun,
    PlayerAttack,
    PlayerDeath,
    PlayerHidding,
    InvestigatorIdle,
    InvestigatorWalk,
    InvestigatorRun,
    VillagerIdle,
    VillagerWalk,
    VillagerFlee,
    VillagerDeath,
    NoiseMaker,
}

#[derive(Event)]
pub struct AnimationFinishedEvent(pub ANIMATIONS);

#[derive(Component)]
pub struct DuringDeathAnimation;

#[derive(Component, Reflect, Clone, Copy)]
pub struct AnimationConfig {
    name: ANIMATIONS,
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    repeat: bool,
    reset: bool,
    current_offset: u8,
}

impl AnimationConfig {
    pub const fn new(name: ANIMATIONS, first: usize, last: usize, fps: u8) -> Self {
        Self {
            name,
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            repeat: false,
            reset: false,
            current_offset: 0,
        }
    }

    pub const fn resets(&self) -> Self {
        Self {
            reset: true,
            ..*self
        }
    }

    pub const fn repeats(&self) -> Self {
        Self {
            repeat: true,
            ..*self
        }
    }

    pub const fn with_offset(&self, offset: u8) -> Self {
        Self {
            current_offset: offset,
            ..*self
        }
    }

    pub fn get_offset(&self) -> u8 {
        self.current_offset
    }

    pub fn get_name(&self) -> ANIMATIONS {
        self.name
    }

    pub fn set_offset_animation(&mut self, shift: u8) {
        (self.first_sprite_index, self.last_sprite_index) = match self.name {
            ANIMATIONS::InvestigatorIdle => INVESTIGATOR_ANIMATION_IDLE.offset(shift),
            ANIMATIONS::InvestigatorRun => INVESTIGATOR_ANIMATION_RUN.offset(shift),
            ANIMATIONS::InvestigatorWalk => INVESTIGATOR_ANIMATION_WALK.offset(shift),
            ANIMATIONS::VillagerDeath => VILLAGER_ANIMATION_DEATH.offset(shift),
            ANIMATIONS::VillagerFlee => VILLAGER_ANIMATION_FLEE.offset(shift),
            ANIMATIONS::VillagerIdle => VILLAGER_ANIMATION_IDLE.offset(shift),
            ANIMATIONS::VillagerWalk => VILLAGER_ANIMATION_WALK.offset(shift),
            _ => {
                return;
            }
        };

        self.current_offset = shift;
    }

    fn offset(&self, shift: u8) -> (usize, usize) {
        (
            self.first_sprite_index + (shift * 4) as usize,
            self.last_sprite_index + (shift * 4) as usize,
        )
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

impl AnimationTimer {
    pub fn new(config: AnimationConfig) -> Self {
        Self(AnimationConfig::timer_from_fps(config.fps))
    }
}

pub fn animation_changed(
    mut query: Query<(&AnimationConfig, &mut TextureAtlas), Changed<AnimationConfig>>,
) {
    for (config, mut atlas) in &mut query {
        atlas.index = config.first_sprite_index;
    }
}

/// Wrapper to limit animations to [`PlayingState::Playing`]
pub fn update_animations(
    time: Res<Time>,
    mut query: Query<(&AnimationConfig, &mut AnimationTimer, &mut TextureAtlas)>,
    mut anim_finished_event: EventWriter<AnimationFinishedEvent>,
) {
    for (config, timer, atlas) in &mut query {
        update_animation_internal(&time, config, timer, atlas, &mut anim_finished_event);
    }
}

/// Wrapper to limit animations to [`PlayingState::Death`] (only animation with the [`DuringDeathAnimation`] tag).
pub fn update_animations_during_death(
    time: Res<Time>,
    mut query: Query<
        (&AnimationConfig, &mut AnimationTimer, &mut TextureAtlas),
        With<DuringDeathAnimation>,
    >,
    mut anim_finished_event: EventWriter<AnimationFinishedEvent>,
) {
    for (config, timer, atlas) in &mut query {
        update_animation_internal(&time, config, timer, atlas, &mut anim_finished_event);
    }
}

/// This system loops through all the sprites in the [`TextureAtlas`], from  `first_sprite_index` to
/// `last_sprite_index` (both defined in [`AnimationConfig`]).
fn update_animation_internal(
    time: &Res<Time>,
    config: &AnimationConfig,
    mut timer: Mut<AnimationTimer>,
    mut atlas: Mut<TextureAtlas>,
    event_writer: &mut EventWriter<AnimationFinishedEvent>,
) {
    // we track how long the current sprite has been displayed for
    timer.0.tick(time.delta());

    // If it has been displayed for the user-defined amount of time (fps)...
    if timer.0.just_finished() {
        if atlas.index == config.last_sprite_index {
            // ...and it IS the last frame

            // emit animation finished event
            event_writer.send(AnimationFinishedEvent(config.name));

            // and resets then we move back to the first frame.
            if config.reset {
                atlas.index = config.first_sprite_index;
            }

            // and looping, go to first frame and reset the timer.
            if config.repeat {
                atlas.index = config.first_sprite_index;
                timer.0 = AnimationConfig::timer_from_fps(config.fps);
            }
        } else {
            // ...and it is NOT the last frame, then we move to the next frame...
            atlas.index += 1;
            // ...and reset the frame timer to start counting all over again
            timer.0 = AnimationConfig::timer_from_fps(config.fps);
        }
    }
}

/// Utiliy function to create a timer for the specified animation config, and return a bundle.
pub fn new_animation(animation: AnimationConfig) -> (AnimationConfig, AnimationTimer) {
    (animation, AnimationTimer::new(animation))
}

/// Same as [`new_animation`] but with the [`DuringDeathAnimation`] tag added.
pub fn new_animation_during_death(
    animation: AnimationConfig,
) -> (AnimationConfig, AnimationTimer, DuringDeathAnimation) {
    (
        animation,
        AnimationTimer::new(animation),
        DuringDeathAnimation,
    )
}
