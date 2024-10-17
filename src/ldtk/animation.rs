use std::time::Duration;

use bevy::prelude::*;

// Enum of all animations
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ANIMATIONS {
    PlayerIdle,
    PlayerDeath,
    InvestigatorIdle,
    VillagerIdle,
    VillagerDeath,
    NoiseMaker,
}

#[derive(Event)]
pub struct AnimationFinishedEvent(pub ANIMATIONS);

#[derive(Component)]
pub struct DuringDeathAnimation;

#[derive(Component, Clone, Copy)]
pub struct AnimationConfig {
    name: ANIMATIONS,
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    repeat: bool,
    reset: bool,
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
