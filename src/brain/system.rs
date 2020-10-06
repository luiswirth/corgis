use crate::{
    brain::*,
    corgi::Corgi,
    universe::tile::{Tile, TileEntities},
};
use amethyst::{
    core::transform::Transform, derive::SystemDesc, ecs::prelude::*, renderer::resources::Tint,
};
use std::collections::HashMap;

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
        Entities<'s>,
        WriteStorage<'s, Corgi>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Tint>,
        ReadExpect<'s, TileEntities>,
    );

    fn run(
        &mut self,
        (entities, mut corgis, transforms, mut tints, tile_entities): Self::SystemData,
    ) {
        log::error!("Brain system");
        // collect perception
        let mut corgi_tile_colors: HashMap<Entity, Hsv> = HashMap::new();
        for (entity, corgi, transform) in (&entities, &mut corgis, &transforms).join() {
            let (x, y) = (
                (transform.translation().x / Tile::SIZE) as u32,
                (transform.translation().y / Tile::SIZE) as u32,
            );
            let tile_index = y * Tile::MAP_WIDTH + x;
            if let Some(tile_entity) = tile_entities.0.get(tile_index as usize) {
                if let Some(tile_tint) = tints.get(*tile_entity) {
                    corgi_tile_colors.insert(entity, Hsv::from(tile_tint.0.color));
                }
            }
        }

        (&entities, &mut corgis, &transforms, &mut tints)
            .par_join()
            .for_each(|(entity, mut corgi, _transform, tint)| {
                // fill Perception
                let perception = Perception {
                    body: BodyPerception {
                        energy: IoF32(corgi.energy),
                        mass: IoF32(corgi.mass),
                    },
                    environment: EnvironmentPerception {
                        velocity: IoVector2(corgi.velocity),
                        tile_color: IoHsv(
                            *corgi_tile_colors
                                .get(&entity)
                                .unwrap_or(&Hsv::new(0.0, 0.0, 0.0)),
                        ),
                    },
                    memory: corgi
                        .brain
                        .memory
                        .take()
                        .unwrap_or(Memory([0.0; Memory::SIZE])),
                };

                // think
                let decisions = corgi.brain.think(perception);

                // apply decisions
                corgi.force += decisions.force.0;
                corgi.reproduction_will = decisions.reproduction_will.0;
                corgi.color = decisions.color.0;
                corgi.brain.memory = Some(decisions.memory);
                *tint = Tint(corgi.color.into());
            });
    }
}
