use std::{
    path::PathBuf,
};

use amethyst::{
    core::{
        transform::Transform,
        math::Vector3,
    },
    assets::{AssetStorage, Handle, ProgressCounter, Prefab, Loader, PrefabLoader, RonFormat},
    prelude::{GameData, SimpleState, StateData},
    ecs::prelude::{World},
    prelude::WorldExt,
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheetFormat, SpriteSheetHandle, SpriteRender},
        SpriteSheet, Texture,
    },
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

pub fn get_sprite_sheet_handle(
    world: &World,
    texture_path: &str,
    ron_path: &str,
    progress_counter: &mut ProgressCounter,
) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = &world.read_resource::<Loader>();
        let texture_storage = &world.read_resource::<AssetStorage<Texture>>();
        loader.load(texture_path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = &world.read_resource::<Loader>();
    let sprite_sheet_store = &world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        progress_counter,
        &sprite_sheet_store,
    )
}

fn load_level(world: &mut World, mut progress_counter: &mut ProgressCounter) {

    let level_root = PathBuf::from("backgrounds/test_level");

    let width = 1200*2;
    let height = 225*2;
    let ground = 33.0*2.0;

    let backgrounds_map = [
        ("z-10_1.png", -10.0f32),
        ("z-10_2.png", -10.0f32),
        ("z-20_1.png", -20.0f32),
        ("z-20_2.png", -20.0f32),
        ("z-20_3.png", -20.0f32),
        ("z-30.png", -30.0f32),
        ("z+0.png", -1.0f32),
        ("z+1.png", 1.0f32),
    ];

    for el in &backgrounds_map {
        let (bg_path, z) = el;
        let bg_handle = 
        get_sprite_sheet_handle(world, level_root.join(bg_path).to_str().unwrap(), "backgrounds/test_bg.ron", &mut progress_counter);

        let mut transform = Transform::default();
        // position left-bottom corner to world's (0; 0) position
        transform.set_translation_xyz(width as f32/2.0, height as f32/2.0 - ground, *z);
        transform.set_scale(Vector3::new(2.0, 2.0, 2.0));

        let sprite_render = SpriteRender {
            sprite_sheet: bg_handle,
            sprite_number: 0,
        };

        world
            .create_entity()
            .with(transform)
            .with(sprite_render)
            .build();
    }
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut progress_counter = ProgressCounter::new();
        let ron_path = "sprites/old_man/old_man.ron";

        let player_prefab_handle =
        get_animation_prefab_handle(world, ron_path, &mut progress_counter);

        load_level(world, &mut progress_counter);
        load_camera(world);
        load_player(world, player_prefab_handle);
    }
}
