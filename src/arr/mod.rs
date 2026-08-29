use crate::backend::{Backend, BackendOps};
use std::marker::PhantomData;

mod arr_ops;
mod conversions;
mod debug;
mod eq;
mod index;

#[derive(Eq)]
pub struct Arr<B, T, const DIMS: &'static [usize]>
where
    B: Backend<T>,
{
    storage: B,
    _phantom: PhantomData<T>,
}

impl<B, T, const D: &'static [usize]> Clone for Arr<B, T, D>
where
    B: Backend<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
            _phantom: PhantomData,
        }
    }
}

/// Functions expressible generically for ALL const arrays
impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T>,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            storage: B::new_bck(),
            _phantom: PhantomData,
        }
    }

    pub const fn storage(&self) -> &B {
        &self.storage
    }

    pub const fn storage_mut(&mut self) -> &mut B {
        &mut self.storage
    }
}

impl<B, T, const D: &'static [usize]> Default for Arr<B, T, D>
where
    B: Backend<T>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T> + BackendOps<T>,
{
    #[must_use]
    fn apply_ops<B2, B3, F, const D2: &'static [usize]>(
        self,
        other: Arr<B2, T, D2>,
        f: F,
    ) -> Arr<B3, T, D>
    where
        B2: Backend<T> + BackendOps<T>,
        B3: Backend<T> + BackendOps<T>,
        F: FnMut((T, T)) -> T,
    {
        Arr {
            storage: self.storage.apply_ops(other.storage, f),
            _phantom: PhantomData,
        }
    }
}
