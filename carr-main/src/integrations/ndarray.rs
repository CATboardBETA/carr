//! Integration for the [`ndarray` crate](https://lib.rs/crates/ndarray).

use crate::backend::Backend;
use crate::integrations::IntegrationError;
use crate::prelude::Arr;
use ndarray::{ArrayBase, Data, Dimension};
use std::marker::PhantomData;

impl<S, B, T, NdD, const D: &'static [usize]> TryFrom<ArrayBase<S, NdD>> for Arr<B, T, D>
where
    S: Data<Elem = T>,
    B: Backend<T>,
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

#[cfg(test)]
mod test {
    use ndarray::Array;
    use crate::arr::Arr;
    use crate::integrations::IntegrationError;
    use crate::ix;

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
}