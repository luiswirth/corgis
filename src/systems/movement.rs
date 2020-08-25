use crate::corgi::Corgi;
use crate::universe::Universe;
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Corgi>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut corgis, mut locals, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        for (corgi, transform) in (&mut corgis, &mut locals).join() {
            corgi.velocity[0] += corgi.force[0];
            corgi.velocity[1] += corgi.force[1];

            // friction
            corgi.velocity[0] *= 0.95;
            corgi.velocity[1] *= 0.95;
            let distance = [
                corgi.velocity[0] * time.delta_seconds(),
                corgi.velocity[1] * time.delta_seconds(),
            ];
            let movement_work = [distance[0] * corgi.force[0], distance[1] * corgi.force[1]];
            let movement_work = (movement_work[0].powf(2.0) + movement_work[1].powf(2.0)).sqrt();

            let life_work = 0.05;

            corgi.energy -= (movement_work + life_work);

            transform.prepend_translation_x(distance[0]);
            transform.prepend_translation_y(distance[1]);

            corgi.force[0] = 0.0;
            corgi.force[1] = 0.0;

            if transform.translation().x < 0.0 {
                transform.translation_mut().x = 0.0;
                corgi.velocity[0] *= -1.0;
            }
            if transform.translation().y < 0.0 {
                transform.translation_mut().y = 0.0;
                corgi.velocity[1] *= -1.0;
            }
            if transform.translation().x >= Universe::WIDTH {
                transform.translation_mut().x = Universe::WIDTH;
                corgi.velocity[0] *= -1.0;
            }
            if transform.translation().y >= Universe::HEIGHT {
                transform.translation_mut().y = Universe::HEIGHT;
                corgi.velocity[1] *= -1.0;
            }
        }
    }
}
