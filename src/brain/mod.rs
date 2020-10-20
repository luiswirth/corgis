mod neural_network;
mod perceive_body;
mod perceive_environment;
mod think;

pub use perceive_body::PerceiveBodySystem;
pub use perceive_environment::PerceiveEnvironmentSystem;
pub use think::ThinkSystem;

use crate::{brain::neural_network::NeuralNetwork, genes::BrainGene};
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

pub trait BrainInput {
    fn len() -> usize;
    fn into_input(self, input: &mut Vec<f32>);
}

pub trait BrainOutput {
    fn len() -> usize;
    fn from_output(output: &mut Vec<f32>) -> Self;
}

/// Every perception type is a single component

//#[derive(BrainInput)]
#[derive(Default)]
pub struct Perception {
    body: BodyPerception,
    environment: EnvironmentPerception,
    memory: Memory,
}

//#[derive(BrainInput)]
#[derive(Component, Default, Clone)]
pub struct BodyPerception {
    energy: IoF32,
    mass: IoF32,
}

//#[derive(BrainInput)]
#[derive(Component, Default, Clone)]
pub struct EnvironmentPerception {
    velocity: IoVector2,
    tile_color: IoHsl,
}

/// All decisions are one big component

//#[derive(BrainOutput)]
#[derive(Component, Default, Clone)]
pub struct Decision {
    pub orientation_change: IoF32,
    pub force: IoF32,
    pub reproduction_will: IoBool,
    pub color: IoHsl,
    pub memory: Memory,
}

#[derive(Default, Debug, Clone)]
pub struct Memory(pub [f32; Memory::SIZE]);

impl Memory {
    const SIZE: usize = 5;
}

impl Brain {
    pub fn new(gene: BrainGene) -> Self {
        Self {
            neural_network: NeuralNetwork::new(gene),
            io_cache: Some(Vec::with_capacity(usize::min(
                Perception::len(),
                Decision::len(),
            ))),
        }
    }

    pub(self) fn think(&mut self, perception: Perception) -> Decision {
        let mut input = self.io_cache.take().unwrap();
        perception.into_input(&mut input);
        let mut output = self
            .neural_network
            .feed(DVector::from_vec(input))
            .data
            .into();
        let decisions = Decision::from_output(&mut output);
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

    fn into_input(self, input: &mut Vec<f32>) {
        self.body.into_input(input);
        self.environment.into_input(input);
        self.memory.into_input(input);
    }
}

impl BrainInput for BodyPerception {
    fn len() -> usize {
        <IoF32 as BrainInput>::len() + <IoF32 as BrainInput>::len()
    }

    fn into_input(self, input: &mut Vec<f32>) {
        self.energy.into_input(input);
        self.mass.into_input(input);
    }
}

impl BrainInput for EnvironmentPerception {
    fn len() -> usize {
        <IoVector2 as BrainInput>::len() + <IoHsl as BrainInput>::len()
    }

    fn into_input(self, input: &mut Vec<f32>) {
        self.velocity.into_input(input);
        self.tile_color.into_input(input);
    }
}

impl BrainOutput for Decision {
    fn len() -> usize {
        <IoVector2 as BrainOutput>::len()
            + <IoBool as BrainOutput>::len()
            + <IoHsl as BrainOutput>::len()
            + <Memory as BrainOutput>::len()
    }

    fn from_output(output: &mut Vec<f32>) -> Self {
        Self {
            orientation_change: IoF32::from_output(output),
            force: IoF32::from_output(output),
            reproduction_will: IoBool::from_output(output),
            color: IoHsl::from_output(output),
            memory: Memory::from_output(output),
        }
    }
}

// ---------------------------------------------------

#[derive(Default, Clone)]
pub struct IoF32(pub f32);
impl BrainInput for IoF32 {
    fn len() -> usize {
        1
    }

    fn into_input(self, input: &mut Vec<f32>) {
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

#[derive(Default, Clone)]
pub struct IoBool(pub bool);
impl BrainInput for IoBool {
    fn len() -> usize {
        1
    }
    fn into_input(self, input: &mut Vec<f32>) {
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

#[derive(Clone)]
pub struct IoVector2(pub Vector2<f32>);
impl BrainInput for IoVector2 {
    fn len() -> usize {
        2
    }
    fn into_input(self, input: &mut Vec<f32>) {
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
impl Default for IoVector2 {
    fn default() -> Self {
        Self(Vector2::new(0.0, 0.0))
    }
}

#[derive(Default, Clone)]
pub struct IoHsl(pub Hsl);
impl BrainInput for IoHsl {
    fn len() -> usize {
        3
    }
    fn into_input(self, input: &mut Vec<f32>) {
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

    fn into_input(self, input: &mut Vec<f32>) {
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
