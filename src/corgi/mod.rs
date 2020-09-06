pub mod movement;
pub mod reproduce;
pub mod spawner;

use crate::{brain::Brain, genes::Genome};
use amethyst::{ecs::prelude::*, renderer::palette::Hsv};
use nalgebra::Vector2;

pub struct Corgi {
    pub uuid: u128,
    pub name: String,
    pub generation: u32,

    pub energy: f32,
    pub mass: f32,
    pub velocity: Vector2<f32>,
    pub force: Vector2<f32>,

    pub genes: Genome,
    pub brain: Brain,

    pub color: Hsv,
    pub reproduction_will: bool,
}

impl Corgi {
    pub const INITAL_ENERGY: f32 = 200.0;
    pub const BORN_ENERGY: f32 = 50.0;
    pub const REPRODUCTION_WORK: f32 = 100.0;

    pub const LIFE_WORK: f32 = 100.0;
    pub const MOVEMENT_WORK: f32 = 0.8;
}

impl Component for Corgi {
    type Storage = DenseVecStorage<Self>;
}
