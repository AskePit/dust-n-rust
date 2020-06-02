use amethyst::{
    animation::AnimationSetPrefab,
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::Entity,
    ecs::{Component, DenseVecStorage},
    error::Error,
    renderer::sprite::{prefab::SpriteScenePrefab, SpriteRender},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum AnimationId {
    Idle,
    Walk,
    Jump,
    Attack,
    Hitted,
    Death,
}

#[derive(Clone, Debug, Deserialize, PrefabData)]
pub struct AnimationPrefab {
    /// Information for rendering a scene with sprites.
    sprite_scene: SpriteScenePrefab,
    /// Аll animations that can be run on the `Entity`.
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Animation {
    pub current: AnimationId,
    pub types: Vec<AnimationId>,
    pub show: bool,
}

impl Animation {
    pub fn new(current: AnimationId, types: Vec<AnimationId>) -> Self {
        Self {
            current,
            types,
            show: true,
        }
    }
}
