/// Weight layout placeholder.
#[derive(Clone, Debug, Default)]
pub struct Weights {
    pub shape: Vec<usize>,
    pub element_count: usize,
}

impl Weights {
    pub fn new(shape: Vec<usize>) -> Self {
        let element_count = shape.iter().product::<usize>();
        Self { shape, element_count }
    }

    pub fn is_empty(&self) -> bool {
        self.element_count == 0
    }
}
