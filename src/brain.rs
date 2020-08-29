use na::{DVector, Vector2, U2};

//#[derive(BrainInput)]
pub struct BrainInputs {
    velocity: Vector2<f32>,
    time: f32,
    energy: f32,
}

//#[derive(BrainOutput)]
pub struct BrainOutputs {
    force: [f32; 2],
    will_to_reproduce: bool,
}

pub trait BrainInput {
    fn to_input(self) -> Vector2<f32>;
}

pub trait BrainOutput {
    fn from_output(output: DVector<f32>) -> Self;
}

// write derive macros for both BrainInput and BrainOutput

// this should be generated
impl BrainInput for BrainInputs {
    fn to_input(self) -> DVector<f32> {
        let mut output = DVector::new();
        output.extend(self.velocity.to_input());
        output.extend(self.time.to_input());
        output.extend(self.energy.to_input);
        output
    }
}

// this should generated
impl BrainOutput for BrainOutputs {
    fn from_output(output: DVector<f32>) -> Self {
        Self {
            // maybe use fixed_rows
            force: Vector2::from_output(output.rows(0, 2)),
            will_to_reproduce: bool::from_output(output.rows(2, 2)),
        }
    }
}

impl< BrainInput for bool {
    fn to_input(self) -> DVector<f32> {
        let output = self as u32 as f32;
        DVector::from_element(1, output)
    }
}

impl BrainOutput for bool {
    fn from_output(output: DVector<f32>) -> Self {
        debug_assert_eq!(output.nrows(), 1);
        output[0] >= 0.5
    }
}
