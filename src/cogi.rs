use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{timing::Time, transform::Transform},
    ecs::prelude::*,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use crate::neural_network::NeuralNetwork;

pub struct Cogi {
    pub name: String,
    pub color: [f32; 4],
    pub velocity: [f32; 2],
    pub force: [f32; 2],

    pub brain: CogiBrain,
}

pub struct CogiBrain {
    pub neural_network: NeuralNetwork,
}

impl Component for Cogi {
    type Storage = DenseVecStorage<Self>;
}

