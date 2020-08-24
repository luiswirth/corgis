use crate::{genes::Genes, neural_network::NeuralNetwork};
use amethyst::ecs::prelude::*;

pub struct Cogi {
    pub name: String,
    pub color: [f32; 4],
    pub energy: f32,

    pub velocity: [f32; 2],
    pub force: [f32; 2],

    pub genes: Genes,
    pub brain: CogiBrain,
}
impl Cogi {
    pub const INITAL_ENERGY: f32 = 200.0;
}

pub struct CogiBrain {
    pub neural_network: NeuralNetwork,
}

impl Component for Cogi {
    type Storage = DenseVecStorage<Self>;
}
