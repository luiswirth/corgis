mod conversion;
mod neural_network;
pub mod perception;
mod think;

pub use perception::{body::{PerceiveBodySystem, BodyPerception}, environment::{EnvironmentPerception, PerceiveEnvironmentSystem}};
pub use think::ThinkSystem;

use crate::genes::BrainGene;
use neural_network::NeuralNetwork;

use amethyst::{
    core::math::{DVector, Vector2},
    ecs::{Component, DenseVecStorage},
    renderer::palette::{Hsl, RgbHue},
};


#[derive(Component)]
pub struct Brain {
    neural_network: NeuralNetwork,
    io_cache: Option<Vec<f32>>,
}

impl Brain {
    pub fn new(gene: BrainGene) -> Self {
        Self {
            neural_network: NeuralNetwork::new(gene),
            io_cache: Some(Vec::new()),
        }
    }

    pub(self) fn think(&mut self, perception: Perception) -> Decision {
        // Perception (refs) -> owned Vec -> DVector -> NeuralNetwork -> DVector -> Vec -> Decisons

        perception.clone_into(&mut self.io_cache.unwrap());
        let input = DVector::from_vec(self.io_cache.take());
        let output = self.neural_network.feed(input);
        let decisions = output.data.into();
        let decisions = Decision(decisions);
        self.io_cache = Some(output);
        decisions
    }
}

// For the NeuroEvolution to work we need to ensure,
// that the order of filling the BrainIO and emptying it remains always the same
// concurrency can therefore be a problem

#[derive(Default, Clone)]
pub struct BrainInput {
    pub data: Vec<f32>,
}

impl BrainInput {
    pub fn perceive(&mut self, value: f32) {
        self.data.push(value);
    }
}

#[derive(Default, Clone)]
pub struct BrainOutput {
    pub data: Vec<f32>,
}

impl BrainOutput {
    pub fn decide(&mut self) -> f32 {
        self.data.pop().unwrap()
    }
}

// For the Input and Output sizes we should implement some runtime checks

struct Perception<'a> {
    environment: &'a EnvironmentPerception,
    body: &'a BodyPerception,
}

impl<'a> Perception {
    fn clone_into(self, dest: &mut Vec<f32>) {
        self.environment.0.data.clone_into(dest);
        self.body.0.data.clone_into(dest);
    }
}


/// Every perception type is a single component which is filled by its own system
/// they are defined in their corresponding system

/// All decisions are one big component

#[derive(Component, Default, Clone)]
pub struct Decision(pub BrainOutput);

#[derive(Default, Debug, Clone)]
pub struct Memory(pub [f32; Memory::SIZE]);

impl Memory {
    const SIZE: usize = 5;
}
