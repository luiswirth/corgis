use amethyst::{
    assets::Handle,
    core::math::Point3,
    core::math::Vector3,
    core::Transform,
    ecs::Entity,
    ecs::{Component, DenseVecStorage, World},
    prelude::Builder,
    prelude::WorldExt,
    renderer::SpriteSheet,
    tiles,
};
use tiles::TileMap;

use super::Universe;

pub struct Tiles(pub Vec<Entity>);

#[derive(Clone)]
pub struct Tile {
    pub ttype: TileType,
}

impl Tile {
    pub const SIZE: f32 = 20.0;
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            ttype: TileType::Energy(0.0),
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Tile>;
}

impl tiles::Tile for Tile {
    fn sprite(&self, _coordinates: Point3<u32>, _world: &World) -> Option<usize> {
        Some(0)
    }
}

#[derive(Clone)]
pub enum TileType {
    Energy(f32),
}

impl Default for TileType {
    fn default() -> Self {
        Self::Energy(0.0)
    }
}

pub fn create_tiles(world: &mut World) {
    let mut entities = Vec::new();
    let tile_count_x = (Universe::WIDTH / Tile::SIZE).floor() as u32;
    let tile_count_y = (Universe::HEIGHT / Tile::SIZE).floor() as u32;

    world.register::<Tile>();
    for y in 0..tile_count_y {
        for x in 0..tile_count_x {
            let tile = Tile {
                ttype: TileType::Energy(0.0),
            };

            let mut transform = Transform::default();
            transform.set_translation_xyz(x as f32 * Tile::SIZE, y as f32 * Tile::SIZE, 0.0);

            let entity = world.create_entity().with(tile).with(transform).build();
            entities.push(entity);
        }
    }
    world.insert(Tiles(entities));

    let map = TileMap::<Tile>::new(
        Vector3::new(tile_count_x, tile_count_y, 1),
        Vector3::new(Tile::SIZE as u32, Tile::SIZE as u32, 1),
        Some((*world.read_resource::<Handle<SpriteSheet>>()).clone()),
    );
    let mut map_transform = Transform::default();
    map_transform.set_translation_xyz(Universe::WIDTH / 2.0, Universe::HEIGHT / 2.0, 0.0);
    world.create_entity().with(map).with(map_transform).build();
}
