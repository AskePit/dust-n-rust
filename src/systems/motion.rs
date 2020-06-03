use amethyst::{
    ecs::{Join, Read, System, WriteStorage, ReadStorage},
	core::transform::Transform,
	core::timing::Time,
};

use crate::components::Motion;

#[derive(Default)]
pub struct MotionSystem;

impl<'s> System<'s> for MotionSystem {
    type SystemData = (
		WriteStorage<'s, Transform>,
		ReadStorage<'s, Motion>,
		Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut transforms, motions, time) = data;

        for (transform, motion) in (&mut transforms, &motions).join() {
            transform.prepend_translation_x(motion.velocity.x * time.delta_seconds());
            transform.prepend_translation_y(motion.velocity.y * time.delta_seconds());
        }
    }
}
