/// Quantized kernel placeholder.
#[derive(Clone, Debug, Default)]
pub struct QuantizedKernel;

impl QuantizedKernel {
    pub fn quantize_int8(values: &[f32], scale: f32) -> Vec<i8> {
        values
            .iter()
            .map(|&v| ((v / scale).clamp(-128.0, 127.0)) as i8)
            .collect()
    }

    pub fn dequantize_int8(values: &[i8], scale: f32) -> Vec<f32> {
        values.iter().map(|&v| (v as f32) * scale).collect()
    }

    pub fn compute_scale(values: &[f32]) -> f32 {
        let max_abs = values.iter().map(|v| v.abs()).fold(0.0, f32::max);
        if max_abs > 0.0 {
            max_abs / 127.0
        } else {
            1.0
        }
    }
}
