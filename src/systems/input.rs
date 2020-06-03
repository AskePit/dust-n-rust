use amethyst::{
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
    core::timing::Time,
};

use crate::components::{Player, PlayerState, Motion};

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

static SPEED: f32 = 60.0;

impl<'s> System<'s> for PlayerInputSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Motion>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut players, mut motions, input, time) = data;

        for (player, motion) in (&mut players, &mut motions).join() {

            // axises
            {
                let move_input = input.axis_value("move").expect("Move action exists");
                let vertical_move_input = input.axis_value("move_vertical").expect("Move action exists");
                motion.velocity.x = move_input * SPEED;
                motion.velocity.y = vertical_move_input * SPEED;
            }

            // actions
            {
                if !self.advance_actions_cooldown(time.delta_seconds()) {
                    return;
                }

                let change = input.action_is_down("anim_change").expect("Jump action exists");

                if change
                {
                    player.state = change_state(player.state);
                    self.set_actions_cooldown();
                }
            }
        }
    }
}
