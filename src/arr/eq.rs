//! Equality check for arrays with differing or equivalent backends.

use crate::arr::Arr;
use crate::backend::Backend;
use std::cmp::Ordering;

impl<B1, B2, T, const D: &'static [usize]> PartialEq<Arr<B2, T, D>> for Arr<B1, T, D>
where
    B1: Backend<T>,
    B2: Backend<T>,
    T: PartialEq,
{
    fn eq(&self, other: &Arr<B2, T, D>) -> bool {
        self.storage().as_vec() == other.storage().as_vec()
    }
}

impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T>,
    T: PartialOrd,
{
    /// Compare two arrays elementwise. This should be used when all three `Orderings` are used, not
    /// to do simple comparisons e.g. `x < y`. If doing basic comparisons, use the other associated
    /// methods, e.g. [`Arr::lt`]
    #[must_use]
    pub fn partial_cmp<B2: Backend<Option<Ordering>>>(
        &self,
        other: &Self,
    ) -> Arr<B2, Option<Ordering>, D> {
        self.iter()
            .zip(other)
            .map(|(x, y)| x.partial_cmp(y))
            .collect()
    }

    /// Checks if the first element of `self` is less than the first element of `other`, the second
    /// element of `self` is less than the second element of `other`, etc.
    ///
    /// Stored in a bool array with the same dimensions as the input.
    #[must_use]
    pub fn lt<B2: Backend<bool>>(&self, other: &Self) -> Arr<B2, bool, D> {
        self.iter().zip(other).map(|(x, y)| x < y).collect()
    }

    /// Checks if the first element of `self` is less than or equal to the first element of `other`, the second
    /// element of `self` is less than or equal to the second element of `other`, etc.
    ///
    /// Stored in a bool array with the same dimensions as the input.
    #[must_use]
    pub fn le<B2: Backend<bool>>(&self, other: &Self) -> Arr<B2, bool, D> {
        self.iter().zip(other).map(|(x, y)| x <= y).collect()
    }

    /// Checks if the first element of `self` is greater than the first element of `other`, the second
    /// element of `self` is greater than the second element of `other`, etc.
    ///
    /// Stored in a bool array with the same dimensions as the input.
    #[must_use]
    pub fn gt<B2: Backend<bool>>(&self, other: &Self) -> Arr<B2, bool, D> {
        self.iter().zip(other).map(|(x, y)| x > y).collect()
    }

    /// Checks if the first element of `self` is greater than or equal to the first element of `other`, the second
    /// element of `self` is greater than or equal to the second element of `other`, etc.
    ///
    /// Stored in a bool array with the same dimensions as the input.
    #[must_use]
    pub fn ge<B2: Backend<bool>>(&self, other: &Self) -> Arr<B2, bool, D> {
        self.iter().zip(other).map(|(x, y)| x <= y).collect()
    }

    /// Checks if the first element of `self` is equal to the first element of `other`, the second
    /// element of `self` is equal to the second element of `other`, etc.
    ///
    /// Stored in a bool array with the same dimensions as the input.
    #[must_use]
    pub fn eq<B2: Backend<bool>>(&self, other: &Self) -> Arr<B2, bool, D> {
        self.iter().zip(other).map(|(x, y)| x == y).collect()
    }

    /// Checks if the first element of `self` is not equal to the first element of `other`, the second
    /// element of `self` is not equal to the second element of `other`, etc.
    ///
    /// Stored in a bool array with the same dimensions as the input.
    #[must_use]
    pub fn ne<B2: Backend<bool>>(&self, other: &Self) -> Arr<B2, bool, D> {
        self.iter().zip(other).map(|(x, y)| x != y).collect()
    }
}

impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T>,
    T: Ord,
{
    /// Total compare two arrays elementwise. See [`Arr::partial_cmp`] for when to use this over
    /// basic comparison operators.
    ///
    /// Use this instead of [`Arr::partial_cmp`] when `T: Ord`
    // `allow` because I am intentionally trying to look like the [`Ord`] method
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn cmp<B2: Backend<Ordering>>(&self, other: &Self) -> Arr<B2, Ordering, D> {
        self.iter().zip(other).map(|(x, y)| x.cmp(y)).collect()
    }

    /// Returns the elementwise maximum of two arrays.
    ///
    /// Note that this returns an array of references, as opposed to just `T`. If T has a trivial
    /// [`Clone`] implementation, consider using [`Self::max_cloned`] instead.
    #[must_use]
    pub fn max<'a, B2>(&'a self, other: &'a Self) -> Arr<B2, &'a T, D>
    where
        B2: Backend<&'a T>,
    {
        self.iter().zip(other).map(|(x, y)| x.max(y)).collect()
    }

    /// Equivalent to [`Self::max`], but clones each value after max'ing.
    #[must_use]
    pub fn max_cloned(&self, other: &Self) -> Self
    where
        T: Clone,
    {
        self.iter()
            .zip(other)
            .map(|(x, y)| x.max(y))
            .cloned()
            .collect()
    }

    /// Returns the elementwise minimum of two arrays.
    ///
    /// Note that this returns an array of references, as opposed to just `T`. If T has a trivial
    /// [`Clone`] implementation, consider using [`Self::min_cloned`] instead.
    #[must_use]
    pub fn min<'a, B2>(&'a self, other: &'a Self) -> Arr<B2, &'a T, D>
    where
        B2: Backend<&'a T>,
    {
        self.iter().zip(other).map(|(x, y)| x.min(y)).collect()
    }

    /// Equivalent to [`Self::min`], but clones each value after min'ing.
    #[must_use]
    pub fn min_cloned(&self, other: &Self) -> Self
    where
        T: Clone,
    {
        self.iter()
            .zip(other)
            .map(|(x, y)| x.min(y))
            .cloned()
            .collect()
    }

    /// Returns an array with each element clamped to the interval `[start, end]`.
    ///
    /// Note that this returns an array of references. If T has a trivial [`Clone`] implementation,
    /// consider using [`Self::clamp_cloned`] instead.
    #[must_use]
    pub fn clamp<'a, B2>(&'a self, start: &'a T, end: &'a T) -> Arr<B2, &'a T, D>
    where
        B2: Backend<&'a T>,
    {
        self.iter()
            .map(|x| {
                if x < start {
                    start
                } else if x > end {
                    end
                } else {
                    x
                }
            })
            .collect()
    }

    /// Equivalent to [`Self::clamp`], but clones each element after clamping.
    #[must_use]
    pub fn clamp_cloned(&self, start: T, end: T) -> Self
    where
        T: Clone,
    {
        self.iter()
            .map(|x| {
                if x < &start {
                    &start
                } else if x > &end {
                    &end
                } else {
                    x
                }
            })
            .cloned()
            .collect()
    }
}
