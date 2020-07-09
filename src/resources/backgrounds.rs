use std::{
    path::PathBuf,
	fs::{self},
	io,
};

use regex::Regex;

use amethyst::{
	assets::{AssetStorage, ProgressCounter, Loader},
	ecs::prelude::{World},
	prelude::WorldExt,
	renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheetFormat, SpriteSheetHandle},
		SpriteSheet, Texture,
    },
};

use crate::utils;

#[derive(Clone)]
pub struct LevelLayer
{
	pub depth: f32,
	pub sprite_handle: Option<SpriteSheetHandle>,
}

#[derive(Default, Clone)]
pub struct LevelData
{
	pub name: String,
	pub layers_list: Vec<LevelLayer>,
	pub width: usize,
	pub height: usize,
}

#[derive(Default)]
pub struct LevelDataList
{
	pub levels_list: Vec<LevelData>
}

impl LevelDataList {
	pub fn load_level_sizes(&mut self, world: &World) {
		for level in &mut self.levels_list {
			if level.layers_list.is_empty() {
				continue;
			}

			let layer = level.layers_list.first().unwrap();

			let dimensions = utils::get_sprite_dimensions(world, &layer.sprite_handle.as_ref().unwrap()).unwrap();
			level.width = dimensions.0;
			level.height = dimensions.1;
		}
	}
}

static LEVELS_ROOT: &str = "assets/backgrounds";

fn get_sprite_sheet_handle(
    world: &World,
    texture_path: &PathBuf,
    ron_path: &PathBuf,
    progress_counter: &mut ProgressCounter,
) -> SpriteSheetHandle {
	let texture_path = texture_path.strip_prefix("assets/").unwrap().to_str().unwrap();
	let ron_path = ron_path.strip_prefix("assets/").unwrap().to_str().unwrap();

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

fn load_level(level_path: &PathBuf, world: &mut World, mut progress_counter: &mut ProgressCounter) -> io::Result<LevelData>
{
	let mut level_data = LevelData::default();

	let error = || io::Error::new(io::ErrorKind::InvalidData, "file does not exist");

	level_data.name =
		level_path.file_name().ok_or_else(error)?
		.to_str().ok_or_else(error)?
		.to_string();

	let mut ron_path: Option<PathBuf> = None;

	for entry in fs::read_dir(level_path)? {
		let entry = entry?;
		let path = entry.path();
		let ext_opt = path.extension();
		if let Some(ext) = ext_opt {
			if ext.to_str().ok_or_else(error)? == "ron" {
				ron_path = Some(path);
				break;
			}
		}
	}

	let ron_path = ron_path.ok_or_else(error)?;

	for entry in fs::read_dir(level_path)? {
		let entry = entry?;
		let path = entry.path();
		if path.is_file() {
			let name =
				path.file_name().ok_or_else(error)?
				.to_str().ok_or_else(error)?;

			let re = Regex::new(
				r#"z([+-]\d*).*\.png"#,
			).map_err(|_e| error())?;

			if re.is_match(name) {
				if let Some(capture) = re.captures(name).ok_or_else(error)?.get(1) {
					if let Some(depth) = capture.as_str().parse::<f32>().ok() {
						let depth = if depth == 0.0 {-0.01f32} else {depth};
						let handle = get_sprite_sheet_handle(world, &path, &ron_path, &mut progress_counter);
						level_data.layers_list.push(
							LevelLayer {
								depth,
								sprite_handle: Some(handle),
							}
						);
					}
				}
			}
		}
	}

	Ok(level_data)
}

pub fn load_levels_list(world: &mut World, mut progress_counter: &mut ProgressCounter) -> io::Result<LevelDataList>
{
	let mut levels_list: LevelDataList = LevelDataList::default();
	let levels_root = PathBuf::from(LEVELS_ROOT);

	for entry in fs::read_dir(levels_root)? {
		let entry = entry?;
		let path = entry.path();
		if path.is_dir() {
			levels_list.levels_list.push(
				load_level(&path, world, &mut progress_counter)?
			);
		}
	}

	Ok(levels_list)
}
