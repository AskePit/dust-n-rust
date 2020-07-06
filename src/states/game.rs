use amethyst::{
    core::{
        transform::Transform,
        math::Vector3,
    },
    assets::{Handle, Prefab},
    prelude::{GameData, SimpleState, StateData},
    ecs::prelude::{World},
    prelude::WorldExt,
    prelude::*,
    renderer::{
        sprite::{SpriteRender},
    },
};

use crate::{
    components::{
        AnimationPrefab,
        camera::add_camera,
        player::add_player,
    },
    resources::*,
};

fn add_level(world: &mut World, level: LevelData) {

    let width = 1200*2;
    let height = 225*2;
    let ground = 33.0*2.0;

    for layer in &level.layers_list {
        let z = layer.depth;

        let mut transform = Transform::default();
        // position left-bottom corner to world's (0; 0) position
        transform.set_translation_xyz(width as f32/2.0, height as f32/2.0 - ground, z);
        transform.set_scale(Vector3::new(2.0, 2.0, 2.0));

        let sprite_render = SpriteRender {
            sprite_sheet: layer.sprite_handle.as_ref().unwrap().clone(),
            sprite_number: 0,
        };

        world
            .create_entity()
            .with(transform)
            .with(sprite_render)
            .build();
    }
}

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let level: LevelData;
        {
            let levels = &world.read_resource::<LevelDataList>();
            level = levels.levels_list[0].clone();
        }

        let prefab: Handle<Prefab<AnimationPrefab>>;
        {
            let prefabs = &world.read_resource::<PrefabsDataList>();
            prefab = prefabs.prefabs_list[0].clone();
        }

        add_level(world, level);
        add_player(world, prefab);
        add_camera(world);
    }
}
