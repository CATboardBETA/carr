#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Dimensions<const N_DIMS: usize> {
    shape: [usize; N_DIMS],
}

impl<const N_DIMS: usize> Dimensions<N_DIMS> {
    #[must_use]
    pub(crate) const fn new(shape: [usize; N_DIMS]) -> Self {
        Self { shape }
    }

    #[must_use]
    pub(crate) const fn shape(&self) -> [usize; N_DIMS] {
        self.shape
    }
}
