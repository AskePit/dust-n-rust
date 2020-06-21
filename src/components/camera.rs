use amethyst::{
    core::{
        Transform,
        math::{Point2},
    },
    ecs::{prelude::World,Component, DenseVecStorage},
    prelude::{Builder, WorldExt},
    renderer::camera::Camera,
    window::ScreenDimensions,
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct CameraMotion
{
	pub velocity: Point2<f32>,
}

impl Default for CameraMotion {
	fn default() -> Self {
		CameraMotion {
			velocity: Point2::new(0.0, 0.0),
		}
	}
}

pub fn add_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.fetch::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(width/2.0, height*0.25, 20.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .with(CameraMotion::default())
        .build();
}
