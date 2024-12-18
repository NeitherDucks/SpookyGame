// Very simple "AI"
// Execute different Actions based on which component exists on the Entity

use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};
use bevy_ecs_ldtk::utils::grid_coords_to_translation;

mod chase;
mod dead;
mod idle;
mod investigate;
mod run_away;
mod talk_to_investigator;
mod transitions;
mod wander;

pub use chase::Chased;
use chase::*;
pub use dead::Dead;
use dead::*;
use idle::*;
use investigate::*;
use run_away::*;
use talk_to_investigator::*;
use transitions::*;
use wander::*;

use crate::{
    config::*,
    ldtk::{animation::AnimationOffset, entities::Aim, Light},
    pathfinding::Path,
    states::PlayingState,
};

// All the logic for transitioning from different Tasks will be executed during this schedule.
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct AiTransition;

// All logic when entering a task will be executed during this schedule.
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct AiOnEnter;

// All logic when leaving a task will be executed during this schedule.
// This is executed before AiOnEnter, so we can clean up nicely and not interfer with components added in the AiOnEnter phase.
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct AiOnExit;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementSpeed(f32);

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(AiTransition);
        app.init_schedule(AiOnEnter);
        app.init_schedule(AiOnExit);
        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_before(Update, AiTransition);
        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_before(Update, AiOnExit);
        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_before(Update, AiOnEnter);

        app.add_systems(
            AiTransition,
            (
                notice_player,
                idle_to_wandering,
                wandering_to_idle,
                idle_or_wandering_to_investigating,
                chasing_to_investigating,
                chasing_to_killing,
                investigating_to_idle,
                run_away_to_idle,
                running_away_to_talk_to_investigator,
                talk_to_investigator_to_running_away,
            )
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            AiOnExit,
            (
                chase_on_exit,
                investigate_on_exit,
                run_away_on_exit,
                wander_on_exit,
                talk_to_investigator_on_exit,
            )
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            AiOnEnter,
            (
                idle_on_enter,
                chase_on_enter,
                investigate_on_enter,
                run_away_on_enter,
                wander_on_enter,
                talk_to_investigator_on_enter,
                dead_on_enter,
            )
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            Update,
            (
                chase_update,
                investigate_update,
                talk_to_investigator_update,
                follow_path,
                update_animation_aim,
            )
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            PostUpdate,
            (check_empty_path, nothing_to_idle).run_if(in_state(PlayingState::Playing)),
        )
        .register_type::<Chase>();
    }
}

/// If has a [`Path`], move the entity along.
fn follow_path(
    mut query: Query<(&mut Transform, &mut Path, &MovementSpeed, &mut Aim)>,
    time: Res<Time>,
) {
    for (mut transform, mut path, speed, mut aim) in &mut query {
        if let Some(next_target) = path.steps.front() {
            let delta =
                grid_coords_to_translation(*next_target, TILE_SIZE) - transform.translation.xy();
            let travel_amount = time.delta_seconds() * speed.0;

            if delta.length() > travel_amount * 1.1 {
                let direction = delta.normalize_or_zero();
                let travel = direction.extend(0.) * travel_amount;
                transform.translation += travel;
                aim.0 = direction;
            } else {
                path.steps.pop_front();
            }
        }
    }
}

fn update_animation_aim(
    mut query: Query<(&Children, &mut AnimationOffset, &Aim), Changed<Aim>>,
    mut lights: Query<&mut Transform, With<Light>>,
) {
    for (children, mut animation, aim) in &mut query {
        let mut angle: f32 = 0.;
        let mut offset: Vec2 = Vec2::new(48., 0.);

        if aim.0.x.abs() > aim.0.y.abs() {
            if aim.0.x > 0. {
                animation.offset = 0;
            } else {
                animation.offset = 1;
                angle = 180.;
                offset = Vec2::new(-48., 0.);
            }
        } else {
            if aim.0.y > 0. {
                animation.offset = 3;
                angle = 90.;
                offset = Vec2::new(0., 48.);
            } else {
                animation.offset = 2;
                angle = -90.;
                offset = Vec2::new(0., -48.);
            }
        }

        for child in children {
            if let Ok(mut light) = lights.get_mut(*child) {
                light.rotation = Quat::from_euler(EulerRot::XYZ, 0., 0., angle.to_radians());
                light.translation = offset.extend(0.);
            }
        }
    }
}
