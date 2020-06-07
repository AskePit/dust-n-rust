use amethyst::{
    core::{Transform},
    ecs::{prelude::World},
    prelude::{Builder, WorldExt},
    renderer::camera::Camera,
    window::ScreenDimensions,
};

pub fn load_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.fetch::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(width/2.0, height*0.25, 20.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}
