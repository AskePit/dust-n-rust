use amethyst::{
    animation::AnimationBundle,
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        sprite::SpriteRender,
    },
    utils::application_root_dir,
    assets::{
        PrefabLoaderSystemDesc
    },
    input::{InputBundle, StringBindings},
};

mod states;
mod components;
mod systems;

use components::{AnimationId, AnimationPrefab};
use systems::*;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(app_root.join("config/bindings.ron"))?;

    let prefab_loader_system_desc = PrefabLoaderSystemDesc::<AnimationPrefab>::default();

    let game_data = GameDataBuilder::default()
        .with_system_desc(prefab_loader_system_desc, "scene_loader", &[])
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with_bundle(input_bundle)?
        .with(
            PlayerAnimationSystem,
            "player_animation_system",
            &[],
        )
        .with(
            AnimationControlSystem,
            "animation_control_system",
            &["player_animation_system"],
        )
        .with(
            PlayerInputSystem::default(),
            "player_input_system",
            &["animation_control_system"],
        )
        .with(
            MotionSystem,
            "motion_system",
            &["player_input_system"],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::new(assets_dir, states::LoadState, game_data)?;
    game.run();

    Ok(())
}