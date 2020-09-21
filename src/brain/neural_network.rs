use crate::{genes::BrainGene, na::DVector};

pub struct NeuralNetwork {
    // get rid of owned copy and replace it with a reference &BrainGene
    genes: BrainGene,
}

impl NeuralNetwork {
    pub fn new(genes: BrainGene) -> Self {
        Self { genes }
    }

    pub fn feed(&self, mut inputs: DVector<f32>) -> DVector<f32> {
        for (w, b) in self.genes.weights.iter().zip(self.genes.biases.iter()) {
            inputs = w * inputs + b;
            inputs.apply(&sigmoid)
        }
        inputs
    }
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}
