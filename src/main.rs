#![allow(clippy::type_complexity)]

pub mod brain;
pub mod core;
pub mod corgi;
pub mod genes;
pub mod universe;
pub mod util;

pub use amethyst::core::math as na;
use universe::tile::Tile;

use crate::{core::bundle::CorgiBundle, universe::Universe};
use amethyst::{
    controls::FlyMovementSystemDesc,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    tiles::RenderTiles2D,
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");
    let key_bindings_path = app_root.join("config/input.ron");
    let assets_dir = app_root.join("assets/");

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            FlyMovementSystemDesc::<StringBindings>::new(
                100.0,
                Some("camera_right".into()),
                Some("camera_up".into()),
                None,
            ),
            "fly_movement",
            &[],
        )
        .with_bundle(TransformBundle::new().with_dep(&["fly_movement"]))?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<Tile>::default()),
        )?
        .with_bundle(CorgiBundle)?;

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
