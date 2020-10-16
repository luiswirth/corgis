use crate::{
    brain::*,
    corgi::Corgi,
    universe::tile::{Tile, TileEntities},
};
use amethyst::{
    core::transform::Transform, derive::SystemDesc, ecs::prelude::*, renderer::resources::Tint,
};
use std::{collections::HashMap, sync::Mutex, mem};

#[derive(Default, SystemDesc)]
pub struct ThinkSystem;

impl<'s> System<'s> for ThinkSystem {
    type SystemData = (
        WriteStorage<'s, Brain>,
        WriteStorage<'s, BodyPerception>,
        WriteStorage<'s, EnvironmentPerception>,
        WriteStorage<'s, Decisions>,
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
                        body: mem::take(body_perception),
                        environment: mem::take(environment_perception),
                        memory: mem::take(&mut decision.memory),
                    };
                    *decision = brain.think(perception);
                },
            );
    }
}
