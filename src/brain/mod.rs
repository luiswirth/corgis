pub mod neural_network;
pub mod system;

use crate::{
    brain::neural_network::NeuralNetwork,
    corgi::Corgi,
    genes::BrainGene,
    na::{DVector, Vector2},
};
use amethyst::renderer::palette::Hsv;

pub struct Brain {
    neural_network: NeuralNetwork,
    pub memory: Memory,
}

pub trait BrainInput {
    fn len() -> usize;
    fn to_input(self) -> Vec<f32>;
}

pub trait BrainOutput {
    fn len() -> usize;
    fn from_output(output: Vec<f32>) -> Self;
}

//#[derive(BrainInput)]
pub struct Perception {
    body: BodyPerception,
    environment: EnvironmentPerception,
    memory: Memory,
}

//#[derive(BrainOutput)]
pub struct Decisions {
    pub force: IoVector2,
    pub reproduction_will: IoBool,
    pub color: IoHsv,
    pub memory: Memory,
}

//#[derive(BrainInput)]
pub struct BodyPerception {
    energy: IoF32,
    mass: IoF32,
}

//#[derive(BrainInput)]
pub struct EnvironmentPerception {
    velocity: IoVector2,
}

#[derive(Debug, Clone)]
pub struct Memory(pub Option<[f32; Memory::SIZE]>);

impl Memory {
    const SIZE: usize = 5;
}

impl Brain {
    pub fn new(gene: BrainGene) -> Self {
        Self {
            neural_network: NeuralNetwork::new(gene),
            memory: Memory(None),
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
        let memory = corgi.brain.memory.0.take();
        Self {
            body: BodyPerception {
                energy: IoF32(corgi.energy),
                mass: IoF32(corgi.mass),
            },
            environment: EnvironmentPerception {
                velocity: IoVector2(corgi.velocity),
            },
            memory: Memory(memory),
        }
    }
}

// ---------------------------------------------------

// write derive macros for both BrainInput and BrainOutput
// this should be generated
impl BrainInput for Perception {
    fn len() -> usize {
        BodyPerception::len() + EnvironmentPerception::len() + <Memory as BrainInput>::len()
    }

    fn to_input(self) -> Vec<f32> {
        let mut input = self.body.to_input();
        input.append(&mut self.environment.to_input());
        input.append(&mut self.memory.to_input());
        input
    }
}

impl BrainInput for BodyPerception {
    fn len() -> usize {
        <IoF32 as BrainInput>::len() + <IoF32 as BrainInput>::len()
    }

    fn to_input(self) -> Vec<f32> {
        let mut input = self.energy.to_input();
        input.append(&mut self.mass.to_input());
        input
    }
}

impl BrainInput for EnvironmentPerception {
    fn len() -> usize {
        <IoVector2 as BrainInput>::len()
    }

    fn to_input(self) -> Vec<f32> {
        self.velocity.to_input()
    }
}

impl BrainOutput for Decisions {
    fn len() -> usize {
        <IoVector2 as BrainOutput>::len()
            + <IoBool as BrainOutput>::len()
            + IoHsv::len()
            + <Memory as BrainOutput>::len()
    }

    fn from_output(output: Vec<f32>) -> Self {
        Self {
            force: IoVector2::from_output(output[0..2].to_vec()),
            reproduction_will: IoBool::from_output(output[2..3].to_vec()),
            color: IoHsv::from_output(output[3..4].to_vec()),
            memory: Memory::from_output(output[4..9].to_vec()),
        }
    }
}

// ---------------------------------------------------

struct IoF32(f32);
impl BrainInput for IoF32 {
    fn len() -> usize {
        1
    }

    fn to_input(self) -> Vec<f32> {
        vec![self.0]
    }
}
impl BrainOutput for IoF32 {
    fn len() -> usize {
        1
    }

    fn from_output(output: Vec<f32>) -> Self {
        Self(output[0])
    }
}

pub struct IoBool(bool);
impl BrainInput for IoBool {
    fn len() -> usize {
        1
    }
    fn to_input(self) -> Vec<f32> {
        vec![if self.0 { 1.0 } else { 0.0 }]
    }
}
impl BrainOutput for IoBool {
    fn len() -> usize {
        1
    }
    fn from_output(output: Vec<f32>) -> Self {
        Self(output[0] >= 0.5)
    }
}

pub struct IoVector2(Vector2<f32>);
impl BrainInput for IoVector2 {
    fn len() -> usize {
        2
    }
    fn to_input(self) -> Vec<f32> {
        self.0.iter().cloned().collect()
    }
}
impl BrainOutput for IoVector2 {
    fn len() -> usize {
        2
    }
    fn from_output(output: Vec<f32>) -> Self {
        IoVector2(Vector2::from_iterator(output.iter().map(|v| v * 2.0 - 1.0)))
    }
}

pub struct IoHsv(Hsv);
impl BrainOutput for IoHsv {
    fn len() -> usize {
        1
    }
    fn from_output(output: Vec<f32>) -> Self {
        IoHsv(Hsv::new(output[0] * 360.0 - 180.0, 1.0, 1.0))
    }
}

impl BrainInput for Memory {
    fn len() -> usize {
        Self::SIZE
    }

    fn to_input(self) -> Vec<f32> {
        self.0.unwrap_or([0.0; Self::SIZE]).into()
    }
}
impl BrainOutput for Memory {
    fn len() -> usize {
        Self::SIZE
    }
    fn from_output(output: Vec<f32>) -> Self {
        Self(Some([
            output[0], output[1], output[2], output[3], output[4],
        ]))
    }
}
