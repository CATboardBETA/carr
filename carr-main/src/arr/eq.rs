//! Equality check for arrays with differing or equivalent backends.

use crate::arr::Arr;
use crate::backend::{Backend, InMemBackend};
use std::cmp::Ordering;

impl<B1, B2, T1, T2, const D: &'static [usize]> PartialEq<Arr<B2, T2, D>> for Arr<B1, T1, D>
where
    B1: Backend<T1>,
    B2: Backend<T2>,
    T1: PartialEq<T2>,
{
    fn eq(&self, other: &Arr<B2, T2, D>) -> bool {
        self.iter().zip(other).all(|(x, y)| x == y)
    }
}

impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T>,
{
    /// Compare two arrays elementwise. This should be used when all three `Orderings` are used, not
    /// to do simple comparisons e.g. `x < y`. If doing basic comparisons, use the other associated
    /// methods, e.g. [`Arr::lt`]
    #[must_use]
    pub fn partial_cmp<B2, T2>(
        &self,
        other: &Arr<B2, T2, D>,
    ) -> Arr<Vec<Option<Ordering>>, Option<Ordering>, D>
    where
        B2: Backend<T2>,
        T: PartialOrd<T2>,
    {
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
    pub fn lt<B2, T2>(&self, other: &Arr<B2, T2, D>) -> Arr<Vec<bool>, bool, D>
    where
        B2: Backend<T2>,
        T: PartialOrd<T2>,
    {
        self.iter().zip(other).map(|(x, y)| x < y).collect()
    }

    /// Checks if the first element of `self` is less than or equal to the first element of `other`, the second
    /// element of `self` is less than or equal to the second element of `other`, etc.
    ///
    /// Stored in a bool array with the same dimensions as the input.
    #[must_use]
    pub fn le<B2, T2>(&self, other: &Arr<B2, T2, D>) -> Arr<Vec<bool>, bool, D>
    where
        B2: Backend<T2>,
        T: PartialOrd<T2>,
    {
        self.iter().zip(other).map(|(x, y)| x <= y).collect()
    }

    /// Checks if the first element of `self` is greater than the first element of `other`, the second
    /// element of `self` is greater than the second element of `other`, etc.
    ///
    /// Stored in a bool array with the same dimensions as the input.
    #[must_use]
    pub fn gt<B2, T2>(&self, other: &Arr<B2, T2, D>) -> Arr<Vec<bool>, bool, D>
    where
        B2: Backend<T2>,
        T: PartialOrd<T2>,
    {
        self.iter().zip(other).map(|(x, y)| x > y).collect()
    }

    /// Checks if the first element of `self` is greater than or equal to the first element of `other`, the second
    /// element of `self` is greater than or equal to the second element of `other`, etc.
    ///
    /// Stored in a bool array with the same dimensions as the input.
    #[must_use]
    pub fn ge<B2, T2>(&self, other: &Arr<B2, T2, D>) -> Arr<Vec<bool>, bool, D>
    where
        B2: Backend<T2>,
        T: PartialOrd<T2>,
    {
        self.iter().zip(other).map(|(x, y)| x >= y).collect()
    }

    /// Checks if the first element of `self` is equal to the first element of `other`, the second
    /// element of `self` is equal to the second element of `other`, etc.
    ///
    /// Stored in a bool array with the same dimensions as the input.
    #[must_use]
    pub fn eq<B2, T2>(&self, other: &Arr<B2, T2, D>) -> Arr<Vec<bool>, bool, D>
    where
        B2: Backend<T2>,
        T: PartialOrd<T2>,
    {
        self.iter().zip(other).map(|(x, y)| x == y).collect()
    }

    /// Checks if the first element of `self` is not equal to the first element of `other`, the second
    /// element of `self` is not equal to the second element of `other`, etc.
    ///
    /// Stored in a bool array with the same dimensions as the input.
    #[must_use]
    pub fn ne<B2, T2>(&self, other: &Arr<B2, T2, D>) -> Arr<Vec<bool>, bool, D>
    where
        B2: Backend<T2>,
        T: PartialOrd<T2>,
    {
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
    pub fn cmp(&self, other: &Self) -> Arr<Vec<Ordering>, Ordering, D> {
        self.iter().zip(other).map(|(x, y)| x.cmp(y)).collect()
    }

    /// Returns the elementwise maximum of two arrays.
    ///
    /// Note that this returns an array of references, as opposed to just `T`. If T has a trivial
    /// [`Clone`] implementation, consider using [`Self::max_cloned`] instead.
    #[must_use]
    pub fn max<'a>(&'a self, other: &'a Self) -> Arr<Vec<&'a T>, &'a T, D> {
        self.iter().zip(other).map(|(x, y)| x.max(y)).collect()
    }

    /// Equivalent to [`Self::max`], but clones each value after max'ing.
    #[must_use]
    pub fn max_cloned(&self, other: &Self) -> Self
    where
        T: Clone,
        B: InMemBackend<T>,
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
    pub fn min<'a>(&'a self, other: &'a Self) -> Arr<Vec<&'a T>, &'a T, D> {
        self.iter().zip(other).map(|(x, y)| x.min(y)).collect()
    }

    /// Equivalent to [`Self::min`], but clones each value after min'ing.
    #[must_use]
    pub fn min_cloned(&self, other: &Self) -> Self
    where
        T: Clone,
        B: InMemBackend<T>,
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
    pub fn clamp<'a>(&'a self, start: &'a T, end: &'a T) -> Arr<Vec<&'a T>, &'a T, D> {
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
    pub fn clamp_cloned(&self, start: &T, end: &T) -> Self
    where
        T: Clone,
        B: InMemBackend<T>,
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
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::arr::Arr;
    use std::cmp::Ordering;

    #[test]
    fn partial_cmp_1() {
        let a = Arr::<_, _, { &[3] }>::from([1, 2, 3]);
        let b = Arr::<_, _, { &[3] }>::from([3, 2, 1]);
        let expect = Arr::<_, _, { &[3] }>::from([
            Some(Ordering::Less),
            Some(Ordering::Equal),
            Some(Ordering::Greater),
        ]);
        // Cannot use assert_eq here due to `Ordering` not implementing `Display`
        assert!(a.partial_cmp(&b) == expect);
    }
    #[test]
    fn partial_cmp_2() {
        let a = Arr::<_, _, { &[3] }>::from([f32::NAN, 3., 1.]);
        let b = Arr::<_, _, { &[3] }>::from([3., 2., f32::NAN]);
        let expect = Arr::<_, _, { &[3] }>::from([None, Some(Ordering::Greater), None]);
        assert!(a.partial_cmp(&b) == expect);
    }

    #[test]
    fn partial_cmp_ops_1() {
        let a = Arr::<_, _, { &[3] }>::from([1., 2., 3.]);
        let b = Arr::<_, _, { &[3] }>::from([3., 2., 1.]);
        let lt = Arr::<_, _, { &[3] }>::from([true, false, false]);
        let le = Arr::<_, _, { &[3] }>::from([true, true, false]);
        let eq = Arr::<_, _, { &[3] }>::from([false, true, false]);
        let gt = Arr::<_, _, { &[3] }>::from([false, false, true]);
        let ge = Arr::<_, _, { &[3] }>::from([false, true, true]);
        let ne = Arr::<_, _, { &[3] }>::from([true, false, true]);
        assert_eq!(a.lt(&b), lt);
        assert_eq!(a.le(&b), le);
        assert_eq!(a.eq(&b), eq);
        assert_eq!(a.gt(&b), gt);
        assert_eq!(a.ge(&b), ge);
        assert_eq!(a.ne(&b), ne);
    }

    #[test]
    fn partial_cmp_ops_2() {
        struct I(i32);
        impl PartialEq<i64> for I {
            fn eq(&self, other: &i64) -> bool {
                &i64::from(self.0) == other
            }
        }
        impl PartialOrd<i64> for I {
            fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
                i64::from(self.0).partial_cmp(other)
            }
        }

        let a = Arr::<_, _, { &[3] }>::from([I(1), I(2), I(3)]);
        let b = Arr::<_, _, { &[3] }>::from([3, 2, 1]);

        let lt = Arr::<_, _, { &[3] }>::from([true, false, false]);
        let le = Arr::<_, _, { &[3] }>::from([true, true, false]);
        let eq = Arr::<_, _, { &[3] }>::from([false, true, false]);
        let gt = Arr::<_, _, { &[3] }>::from([false, false, true]);
        let ge = Arr::<_, _, { &[3] }>::from([false, true, true]);
        let ne = Arr::<_, _, { &[3] }>::from([true, false, true]);
        assert_eq!(a.lt(&b), lt);
        assert_eq!(a.le(&b), le);
        assert_eq!(a.eq(&b), eq);
        assert_eq!(a.gt(&b), gt);
        assert_eq!(a.ge(&b), ge);
        assert_eq!(a.ne(&b), ne);
    }

    #[test]
    fn partial_eq() {
        struct I(i32);
        impl PartialEq<i64> for I {
            fn eq(&self, other: &i64) -> bool {
                &i64::from(self.0) == other
            }
        }
        let a = Arr::<_, I, { &[3] }>::from([I(1), I(2), I(3)]);
        let b = Arr::<_, i64, { &[3] }>::from([3, 2, 1]);
        assert!(a != b);
    }

    #[test]
    fn total_cmp() {
        let a = Arr::<_, _, { &[3] }>::from([1, 2, 3]);
        let b = Arr::<_, _, { &[3] }>::from([3, 2, 1]);
        assert!(
            a.partial_cmp(&b)
                .iter()
                .map(|x| x.unwrap())
                .collect::<Arr<Vec<_>, _, _>>()
                == a.cmp(&b)
        );
    }

    #[test]
    fn min_max_ref() {
        let a = Arr::<_, _, { &[3] }>::from([1, 2, 3]);
        let b = Arr::<_, _, { &[3] }>::from([3, 2, 1]);
        let min = Arr::<_, _, { &[3] }>::from([&1, &2, &1]);
        let max = Arr::<_, _, { &[3] }>::from([&3, &2, &3]);
        assert!(a.min(&b) == min);
        assert!(a.max(&b) == max);
    }

    #[test]
    fn min_max() {
        let a = Arr::<_, _, { &[3] }>::from([1, 2, 3]);
        let b = Arr::<_, _, { &[3] }>::from([3, 2, 1]);
        let min = Arr::<_, _, { &[3] }>::from([1, 2, 1]);
        let max = Arr::<_, _, { &[3] }>::from([3, 2, 3]);
        assert_eq!(a.min_cloned(&b), min);
        assert_eq!(a.max_cloned(&b), max);
    }

    #[test]
    fn clamp_ref() {
        let a = Arr::<_, _, { &[5] }>::from([1, 2, 3, 4, 5]);
        let expect = Arr::<_, _, { &[5] }>::from([&2, &2, &3, &4, &4]);
        assert!(a.clamp(&2, &4) == expect);
    }

    #[test]
    fn clamp() {
        let a = Arr::<_, _, { &[5] }>::from([1, 2, 3, 4, 5]);
        let expect = Arr::<_, _, { &[5] }>::from([2, 2, 3, 4, 4]);
        assert_eq!(a.clamp_cloned(&2, &4), expect);
    }
}
