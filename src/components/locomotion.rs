use amethyst::{
    core::{
        math::{Point2},
	},
	ecs::{Component, DenseVecStorage},
};

#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum LocomotionState {
    Idling,
    Walking,
    JumpStart,
    Jumping,
    JumpEnd,
    Attacking,
    Hitted,
    Dead,
}

impl Default for LocomotionState {
    fn default() -> Self {
        LocomotionState::Idling
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Locomotion
{
	pub state: LocomotionState,
	pub velocity: Point2<f32>,
	pub jump_trigger: bool,
	pub lift_trigger: bool,
	pub grounded: bool,
}

impl Default for Locomotion {
	fn default() -> Self {
		Locomotion {
			state: LocomotionState::Idling,
			velocity: Point2::new(0.0, 0.0),
			jump_trigger: false,
			lift_trigger: false,
			grounded: false,
		}
	}
}