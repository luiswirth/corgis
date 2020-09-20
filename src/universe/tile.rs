use amethyst::{
    assets::Handle,
    core::math::Point3,
    core::math::Vector3,
    core::Transform,
    ecs::prelude::{Join, System, WriteStorage},
    ecs::Entity,
    ecs::{Component, DenseVecStorage, World},
    prelude::Builder,
    prelude::WorldExt,
    renderer::palette::Hsl,
    renderer::palette::Srgba,
    renderer::{resources::Tint, SpriteSheet},
    tiles,
};
use rand::{thread_rng, Rng};
use tiles::TileMap;

use super::Universe;

pub struct Tiles(pub Vec<Entity>);

#[derive(Clone)]
pub struct Tile {
    pub ttype: TileType,
}

impl Tile {
    pub const SIZE: f32 = 10.0;
    pub const MAP_WIDTH: u32 = (Universe::WIDTH / Tile::SIZE) as u32;
    pub const MAP_HEIGHT: u32 = (Universe::HEIGHT / Tile::SIZE) as u32;
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            ttype: TileType::default(),
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Tile>;
}

impl tiles::Tile for Tile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        Some(0)
    }

    fn tint(&self, coord: Point3<u32>, world: &World) -> Srgba {
        use TileType::*;
        let tile_array = &world.fetch::<Tiles>().0;
        let tile = tile_array[(coord.x + coord.y * (Tile::MAP_WIDTH)) as usize];
        let tiles = world.read_storage::<Tile>();
        let tile = tiles.get(tile).unwrap();
        println!(
            "SpriteTile: {:?}, Acctual Tile: {:?}",
            self.ttype, tile.ttype
        );
        match tile.ttype {
            Neutral => Srgba::new(1.0, 1.0, 1.0, 1.0),
            Energy(_) => Srgba::new(1.0, 0.0, 0.0, 1.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    Neutral,
    Energy(f32),
}

impl Default for TileType {
    fn default() -> Self {
        Self::Neutral
    }
}

pub fn create_tiles(world: &mut World) {
    let mut entities = Vec::new();

    world.register::<Tile>();
    for y in 0..Tile::MAP_HEIGHT {
        for x in 0..Tile::MAP_WIDTH {
            let tile = Tile {
                ttype: TileType::default(),
            };

            let mut transform = Transform::default();
            transform.set_translation_xyz(x as f32 * Tile::SIZE, y as f32 * Tile::SIZE, 0.0);

            let entity = world
                .create_entity()
                .with(tile)
                .with(transform)
                .with(Tint(Hsl::new(1.0, 1.0, 1.0).into()))
                .build();
            entities.push(entity);
        }
    }
    world.insert(Tiles(entities));

    let map = TileMap::<Tile>::new(
        Vector3::new(Tile::MAP_WIDTH, Tile::MAP_HEIGHT, 1),
        Vector3::new(Tile::SIZE as u32, Tile::SIZE as u32, 1),
        Some((*world.read_resource::<Handle<SpriteSheet>>()).clone()),
    );
    let mut map_transform = Transform::default();
    map_transform.set_translation_xyz(
        Universe::WIDTH / 2.0 + Tile::SIZE / 2.0,
        Universe::HEIGHT / 2.0 - Tile::SIZE / 2.0,
        0.0,
    );
    world.create_entity().with(map).with(map_transform).build();
}

pub struct TileSystem;

impl<'s> System<'s> for TileSystem {
    type SystemData = WriteStorage<'s, Tile>;

    fn run(&mut self, mut tiles: Self::SystemData) {
        let mut rng = thread_rng();
        for tile in (&mut tiles).join() {
            let is_energy = rng.gen_bool(0.001);
            if is_energy {
                tile.ttype = TileType::Energy(100.0);
            }
        }
    }
}
