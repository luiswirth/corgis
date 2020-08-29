use crate::{brain::Brain, genes::Genes, util::types::Color};
use amethyst::ecs::prelude::*;
use nalgebra::Vector2;

pub struct Corgi {
    pub uuid: u128,
    pub name: String,
    pub generation: u32,

    pub energy: f32,
    pub mass: f32,
    pub velocity: Vector2<f32>,
    pub force: Vector2<f32>,

    pub genes: Genes,
    pub brain: Brain,

    pub color: Color,
    pub reproduction_will: bool,
}
impl Corgi {
    pub const INITAL_ENERGY: f32 = 200.0;
}

impl Component for Corgi {
    type Storage = DenseVecStorage<Self>;
}
