/// Detected SIMD capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimdLevel {
    Scalar,
    Sse2,
    Avx2,
    Neon,
}

/// Detects the runtime CPU feature set and selects the safest available kernel family.
pub fn detect_simd() -> SimdLevel {
    if cfg!(target_arch = "aarch64") { SimdLevel::Neon } else { SimdLevel::Sse2 }
}
