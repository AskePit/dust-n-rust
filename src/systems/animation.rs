use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{
    Animation, AnimationId,
    Player, PlayerState,
};

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
                            /*AnimationId::Idle | AnimationId::Walk => EndControl::Loop(None),
                            AnimationId::Death => EndControl::Stay,
                            _ => EndControl::Normal,*/
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
        ReadStorage<'s, Player>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, players, mut animations, mut animation_control_sets) = data;

        for (entity, player, mut animation, animation_control_set) in (
            &entities,
            &players,
            &mut animations,
            &mut animation_control_sets,
        ).join()
        {
            let new_animation_id = match player.state {
                PlayerState::Walking => AnimationId::Walk,
                PlayerState::Jumping => AnimationId::Jump,
                PlayerState::Attacking => AnimationId::Attack,
                PlayerState::Hitted => AnimationId::Hitted,
                PlayerState::Dead => AnimationId::Death,
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

                animation_control_set.abort(animation.current);
                animation_control_set.start(new_animation_id);

                animation.current = new_animation_id;
            } else if new_animation_id == AnimationId::Death {
                //animation.show = false;
            }
        }
    }
}
