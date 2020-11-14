use super::perception::{body::BodyPerception, environment::EnvironmentPerception};
use crate::brain::*;
use amethyst::{derive::SystemDesc, ecs::prelude::*};
use std::mem;

#[derive(Default, SystemDesc)]
pub struct ThinkSystem;

impl<'s> System<'s> for ThinkSystem {
    type SystemData = (
        WriteStorage<'s, Brain>,
        WriteStorage<'s, BodyPerception>,
        WriteStorage<'s, EnvironmentPerception>,
        WriteStorage<'s, Decision>,
    );

    fn run(
        &mut self,
        (mut brains, mut body_perceptions, mut environment_perceptions, mut decisions): Self::SystemData,
    ) {
        (
            &mut brains,
            &mut body_perceptions,
            &mut environment_perceptions,
            &mut decisions,
        )
            .par_join()
            .for_each(
                |(brain, body_perception, environment_perception, decision)| {
                    let perception = Perception {
                        body: &body_perceptions,
                        environment: &environment_perception,
                    };
                    *decision = brain.think(perception);
                },
            );
    }
}
