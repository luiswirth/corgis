use crate::{brain::Perception, corgi::Corgi, universe::Values};
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{
        ParJoin, ParallelIterator, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage,
    },
    renderer::resources::Tint,
};

// remove color stuff again
#[derive(SystemDesc)]
pub struct BrainSystem;

impl Default for BrainSystem {
    fn default() -> Self {
        Self
    }
}

impl<'s> System<'s> for BrainSystem {
    type SystemData = (
        WriteStorage<'s, Corgi>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Tint>,
        Read<'s, Time>,
        ReadExpect<'s, Values>,
    );

    fn run(&mut self, (mut corgis, transforms, mut tints, _time, values): Self::SystemData) {
        (&mut corgis, &transforms, &mut tints).par_join().for_each(
            |(mut corgi, _transform, tint)| {
                let perception = Perception::collect(corgi);
                let decisions = corgi.brain.think(perception);

                corgi.force += decisions.force.0;
                corgi.reproduction_will = decisions.reproduction_will.0;
                corgi.color = decisions.color.0;
                corgi.brain.memory = decisions.memory;
                *tint = Tint(corgi.color.into());

                //println!("{:#?}", corgi.brain.memory.clone().unwrap().0);

                let Values {
                    color, ref epsilon, ..
                } = *values;

                // TRICKERY
                //*tint = Tint(color.into());
                //corgi.reproduction_will = false;

                if corgi.color.hue.to_degrees() + epsilon > color.hue.to_degrees()
                    && corgi.color.hue.to_degrees() - epsilon < color.hue.to_degrees()
                {
                    corgi.energy += 10.0;
                }
            },
        );
    }
}
