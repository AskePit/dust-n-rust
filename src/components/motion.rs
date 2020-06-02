use amethyst::{
    core::{
        math::{Point2},
	},
	ecs::{Component, DenseVecStorage},
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Motion
{
	pub velocity: Point2<f32>,
}

impl Default for Motion {
	fn default() -> Self {
		Motion {
			velocity: Point2::new(0.0, 0.0)
		}
	}
}