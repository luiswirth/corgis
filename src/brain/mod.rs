pub mod neural_network;
pub mod system;

use crate::{brain::neural_network::NeuralNetwork, corgi::Corgi, genes::BrainGene};
use amethyst::renderer::palette::Hsv;
use na::{DVector, Vector2};

// traits
pub trait BrainInput {
    fn len() -> usize;
    fn to_input(self) -> DVector<f32>;
}

// brain structs
pub trait BrainOutput {
    fn len() -> usize;
    fn from_output(output: DVector<f32>) -> Self;
}

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
    energy: f32,
    mass: f32,
}

//#[derive(BrainInput)]
pub struct EnvironmentPerception {
    velocity: Vector2<f32>,
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
        Decisions::from_output(self.neural_network.feed(perception.to_input()))
    }
}

impl Perception {
    pub fn collect(corgi: &mut Corgi) -> Self {
        Self {
            body: BodyPerception {
                energy: corgi.energy,
                mass: corgi.mass,
            },
            environment: EnvironmentPerception {
                velocity: corgi.velocity,
            },
            memory: corgi.brain.memory.take(),
        }
    }
}

// ---------------------------------------------------

// write derive macros for both BrainInput and BrainOutput
// this should be generated
impl BrainInput for Perception {
    fn len() -> usize {
        BodyPerception::len() + EnvironmentPerception::len() + 5
    }

    fn to_input(self) -> DVector<f32> {
        let body = self.body.to_input();
        let environment = self.environment.to_input();
        if let Some(memory) = self.memory {
            let memory = memory.to_input();
            DVector::from_iterator(
                Self::len(),
                body.into_iter()
                    .chain(environment.into_iter())
                    .chain(memory.into_iter())
                    .cloned(),
            )
        } else {
            let memory = [0.0; 5];
            DVector::from_iterator(
                Self::len(),
                body.into_iter()
                    .chain(environment.into_iter())
                    .chain(memory.iter())
                    .cloned(),
            )
        }
    }
}

impl BrainInput for BodyPerception {
    fn len() -> usize {
        f32::len() + f32::len()
    }

    fn to_input(self) -> DVector<f32> {
        let energy = self.energy.to_input();
        let mass = self.mass.to_input();
        DVector::from_iterator(
            Self::len(),
            energy.into_iter().chain(mass.into_iter()).cloned(),
        )
    }
}

impl BrainInput for EnvironmentPerception {
    fn len() -> usize {
        //Vector2::len() + f32::len()
        2
    }

    fn to_input(self) -> DVector<f32> {
        let velocity = self.velocity.to_input();
        DVector::from_iterator(Self::len(), velocity.into_iter().cloned())
    }
}

// this should generated
impl BrainOutput for Decisions {
    fn len() -> usize {
        6 + 5
    }

    fn from_output(output: DVector<f32>) -> Self {
        Self {
            force: Vector2::from_output(output.rows(0, 2).into_owned()),
            reproduction_will: bool::from_output(output.rows(2, 1).into_owned()),
            color: Hsv::from_output(output.rows(3, 3).into_owned()),
            memory: Memory::from_output(output.rows(6, 5).into_owned()),
        }
    }
}

// ---------------------------------------------------

impl BrainInput for f32 {
    fn len() -> usize {
        1
    }

    fn to_input(self) -> DVector<f32> {
        DVector::from_element(1, self)
    }
}

impl BrainInput for bool {
    fn len() -> usize {
        1
    }

    fn to_input(self) -> DVector<f32> {
        let output = self as u32 as f32;
        DVector::from_element(1, output)
    }
}

impl BrainInput for Vector2<f32> {
    fn len() -> usize {
        2
    }

    fn to_input(self) -> DVector<f32> {
        DVector::from_iterator(2, self.iter().cloned())
    }
}

impl BrainInput for Memory {
    fn len() -> usize {
        5
    }

    fn to_input(self) -> DVector<f32> {
        DVector::from_iterator(5, self.0.iter().cloned())
    }
}

impl BrainOutput for bool {
    fn len() -> usize {
        1
    }

    fn from_output(output: DVector<f32>) -> Self {
        debug_assert_eq!(output.nrows(), 1);
        output[0] >= 0.5
    }
}

impl BrainOutput for Vector2<f32> {
    fn len() -> usize {
        2
    }

    fn from_output(output: DVector<f32>) -> Self {
        Self::new(output[0] * 2.0 - 1.0, output[1] * 2.0 - 1.0)
    }
}

impl BrainOutput for Hsv {
    fn len() -> usize {
        3
    }

    fn from_output(output: DVector<f32>) -> Self {
        //Hsv::new(output[0] * 360.0 - 180.0, output[1], output[2])
        Hsv::new(output[0] * 360.0 - 180.0, 1.0, 1.0)
    }
}

impl BrainOutput for Memory {
    fn len() -> usize {
        5
    }

    fn from_output(output: DVector<f32>) -> Self {
        Memory([output[0], output[1], output[2], output[3], output[4]])
    }
}
