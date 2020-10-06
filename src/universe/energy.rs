use amethyst::{
    core::Transform,
    ecs::prelude::{
        Entities, ParJoin, ParallelIterator, ReadExpect, ReadStorage, System, WriteStorage,
    },
    renderer::resources::Tint,
};

use crate::corgi::Corgi;

use super::tile::{Tile, TileEntities};

pub struct EnergySystem;

impl<'s> System<'s> for EnergySystem {
    type SystemData = (
        WriteStorage<'s, Corgi>,
        ReadStorage<'s, Tile>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Tint>,
        ReadExpect<'s, TileEntities>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut corgis, _tiles, transforms, tints, tile_entities, _entities): Self::SystemData,
    ) {
        log::error!("energy system");
        (&mut corgis, &transforms, &tints)
            .par_join()
            .for_each(|(corgi, transform, corgi_tint)| {
                let (x, y) = (
                    ((transform.translation().x - Tile::SIZE as f32 / 2.0) / Tile::SIZE as f32)
                        as u32,
                    ((transform.translation().y - Tile::SIZE as f32 / 2.0) / Tile::SIZE as f32)
                        as u32,
                );
                let tile_index = y * Tile::MAP_WIDTH + x;
                if let Some(tile_entity) = tile_entities.0.get(tile_index as usize) {
                    let tile_color = tints.get(*tile_entity).unwrap().0;
                    let corgi_color = corgi_tint.0;
                    let r_diff = (tile_color.red - corgi_color.red).abs();
                    let g_diff = (tile_color.green - corgi_color.green).abs();
                    let b_diff = (tile_color.blue - corgi_color.blue).abs();
                    if r_diff + g_diff + b_diff < 0.2 {
                        corgi.energy += 2.0;
                    }
                }
            });
    }
}
