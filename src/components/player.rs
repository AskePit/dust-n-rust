use amethyst::{
    assets::{Prefab, Handle},
    ecs::prelude::World,
    ecs::{Component, DenseVecStorage},
    core::{
        Transform,
        WithNamed,
    },
    prelude::{Builder, WorldExt},
};

use crate::{
    components::{Animation, AnimationId, AnimationPrefab, Motion}
};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum PlayerState {
    Idling,
    Walking,
    Jumping,
    Attacking,
    Hitted,
    Dead,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idling
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub state: PlayerState,
}

impl Player {
    pub fn new() -> Self {
        Player {
            state: PlayerState::Idling,
        }
    }
}

pub fn load_player(world: &mut World, prefab: Handle<Prefab<AnimationPrefab>>) {
    let mut transform = Transform::default();
    transform.set_translation_x(0.0);
    transform.set_translation_y(0.0);

    world
        .create_entity()
        .with(Player::new())
        .named("Player")
        .with(transform)
        .with(Motion::default())
        .with(Animation::new(
            AnimationId::Idle,
            vec![
                AnimationId::Idle,
                AnimationId::Walk,
                AnimationId::Jump,
                AnimationId::Attack,
                AnimationId::Hitted,
                AnimationId::Death,
            ],
        ))
        .with(prefab)
        .build();
}