/// Activation function placeholder.
#[derive(Clone, Debug, Default)]
pub struct ActivationKernel;

impl ActivationKernel {
    pub fn gelu(x: f32) -> f32 {
        const C: f32 = 0.7978845608;
        let x2 = x * x;
        let x3 = x2 * x;
        x * (0.5 + 0.5 * ((C * (x + 0.044715 * x3)).tanh()))
    }

    pub fn relu(x: f32) -> f32 {
        x.max(0.0)
    }

    pub fn apply_gelu(values: &[f32]) -> Vec<f32> {
        values.iter().map(|&v| Self::gelu(v)).collect()
    }

    pub fn apply_relu(values: &[f32]) -> Vec<f32> {
        values.iter().map(|&v| Self::relu(v)).collect()
    }
}
