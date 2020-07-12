use amethyst::{
	assets::AssetStorage,
	ecs::prelude::{World},
	prelude::WorldExt,
	renderer::{
        sprite::SpriteSheetHandle,
		SpriteSheet, Texture,
	},
	animation::{
        AnimationControlSet, AnimationControl, AnimationSampling,
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

pub fn get_animation_from_control_set<I, T>(animations_control_set: &AnimationControlSet<I, T>, id: I) -> Option<&AnimationControl<T>>
where
	T: AnimationSampling,
	I: std::cmp::PartialEq,
{
	let mut it = animations_control_set.animations.iter();
	if let Some(found) = it.find(|x| x.0 == id) {
		return Some(&found.1);
	}

	None
}