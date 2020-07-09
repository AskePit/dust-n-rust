use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct LevelLayerComponent
{
	pub depth: f32,
	pub parallax_shift: f32,
}

impl Default for LevelLayerComponent {
	fn default() -> Self {
		LevelLayerComponent {
			depth: 0.0,
			parallax_shift: 0.0,
		}
	}
}
