pub mod detector;
pub mod dispatcher;
pub mod fallback;
pub mod kernels;

pub use detector::{detect_simd, SimdLevel};
pub use dispatcher::{dot_product, rope_apply, rmsnorm, softmax};
pub use fallback::{scalar_dot, scalar_rmsnorm};
pub use kernels::{dot_kernel, matmul_kernel, normalize_kernel, rope_kernel, softmax_kernel};

/// Public SIMD facade with scalar fallback.
pub fn add(a: f32, b: f32) -> f32 {
    a + b
}
