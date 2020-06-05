use amethyst::{
    ecs::{Join, Read, System, WriteStorage},
	core::transform::Transform,
	core::timing::Time,
};

use crate::components::MotionState;
use crate::components::Motion;

#[derive(Default)]
pub struct MotionSystem;

pub const SPEED: f32 = 120.0;
const GRAVITY: f32 = 1100.;
const JUMP_IMPULSE: f32 = 320.0;
const LIFT_HEIGHT: f32 = 200.;
const GROUND_LEVEL: f32 = 0.;

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

            if motion.velocity.x.abs() < f32::EPSILON {
                motion.state = MotionState::Idling;
            } else {
                motion.state = MotionState::Walking;
                if motion.velocity.x > 0.0 {
                    transform.set_rotation_y_axis(0.0);
                }
                if motion.velocity.x < 0.0 {
                    transform.set_rotation_y_axis(std::f32::consts::PI);
                }
            }

            if motion.jump_trigger {
                motion.velocity.y = JUMP_IMPULSE;
                motion.grounded = false;
                motion.jump_trigger = false;
            }

            if motion.lift_trigger {
                transform.prepend_translation_y(LIFT_HEIGHT);
                motion.grounded = false;
                motion.lift_trigger = false;
            }

            motion.velocity.y -= GRAVITY * dt;
            let y = transform.translation().y;

            if y > 0.0 {
                motion.state = MotionState::Jumping;
            }

            if motion.velocity.y < 0.0 && y <= GROUND_LEVEL {
                motion.velocity.y = 0.0;
                transform.set_translation_y(GROUND_LEVEL);
                motion.grounded = true;
            } else {
                transform.set_translation_y(
                    (y + motion.velocity.y * dt)
                    .max(GROUND_LEVEL)
                );
            }
        }
    }
}
