use crate::tensor::dtype::DType;
use crate::tensor::shape::Shape;

/// Tensor storage abstraction used by runtime kernels.
#[derive(Clone, Debug, PartialEq)]
pub struct Tensor {
    pub data: Vec<f32>,
    pub shape: Shape,
    pub dtype: DType,
}

impl Tensor {
    pub fn new(data: Vec<f32>, shape: Shape, dtype: DType) -> Self {
        Self { data, shape, dtype }
    }
}
