use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl, ControlState
    },
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{
    Animation, AnimationId,
    Locomotion, LocomotionState,
};

use crate::utils;

#[derive(Default)]
pub struct AnimationControlSystem;

impl<'s> System<'s> for AnimationControlSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Animation>,
        ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, animations, animation_sets, mut animation_control_sets) = data;

        // Iterate over all entities having Animation and AnimationSet components.
        for (entity, animation, animation_set) in (&entities, &animations, &animation_sets).join() {
            // Fetch or create the AnimationControlSet for this entity.
            let animation_control_set =
                get_animation_set(&mut animation_control_sets, entity).unwrap();

            if animation.show {
                animation.types.iter().for_each(|&animation_id| {
                    // Add the animations to the AnimationControlSet if it doesn't exist already.
                    // This ensures they are re-added after a call to abort().
                    if !animation_control_set.has_animation(animation_id) {
                        log::trace!(
                            "Added animation with id {:?} for entity: {:?}",
                            animation_id,
                            entity
                        );

                        let end = match animation_id {
                            AnimationId::JumpStart | AnimationId::JumpEnd => EndControl::Stay,
                            _ => EndControl::Loop(None)
                        };
                        animation_control_set.add_animation(
                            animation_id,
                            &animation_set.get(&animation_id).unwrap(),
                            end,
                            1.0,
                            AnimationCommand::Init,
                        );
                    }
                });
            }

            // Start the animation for the current AnimationId
            animation_control_set.start(animation.current);
        }
    }
}

#[derive(Default)]
pub struct PlayerAnimationSystem;

impl<'s> System<'s> for PlayerAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Locomotion>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, locomotions, mut animations, mut animation_control_sets) = data;

        for (entity, locomotion, mut animation, animation_control_set) in (
            &entities,
            &locomotions,
            &mut animations,
            &mut animation_control_sets,
        ).join()
        {
            use LocomotionState::*;

            let new_animation_id = match locomotion.state {
                Walking => AnimationId::Walk,
                JumpStart => AnimationId::JumpStart,
                Jumping => AnimationId::Jumping,
                JumpEnd => AnimationId::JumpEnd,
                Attacking => AnimationId::Attack,
                Hitted => AnimationId::Hitted,
                Dead => AnimationId::Death,
                _ => AnimationId::Idle,
            };

            // If the new AnimationId is different to the current one, abort the
            // current animation and start the new one
            if animation.current != new_animation_id {
                log::trace!(
                    "Updating animation for entity: {:?} from={:?}, to={:?}",
                    entity,
                    animation.current,
                    new_animation_id
                );

                // do not interrupt animations with `EndControl::Stay`
                // (like JumpStart and JumpEnd)
                if let Some(animation) = utils::get_animation_from_control_set(&animation_control_set, animation.current) {
                    if let EndControl::Stay = animation.end {
                        if animation.state != ControlState::Done {
                            continue;
                        }
                    }
                }

                animation_control_set.abort(animation.current);
                animation_control_set.start(new_animation_id);
                animation.current = new_animation_id;
            } else if new_animation_id == AnimationId::Death {
                //animation.show = false;
            }
        }
    }
}
