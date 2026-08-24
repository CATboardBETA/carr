use crate::arr::Arr;
use crate::backend::Backend;
use std::marker::PhantomData;

impl<T, const N: usize, const D: &'static [usize]> From<[T; N]> for Arr<[T; N], T, D>
where
    T: Default,
{
    fn from(storage: [T; N]) -> Self {
        Self {
            storage,
            _phantom: PhantomData,
        }
    }
}

impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T>,
{
    pub const fn from(storage: B) -> Self {
        Self {
            storage,
            _phantom: PhantomData,
        }
    }
}
