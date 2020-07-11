use amethyst::{
    ecs::{Join, System, WriteStorage, ReadStorage},
	core::transform::Transform,
	core::math::Translation3,
	renderer::camera::Camera,
};

use crate::{
	components::{
		Player,
		CameraMotion,
		LevelLayerComponent,
	},
};

#[derive(Default)]
pub struct CameraMotionSystem;

impl<'s> System<'s> for CameraMotionSystem {
    type SystemData = (
		WriteStorage<'s, Transform>,
		ReadStorage<'s, Camera>,
		ReadStorage<'s, CameraMotion>,
		ReadStorage<'s, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
		let (mut transforms, cameras, _camera_motions, players) = data;

		let mut target_transform: Option<Transform> = None;
		{
			for (transform, _player) in (&transforms, &players).join() {
				let mut temp_transform = Transform::new(Translation3::new(transform.translation().x, transform.translation().y, transform.translation().z), *transform.rotation(), *transform.scale());
				temp_transform.prepend_translation_y(150.0);
				temp_transform.set_translation_z(20.0);

				target_transform = Some(temp_transform);
				break;
			}
		}

		let mut target_transform = match target_transform {
			Some(t) => t,
			None => return,
		};

		let mut camera_transform: Option<&mut Transform> = None;
		let mut camera_half_width: f32 = 0.0;
		{
			for (transform, camera) in (&mut transforms, &cameras).join() {
				camera_transform = Some(transform);
				camera_half_width = camera.projection().as_orthographic().unwrap().right();
				break;
			}
		}

		let camera_transform = match camera_transform {
			Some(t) => t,
			None => return,
		};

		if target_transform.translation().x - camera_half_width < 0.0 {
			target_transform.set_translation_x(camera_half_width);
		}

		if target_transform.translation().x + camera_half_width > 1200.0*2.0 {
			target_transform.set_translation_x(1200.0*2.0 - camera_half_width);
		}

		camera_transform.set_translation(*target_transform.translation());
    }
}

#[derive(Default)]
pub struct ParallaxSystem;

const PARALLAX_COEFF: f32 = 0.03;

impl<'s> System<'s> for ParallaxSystem {
    type SystemData = (
		WriteStorage<'s, Transform>,
		ReadStorage<'s, Camera>,
		WriteStorage<'s, LevelLayerComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
		let (mut transforms, cameras, mut level_layers) = data;

		let mut camera_transform: Option<&Transform> = None;
		let mut camera_half_width: f32 = 0.0;
		{
			for (transform, camera) in (&transforms, &cameras).join() {
				camera_transform = Some(transform);
				camera_half_width = camera.projection().as_orthographic().unwrap().right();
				break;
			}
		}

		let camera_transform = match camera_transform {
			Some(t) => t,
			None => return,
		};

		let camera_x = camera_transform.translation().x;

		for (transform, level_layer) in (&mut transforms, &mut level_layers).join() {

			if level_layer.depth >= -0.99 {
				continue;
			}

			let shift = -level_layer.depth * (camera_x - camera_half_width) * PARALLAX_COEFF;
			let shift = shift.min(camera_x - camera_half_width);

			let old_shift = level_layer.parallax_shift;

			level_layer.parallax_shift = shift;
			transform.prepend_translation_x(shift-old_shift);
		}
    }
}
