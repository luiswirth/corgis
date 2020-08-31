use amethyst::{
    core::math::Point3,
    ecs::{Component, DenseVecStorage, World},
    tiles,
};
use na::Point2;

#[derive(Clone)]
pub struct Tile {
    pub position: Point2<f32>,
    pub ttype: TileType,
}

impl Tile {
    pub const SIZE: f32 = 20.0;
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            position: Point2::new(0.0, 0.0),
            ttype: TileType::Energy(0.0),
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Tile>;
}

impl tiles::Tile for Tile {
    fn sprite(&self, _coordinates: Point3<u32>, _world: &World) -> Option<usize> {
        Some(1)
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
