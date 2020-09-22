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
        let mut counter = 0;
        for (transform, corgi) in (&transforms, &mut corgis).join() {
            let corgi_x = transform.translation().x; // - (Universe::WIDTH / 2.0 + Tile::SIZE / 2.0);
            let corgi_y = transform.translation().y; // - (Universe::HEIGHT / 2.0 - Tile::SIZE / 2.0);
            let tile_x = (corgi_x / Tile::SIZE).floor() as u32;
            let tile_y = Tile::MAP_WIDTH - (corgi_y / Tile::SIZE).ceil() as u32;
            let tile_index = (tile_x + tile_y * Tile::MAP_WIDTH) as usize;

            let tile = if let Some(tile) = tile_array.0.get(tile_index) {
                tile
            } else {
                counter += 1;
                continue;
            };

            let tile = tiles.get_mut(*tile).expect("tiles should never be deleted");
            if counter == 0 {
                tile.ttype = TileType::Blue;
            }
            if counter % 2 == 0 && TileType::Red == tile.ttype {
                corgi.energy += 1.0;
            }
            if counter == 1 {
                tile.ttype = TileType::Red;
            }
            if counter % 2 == 1 && TileType::Blue == tile.ttype {
                corgi.energy += 1.0;
            }
            counter += 1;
        }
    }
}
