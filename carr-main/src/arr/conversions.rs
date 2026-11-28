//! Basic [`From`]/[`Into`] conversions from common types into an array.

use crate::arr::Arr;
use crate::backend::{Backend, InMemBackend};
use crate::dim::{TOTAL_DIM, UNIT_ARR};
use crate::prop::{Prop, PropTrue};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

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
    pub fn from(storage: B) -> Self {
        // >=, not ==, because backends that aren't in memory (e.g. functional backends) have an
        // infinite size (usize::MAX, to be precise)
        assert!(storage.length() >= TOTAL_DIM::<D>);
        Self {
            storage,
            _phantom: PhantomData,
        }
    }
}

impl<B, T, const D: &'static [usize]> Deref for Arr<B, T, D>
where
    B: Backend<T>,
    Prop<{ UNIT_ARR::<D> }>: PropTrue,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.item_ref()
    }
}

impl<B, T, const D: &'static [usize]> DerefMut for Arr<B, T, D>
where
    B: Backend<T> + InMemBackend<T>,
    Prop<{ UNIT_ARR::<D> }>: PropTrue,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.item_mut()
    }
}
