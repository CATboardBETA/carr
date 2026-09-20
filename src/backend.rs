//! Backend storage, internal to [`arr::Arr`](crate::arr::Arr). Also contains a trait for numerical
//! types (e.g. i32, f64, u8, etc.)

use itertools::Itertools;
#[cfg(feature = "rand")]
use rand::RngExt;
#[cfg(feature = "rand")]
use rand::distr::uniform::SampleUniform;
use std::array;
use std::ops::{Index, IndexMut, Range};

/// A storage backend for arrays. This does not store the dimensions, and can be a variable-length
/// type (e.g. a [`Vec`]).
///
/// By default, this is implemented for [`Vec<T>`] and [`[T; N]`](array)
pub trait Backend<T>:
    Index<usize, Output = T>
    + IndexMut<usize, Output = T>
    + Index<Range<usize>, Output = [T]>
    + IndexMut<Range<usize>, Output = [T]>
    + Sized
{
    /// Creates a new, empty instance of the backend.
    #[must_use]
    fn new_bck<const N: usize>() -> Self;
    /// Create an instance of this backend from a [`Vec<T>`].
    #[must_use]
    fn from_vec(v: Vec<T>) -> Option<Self>;
    /// Get the 1-D length of this backend. This should be equivalent to the product of the
    /// dimensions slice of the outer array.
    #[must_use]
    fn length(&self) -> usize;
    /// Creates a [`Vec<T>`] from a backend, consuming the underlying storage.
    #[must_use]
    fn into_vec(self) -> Vec<T>;
    /// Creates a [`Vec<T>`] from a backend, without consuming the underlying storage.
    #[must_use]
    fn as_vec(&self) -> Vec<&T>;
}

/// Implemented for [backends](Backend) that can have operations applied to them. This is required
/// for most operations, but not all.
pub trait BackendOps<T>: Backend<T> + IntoIterator<Item = T> {
    /// Applies a function element-wise between two arrays, returning the result collected into
    /// a backend.
    #[must_use]
    fn apply_ops<
        B2: Backend<T> + BackendOps<T>,
        B3: Backend<T> + BackendOps<T>,
        F: FnMut((T, T)) -> T,
    >(
        self,
        other: B2,
        op: F,
    ) -> B3;
}

/// TODO: Remove bound on `T`. `MaybeUninit` perhaps?
impl<T: Default + Clone, const N: usize> Backend<T> for [T; N] {
    fn new_bck<const N2: usize>() -> [T; N] {
        array::from_fn::<_, N2, _>(|_| T::default())
            .as_array()
            .cloned()
            .unwrap()
    }

    fn from_vec(v: Vec<T>) -> Option<Self> {
        v.try_into().ok()
    }

    fn length(&self) -> usize {
        self.len()
    }

    fn into_vec(self) -> Vec<T> {
        self.into_iter().collect_vec()
    }

    fn as_vec(&self) -> Vec<&T> {
        self.iter().collect_vec()
    }
}

impl<T: Default + Clone, const N: usize> BackendOps<T> for [T; N] {
    fn apply_ops<B2, B3, F>(self, other: B2, op: F) -> B3
    where
        B2: Backend<T> + BackendOps<T>,
        B3: Backend<T> + BackendOps<T>,
        F: FnMut((T, T)) -> T,
    {
        B3::from_vec(self.into_iter().zip(other).map(op).collect::<Vec<_>>()).unwrap()
    }
}

impl<T: Default> Backend<T> for Vec<T> {
    fn new_bck<const N: usize>() -> Self {
        Self::from_fn(N, |_| T::default())
    }

    fn from_vec(v: Self) -> Option<Self> {
        Some(v)
    }

    fn length(&self) -> usize {
        self.len()
    }

    fn into_vec(self) -> Self {
        self
    }

    fn as_vec(&self) -> Vec<&T> {
        self.iter().collect_vec()
    }
}

impl<T: Default> BackendOps<T> for Vec<T> {
    fn apply_ops<B2, B3, F>(self, other: B2, op: F) -> B3
    where
        B2: Backend<T> + BackendOps<T>,
        B3: Backend<T> + BackendOps<T>,
        F: FnMut((T, T)) -> T,
    {
        B3::from_vec(self.into_iter().zip(other).map(op).collect::<Self>()).unwrap()
    }
}

/// Implemented for all numerical types (including char). This is an implementation detail of some
/// array creation functions, such as [`Arr::new_zeroed`](crate::arr::Arr::new_zeroed),
/// [`Arr::new_ones`](crate::arr::Arr::new_ones), [`Arr::new_random`](crate::arr::Arr::new_random),
/// etc.
pub trait NumericalValue: Sized {
    /// Number of bits in the representation of `Self`
    const BIT_WIDTH: usize;
    /// This type's representation of zero (0)
    const ZERO: Self;
    /// This type's representation of one (1).
    const ONE: Self;

    /// Creates a new instance of this type, given an RNG and a distribution. Used by
    /// [`Arr::new_random`](crate::arr::Arr::new_random) and friends.
    #[cfg(feature = "rand")]
    fn new_random<R: rand::Rng + ?Sized, D: rand::distr::Distribution<Self>>(
        rng: &mut R,
        distr: D,
    ) -> Self
    where
        Self: SampleUniform;
}

macro_rules! numerical_value {
    ($(for $ty:ty; $bitwidth:expr;)*) => {
        $(impl NumericalValue for $ty {
            const BIT_WIDTH: usize = $bitwidth;
            #[allow(clippy::cast_precision_loss)]
            const ZERO: Self = 0 as Self;
            #[allow(clippy::cast_precision_loss)]
            const ONE: Self = 1 as Self;
            #[cfg(feature = "rand")]
            fn new_random<R: rand::Rng + ?Sized, D: rand::distr::Distribution<Self>>(rng: &mut R, distr: D) -> Self
            where
                Self: SampleUniform { rng.sample(distr) }
        })*
    };
}
numerical_value! {
    for i8; 8;
    for u8; 8;
    for i16; 16;
    for u16; 16;
    for i32; 32;
    for u32; 32;
    for i64; 64;
    for u64; 64;
    for i128; 128;
    for u128; 128;
    for char; 32;
    for f32; 32;
    for f64; 64;
}
