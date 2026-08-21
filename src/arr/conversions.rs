use crate::arr::Arr;
use crate::backend::Backend;
use crate::dimension::Dimensions;
use std::marker::PhantomData;

impl<T, D, const N: usize> From<[T; N]> for Arr<[T; N], T, D, 1>
where
    T: Default,
    D: Dimensions<1>,
{
    fn from(storage: [T; N]) -> Self {
        Self {
            storage,
            _phantom: PhantomData,
        }
    }
}

impl<B, T, D, const N_DIMS: usize> Arr<B, T, D, N_DIMS>
where
    B: Backend<T>,
    D: Dimensions<N_DIMS>,
{
    pub const fn from(storage: B) -> Self {
        Self {
            storage,
            _phantom: PhantomData,
        }
    }
}
