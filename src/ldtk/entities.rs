use bevy::{prelude::*, render::view::RenderLayers};
use bevy_ecs_ldtk::{GridCoords, LdtkEntity, LdtkIntCell, LdtkSpriteSheetBundle};
use bevy_rapier2d::prelude::*;

use crate::{
    config::{INVESTIGATOR_ANIMATION_IDLE, PLAYER_ANIMATION_IDLE, VILLAGER_ANIMATION_IDLE},
    grid::Tile,
    player::PlayerTag,
    rendering::PIXEL_PERFECT_LAYERS,
};

use super::animation::{AnimationConfig, AnimationTimer};

#[derive(Component, PartialEq, Eq)]
pub enum EnemyTag {
    Investigator,
    Villager,
}

#[derive(Component, Clone, Copy)]
pub struct Aim(pub Vec2);

impl Default for Aim {
    fn default() -> Self {
        Aim(Vec2::new(1., 0.))
    }
}

#[derive(Component)]
pub struct InteractibleTriggered {
    pub location: GridCoords,
}

#[derive(Clone, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl Default for ColliderBundle {
    fn default() -> Self {
        ColliderBundle {
            collider: Collider::cuboid(8., 8.),
            rigid_body: RigidBody::Fixed,
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            density: ColliderMassProperties::default(),
            gravity_scale: GravityScale::default(),
            velocity: Velocity::default(),
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    collider: ColliderBundle,
    controller: KinematicCharacterController,
    animation: AnimationConfig,
    animation_timer: AnimationTimer,
    tag: PlayerTag,
    render_layer: RenderLayers,
    name: Name,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            collider: ColliderBundle::default(),
            controller: KinematicCharacterController::default(),
            animation: PLAYER_ANIMATION_IDLE,
            animation_timer: AnimationTimer::new(PLAYER_ANIMATION_IDLE),
            tag: PlayerTag,
            render_layer: PIXEL_PERFECT_LAYERS,
            name: Name::new("Player"),
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
            grid_coords: GridCoords::default(),
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct InvestigatorBundle {
    collider: ColliderBundle,
    animation: AnimationConfig,
    animation_timer: AnimationTimer,
    tag: EnemyTag,
    render_layer: RenderLayers,
    name: Name,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    aim: Aim,
}

impl Default for InvestigatorBundle {
    fn default() -> Self {
        InvestigatorBundle {
            collider: ColliderBundle::default(),
            animation: INVESTIGATOR_ANIMATION_IDLE,
            animation_timer: AnimationTimer::new(INVESTIGATOR_ANIMATION_IDLE),
            tag: EnemyTag::Investigator,
            render_layer: PIXEL_PERFECT_LAYERS,
            name: Name::new("Investigator"),
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
            grid_coords: GridCoords::default(),
            aim: Aim::default(),
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct VillagerBundle {
    collider: ColliderBundle,
    animation: AnimationConfig,
    animation_timer: AnimationTimer,
    tag: EnemyTag,
    render_layer: RenderLayers,
    name: Name,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    aim: Aim,
}

impl Default for VillagerBundle {
    fn default() -> Self {
        VillagerBundle {
            collider: ColliderBundle::default(),
            animation: VILLAGER_ANIMATION_IDLE,
            animation_timer: AnimationTimer::new(VILLAGER_ANIMATION_IDLE),
            tag: EnemyTag::Villager,
            render_layer: PIXEL_PERFECT_LAYERS,
            name: Name::new("Villager"),
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
            grid_coords: GridCoords::default(),
            aim: Aim::default(),
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct HiddingSpotBundle {
    render_layer: RenderLayers,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

impl Default for HiddingSpotBundle {
    fn default() -> Self {
        HiddingSpotBundle {
            render_layer: PIXEL_PERFECT_LAYERS,
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct InteractibleBundle {
    render_layer: RenderLayers,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

impl Default for InteractibleBundle {
    fn default() -> Self {
        InteractibleBundle {
            render_layer: PIXEL_PERFECT_LAYERS,
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
        }
    }
}

#[derive(Bundle, LdtkIntCell)]
pub struct CollisionTileBundle {
    tile: Tile,
    collider: ColliderBundle,
}

impl Default for CollisionTileBundle {
    fn default() -> Self {
        CollisionTileBundle {
            tile: Tile,
            collider: ColliderBundle::default(),
        }
    }
}
