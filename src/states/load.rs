use amethyst::{
    assets::{Handle, ProgressCounter, Prefab, PrefabLoader, RonFormat},
    prelude::{GameData, SimpleState, StateData},
    ecs::prelude::World,
};

use crate::components::{
    AnimationPrefab,
    camera::load_camera,
    player::load_player,
};

#[derive(Default)]
pub struct LoadState;

fn get_animation_prefab_handle(
    world: &mut World,
    ron_path: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<AnimationPrefab>> {
    world.exec(|loader: PrefabLoader<'_, AnimationPrefab>| {
        loader.load(ron_path, RonFormat, progress_counter)
    })
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut progress_counter = ProgressCounter::new();
        let ron_path = "sprites/old_man/old_man.ron";

        let player_prefab_handle =
        get_animation_prefab_handle(world, ron_path, &mut progress_counter);

        load_camera(world);
        load_player(world, player_prefab_handle);
    }
}
