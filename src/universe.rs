use bevy::prelude::*;
use bevy_rapier2d::{na::Vector2, physics::RapierConfiguration};

pub const UNIVERSE_WIDTH: f32 = 500.0;
pub const UNIVERSE_HEIGHT: f32 = 500.0;

pub fn setup_physics(mut configuration: ResMut<RapierConfiguration>) {
    configuration.gravity = Vector2::new(0.0, 0.0);
}

pub fn setup_graphics(commands: &mut Commands) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(1000.0, 100.0, 2000.0)),
            ..Default::default()
        })
        .spawn(Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Camera2dBundle::default()
        });
}
