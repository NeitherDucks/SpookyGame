use bevy::prelude::*;

pub mod investigator;
pub mod villager;

pub use investigator::InvestigatorBundle;
pub use villager::VillagerBundle;

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
