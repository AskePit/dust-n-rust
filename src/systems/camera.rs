use amethyst::{
    ecs::{Join, System, WriteStorage, ReadStorage},
	core::transform::Transform,
	core::math::Translation3,
	renderer::camera::Camera,
};

use crate::{
    components::{Player, CameraMotion}
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

		if target_transform.is_none() {
			return;
		}
		let mut target_transform = target_transform.unwrap();

		let mut camera_transform: Option<&mut Transform> = None;
		let mut camera_half_width: f32 = 0.0;
		{
			for (transform, camera) in (&mut transforms, &cameras).join() {
				camera_transform = Some(transform);
				camera_half_width = camera.projection().as_orthographic().unwrap().right();
				break;
			}
		}

		if camera_transform.is_none() {
			return;
		}
		let camera_transform = camera_transform.unwrap();

		if target_transform.translation().x - camera_half_width < 0.0 {
			target_transform.set_translation_x(camera_half_width);
		}

		if target_transform.translation().x + camera_half_width > 1200.0*2.0 {
			target_transform.set_translation_x(1200.0*2.0 - camera_half_width);
		}

		camera_transform.set_translation(*target_transform.translation());
    }
}
