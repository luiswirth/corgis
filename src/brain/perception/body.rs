use crate::{
    brain::*,
    corgi::{Corgi, Physique},
};
use amethyst::ecs::{prelude::*, System};

#[derive(Component, Default, Clone)]
pub struct BodyPerception(pub BrainInput);

#[derive(Default)]
pub struct PerceiveBodySystem;

impl<'s> System<'s> for PerceiveBodySystem {
    type SystemData = (
        WriteStorage<'s, BodyPerception>,
        ReadStorage<'s, Corgi>,
        WriteStorage<'s, Physique>,
    );

    fn run(&mut self, (mut perceptions, corgis, physiques): Self::SystemData) {
        for (perception, corgi, physique) in (&mut perceptions, &corgis, &physiques).join() {
            perception.0.perceive(corgi.energy);
            perception.0.perceive(physique.mass);
        }
    }
}
