//! Basic [`From`]/[`Into`] conversions from common types into an array.

use crate::arr::Arr;
use crate::backend::Backend;
use std::marker::PhantomData;

impl<T, const N: usize, const D: &'static [usize]> From<[T; N]> for Arr<[T; N], T, D>
where
    T: Clone + Default,
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
    /// Convert from a [`Backend`]-implementor directly into an array, without copying.
    pub const fn from(storage: B) -> Self {
        Self {
            storage,
            _phantom: PhantomData,
        }
    }
}
