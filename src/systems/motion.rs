use amethyst::{
    ecs::{Join, Read, System, WriteStorage},
	core::transform::Transform,
	core::timing::Time,
};

use crate::components::Motion;

#[derive(Default)]
pub struct MotionSystem;

pub const SPEED: f32 = 120.0;
const GRAVITY: f32 = 1100.;
const JUMP_IMPULSE: f32 = 320.0;
const LIFT_HEIGHT: f32 = 50.;
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

            if motion.jump_trigger {
                motion.velocity.y = JUMP_IMPULSE;
                motion.jump_trigger = false;
            }

            if motion.lift_trigger {
                transform.prepend_translation_y(LIFT_HEIGHT);
                motion.lift_trigger = false;
            }

            motion.velocity.y -= GRAVITY * dt;
            let y = transform.translation().y;

            if motion.velocity.y < 0.0 && y <= GROUND_LEVEL {
                motion.velocity.y = 0.0;
                transform.set_translation_y(GROUND_LEVEL);
            } else {
                transform.set_translation_y(
                    (y + motion.velocity.y * dt)
                    .max(GROUND_LEVEL)
                );
            }
        }
    }
}
