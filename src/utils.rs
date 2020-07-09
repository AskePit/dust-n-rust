use amethyst::{
	assets::AssetStorage,
	ecs::prelude::{World},
	prelude::WorldExt,
	renderer::{
        sprite::SpriteSheetHandle,
		SpriteSheet, Texture,
    },
};

pub fn get_sprite_dimensions(world: &World, spritesheet_handle: &SpriteSheetHandle) -> Option<(usize, usize)> {
	if let Some(sprite) = world.read_resource::<AssetStorage<SpriteSheet>>().get(spritesheet_handle) {
		if let Some(texture) = world.read_resource::<AssetStorage<Texture>>().get(&sprite.texture) {
			let Texture::Vulkan(inner) = texture;
			let image = inner.image();
			let extent = image.info().kind.extent();
			return Some((extent.width as usize, extent.height as usize));
		}
	}

	None
}