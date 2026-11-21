//! Indexing operations. Includes a custom trait for indexing with constant dimension,

use crate::arr::Arr;
use crate::backend::Backend;
use crate::dim::INDEX_ARR;
use std::marker::PhantomData;

/// Index/slice an [`Arr`]. Equivalent to [`std::ops::Index`], except uses const generics instead of
/// a parameter to specify where to index/slice.
pub trait IndexConstSlice<const AT: &'static [usize]> {
    /// Output type from indexing
    type Output;
    /// Checked indexing operation. Panics on out-of-bounds.
    fn index(&self) -> Self::Output;
    /// Checked indexing operation. Returns [`None`] on out-of-bounds
    fn try_index(&self) -> Option<Self::Output>;
}

/// Index/slice an [`Arr`] mutably. Equivalent to [`std::ops::IndexMut`], except uses const generics
/// instead of  a parameter to specify where to index/slice.
pub trait IndexConstSliceMut<'a, const AT: &'static [usize]> {
    /// Output type from indexing
    type Output;
    /// Checked mutable indexing operation. Panics on out-of-bounds.
    fn index_mut(self) -> Self::Output;
    /// Checked mutable indexing operation. Returns [`None`] on out-of-bounds
    fn try_index_mut(self) -> Option<Self::Output>;
}

impl<'a, B, T, const D: &'static [usize], const AT: &'static [usize]> IndexConstSlice<AT>
    for &'a Arr<B, T, D>
where
    B: Backend<T>,
{
    type Output = Arr<Vec<&'a T>, &'a T, { INDEX_ARR::<D, AT> }>;

    fn index(&self) -> Self::Output {
        const {
            assert!(D.len() >= AT.len());
        }
        let index = AT
            .iter()
            .enumerate()
            .map(|(i, x)| D[(i + 1)..].iter().product::<usize>() * x)
            .sum::<usize>();
        let slice_len = D[AT.len()..].iter().product::<usize>();
        let out_storage: &[T] = &self.storage()[index..(index + slice_len)];
        Arr {
            storage: out_storage.iter().collect(),
            _phantom: PhantomData,
        }
    }

    fn try_index(&self) -> Option<Self::Output> {
        if D.len() < AT.len() {
            None
        } else {
            Some(IndexConstSlice::<AT>::index(self))
        }
    }
}

impl<'a, B, T, const D: &'static [usize], const AT: &'static [usize]> IndexConstSliceMut<'a, AT>
    for &'a mut Arr<B, T, D>
where
    B: Backend<T>,
{
    type Output = Arr<Vec<&'a mut T>, &'a mut T, { INDEX_ARR::<D, AT> }>;

    fn index_mut(self) -> Self::Output {
        const {
            assert!(D.len() >= AT.len());
        }
        let index = AT
            .iter()
            .enumerate()
            .map(|(i, x)| D[(i + 1)..].iter().product::<usize>() * x)
            .sum::<usize>();
        let slice_len = D[AT.len()..].iter().product::<usize>();
        let out_storage: &'a mut [T] = &mut self.storage_mut()[index..(index + slice_len)];
        Arr {
            storage: out_storage.iter_mut().collect(),
            _phantom: PhantomData,
        }
    }

    fn try_index_mut(self) -> Option<Self::Output> {
        if D.len() < AT.len() {
            None
        } else {
            Some(IndexConstSliceMut::<AT>::index_mut(self))
        }
    }
}
#[macro_export]
/// Shorthand for indexing an array.
///
/// The expression `ix!(a => [b, c])` is equivalent to
/// [`IndexConstSlice::<{ &[b, c] }>::index(a)`](IndexConstSlice::index)
macro_rules! ix {
    ($x:expr => [$($indices:expr),*]) => {
        {
            $crate::arr::index::IndexConstSlice::<{ &[$($indices),*] }>::index(&$x)
        }
    };
    ($x:expr => $indices:expr) => {
        {
            $crate::arr::index::IndexConstSlice::<$indices>::index(&$x)
        }
    };
    ($x:expr => mut [$($indices:expr),*]) => {
        {
            $crate::arr::index::IndexConstSliceMut::<{ &[$($indices),*] }>::index_mut($x)
        }
    };
    ($x:expr => mut $indices:expr) => {
        {
            $crate::arr::index::IndexConstSliceMut::<$indices>::index_mut($x)
        }
    };
}

#[macro_export]
/// Like the [`ix`] macro, the expression
/// `try_ix!(a =>  [b, c])` is equivalent to
/// [`IndexConstSlice::<{ &[b, c] }>::try_index(a)`](IndexConstSlice::try_index).
macro_rules! try_ix {
    ($x:expr =>  [$($indices:expr),*]) => {
        {
            $crate::arr::index::IndexConstSlice::<{ &[$($indices),*] }>::try_index(&$x)
        }
    };

    ($x:expr =>  $indices:expr) => {
        {
            $crate::arr::index::IndexConstSlice::<{ $indices }>::try_index(&$x)
        }
    };

    ($x:expr => mut [$($indices:expr),*]) => {
        {
            $crate::arr::index::IndexConstSliceMut::<{ &[$($indices),*] }>::try_index_mut(&mut $x)
        }
    };

    ($x:expr => mut $indices:expr) => {
        {
            $crate::arr::index::IndexConstSliceMut::<{ $indices }>::try_index_mut(&mut $x)
        }
    };
}

impl<B, T, const D: &'static [usize], const AT: &'static [usize]> IndexConstSlice<AT>
    for Arr<B, T, D>
where
    B: Backend<T>,
    T: Clone,
{
    type Output = Arr<Vec<T>, T, { INDEX_ARR::<D, AT> }>;

    fn index(&self) -> Self::Output {
        ix!(self => AT).into_iter().cloned().collect()
    }

    fn try_index(&self) -> Option<Self::Output> {
        try_ix!(self => AT).map(|x| x.into_iter().cloned().collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::arr::Arr;

    #[test]
    fn index_1() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Arr::<_, _, { &[3] }>::from([3, 2, 1]);
        let x = IndexConstSlice::<{ &[0] }>::index(&arr1);
        assert_eq!(x, expected);
    }

    #[test]
    fn index_2() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Arr::<_, _, { &[] }>::from([1]);
        let x = IndexConstSlice::<{ &[0, 2] }>::index(&arr1);
        assert_eq!(x, expected);
    }

    #[test]
    fn try_index_1() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Some(Arr::<_, _, { &[3] }>::from([3, 2, -2]));
        let x = IndexConstSlice::<{ &[1] }>::try_index(&arr1);
        assert!(x.zip(expected).is_some_and(|(x, y)| x == y));
    }

    #[test]
    fn try_index_2() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Some(Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]));
        let x = IndexConstSlice::<{ &[] }>::try_index(&arr1);
        assert!(x.zip(expected).is_some_and(|(x, y)| x == y));
    }

    #[test]
    fn index_macro_1() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Arr::<_, _, { &[3] }>::from([3, 2, 1]);
        let x = crate::ix!(arr1 => [0]);
        assert_eq!(x, expected);
    }

    #[test]
    fn index_macro_2() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let x = crate::ix!(arr1 => []);
        assert_eq!(x, expected);
    }

    #[test]
    fn index_macro_3() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Some(Arr::<_, _, { &[3] }>::from([3, 2, 1]));
        let x = try_ix!(arr1 =>  [0]);
        assert!(x.zip(expected).is_some_and(|(x, y)| x == y));
    }

    #[test]
    fn index_macro_4() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Some(Arr::<_, _, { &[] }>::from([-2]));
        let x = try_ix!(arr1 =>  [1, 2]);
        assert!(x.zip(expected).is_some_and(|(x, y)| x == y));
    }
}
