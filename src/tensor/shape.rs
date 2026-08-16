/// Shape metadata for tensors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shape {
    pub dims: Vec<usize>,
}

impl Shape {
    pub fn new(dims: Vec<usize>) -> Self { Self { dims } }
}
