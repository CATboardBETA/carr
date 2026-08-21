use crate::backend::{Backend, BackendOps};
use crate::dimension::Dimensions;
use std::marker::PhantomData;

mod arr_ops;
mod conversions;
mod debug;

#[derive(PartialEq, Eq)]
pub struct Arr<B, T, D, const N_DIMS: usize>
where
    B: Backend<T>,
    D: Dimensions<N_DIMS>,
{
    storage: B,
    _phantom: PhantomData<(T, D)>,
}

impl<B, T, D, const N_DIMS: usize> Clone for Arr<B, T, D, N_DIMS>
where
    B: Backend<T> + Clone,
    D: Dimensions<N_DIMS>,
{
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
            _phantom: PhantomData,
        }
    }
}

/// Functions expressible generically for ALL const arrays
impl<B, T, D, const N_DIMS: usize> Arr<B, T, D, N_DIMS>
where
    B: Backend<T>,
    D: Dimensions<N_DIMS>,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            storage: B::new_bck(),
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub const fn shape(&self) -> [usize; N_DIMS] {
        D::SHAPE
    }

    pub const fn storage(&self) -> &B {
        &self.storage
    }

    pub const fn storage_mut(&mut self) -> &mut B {
        &mut self.storage
    }
}

impl<B, T, D, const N_DIMS: usize> Arr<B, T, D, N_DIMS>
where
    B: Backend<T> + BackendOps<T>,
    D: Dimensions<N_DIMS>,
{
    #[must_use]
    fn apply_ops<B2, D2, F, const NEW_DIMS: usize>(
        self,
        other: Arr<B2, T, D, N_DIMS>,
        f: F,
    ) -> Arr<B, T, D2, NEW_DIMS>
    where
        B2: Backend<T> + BackendOps<T> + IntoIterator<Item = T>,
        D2: Dimensions<NEW_DIMS>,
        F: FnMut((T, T)) -> T,
    {
        Arr {
            storage: self.storage.apply_ops(other.storage, f),
            _phantom: PhantomData,
        }
    }
}
