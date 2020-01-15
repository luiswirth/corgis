use amethyst::{
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod pong;
use crate::pong::Pong;
mod audio;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let bindings_path = app_root.join("config").join("bindings.ron");
    let assets_dir = app_root.join("assets");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?;

    // central repository for all game logic (systems and bundles)
    let game_data = GameDataBuilder::default()
        // bundle is a set of systems preconfigured to work together
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(
            systems::BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        )
        .with(systems::WinnerSystem, "winner_system", &["ball_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // enables opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // enables to render entities with a SpriteRender component
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::new(assets_dir, Pong::default(), game_data)?; // amethyst root object
    game.run(); // starts game loop (runs until Trans::Quit or StateStack empty)

    Ok(())
}
