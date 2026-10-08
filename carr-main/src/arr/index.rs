//! Indexing operations. Includes a custom trait for indexing with constant dimension,

use crate::arr::Arr;
use crate::backend::Backend;
use crate::dim::INDEX_ARR;
use itertools::Itertools;
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

impl<B, T, const D: &'static [usize], const AT: &'static [usize]> IndexConstSlice<AT>
    for Arr<B, T, D>
where
    B: Backend<T>,
    T: Default + Clone,
{
    type Output = Arr<Vec<T>, T, { INDEX_ARR::<D, AT> }>;

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
        let out_storage = out_storage.iter().cloned().collect_vec();
        Arr {
            storage: out_storage,
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
}
