use amethyst::{
    core::{
        math::{Point2},
	},
	ecs::{Component, DenseVecStorage},
};

#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum MotionState {
    Idling,
    Walking,
    JumpStart,
    Jumping,
    JumpEnd,
    Attacking,
    Hitted,
    Dead,
}

impl Default for MotionState {
    fn default() -> Self {
        MotionState::Idling
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Motion
{
	pub state: MotionState,
	pub velocity: Point2<f32>,
	pub jump_trigger: bool,
	pub lift_trigger: bool,
}

impl Default for Motion {
	fn default() -> Self {
		Motion {
			state: MotionState::Idling,
			velocity: Point2::new(0.0, 0.0),
			jump_trigger: false,
			lift_trigger: false,
		}
	}
}