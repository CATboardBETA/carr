//! Integration for the [`ndarray` crate](https://lib.rs/crates/ndarray).

use crate::backend::{Backend, InMemBackend};
use crate::integrations::IntegrationError;
use crate::prelude::Arr;
use ndarray::{Array, ArrayBase, Data, Dimension, IxDyn};
use std::marker::PhantomData;

impl<S, B, T, NdD, const D: &'static [usize]> TryFrom<ArrayBase<S, NdD>> for Arr<B, T, D>
where
    S: Data<Elem = T>,
    B: Backend<T> + InMemBackend<T>,
    T: Clone,
    NdD: Dimension,
{
    type Error = IntegrationError;

    fn try_from(value: ArrayBase<S, NdD>) -> Result<Self, Self::Error> {
        if value.shape() == D {
            Ok(Self {
                storage: B::from_vec(value.as_slice().unwrap().to_vec()).unwrap(),
                _phantom: PhantomData,
            })
        } else {
            Err(IntegrationError::DimMismatch)
        }
    }
}

impl<B, T, const D: &'static [usize]> TryFrom<Arr<B, T, D>> for Array<T, IxDyn>
where
    B: InMemBackend<T>,
{
    type Error = IntegrationError;

    fn try_from(value: Arr<B, T, D>) -> Result<Self, Self::Error> {
        // Unfortunately, it's not possible to check dimension compatibility at compile time like
        // it is with `image`. This is in part because `ndarray` only has statically-dimensioned
        // arrays up to 6 dimensions, whereas an `Arr` can go to effectively infinite dimensions.
        Self::from_shape_vec(D, value.storage.into_vec()).map_err(|_| IntegrationError::DimMismatch)
    }
}

#[cfg(test)]
mod test {
    use crate::arr::Arr;
    use crate::integrations::IntegrationError;
    use crate::ix;
    use ndarray::Array;

    #[test]
    fn dim_mismatch() {
        let ndarray = Array::<i32, _>::ones((2, 1, 3));
        let expected = Err::<Arr<Vec<i32>, i32, { &[2, 2, 3] }>, _>(IntegrationError::DimMismatch);
        assert_eq!(ndarray.try_into(), expected);
    }

    #[test]
    fn okay_1() {
        let ndarray = Array::<i32, _>::ones((2, 1, 3));
        let expected = Arr::<Vec<i32>, i32, { &[2, 1, 3] }>::new_ones();
        assert_eq!(ndarray.try_into(), Ok(expected));
    }

    #[test]
    fn okay_2() {
        let mut ndarray = Array::<i32, _>::ones((2, 4, 3));
        *ndarray.get_mut([1, 2, 1]).unwrap() = 27;
        let mut expected = Arr::<Vec<i32>, i32, { &[2, 4, 3] }>::new_ones();
        *ix!(&mut expected => mut [1, 2, 1]).item() = 27;
        assert_eq!(ndarray.try_into(), Ok(expected));
    }

    #[test]
    fn from_arr_1() {
        let mut a = Arr::<Vec<u16>, u16, { &[8, 8, 3] }>::new_zeroed();
        *ix!(&mut a => mut [0, 1, 2]).item() = 38;
        // `into_dyn` because it is the only Array dimensionality convertible *from* an `Arr`
        let mut expected = Array::<u16, _>::zeros((8, 8, 3)).into_dyn();
        *expected.get_mut([0, 1, 2]).unwrap() = 38;

        assert_eq!(a.try_into(), Ok(expected));
    }
}
