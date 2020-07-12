use amethyst::{
    ecs::{Join, System, WriteStorage, ReadStorage, Read},
	core::transform::Transform,
	core::math::Translation3,
	core::timing::Time,
	renderer::camera::Camera,
};

use crate::{
	components::{
		Player,
		CameraMotion,
		LevelLayerComponent,
	},
	utils::{
		non_zero
	}
};

pub const SPEED: f32 = 250.0;
pub const RETURN_SPEED: f32 = 450.0;
pub const MAX_X_ARM: f32 = 300.0;
pub const MAX_Y_ARM: f32 = 150.0;

#[derive(Default)]
pub struct CameraMotionSystem;

impl<'s> System<'s> for CameraMotionSystem {
    type SystemData = (
		WriteStorage<'s, Transform>,
		ReadStorage<'s, Camera>,
		WriteStorage<'s, CameraMotion>,
		ReadStorage<'s, Player>,
		Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
		let (mut transforms, cameras, mut camera_motions, players, time) = data;

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

		// camera shift
		let mut camera_motion: Option<&mut CameraMotion> = None;
		{
			for motion in (&mut camera_motions).join() {
				camera_motion = Some(motion);
				break;
			}
		}

		let camera_motion = match camera_motion {
			Some(t) => t,
			None => return,
		};

		// camera arm motion
		{
			let dt = time.delta_seconds();

			let velocity = &camera_motion.velocity;
			let offset = &mut camera_motion.player_offset;

			// camera move from player
			if non_zero(velocity.x) || non_zero(velocity.y) {
				offset.x += velocity.x * dt;
				offset.y += velocity.y * dt;

				offset.x = offset.x.min(MAX_X_ARM).max(-MAX_X_ARM);
				offset.y = offset.y.min(MAX_Y_ARM).max(-MAX_Y_ARM);
			// camera return back to player
			} else if non_zero(offset.x) || non_zero(offset.y) {
				let len = (offset.x*offset.x + offset.y*offset.y).sqrt();
				let new_len = (len - RETURN_SPEED*dt).max(0.0);
				let ratio = new_len / len;
				offset.x *= ratio;
				offset.y *= ratio;
			}

			target_transform.prepend_translation_x(offset.x);
			target_transform.prepend_translation_y(offset.y);

			if target_transform.translation().x - camera_half_width < 0.0 {
				target_transform.set_translation_x(camera_half_width);
			}

			if target_transform.translation().x + camera_half_width > 1200.0*2.0 {
				target_transform.set_translation_x(1200.0*2.0 - camera_half_width);
			}
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
