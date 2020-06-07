use amethyst::{
    assets::{Prefab, Handle},
    ecs::prelude::World,
    ecs::{Component, DenseVecStorage},
    core::{
        Transform,
        WithNamed,
        math::Vector3,
    },
    prelude::{Builder, WorldExt},
};

use crate::{
    components::{Animation, AnimationId, AnimationPrefab, Locomotion}
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Player;

pub fn load_player(world: &mut World, prefab: Handle<Prefab<AnimationPrefab>>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(100.0, 0.0, 0.0);
    transform.set_scale(Vector3::new(2.0, 2.0, 2.0));

    world
        .create_entity()
        .with(Player)
        .named("Player")
        .with(transform)
        .with(Locomotion::default())
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