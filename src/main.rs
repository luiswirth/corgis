mod corgi;
mod intelligence;
mod loader;
mod universe;

use bevy::{pbr::PbrPlugin, prelude::*, render::pass::ClearColor};
use bevy_rapier2d::{physics::RapierPhysicsPlugin, render::RapierRenderPlugin};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_plugin(PbrPlugin)
        .add_resource(Msaa::default())
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(universe::setup_graphics.system())
        .add_startup_system(universe::setup_physics.system())
        .add_startup_system(loader::load_assets.system())
        .add_system(corgi::corgi_spawner.system())
        .add_plugin(intelligence::IntelligencePlugin)
        .run();
}
