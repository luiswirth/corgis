use crate::corgi::Corgi;
use crate::universe::Universe;
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

const FRICTION: f32 = 0.97;

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Corgi>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut corgis, mut locals, time): Self::SystemData) {
        for (corgi, transform) in (&mut corgis, &mut locals).join() {
            corgi.velocity += corgi.force / corgi.mass;

            // friction
            corgi.velocity *= FRICTION;

            let distance = corgi.velocity * time.delta_seconds();
            let mut movement_work = distance.dot(&corgi.force);
            movement_work *= Corgi::MOVEMENT_WORK;

            let life_work = 0.05;

            corgi.energy -= movement_work + life_work;

            transform.prepend_translation_x(distance[0]);
            transform.prepend_translation_y(distance[1]);

            corgi.force[0] = 0.0;
            corgi.force[1] = 0.0;

            if transform.translation().x < 0.0 {
                transform.translation_mut().x = Universe::WIDTH;
            }
            if transform.translation().y < 0.0 {
                transform.translation_mut().y = Universe::HEIGHT;
            }
            if transform.translation().x > Universe::WIDTH {
                transform.translation_mut().x = 0.0;
            }
            if transform.translation().y > Universe::HEIGHT {
                transform.translation_mut().y = 0.0;
            }
        }
    }
}
