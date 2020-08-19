use nalgebra::{DMatrix, DVector};
use rand_distr::Normal;

const INITAL_MEAN: f32 = 0.0;
const INITAL_VARIANCE: f32 = 5.0;

const MUTATION_MEAN: f32 = 0.0;
const MUTATION_VARIANCE: f32 = 1.0;

struct NeuralNetwork {
    shape: Vec<usize>,
    weights: Vec<DMatrix<f32>>,
    biases: Vec<DVector<f32>>,
}

impl NeuralNetwork {
    fn random(shape: Vec<usize>) -> Self {
        let mut rng = rand::thread_rng();
        let distr = Normal::new(INITAL_MEAN, INITAL_VARIANCE).unwrap();
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        for window in shape.windows(2) {
            match *window {
                [c, n] => {
                    // TODO: make DMAtrix::from_distribution()
                    weights.push(DMatrix::from_distribution(n, c, &distr, &mut rng));
                    biases.push(DVector::from_distribution(n, &distr, &mut rng));
                }
                _ => {}
            }
        }

        Self {
            shape,
            weights,
            biases,
        }
    }

    fn feed(&self, mut inputs: DVector<f32>) -> DVector<f32> {
        for (w, b) in self.weights.iter().zip(self.biases.iter()) {
            inputs = w * inputs + b;
            inputs.apply(&sigmoid)
        }
        return inputs;
    }

    fn mutate_weights(&mut self) {
        let mut rng = rand::thread_rng();
        let distr = Normal::new(MUTATION_MEAN, MUTATION_VARIANCE).unwrap();

        for (w, b) in self.weights.iter_mut().zip(self.biases.iter_mut()) {
            let weight_mutation =
                DMatrix::from_distribution(w.ncols(), w.nrows(), &distr, &mut rng);
            let bias_mutation = DVector::from_distribution(b.nrows(), &distr, &mut rng);
            *w += weight_mutation;
            *b += bias_mutation;
        }
    }
}

fn sigmoid(x: f32) -> f32 {
    return 1.0 / (1.0 - (-x).exp());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_random() {
        let shape = vec![3, 4, 5, 3, 2];
        let nn = NeuralNetwork::random(shape.clone());
        assert_eq!(nn.weights.len(), shape.len() - 1);
        assert_eq!(nn.biases.len(), shape.len() - 1);
    }

    #[test]
    fn create_and_feed() {
        let shape = vec![3, 4, 5, 3, 5];
        let nn = NeuralNetwork::random(shape.clone());
        let inputs = DVector::from_element(shape[0], 8.0);
        let output = nn.feed(inputs);
        println!("{}", output);
    }
}