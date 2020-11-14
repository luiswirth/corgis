use crate::brain::{BrainInput, BrainOutput, Decision};
use amethyst::core::math::{DMatrix, DVector};
use rand::{distributions::Uniform, Rng};
use rand_distr::Normal;

const MUTATION_STRENGTH: f32 = 1.0;

#[derive(Debug, Clone)]
pub struct Genome {
    pub brain: BrainGene,
    pub color: ColorGene,
}

impl Genome {
    pub fn random(rng: &mut impl Rng) -> Self {
        Self {
            brain: BrainGene::random(rng),
            color: ColorGene::random(rng),
        }
    }

    pub fn mutate(&mut self, rng: &mut impl Rng) {
        self.brain.mutate(MUTATION_STRENGTH, rng);
        self.color.mutate(MUTATION_STRENGTH, rng);
    }
}

trait Gene {
    // maybe use rand::distributions::Distribution
    fn random(rng: &mut impl Rng) -> Self;
    fn mutate(&mut self, strength: f32, rng: &mut impl Rng);
}

/// A mutation factor which dictates how much all genes are mutated.
/// It itself is a gene so that itself can be mutated.
//#[derive(Debug, Clone)]
//pub struct MutationGene(f32);

//impl Gene for MutationGene {
//    fn random(rng: &mut impl Rng) -> Self {
//        MutationGene(rng.sample(Normal::new(Self::MEAN, Self::VARIANCE).unwrap()))
//    }
//
//    fn mutate(&mut self, _: f32, rng: &mut impl Rng) {
//        self.0 += rng.sample(Normal::new(Self::MEAN, Self::VARIANCE).unwrap());
//    }
//}

#[derive(Debug, Clone)]
pub struct BrainGene {
    pub shape: Vec<usize>,
    pub weights: Vec<DMatrix<f32>>,
    pub biases: Vec<DVector<f32>>,
}

impl BrainGene {
    const DEPTH_MEAN: f32 = 2.0;
    const DEPTH_VARIANCE: f32 = 2.0;

    const WIDTH_MEAN: f32 = 5.0;
    const WIDTH_VARIANCE: f32 = 4.0;

    const WEIGHT_MEAN: f32 = 0.0;
    const WEIGHT_VARIANCE: f32 = 5.0;

    const WEIGHT_MUTATION_MEAN: f32 = 0.0;
    const WEIGHT_MUTATION_VARIANCE: f32 = 0.1;
}

impl Gene for BrainGene {
    fn random(rng: &mut impl Rng) -> Self {
        // generate shape
        let depth = rng
            .sample(Normal::new(Self::DEPTH_MEAN, Self::DEPTH_VARIANCE).unwrap())
            .round() as usize;
        let mut shape = Vec::with_capacity(depth + 2);
        for _ in 0..depth {
            shape.push(
                rng.sample(Normal::new(Self::WIDTH_MEAN, Self::WIDTH_VARIANCE).unwrap())
                    .round() as usize,
            );
        }

        shape.insert(0, 10); // TODO: find out acctual number
        shape.push(10); // TODO: find out acctual number

        // generate weights and biases
        let distr = Normal::new(Self::WEIGHT_MEAN, Self::WEIGHT_VARIANCE).unwrap();
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        for window in shape.windows(2) {
            if let [c, n] = *window {
                weights.push(DMatrix::from_distribution(n, c, &distr, rng));
                biases.push(DVector::from_distribution(n, &distr, rng));
            }
        }

        Self {
            shape,
            weights,
            biases,
        }
    }

    // TODO: implement shape mutation
    fn mutate(&mut self, strength: f32, rng: &mut impl Rng) {
        let distr = Normal::<f32>::new(
            Self::WEIGHT_MUTATION_MEAN,
            Self::WEIGHT_MUTATION_VARIANCE * strength,
        )
        .unwrap();

        for (w, b) in self.weights.iter_mut().zip(self.biases.iter_mut()) {
            let weight_mutation = DMatrix::from_distribution(w.nrows(), w.ncols(), &distr, rng);
            let bias_mutation = DVector::from_distribution(b.nrows(), &distr, rng);
            *w += weight_mutation;
            *b += bias_mutation;
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorGene {
    r: u8,
    g: u8,
    b: u8,
}

impl ColorGene {
    const MUTATION_MEAN: f32 = 1.0;
    const MUTATION_VARIATION: f32 = 1.0;
}

impl Gene for ColorGene {
    fn random(rng: &mut impl Rng) -> Self {
        let r = rng.sample(Uniform::new(0, 255));
        let g = rng.sample(Uniform::new(0, 255));
        let b = rng.sample(Uniform::new(0, 255));
        Self { r, g, b }
    }

    fn mutate(&mut self, strength: f32, rng: &mut impl Rng) {
        self.r = self.r.wrapping_add(
            rng.sample(
                Normal::new(Self::MUTATION_MEAN, Self::MUTATION_VARIATION * strength).unwrap(),
            )
            .round() as u8,
        );
        self.g = self.g.wrapping_add(
            rng.sample(
                Normal::new(Self::MUTATION_MEAN, Self::MUTATION_VARIATION * strength).unwrap(),
            )
            .round() as u8,
        );
        self.b = self.b.wrapping_add(
            rng.sample(
                Normal::new(Self::MUTATION_MEAN, Self::MUTATION_VARIATION * strength).unwrap(),
            )
            .round() as u8,
        );
    }
}
