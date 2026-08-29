use crate::arr::Arr;
use crate::backend::Backend;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

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

impl<B: Backend<T>, T, const D: &'static [usize]> Deref for Arr<B, T, D> {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        self.storage()
    }
}

impl<B: Backend<T>, T, const D: &'static [usize]> DerefMut for Arr<B, T, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.storage_mut()
    }
}
