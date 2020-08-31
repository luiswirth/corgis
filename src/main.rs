pub mod brain;
pub mod core;
pub mod corgi;
pub mod genes;
pub mod universe;
pub mod util;

extern crate nalgebra as na;

use crate::{core::bundle::CorgiBundle, universe::Universe};
use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config/display.ron");

    let key_bindings_path = {
        if cfg!(feature = "sdl_controller") {
            app_root.join("config/input_controller.ron")
        } else {
            app_root.join("config/input.ron")
        }
    };

    let assets_dir = app_root.join("assets/");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        .with_bundle(CorgiBundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()), //.with_plugin(RenderTiles2D::<Tile>::default()),
        )?;

    let mut game = Application::build(assets_dir, Universe::default())?
        .with_frame_limit(
            //FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            FrameRateLimitStrategy::Unlimited,
            144,
        )
        .build(game_data)?;

    game.run();
    Ok(())
}
