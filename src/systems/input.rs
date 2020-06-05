use std::fmt::{self, Display};

use amethyst::{
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, BindingTypes},
    core::timing::Time,
};

use serde::{Serialize, Deserialize};

use crate::components::{Player, PlayerState, Motion};
use crate::systems::motion::SPEED;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
    Move,
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

fn change_state(state: PlayerState) -> PlayerState
{
    use PlayerState::*;

    match state {
        Idling => Walking,
        Walking => Jumping,
        Jumping => Attacking,
        Attacking => Hitted,
        Hitted => Dead,
        Dead => Idling,
        _ => Idling,
    }
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
        WriteStorage<'s, Player>,
        WriteStorage<'s, Motion>,
        Read<'s, InputHandler<InputBindingTypes>>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut players, mut motions, input, time) = data;

        for (player, motion) in (&mut players, &mut motions).join() {

            // axises
            {
                let move_input = input.axis_value(&AxisBinding::Move).expect("Move action exists");
                motion.velocity.x = move_input * SPEED;
            }

            // actions
            {
                if !self.advance_actions_cooldown(time.delta_seconds()) {
                    return;
                }

                if input.action_is_down(&ActionBinding::AnimChange).expect("animation change action exists")
                {
                    player.state = change_state(player.state);
                    self.set_actions_cooldown();
                }

                if input.action_is_down(&ActionBinding::Jump).expect("Jump action exists")
                {
                    motion.jump_trigger = true;
                    self.set_actions_cooldown();
                }

                if input.action_is_down(&ActionBinding::Lift).expect("Lift action exists")
                {
                    motion.lift_trigger = true;
                    self.set_actions_cooldown();
                }
            }
        }
    }
}
