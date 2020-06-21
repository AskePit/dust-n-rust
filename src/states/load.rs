use std::{
    path::PathBuf,
};

use amethyst::{
    assets::{AssetStorage, Handle, ProgressCounter, Prefab, Loader,},
    prelude::{GameData, SimpleState, StateData},
    ecs::prelude::{World},
    prelude::WorldExt,
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheetFormat, SpriteSheetHandle},
        SpriteSheet, Texture,
    },
};

use crate::{
    components::{
        AnimationPrefab,
        player::load_player,
    },
    states::GameState,
    resources::BackgroundsMeta,
};

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

fn load_level(world: &mut World, handlers: &mut Vec<Handle<SpriteSheet>>, mut progress_counter: &mut ProgressCounter) {

    let level_root = PathBuf::from("backgrounds/test_level");

    {
        let backgrounds_meta = BackgroundsMeta::new();
        world.insert(backgrounds_meta);
    }

    let backgrounds_meta = world.read_resource::<BackgroundsMeta>();

    for el in &backgrounds_meta.data {
        let (bg_path, _) = el;
        let bg_handle = 
        get_sprite_sheet_handle(world, level_root.join(bg_path).to_str().unwrap(), "backgrounds/test_bg.ron", &mut progress_counter);

        handlers.push(bg_handle);
    }
}

pub struct AssetHandlers
{
    pub prefabs: Vec<Handle<Prefab<AnimationPrefab>>>,
    pub sprite_sheets: Vec<Handle<SpriteSheet>>,
}

impl Default for AssetHandlers {
    fn default() -> Self {
        AssetHandlers {
            prefabs: Vec::new(),
            sprite_sheets: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct LoadState {
    progress_counter: ProgressCounter,
    handlers: Option<AssetHandlers>,
}

impl LoadState {
    pub fn new() -> Self {
        Self {
            progress_counter: ProgressCounter::new(),
            handlers: Some(AssetHandlers::default())
        }
    }
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        load_level(world, &mut self.handlers.as_mut().unwrap().sprite_sheets, &mut self.progress_counter);
        load_player(world, &mut self.handlers.as_mut().unwrap().prefabs, &mut self.progress_counter);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            Trans::Switch(Box::new(GameState::new(self.handlers.take().unwrap())))
        } else {
            Trans::None
        }
    }
}
