use amethyst::{
    core::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::corgi::Corgi;

use super::{
    tile::{Tile, TileType, Tiles},
    Universe,
};

pub struct EnergySystem;

impl<'s> System<'s> for EnergySystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Corgi>,
        WriteStorage<'s, Tile>,
        ReadExpect<'s, Tiles>,
    );

    fn run(&mut self, (transforms, mut corgis, mut tiles, tile_array): Self::SystemData) {
        for (transform, corgi) in (&transforms, &mut corgis).join() {
            let corgi_x = transform.translation().x - (Universe::WIDTH / 2.0 + Tile::SIZE / 2.0);
            let corgi_y = transform.translation().y - (Universe::HEIGHT / 2.0 - Tile::SIZE / 2.0);
            let tile_x = (corgi_x / Tile::SIZE).floor() as u32;
            let tile_y = (corgi_y / Tile::SIZE).floor() as u32;
            if !(0..Tile::MAP_WIDTH).contains(&tile_x) || !(0..Tile::MAP_HEIGHT).contains(&tile_y) {
                continue;
            }
            let tile = tile_array.0[(tile_x + tile_y * (Tile::MAP_WIDTH)) as usize];
            let tile = tiles.get_mut(tile).expect("tiles should never be deleted");
            if let TileType::Energy(tile_energy) = tile.ttype {
                corgi.energy += tile_energy;
                tile.ttype = TileType::Neutral;
            }
        }
    }
}
