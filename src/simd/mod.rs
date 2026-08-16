pub mod detector;
pub mod dispatcher;
pub mod fallback;
pub mod kernels;

pub use detector::{detect_simd, SimdLevel};
pub use dispatcher::{dot_product, rope_apply, rmsnorm, softmax};

/// Public SIMD facade with scalar fallback.
pub fn add(a: f32, b: f32) -> f32 {
    a + b
}
