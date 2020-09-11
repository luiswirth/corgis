pub mod neural_network;
pub mod system;

use crate::{brain::neural_network::NeuralNetwork, corgi::Corgi, genes::BrainGene};
use amethyst::renderer::palette::Hsv;
use na::{DVector, Vector2};

pub trait BrainInput {
    fn to_input(self) -> Vec<f32>;
}

// traits
pub trait BrainOutput {
    fn from_output(output: &mut Vec<f32>) -> Self;
}

// brain structs
pub struct Brain {
    neural_network: NeuralNetwork,
    pub memory: Option<Memory>,
}

//#[derive(BrainInput)]
pub struct Perception {
    body: BodyPerception,
    environment: EnvironmentPerception,
    memory: Option<Memory>,
}

//#[derive(BrainOutput)]
pub struct Decisions {
    pub force: Vector2<f32>,
    pub reproduction_will: bool,
    pub color: Hsv,
    pub memory: Memory,
}

//#[derive(BrainInput)]
pub struct BodyPerception {
    energy: InputF32,
    mass: InputF32,
}

//#[derive(BrainInput)]
pub struct EnvironmentPerception {
    velocity: InputVector2,
}

#[derive(Debug, Clone)]
pub struct Memory(pub [f32; 5]);

impl Brain {
    pub fn new(gene: BrainGene) -> Self {
        Self {
            neural_network: NeuralNetwork::new(gene),
            memory: None,
        }
    }

    pub fn think(&self, perception: Perception) -> Decisions {
        Decisions::from_output(
            self.neural_network
                .feed(DVector::from_vec(perception.to_input()))
                .iter()
                .cloned()
                .collect(),
        )
    }
}

impl Perception {
    pub fn collect(corgi: &mut Corgi) -> Self {
        Self {
            body: BodyPerception {
                energy: InputF32(corgi.energy),
                mass: InputF32(corgi.mass),
            },
            environment: EnvironmentPerception {
                velocity: InputVector2(corgi.velocity),
            },
            memory: corgi.brain.memory.take(),
        }
    }
}

// ---------------------------------------------------

// write derive macros for both BrainInput and BrainOutput
// this should be generated
impl BrainInput for Perception {
    fn to_input(self) -> Vec<f32> {
        self.body
            .to_input()
            .append(&mut self.environment.to_input())
            .append(&mut self.memory.to_input())
    }
}

impl BrainInput for BodyPerception {
    fn to_input(self) -> Vec<f32> {
        self.energy.to_input().append(self.mass.to_input())
    }
}

impl BrainInput for EnvironmentPerception {
    fn to_input(self) -> Vec<f32> {
        self.velocity.to_input()
    }
}

impl BrainInput for Memory {
    fn to_input(self) -> Vec<f32> {
        self.0.into()
    }
}

// this should generated
impl BrainOutput for Decisions {
    fn from_output(output: Vec<f32>) -> Self {
        Self {
            force: Vector2::from_output(output[0..2]),
            reproduction_will: bool::from_output(output[2..3]),
            color: Hsv::from_output(output[3..6]),
            memory: Memory::from_output(output[6..11]),
        }
    }
}

// ---------------------------------------------------
struct InputF32(f32);
impl BrainInput for InputF32 {
    fn to_input(self) -> Vec<f32> {
        vec![self.0]
    }
}

struct InputBool(bool);
impl BrainInput for InputBool {
    fn to_input(self) -> Vec<f32> {
        vec![if self.0 { 1.0 } else { 0.0 }]
    }
}

struct InputVector2(Vector2<f32>);
impl BrainInput for InputVector2 {
    fn to_input(self) -> Vec<f32> {
        self.0.iter().cloned().collect()
    }
}

struct OutputHsv(Hsv);
impl BrainOutput for OutputHsv {
    fn from_output(output: Vec<f32>) -> Self {
        OutputHsv(Hsv::new(output[0] * 360.0 - 180.0, 1.0, 1.0))
    }
}

impl BrainOutput for Memory {
    fn from_output(output: Vec<f32>) -> Self {
        Memory([output[0], output[1], output[2], output[3], output[4]])
    }
}
