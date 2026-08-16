/// Matrix multiply placeholder.
#[derive(Clone, Debug, Default)]
pub struct MatmulKernel;

impl MatmulKernel {
    /// Multiply (m, k) @ (k, n) -> (m, n)
    pub fn matmul(lhs: &[f32], m: usize, k: usize, rhs: &[f32], n: usize) -> Vec<f32> {
        let mut out = vec![0.0; m * n];
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for p in 0..k {
                    sum += lhs[i * k + p] * rhs[p * n + j];
                }
                out[i * n + j] = sum;
            }
        }
        out
    }

    /// Batched matrix multiply
    pub fn batched_matmul(lhs: &[f32], m: usize, k: usize, rhs: &[f32], n: usize, batch: usize) -> Vec<f32> {
        let mut out = vec![0.0; batch * m * n];
        for b in 0..batch {
            let lhs_off = b * m * k;
            let rhs_off = b * k * n;
            let out_off = b * m * n;
            for i in 0..m {
                for j in 0..n {
                    let mut sum = 0.0;
                    for p in 0..k {
                        sum += lhs[lhs_off + i * k + p] * rhs[rhs_off + p * n + j];
                    }
                    out[out_off + i * n + j] = sum;
                }
            }
        }
        out
    }
}
