use amethyst::{
    assets::{Prefab, Handle, ProgressCounter, PrefabLoader, RonFormat},
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

fn get_animation_prefab_handle(
    world: &mut World,
    ron_path: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<AnimationPrefab>> {
    world.exec(|loader: PrefabLoader<'_, AnimationPrefab>| {
        loader.load(ron_path, RonFormat, progress_counter)
    })
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Player;

pub fn load_player(world: &mut World, handlers: &mut Vec<Handle<Prefab<AnimationPrefab>>>, mut progress_counter: &mut ProgressCounter) {
    let ron_path = "sprites/old_man/old_man.ron";

    let player_prefab_handle =
    get_animation_prefab_handle(world, ron_path, &mut progress_counter);

    handlers.push(player_prefab_handle);
}

pub fn add_player(world: &mut World, handle: Handle<Prefab<AnimationPrefab>>) {
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
        .with(handle)
        .build();
}