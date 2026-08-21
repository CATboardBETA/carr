use crate::arr::Arr;
use crate::backend::Backend;
use crate::dimension::Dimensions;
use std::marker::PhantomData;

impl<T: Default, const N: usize> From<[T; N]> for Arr<[T; N], T, 1> {
    fn from(storage: [T; N]) -> Self {
        Self {
            storage,
            dims: Dimensions::new([N]),
            _phantom: PhantomData,
        }
    }
}

impl<B: Backend<T>, T, const N_DIMS: usize> Arr<B, T, N_DIMS> {
    pub const fn from(storage: B, dims: Dimensions<N_DIMS>) -> Self {
        Self {
            storage,
            dims,
            _phantom: PhantomData,
        }
    }
}
