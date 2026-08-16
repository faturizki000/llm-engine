/// Softmax kernel placeholder.
#[derive(Clone, Debug, Default)]
pub struct SoftmaxKernel;

impl SoftmaxKernel {
    pub fn softmax(x: &[f32]) -> Vec<f32> {
        if x.is_empty() {
            return vec![];
        }
        let max_val = x.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let exps: Vec<f32> = x.iter().map(|v| (v - max_val).exp()).collect();
        let sum: f32 = exps.iter().sum();
        exps.iter().map(|v| v / sum).collect()
    }

    pub fn softmax_in_place(x: &mut [f32]) {
        if x.is_empty() {
            return;
        }
        let max_val = x.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        for v in x.iter_mut() {
            *v = (*v - max_val).exp();
        }
        let sum: f32 = x.iter().sum();
        for v in x.iter_mut() {
            *v /= sum;
        }
    }
}
