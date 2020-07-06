use crate::{
    components::{
        AnimationPrefab,
    },
};

use amethyst::{
	assets::{Handle, Prefab},
};

#[derive(Default)]
pub struct PrefabsDataList
{
	pub prefabs_list: Vec<Handle<Prefab<AnimationPrefab>>>
}
