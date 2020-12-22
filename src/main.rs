mod corgi;

use bevy::{prelude::*, render::pass::ClearColor};
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
    render::RapierRenderPlugin,
};
use std::ops::AddAssign;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_physics.system())
        .add_startup_system(setup_graphics.system())
        .add_system(center_of_mass.system())
        .add_resource(Msaa::default())
        .add_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .run();
}

fn setup_graphics(commands: &mut Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = 10.0;
    configuration.gravity = Vector2::new(0.0, 0.0);

    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(1000.0, 100.0, 2000.0)),
            ..Default::default()
        })
        .spawn(Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..Camera2dBundle::default()
        });
}

pub fn setup_physics(commands: &mut Commands) {
    /*
     * Ground
     */
    //let ground_size = 25.0;

    //let rigid_body = RigidBodyBuilder::new_static();
    //let collider = ColliderBuilder::cuboid(ground_size, 1.2);
    //commands.spawn((rigid_body, collider));

    //let rigid_body = RigidBodyBuilder::new_static()
    //    .rotation(std::f32::consts::FRAC_PI_2)
    //    .translation(ground_size, ground_size * 2.0);
    //let collider = ColliderBuilder::cuboid(ground_size * 2.0, 1.2);
    //commands.spawn((rigid_body, collider));

    //let body = RigidBodyBuilder::new_static()
    //    .rotation(std::f32::consts::FRAC_PI_2)
    //    .translation(-ground_size, ground_size * 2.0);
    //let collider = ColliderBuilder::cuboid(ground_size * 2.0, 1.2);
    //commands.spawn((body, collider));

    /*
     * Create the cubes
     */
    let num = 10;
    let rad = 0.5;

    let shift = rad * 2.0;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0;

    for i in 0..num {
        for j in 0usize..num * 5 {
            let x = i as f32 * shift - centerx;
            let y = j as f32 * shift + centery + 2.0;

            // Build the rigid body.
            let body = RigidBodyBuilder::new_dynamic().translation(x, y);
            let collider = ColliderBuilder::cuboid(rad, rad).density(1.0);
            commands.spawn((body, collider));
        }
    }
    let body = RigidBodyBuilder::new_dynamic()
        .translation(-20.0, 10.0)
        .angvel(10.0)
        .linvel(0.0, 0.0);

    let collider = ColliderBuilder::cuboid(5.0 * rad, 5.0 * rad).density(100.0);
    commands.spawn((body, collider));
}

fn center_of_mass(mut query: Query<&RigidBodyHandleComponent>, mut set: ResMut<RigidBodySet>) {
    let mut position_sum = Vector2::new(0.0, 0.0);
    let mut weight_sum = 0.0;
    for body in query.iter_mut() {
        let body = set.get(body.handle()).unwrap();
        position_sum.add_assign(body.position().translation.vector * body.mass());
        weight_sum += body.mass();
    }
    let center_of_mass = (1.0 / weight_sum) * position_sum;

    for body in query.iter_mut() {
        let body = set.get_mut(body.handle()).unwrap();
        let force = center_of_mass - body.position().translation.vector;
        let gravitation = force / (force.magnitude()) * 100.0;
        body.apply_force(gravitation, true);
        let strong_force = -force / force.magnitude().powf(2.0) * 1000.0;
        body.apply_force(strong_force, true);
    }
}
