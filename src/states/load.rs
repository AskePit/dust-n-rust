use amethyst::{
    assets::ProgressCounter,
    prelude::{GameData, SimpleState, StateData},
    prelude::*,
};

use crate::{
    components::{
        player::load_player,
    },
    states::GameState,
    resources::load_levels_list,
    resources::PrefabsDataList,
};

#[derive(Default)]
pub struct LoadState {
    progress_counter: ProgressCounter,
}

impl LoadState {
    pub fn new() -> Self {
        Self {
            progress_counter: ProgressCounter::new(),
        }
    }
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        {
            let levels = load_levels_list(world, &mut self.progress_counter).unwrap();
            world.insert(levels);
        }

        {
            let mut prefabs = PrefabsDataList::default();
            prefabs.prefabs_list.push(
                load_player(world, &mut self.progress_counter)
            );

            world.insert(prefabs);
        }
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            Trans::Switch(Box::new(GameState))
        } else {
            Trans::None
        }
    }
}
