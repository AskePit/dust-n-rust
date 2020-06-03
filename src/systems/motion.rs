use amethyst::{
    ecs::{Join, Read, System, WriteStorage, ReadStorage},
	core::transform::Transform,
	core::timing::Time,
};

use crate::components::Motion;

#[derive(Default)]
pub struct MotionSystem;

static GRAVITY: f32 = 400.;

impl<'s> System<'s> for MotionSystem {
    type SystemData = (
		WriteStorage<'s, Transform>,
		ReadStorage<'s, Motion>,
		Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut transforms, motions, time) = data;

        let dt = time.delta_seconds();

        for (transform, motion) in (&mut transforms, &motions).join() {
            transform.prepend_translation_x(motion.velocity.x * dt);

            let y = transform.translation().y;

            let dy = if motion.velocity.y.abs() > f32::EPSILON {
                motion.velocity.y
            } else {
                -GRAVITY
            };

            transform.set_translation_y(
                (y + dy * dt)
                .max(0.0)
            );
        }
    }
}
