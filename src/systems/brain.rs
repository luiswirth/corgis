use crate::corgi::Corgi;
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
use nalgebra::DVector;

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
#[derive(SystemDesc)]
pub struct BrainSystem;

impl<'s> System<'s> for BrainSystem {
    type SystemData = (
        WriteStorage<'s, Corgi>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut corgis, transforms, time): Self::SystemData) {
        for (mut corgi, transform) in (&mut corgis, &transforms).join() {
            let input = |_c, r| match r {
                0 => transform.translation().x,
                1 => transform.translation().x,
                2 => corgi.velocity[0],
                3 => corgi.velocity[1],
                4 => time.absolute_time().as_secs_f32(),
                _ => unreachable!(),
            };
            let input: DVector<f32> = DVector::from_fn(5, input);
            let output = corgi.brain.neural_network.feed(input);
            if output[0].max(output[1]) > 2.5 {
                // BUG: values should always be in 0.0..1.0
                dbg!(&output);
            }
            corgi.force[0] = output[0] - 0.5;
            corgi.force[1] = output[1] - 0.5;
            corgi.will_to_reproduce = output[0] >= 0.5;
        }
    }
}
