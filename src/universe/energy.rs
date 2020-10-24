use amethyst::{
    core::Transform,
    ecs::prelude::*,
    renderer::{palette::Hsl, resources::Tint},
};

use crate::corgi::Corgi;

use super::tile::{Tile, TileEntities};
use std::sync::Mutex;

#[derive(Default)]
pub struct EnergySystem;

impl<'s> System<'s> for EnergySystem {
    type SystemData = (
        WriteStorage<'s, Corgi>,
        ReadStorage<'s, Tile>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Tint>,
        ReadExpect<'s, TileEntities>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut corgis, _tiles, transforms, mut tints, tile_entities, _entities): Self::SystemData,
    ) {
        let eaten_tiles: Mutex<Vec<Entity>> = Mutex::default();

        (&mut corgis, &transforms, &tints)
            .par_join()
            .for_each(|(corgi, transform, corgi_tint)| {
                if let Some(tile_entity) =
                    tile_entities.get_at_pos(transform.translation().x, transform.translation().y)
                {
                    let tile_color: Hsl = tints.get(*tile_entity).unwrap().0.into();
                    let corgi_color: Hsl = corgi_tint.0.into();
                    let diff = (tile_color.hue - corgi_color.hue).to_radians().abs()
                        / std::f32::consts::TAU;
                    if diff < 0.08 {
                        corgi.energy += tile_color.saturation;
                        let mut eaten_tiles = eaten_tiles.lock().unwrap();
                        eaten_tiles.push(*tile_entity);
                    }
                }
            });

        let eaten_tiles = eaten_tiles.into_inner().unwrap();
        for e in eaten_tiles {
            let mut hsl = Hsl::from(tints.get(e).unwrap().0);
            hsl.saturation = 0.0;
            *tints.get_mut(e).unwrap() = Tint(hsl.into());
        }
    }
}
