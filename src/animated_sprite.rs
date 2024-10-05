use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
// pub struct Animations(pub Vec<Animation>);
pub struct Animations(pub HashMap<String, AnimationIndices>);

#[derive(Component, Clone, Copy)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Bundle)]
pub struct AnimatedSprite {
    pub sprite: SpriteBundle,
    pub animation: AnimationIndices,
    pub atlas: TextureAtlas,
    pub timer: AnimationTimer,
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationIndices,
        &mut TextureAtlas,
        &mut AnimationTimer,
    )>,
) {
    for (animation, mut atlas, mut timer) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == animation.last {
                animation.first
            } else {
                atlas.index + 1
            };
        }
    }
}

pub fn set_animation(sprite: &mut AnimatedSprite, animations: &Animations, name: &String) {
    if animations.0.contains_key(name) {
        let animation = animations.0.get(name).unwrap();

        sprite.animation = *animation;
        sprite.timer.reset();
    } else {
        panic!("Could not switch to animation: {}", name);
    }
}
