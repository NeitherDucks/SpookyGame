use std::time::Duration;

use bevy::prelude::*;

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

#[derive(Event, Reflect)]
pub struct AnimationFinishedEvent(pub ANIMATIONS);

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct DuringDeathAnimation;

#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
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

    pub fn get_name(&self) -> ANIMATIONS {
        self.name
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct AnimationOffset {
    pub actual: usize,
    pub offset: usize,
}

impl Default for AnimationOffset {
    fn default() -> Self {
        Self {
            actual: 0,
            offset: 0,
        }
    }
}

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct AnimationTimer(pub Timer);

impl AnimationTimer {
    pub fn new(config: AnimationConfig) -> Self {
        Self(AnimationConfig::timer_from_fps(config.fps))
    }
}

pub fn animation_changed(
    mut query: Query<
        (
            &AnimationConfig,
            &mut TextureAtlas,
            Option<&mut AnimationOffset>,
        ),
        Changed<AnimationConfig>,
    >,
) {
    for (config, mut atlas, offset) in &mut query {
        match offset {
            Some(mut offset) => {
                offset.actual = config.first_sprite_index;
            }
            None => atlas.index = config.first_sprite_index,
        }
    }
}

pub fn animation_offset_changed(
    mut query: Query<(&mut TextureAtlas, &mut AnimationOffset), Changed<AnimationOffset>>,
) {
    for (mut atlas, offset) in &mut query {
        atlas.index = offset.actual + (offset.offset * 4);
    }
}

/// Wrapper to limit animations to [`PlayingState::Playing`]
pub fn update_animations(
    time: Res<Time>,
    mut query: Query<(
        &AnimationConfig,
        Option<&mut AnimationOffset>,
        &mut AnimationTimer,
        &mut TextureAtlas,
    )>,
    mut anim_finished_event: EventWriter<AnimationFinishedEvent>,
) {
    for (config, offset, timer, atlas) in &mut query {
        match offset {
            Some(offset) => update_animation_internal_with_offset(
                &time,
                config,
                timer,
                offset,
                &mut anim_finished_event,
            ),
            None => {
                update_animation_internal(&time, config, timer, atlas, &mut anim_finished_event)
            }
        }
    }
}

/// Wrapper to limit animations to [`PlayingState::Death`] (only animation with the [`DuringDeathAnimation`] tag).
pub fn update_animations_during_death(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationConfig,
            Option<&mut AnimationOffset>,
            &mut AnimationTimer,
            &mut TextureAtlas,
        ),
        With<DuringDeathAnimation>,
    >,
    mut anim_finished_event: EventWriter<AnimationFinishedEvent>,
) {
    for (config, offset, timer, atlas) in &mut query {
        match offset {
            Some(offset) => update_animation_internal_with_offset(
                &time,
                config,
                timer,
                offset,
                &mut anim_finished_event,
            ),
            None => {
                update_animation_internal(&time, config, timer, atlas, &mut anim_finished_event)
            }
        }
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

            // and resets when we move back to the first frame.
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

/// This system loops through all the sprites in the [`TextureAtlas`], from  `first_sprite_index` to
/// `last_sprite_index` (both defined in [`AnimationConfig`]).
fn update_animation_internal_with_offset(
    time: &Res<Time>,
    config: &AnimationConfig,
    mut timer: Mut<AnimationTimer>,
    mut offset: Mut<AnimationOffset>,
    event_writer: &mut EventWriter<AnimationFinishedEvent>,
) {
    // we track how long the current sprite has been displayed for
    timer.0.tick(time.delta());

    // If it has been displayed for the user-defined amount of time (fps)...
    if timer.0.just_finished() {
        if offset.actual == config.last_sprite_index {
            // ...and it IS the last frame

            // emit animation finished event
            event_writer.send(AnimationFinishedEvent(config.name));

            // and resets when we move back to the first frame.
            if config.reset {
                offset.actual = config.first_sprite_index;
            }

            // and looping, go to first frame and reset the timer.
            if config.repeat {
                offset.actual = config.first_sprite_index;
                timer.0 = AnimationConfig::timer_from_fps(config.fps);
            }
        } else {
            // ...and it is NOT the last frame, then we move to the next frame...
            offset.actual += 1;
            // ...and reset the frame timer to start counting all over again
            timer.0 = AnimationConfig::timer_from_fps(config.fps);
        }

        // atlas.index = offset.actual + (offset.offset * 4);
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
