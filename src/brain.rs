use nalgebra::DVector;


//#[derive(BrainInput)]
pub struct BrainInputs {
    velocity: [f32; 2],
    time: f32,
    energy: f32,
}

//#[derive(BrainOutput)]
pub struct BrainOutputs {
    force: [f32; 2],
    will_to_reproduce: bool,
}

pub trait BrainInput {
    fn to_input(self) -> DVector<f32>;
}

pub trait BrainOutput {
    fn from_output(output: DVector<f32>) -> Self;
}

// write derive macros for both BrainInput and BrainOutput

// this will generate something like this:
impl BrainInput for BrainInputs {
    fn to_input(self) -> DVector<f32> {
        let mut output = DVector::new();
        output.extend(self.velocity.to_input());
        output.extend(self.time.to_input());
        output.extend(self.energy.to_input);
        output
    }
}

impl BrainOutput for BrainOutputs {
    fn from_output(output: DVector<f32>) -> Self {
        Self {
            force: [f32; 2]::from_output(&output[0..2]),
            will_to_reproduce: bool::from_output(&output[2..3])
        }
    }
}

impl BrainOutput for bool {
    fn from_output(output: DVector<f32>) -> Self {
        debug_assert_eq!(output.nrows(), 1);
        output[0] >= 0.5
    }
}
