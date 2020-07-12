use amethyst::{
    ecs::{Join, Read, System, WriteStorage},
	core::transform::Transform,
	core::timing::Time,
};

use crate::components::LocomotionState;
use crate::components::Locomotion;

#[derive(Default)]
pub struct LocomotionSystem;

pub const SPEED: f32 = 120.0;
const GRAVITY: f32 = 1100.;
const JUMP_IMPULSE: f32 = 400.0;
const LIFT_HEIGHT: f32 = 200.;
const GROUND_LEVEL: f32 = 0.;

impl<'s> System<'s> for LocomotionSystem {
    type SystemData = (
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Locomotion>,
		Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut transforms, mut locomotions, time) = data;

        let dt = time.delta_seconds();

        for (transform, locomotion) in (&mut transforms, &mut locomotions).join() {
            transform.prepend_translation_x(locomotion.velocity.x * dt);

            if locomotion.velocity.x.abs() < f32::EPSILON {
                locomotion.state = match locomotion.state {
                    LocomotionState::JumpStart | LocomotionState::Jumping => locomotion.state,
                    _ => LocomotionState::Idling,
                };
            } else {
                locomotion.state = match locomotion.state {
                    LocomotionState::JumpStart | LocomotionState::Jumping => locomotion.state,
                    _ => LocomotionState::Walking,
                };
                if locomotion.velocity.x > 0.0 {
                    transform.set_rotation_y_axis(0.0);
                }
                if locomotion.velocity.x < 0.0 {
                    transform.set_rotation_y_axis(std::f32::consts::PI);
                }
            }

            if locomotion.jump_trigger {
                locomotion.velocity.y = JUMP_IMPULSE;
                locomotion.grounded = false;
                locomotion.jump_trigger = false;
            }

            if locomotion.lift_trigger {
                transform.prepend_translation_y(LIFT_HEIGHT);
                locomotion.grounded = false;
                locomotion.lift_trigger = false;
            }

            locomotion.velocity.y -= GRAVITY * dt;
            let y = transform.translation().y;

            if y > 0.0 {
                locomotion.state = match locomotion.state {
                    LocomotionState::JumpStart | LocomotionState::Jumping => LocomotionState::Jumping,
                    _ => LocomotionState::JumpStart,
                };
            } else if locomotion.state == LocomotionState::JumpStart || locomotion.state == LocomotionState::Jumping {
                locomotion.state = LocomotionState::JumpEnd;
            }

            if locomotion.velocity.y < 0.0 && y <= GROUND_LEVEL {
                locomotion.velocity.y = 0.0;
                transform.set_translation_y(GROUND_LEVEL);
                locomotion.grounded = true;
            } else {
                transform.set_translation_y(
                    (y + locomotion.velocity.y * dt)
                    .max(GROUND_LEVEL)
                );
            }
        }
    }
}
