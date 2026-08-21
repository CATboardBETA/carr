use crate::backend::{Backend, BackendOps};
use crate::dimension::Dimensions;
use std::marker::PhantomData;

mod arr_ops;
mod conversions;
mod debug;

#[derive(PartialEq, Eq)]
pub struct Arr<B, T, const N_DIMS: usize>
where
    B: Backend<T>,
{
    storage: B,
    dims: Dimensions<N_DIMS>,
    _phantom: PhantomData<T>,
}

impl<B, T, const N_DIMS: usize> Clone for Arr<B, T, N_DIMS>
where
    B: Backend<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
            dims: self.dims,
            _phantom: PhantomData,
        }
    }
}

/// Functions expressible generically for ALL const arrays
impl<B, T, const N_DIMS: usize> Arr<B, T, N_DIMS>
where
    B: Backend<T>,
{
    #[must_use]
    pub fn new(dims: Dimensions<N_DIMS>) -> Self {
        Self {
            storage: B::new_bck(),
            dims,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub const fn shape(&self) -> [usize; N_DIMS] {
        self.dims.shape()
    }

    pub const fn storage(&self) -> &B {
        &self.storage
    }

    pub const fn storage_mut(&mut self) -> &mut B {
        &mut self.storage
    }
}

impl<B, T, const N_DIMS: usize> Arr<B, T, N_DIMS>
where
    B: Backend<T> + BackendOps<T>,
{
    #[must_use]
    fn apply_ops<B2, F, const NEW_DIMS: usize>(
        self,
        other: Arr<B2, T, N_DIMS>,
        new_dims: Dimensions<NEW_DIMS>,
        f: F,
    ) -> Arr<B, T, NEW_DIMS>
    where
        B2: Backend<T> + BackendOps<T> + IntoIterator<Item = T>,
        F: FnMut((T, T)) -> T,
    {
        Arr {
            storage: self.storage.apply_ops(other.storage, f),
            dims: new_dims,
            _phantom: PhantomData,
        }
    }
}
