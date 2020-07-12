use std::fmt::{self, Display};

use amethyst::{
    ecs::{Join, Read, System, WriteStorage, ReadStorage},
    input::{InputHandler, BindingTypes},
    core::timing::Time,
};

use serde::{Serialize, Deserialize};

use crate::components::{Locomotion, Player, CameraMotion};
use crate::systems::locomotion;
use crate::systems::camera;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
    Move,
    CameraMoveX,
    CameraMoveY,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {
    AnimChange,
    Jump,
    Lift,
}

impl Display for AxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for ActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct InputBindingTypes;

impl BindingTypes for InputBindingTypes {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}

static COOLDOWN_TIME: f32 = 0.5;

#[derive(Default)]
pub struct PlayerInputSystem
{
    actions_cooldown: Option<f32>,
}

impl PlayerInputSystem {
    fn advance_actions_cooldown(&mut self, delta_seconds: f32) -> bool {
        if let Some(ref mut actions_cooldown) = self.actions_cooldown {
            *actions_cooldown -= delta_seconds;
            if *actions_cooldown <= 0. {
                self.actions_cooldown = None;
            }
        }

        self.actions_cooldown.is_none()
    }

    fn set_actions_cooldown(&mut self) {
        self.actions_cooldown.replace(COOLDOWN_TIME);
    }
}

impl<'s> System<'s> for PlayerInputSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Locomotion>,
        Read<'s, InputHandler<InputBindingTypes>>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_players, mut motions, input, time) = data;

        for locomotion in (&mut motions).join() {

            // axises
            {
                let move_input = input.axis_value(&AxisBinding::Move).expect("Move action exists");
                locomotion.velocity.x = move_input * locomotion::SPEED;
            }

            // actions
            {
                if !self.advance_actions_cooldown(time.delta_seconds()) {
                    return;
                }

                if input.action_is_down(&ActionBinding::Jump).expect("Jump action exists") && locomotion.grounded
                {
                    locomotion.jump_trigger = true;
                    self.set_actions_cooldown();
                }

                if input.action_is_down(&ActionBinding::Lift).expect("Lift action exists")
                {
                    locomotion.lift_trigger = true;
                    self.set_actions_cooldown();
                }
            }
        }
    }
}

#[derive(Default)]
pub struct CameraInputSystem;

impl<'s> System<'s> for CameraInputSystem {
    type SystemData = (
        WriteStorage<'s, CameraMotion>,
        Read<'s, InputHandler<InputBindingTypes>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut motions, input) = data;

        for camera_motion in (&mut motions).join() {
            let move_x = input.axis_value(&AxisBinding::CameraMoveX).expect("CameraMoveX action exists");
            camera_motion.velocity.x = move_x * camera::SPEED;

            let move_y = input.axis_value(&AxisBinding::CameraMoveY).expect("CameraMoveY action exists");
            camera_motion.velocity.y = move_y * camera::SPEED;
        }
    }
}
