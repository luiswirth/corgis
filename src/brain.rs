use crate::{corgi::Corgi, genes::BrainGene, neural_network::NeuralNetwork};
use amethyst::core::Time;
use na::{DVector, Vector2};

pub struct Brain {
    neural_network: NeuralNetwork,
}

//#[derive(BrainInput)]
pub struct Perception {
    body: BodyPerception,
    environment: EnvironmentPerception,
}

//#[derive(BrainOutput)]
pub struct Decisions {
    pub force: Vector2<f32>,
    pub reproduction_will: bool,
}

//#[derive(BrainInput)]
pub struct BodyPerception {
    energy: f32,
    mass: f32,
}

//#[derive(BrainInput)]
pub struct EnvironmentPerception {
    velocity: Vector2<f32>,
    time: f32,
}

pub trait BrainInput {
    fn len() -> usize;
    fn to_input(self) -> DVector<f32>;
}

pub trait BrainOutput {
    fn len() -> usize;
    fn from_output(output: DVector<f32>) -> Self;
}

impl Brain {
    pub fn new(gene: BrainGene) -> Self {
        Self {
            neural_network: NeuralNetwork::new(gene),
        }
    }

    pub fn think(&self, perception: Perception) -> Decisions {
        Decisions::from_output(self.neural_network.feed(perception.to_input()))
    }
}

impl Perception {
    pub fn collect(corgi: &Corgi, time: &Time) -> Self {
        Self {
            body: BodyPerception {
                energy: corgi.energy,
                mass: corgi.mass,
            },
            environment: EnvironmentPerception {
                velocity: corgi.velocity,
                time: time.absolute_time().as_secs_f32(),
            },
        }
    }
}

// ---------------------------------------------------

// write derive macros for both BrainInput and BrainOutput
// this should be generated
impl BrainInput for Perception {
    fn len() -> usize {
        BodyPerception::len() + EnvironmentPerception::len()
    }

    fn to_input(self) -> DVector<f32> {
        let body = self.body.to_input();
        let environment = self.environment.to_input();
        DVector::from_iterator(
            Self::len(),
            body.into_iter().chain(environment.into_iter()).cloned(),
        )
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
        2 + 1
    }

    fn to_input(self) -> DVector<f32> {
        let velocity = self.velocity.to_input();
        let time = self.time.to_input();
        DVector::from_iterator(
            Self::len(),
            velocity.into_iter().chain(time.into_iter()).cloned(),
        )
    }
}

// this should generated
impl BrainOutput for Decisions {
    fn len() -> usize {
        3
    }

    fn from_output(output: DVector<f32>) -> Self {
        Self {
            force: Vector2::from_output(output.rows(0, 2).into_owned()),
            reproduction_will: bool::from_output(output.rows(2, 1).into_owned()),
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
        Self::from_iterator(output.iter().cloned())
    }
}
