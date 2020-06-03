use amethyst::{
    ecs::{Join, Read, System, WriteStorage},
	core::transform::Transform,
	core::timing::Time,
};

use crate::components::Motion;

#[derive(Default)]
pub struct MotionSystem;

pub static SPEED: f32 = 120.0;
static GRAVITY: f32 = 1100.;
static JUMP_IMPULSE: f32 = 320.0;
static JUMP_DECELL: f32 = 1100.;
static LIFT_HEIGHT: f32 = 50.;

impl<'s> System<'s> for MotionSystem {
    type SystemData = (
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Motion>,
		Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut transforms, mut motions, time) = data;

        let dt = time.delta_seconds();

        for (transform, motion) in (&mut transforms, &mut motions).join() {
            transform.prepend_translation_x(motion.velocity.x * dt);

            if motion.jump_trigger {
                motion.velocity.y = JUMP_IMPULSE;
                motion.jump_trigger = false;
            }

            if motion.lift_trigger {
                transform.prepend_translation_y(LIFT_HEIGHT);
                motion.lift_trigger = false;
            }

            let dy = if motion.velocity.y.abs() > f32::EPSILON {
                motion.velocity.y -= JUMP_DECELL*dt;
                motion.velocity.y
            } else {
                motion.velocity.y -= GRAVITY*dt;
                motion.velocity.y
            };

            let y = transform.translation().y;

            if motion.velocity.y < 0. && y.abs() <= f32::EPSILON {
                motion.velocity.y = 0.
            }

            transform.set_translation_y(
                (y + dy * dt)
                .max(0.0)
            );
        }
    }
}
