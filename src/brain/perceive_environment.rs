use crate::{
    brain::*,
    corgi::{Corgi, Physique},
    universe::tile::{Tile, TileEntities},
};
use amethyst::{
    core::Transform,
    ecs::{prelude::*, System},
    renderer::{palette::Hsl, resources::Tint},
};

#[derive(Default)]
pub struct PerceiveEnvironmentSystem;

impl<'s> System<'s> for PerceiveEnvironmentSystem {
    type SystemData = (
        WriteStorage<'s, EnvironmentPerception>,
        ReadStorage<'s, Corgi>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Physique>,
        ReadStorage<'s, Tint>,
        ReadExpect<'s, TileEntities>,
    );

    fn run(
        &mut self,
        (mut perceptions, corgis, transforms, physiques, tints, tile_entities): Self::SystemData,
    ) {
        for (_corgi, transform, physique, mut perception) in
            (&corgis, &transforms, &physiques, &mut perceptions).join()
        {
            let (x, y) = (
                (transform.translation().x / Tile::SIZE) as u32,
                (transform.translation().y / Tile::SIZE) as u32,
            );
            let tile_index = y * Tile::MAP_WIDTH + x;
            if let Some(tile_entity) = tile_entities.0.get(tile_index as usize) {
                if let Some(tile_tint) = tints.get(*tile_entity) {
                    *perception = EnvironmentPerception {
                        tile_color: IoHsl(Hsl::from(tile_tint.0.color)),
                        velocity: IoVector2(physique.velocity.clone_owned()),
                    };
                }
            }
        }
    }
}
