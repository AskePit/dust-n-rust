use amethyst::{
    core::{
        transform::Transform,
        math::Vector3,
    },
    assets::{Handle},
    prelude::{GameData, SimpleState, StateData},
    ecs::prelude::{World},
    prelude::WorldExt,
    prelude::*,
    renderer::{
        sprite::{SpriteRender},
        SpriteSheet,
    },
};

use crate::{
    components::{
        camera::add_camera,
        player::add_player,
    },
    states::load::{
        AssetHandlers,
    },
    resources::BackgroundsMeta,
};

fn add_level(world: &mut World, handlers: Vec<Handle<SpriteSheet>>) {

    let width = 1200*2;
    let height = 225*2;
    let ground = 33.0*2.0;

    let zs: Vec<f32>;
    {
        let backgrounds_meta = world.read_resource::<BackgroundsMeta>();
        zs = backgrounds_meta.data.iter().map(|x| x.1).collect();
    }

    for (pos, handler) in handlers.iter().enumerate() {
        let z = zs[pos];

        let mut transform = Transform::default();
        // position left-bottom corner to world's (0; 0) position
        transform.set_translation_xyz(width as f32/2.0, height as f32/2.0 - ground, z);
        transform.set_scale(Vector3::new(2.0, 2.0, 2.0));

        let sprite_render = SpriteRender {
            sprite_sheet: handler.clone(),
            sprite_number: 0,
        };

        world
            .create_entity()
            .with(transform)
            .with(sprite_render)
            .build();
    }
}

#[derive(Default)]
pub struct GameState {
    handlers: AssetHandlers,
}

impl GameState {
    pub fn new(handlers: AssetHandlers) -> Self {
        GameState {
            handlers
        }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        add_level(world, self.handlers.sprite_sheets.clone());
        add_player(world, self.handlers.prefabs[0].clone());
        add_camera(world);
    }
}
