pub mod neural_network;
pub mod system;

use crate::{
    brain::neural_network::NeuralNetwork,
    genes::BrainGene,
    na::{DVector, Vector2},
};
use amethyst::renderer::palette::{Hsl, RgbHue};

pub struct Brain {
    neural_network: NeuralNetwork,
    pub(self) memory: Option<Memory>,
    io_cache: Option<Vec<f32>>,
}

pub trait BrainInput {
    fn len() -> usize;
    fn to_input(self, input: &mut Vec<f32>);
}

pub trait BrainOutput {
    fn len() -> usize;
    fn from_output(output: &mut Vec<f32>) -> Self;
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
    pub color: IoHsl,
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
    tile_color: IoHsl,
}

#[derive(Debug, Clone)]
pub struct Memory(pub [f32; Memory::SIZE]);

impl Memory {
    const SIZE: usize = 5;
}

impl Brain {
    pub fn new(gene: BrainGene) -> Self {
        Self {
            neural_network: NeuralNetwork::new(gene),
            memory: None,
            io_cache: Some(Vec::with_capacity(usize::min(
                Perception::len(),
                Decisions::len(),
            ))),
        }
    }

    pub fn think(&mut self, perception: Perception) -> Decisions {
        let mut input = self.io_cache.take().unwrap();
        perception.to_input(&mut input);
        let mut output = self
            .neural_network
            .feed(DVector::from_vec(input))
            .data
            .into();
        let decisions = Decisions::from_output(&mut output);
        self.io_cache = Some(output);
        decisions
    }
}

// ---------------------------------------------------

// write derive macros for both BrainInput and BrainOutput
// this should be generated
impl BrainInput for Perception {
    fn len() -> usize {
        BodyPerception::len() + EnvironmentPerception::len() + <Memory as BrainInput>::len()
    }

    fn to_input(self, input: &mut Vec<f32>) {
        self.body.to_input(input);
        self.environment.to_input(input);
        self.memory.to_input(input);
    }
}

impl BrainInput for BodyPerception {
    fn len() -> usize {
        <IoF32 as BrainInput>::len() + <IoF32 as BrainInput>::len()
    }

    fn to_input(self, input: &mut Vec<f32>) {
        self.energy.to_input(input);
        self.mass.to_input(input);
    }
}

impl BrainInput for EnvironmentPerception {
    fn len() -> usize {
        <IoVector2 as BrainInput>::len() + <IoHsl as BrainInput>::len()
    }

    fn to_input(self, input: &mut Vec<f32>) {
        self.velocity.to_input(input);
        self.tile_color.to_input(input);
    }
}

impl BrainOutput for Decisions {
    fn len() -> usize {
        <IoVector2 as BrainOutput>::len()
            + <IoBool as BrainOutput>::len()
            + <IoHsl as BrainOutput>::len()
            + <Memory as BrainOutput>::len()
    }

    fn from_output(output: &mut Vec<f32>) -> Self {
        Self {
            force: IoVector2::from_output(output),
            reproduction_will: IoBool::from_output(output),
            color: IoHsl::from_output(output),
            memory: Memory::from_output(output),
        }
    }
}

// ---------------------------------------------------

struct IoF32(f32);
impl BrainInput for IoF32 {
    fn len() -> usize {
        1
    }

    fn to_input(self, input: &mut Vec<f32>) {
        input.push(self.0)
    }
}
impl BrainOutput for IoF32 {
    fn len() -> usize {
        1
    }

    fn from_output(output: &mut Vec<f32>) -> Self {
        Self(output.pop().unwrap())
    }
}

pub struct IoBool(bool);
impl BrainInput for IoBool {
    fn len() -> usize {
        1
    }
    fn to_input(self, input: &mut Vec<f32>) {
        input.push(if self.0 { 1.0 } else { 0.0 })
    }
}
impl BrainOutput for IoBool {
    fn len() -> usize {
        1
    }
    fn from_output(output: &mut Vec<f32>) -> Self {
        Self(output.pop().unwrap() >= 0.5)
    }
}

pub struct IoVector2(Vector2<f32>);
impl BrainInput for IoVector2 {
    fn len() -> usize {
        2
    }
    fn to_input(self, input: &mut Vec<f32>) {
        input.extend(self.0.into_iter())
    }
}
impl BrainOutput for IoVector2 {
    fn len() -> usize {
        2
    }
    fn from_output(output: &mut Vec<f32>) -> Self {
        Self(Vector2::from_iterator(
            output
                .drain((output.len() - <Self as BrainOutput>::len())..)
                .map(|v| v * 2.0 - 1.0),
        ))
    }
}

pub struct IoHsl(Hsl);
impl BrainInput for IoHsl {
    fn len() -> usize {
        3
    }
    fn to_input(self, input: &mut Vec<f32>) {
        input.extend([self.0.hue.to_radians(), self.0.saturation, self.0.lightness].iter());
    }
}
impl BrainOutput for IoHsl {
    fn len() -> usize {
        1
    }
    fn from_output(output: &mut Vec<f32>) -> Self {
        let hue = RgbHue::from_radians(
            output.pop().unwrap() * std::f32::consts::TAU - std::f32::consts::PI,
        );
        IoHsl(Hsl::new(hue, 1.0, 0.5))
    }
}

impl BrainInput for Memory {
    fn len() -> usize {
        Self::SIZE
    }

    fn to_input(self, input: &mut Vec<f32>) {
        input.extend(self.0.iter())
    }
}
impl BrainOutput for Memory {
    fn len() -> usize {
        Self::SIZE
    }
    fn from_output(output: &mut Vec<f32>) -> Self {
        //Self(output.drain((output.len() - Self::len())..).collect())
        Self([
            output.pop().unwrap(),
            output.pop().unwrap(),
            output.pop().unwrap(),
            output.pop().unwrap(),
            output.pop().unwrap(),
        ])
    }
}
