use crate::brain::{BrainInput, BrainOutput, Decisions, Perception};
use crate::corgi::Corgi;
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

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
            let perception = Perception::collect(corgi, &time);
            let decisions = corgi.brain.think(perception);

            corgi.force = decisions.force;
            corgi.reproduction_will = decisions.reproduction_will;
        }
    }
}
