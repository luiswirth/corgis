pub mod movement;
pub mod reproduce;
pub mod spawner;

use std::fmt::Debug;

use crate::{brain::Brain, genes::Genome, na::Vector2};
use amethyst::{ecs::prelude::*, renderer::palette::Hsl};

pub struct Corgi {
    pub uuid: u128,
    pub name: String,
    pub generation: u32,
    pub age: u32,

    pub energy: f32,
    pub mass: f32,
    pub velocity: Vector2<f32>,
    pub force: Vector2<f32>,

    pub genes: Genome,
    pub brain: Brain,

    pub color: Hsl,
    pub reproduction_will: bool,
}

impl Corgi {
    pub const INITAL_ENERGY: f32 = 200.0;
    pub const BORN_ENERGY: f32 = 50.0;
    pub const REPRODUCTION_WORK: f32 = 300.0;

    pub const LIFE_WORK: f32 = 100.0;
    pub const MOVEMENT_WORK: f32 = 0.8;
}

impl Component for Corgi {
    type Storage = DenseVecStorage<Self>;
}

impl Debug for Corgi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Corgi {{ uuid: {}}}", self.uuid)
    }
}
