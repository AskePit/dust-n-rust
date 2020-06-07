use amethyst::{
    core::transform::Transform,
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

fn load_level(world: &mut World, background_handle: Handle<SpriteSheet>) {
    //let sprite_sheet: &SpriteSheet;
    let background_width = 1200;
    let background_height = 450;
    /*{
        let sprite_sheet_store = &world.read_resource::<AssetStorage<SpriteSheet>>();
        sprite_sheet = sprite_sheet_store.get(&background_handle).unwrap();
        background_width = sprite_sheet.sprites[0].width;
        background_height = sprite_sheet.sprites[0].height;
    }*/

    let mut background_transform = Transform::default();
    // position left-bottom corner to world's (0; 0) position
    background_transform.set_translation_xyz(background_width as f32/2.0, background_height as f32/2.0, -10.0);

    let sprite_render = SpriteRender {
        sprite_sheet: background_handle,
        sprite_number: 0,
    };

    world
    .create_entity()
    .with(background_transform)
    .with(sprite_render)
    .build();
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut progress_counter = ProgressCounter::new();
        let ron_path = "sprites/old_man/old_man.ron";

        let background_handle = 
        get_sprite_sheet_handle(world, "backgrounds/brick_wall.png", "backgrounds/brick_wall.ron", &mut progress_counter);

        let player_prefab_handle =
        get_animation_prefab_handle(world, ron_path, &mut progress_counter);

        load_level(world, background_handle);
        load_camera(world);
        load_player(world, player_prefab_handle);
    }
}
