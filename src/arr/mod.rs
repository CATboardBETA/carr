//! Primary module for this crate. Holds all structs and impls for the [`Arr`] type.

use crate::backend::{Backend, BackendOps, NumericalValue};
use crate::dim::TOTAL_DIM;
#[cfg(feature = "rand")]
use rand::RngExt;
#[cfg(feature = "rand")]
use rand::distr::StandardUniform;
#[cfg(feature = "rand")]
use rand::distr::Uniform;
#[cfg(feature = "rand")]
use rand::distr::uniform::SampleUniform;
use std::marker::PhantomData;

mod arr_ops;
mod conversions;
#[coverage(off)]
mod debug;
mod eq;
pub mod index;

/// The primary struct in `carr`.
///
/// # Generics:
/// `B`: [`Backend`] to use for the internal storage of this array.\
/// `T`: Type that the array is storing.\
/// `const DIMS`:  Dimensions of the array, as a slice. Each element represents a dimension's length
///
/// # Examples:
/// Create a zeroed, 1-D array:
/// ```
/// # use carr::arr::Arr;
/// let arr: Arr<Vec<i32>, i32, { &[10] }> = Arr::new_zeroed();
/// println!("{arr}")
/// ```
///
/// Create a multidimensional array of any type implementing [`Default`]
/// ```
/// # #![feature(min_adt_const_params, adt_const_params, min_generic_const_args, generic_const_args, unsized_const_params, generic_const_items)]
/// # #![expect(incomplete_features)]
/// # use carr::arr::Arr;
/// fn new_2d<T: Default, const D: &'static [usize]>() -> Arr<Vec<T>, T, D> {
///     Arr::new()
/// }
/// ```
pub struct Arr<B, T, const DIMS: &'static [usize]>
where
    B: Backend<T>,
{
    /// Internal storage of the [`Arr`], must implement [`Backend<T>`] and usually [`BackendOps<T>`].
    storage: B,
    /// Phantom, since `T` isn't used in the type for `storage`
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
    T: Default,
    B: Backend<T>,
{
    /// Create a new [`Arr`], with dimensions specified by `D`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            storage: B::new_bck::<{ TOTAL_DIM::<D> }>(),
            _phantom: PhantomData,
        }
    }

    /// Returns an immutable reference to the internal storage.
    pub const fn storage(&self) -> &B {
        &self.storage
    }

    /// Returns a mutable reference to the internal storage.
    pub const fn storage_mut(&mut self) -> &mut B {
        &mut self.storage
    }
}

impl<B, T, const D: &'static [usize]> Default for Arr<B, T, D>
where
    T: Default,
    B: Backend<T>,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Functions expressible generically for all arrays that implement [`BackendOps<T>`] as well as
/// [`Backend<T>`].
impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T> + BackendOps<T>,
{
    /// Applies a function between two [`Arr`]s of the same or different [`Backend`].
    ///
    /// Currently, this always returns an [`Arr`] with [`Vec`] as the [`Backend`], but this may
    /// be changed in the future without a semver update. Do not rely on this feature.
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

impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T>,
    T: NumericalValue,
{
    /// Generates a new array, with each value being zero.
    #[must_use]
    pub fn new_zeroed() -> Self {
        Self::from(B::from_vec(Vec::from_fn(TOTAL_DIM::<D>, |_| T::ZERO)).unwrap())
    }

    /// Generates a new array, with all values being equal to one.
    #[must_use]
    pub fn new_ones() -> Self {
        Self::from(B::from_vec(Vec::from_fn(TOTAL_DIM::<D>, |_| T::ONE)).unwrap())
    }

    /// Generates a new array, with random values. The min/max of these values is defined by
    /// [`StandardUniform`].
    ///
    /// This function requires the `rand` feature to be enabled.
    #[cfg(feature = "rand")]
    #[must_use]
    pub fn new_random() -> Self
    where
        T: SampleUniform,
        StandardUniform: rand::distr::Distribution<T>,
    {
        let mut rng = rand::rng();
        let distr = StandardUniform;
        let a = Vec::from_fn(TOTAL_DIM::<D>, |_| rng.sample(distr));
        Self::from(B::from_vec(a).unwrap())
    }

    /// Generates a new array with random values, given an RNG implementation. The min/max of these
    /// values is defined by [`StandardUniform`].
    ///
    ///
    /// This function requires the `rand` feature to be enabled.
    #[cfg(feature = "rand")]
    #[must_use]
    pub fn new_random_rng<R: rand::Rng + ?Sized>(rng: &mut R) -> Self
    where
        T: SampleUniform,
        StandardUniform: rand::distr::Distribution<T>,
    {
        let distr = StandardUniform;
        let a = Vec::from_fn(TOTAL_DIM::<D>, |_| rng.sample(distr));
        Self::from(B::from_vec(a).unwrap())
    }

    /// Generates an array with random values, each within the range `start` (inclusive) to `end`
    /// (exclusive). The distribution is uniform.
    ///
    /// This function requires the `rand` feature to be enabled.
    #[cfg(feature = "rand")]
    #[must_use]
    pub fn new_random_range(start: T, end: T) -> Self
    where
        T: SampleUniform,
    {
        let mut rng = rand::rng();
        let distr = &Uniform::new(start, end).unwrap();
        let a = Vec::from_fn(TOTAL_DIM::<D>, |_| rng.sample(distr));
        Self::from(B::from_vec(a).unwrap())
    }

    /// Generates an array with random values, each within the range `start` (inclusive) to `end`
    /// (exclusive). The distribution is uniform, using a provided RNG.
    ///
    /// This function requires the `rand` feature to be enabled.
    #[cfg(feature = "rand")]
    #[must_use]
    pub fn new_random_range_rng<R: rand::Rng + ?Sized>(rng: &mut R, start: T, end: T) -> Self
    where
        T: SampleUniform,
    {
        let distr = &Uniform::new(start, end).unwrap();
        let a = Vec::from_fn(TOTAL_DIM::<D>, |_| rng.sample(distr));
        Self::from(B::from_vec(a).unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::arr::Arr;
    use std::array;

    #[test]
    fn new() {
        let arr = Arr::<Vec<_>, (), { &[1, 2, 4] }>::new();
        assert_eq!(arr.storage().len(), 8);
    }

    #[test]
    fn zeroed() {
        let arr = Arr::<Vec<_>, f32, { &[1, 2] }>::new_zeroed();
        let expected = Arr::<_, f32, { &[1, 2] }>::from([0., 0.]);
        assert_eq!(arr, expected);
    }
    #[test]
    fn ones() {
        let arr = Arr::<Vec<_>, f32, { &[5, 2, 1, 2] }>::new_ones();
        let expected =
            Arr::<_, f32, { &[5, 2, 1, 2] }>::from(array::repeat::<_, { 5 * 2 * 2 }>(1.));
        assert_eq!(arr, expected);
    }
}
