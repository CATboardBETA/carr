//! Backend storage, internal to [`arr::Arr`](crate::arr::Arr). Also contains a trait for numerical
//! types (e.g. i32, f64, u8, etc.)

use bumpalo::Bump;
use bytemuck::Pod;
use itertools::Itertools;
#[cfg(feature = "rand")]
use rand::distr::uniform::SampleUniform;
#[cfg(feature = "rand")]
use rand::distr::Distribution;
#[cfg(feature = "rand")]
use rand::Rng;
#[cfg(feature = "rand")]
use rand::RngExt;
use std::cell::RefCell;
use std::ops::{Index, IndexMut, Range};
use std::{array, iter};

/// A storage backend for arrays. This does not store the dimensions, and can be a variable-length
/// type (e.g. a [`Vec`]).
///
/// By default, this is implemented for [`Vec<T>`] and [`[T; N]`](prim@array)
#[must_use]
pub trait Backend<T>: Sized {
    /// Get the 1-D length of this backend. This should be equivalent to the product of the
    /// dimensions slice of the outer array.
    #[must_use]
    fn length(&self) -> usize;

    /// Iterate over all elements of the backend by-value.
    fn into_iter_all(self) -> impl Iterator<Item = T>;

    /// Gets a value from a specific index
    fn get<'a>(&'a self, index: usize) -> impl FnOnce() -> Option<&'a T> where T: 'a;

    /// Gets a range of values from a range of indices
    fn get_range<'a>(&'a self, index: Range<usize>) -> Option<impl Iterator<Item=&'a T>> where T: 'a;

    /// Gets a value from a specific index, panicking on OOB
    #[must_use]
    fn index(&self,index: usize) -> &T {
        self.get(index)().unwrap()
    }

    /// Gets a range of values from a range of indices, panicking on OOB
    fn index_range<'a>(&'a self, index: Range<usize>) -> impl Iterator<Item=&'a T> where T: 'a {
        self.get_range(index).unwrap()
    }
}

/// A backend that is stored in memory.
pub trait InMemBackend<T>:
    Index<usize, Output = T>
    + Index<Range<usize>, Output = [T]>
    + IndexMut<usize, Output = T>
    + IndexMut<Range<usize>, Output = [T]>
    + Backend<T>
{
    /// Creates a new, empty instance of the backend.
    #[must_use]
    fn new_bck<const N: usize>(x: &T) -> Self
    where
        T: Clone;
    /// Creates a [`Vec<&mut T>`] from a backend, without consuming the underlying storage.
    #[must_use]
    fn as_vec_mut(&mut self) -> Vec<&mut T>;

    /// Create an instance of this backend from a [`Vec<T>`].
    #[must_use]
    fn from_vec(v: Vec<T>) -> Option<Self>;

    /// Creates a [`Vec<T>`] from a backend, consuming the underlying storage.
    #[must_use]
    fn into_vec(self) -> Vec<T>;

    /// Creates a [`Vec<&T>`] from a backend, without consuming the underlying storage.
    #[must_use]
    fn as_vec(&self) -> Vec<&T>;

    /// Splits an array into an array of arrays.
    #[must_use]
    fn as_split<const X: usize, const Y: usize>(&self) -> &[[T; X]; Y]
    where
        T: Pod;
}

/// TODO: Remove bound on `T`. `MaybeUninit` perhaps?
impl<T, const N: usize> Backend<T> for [T; N] {
    fn length(&self) -> usize {
        self.len()
    }

    fn into_iter_all(self) -> impl Iterator<Item = T> {
        self.into_iter()
    }

    fn get<'a>(&'a self, index: usize) -> impl FnOnce() -> Option<&'a T> where T: 'a {
        move || <[T]>::get(self, index)
    }

    fn get_range<'a>(&'a self, index: Range<usize>) -> Option<impl Iterator<Item=&'a T>>
    where
        T: 'a
    {
        <[T]>::get(self, index).map(|x| x.iter())
    }
}

impl<T, const N: usize> InMemBackend<T> for [T; N] {
    fn new_bck<const N2: usize>(v: &T) -> [T; N]
    where
        T: Clone,
    {
        const { assert!(N == N2, "internal error") }
        array::from_fn::<_, N, _>(|_| v.clone())
    }

    fn as_vec_mut(&mut self) -> Vec<&mut T> {
        self.iter_mut().collect_vec()
    }

    fn from_vec(v: Vec<T>) -> Option<Self> {
        v.try_into().ok()
    }

    fn into_vec(self) -> Vec<T> {
        self.into_iter().collect_vec()
    }

    fn as_vec(&self) -> Vec<&T> {
        self.iter().collect_vec()
    }

    fn as_split<const X: usize, const Y: usize>(&self) -> &[[T; X]; Y]
    where
        T: Pod,
    {
        const {
            assert!(X * Y == N);
        }
        bytemuck::must_cast_ref(self)
    }
}

impl<T> Backend<T> for Vec<T> {
    fn length(&self) -> usize {
        self.len()
    }

    fn into_iter_all(self) -> impl Iterator<Item = T> {
        self.into_iter()
    }

    fn get<'a>(&'a self, index: usize) -> impl FnOnce() -> Option<&'a T> where T: 'a {
        move || <[T]>::get(self, index)
    }

    fn get_range<'a>(&'a self, index: Range<usize>) -> Option<impl Iterator<Item=&'a T>> where T: 'a {
        <[T]>::get(self, index).map(|x| x.iter())
    }
}

impl<T> InMemBackend<T> for Vec<T> {
    fn new_bck<const N: usize>(x: &T) -> Self
    where
        T: Clone,
    {
        Self::from_fn(N, |_| x.clone())
    }

    fn as_vec_mut(&mut self) -> Vec<&mut T> {
        self.iter_mut().collect_vec()
    }

    fn from_vec(v: Self) -> Option<Self> {
        Some(v)
    }

    fn into_vec(self) -> Self {
        self
    }

    fn as_vec(&self) -> Vec<&T> {
        self.iter().collect_vec()
    }

    fn as_split<const X: usize, const Y: usize>(&self) -> &[[T; X]; Y]
    where
        T: Pod,
    {
        bytemuck::cast_ref(self.as_array::<{ crate::dim::PROD::<X, Y> }>().unwrap())
    }
}

/// Backend that always returns the same value:
pub struct ConstantBackend<'a, T: Clone>(&'a T);

impl<'a, T: Clone> ConstantBackend<'a, T> {
    /// Create a new [`ConstantBackend`], from a given value
    pub const fn new(v: &'a T) -> Self {
        Self(v)
    }
}

// I give up, I'll just let it require `Clone` ig
impl<T: Clone> Backend<T> for ConstantBackend<'_, T> {
    fn length(&self) -> usize {
        usize::MAX
    }

    fn into_iter_all(self) -> impl Iterator<Item=T> {
        iter::repeat(self.0.clone())
    }

    fn get<'a>(&'a self, _index: usize) -> impl FnOnce() -> Option<&'a T> where T: 'a {
        move || Some(self.0)
    }

    fn get_range<'a>(&'a self, index: Range<usize>) -> Option<impl Iterator<Item=&'a T>> where T: 'a {
        Some(iter::repeat_n(self.0, index.len()))
    }
}

/// Backend that calls a closure with an index for each element
pub struct FnBackend<T, F>
where
    F: FnMut(usize) ->  T
{
    func: RefCell<F>,
    cache: Bump
}

impl< T, F> FnBackend<T, F>
where
    F: FnMut(usize) -> T
{
    /// Create a new functional backend from a [`FnMut`] closure.
    pub fn new(closure: F) -> Self {
        Self {
            func: RefCell::new(closure),
            cache: Bump::new(),
        }
    }
}


impl<T, F> Backend<T> for FnBackend<T, F>
where
    F: FnMut(usize) -> T
{
    fn length(&self) -> usize {
        usize::MAX
    }

    fn into_iter_all(self) -> impl Iterator<Item=T> {
        let mut i = 0;
        iter::from_fn(move || {
            let x = self.func.borrow_mut()(i);
            i += 1;
            Some(x)
        })
    }

    fn get<'a>(&'a self, index: usize) -> impl FnOnce() -> Option<&'a T> where T: 'a
    {
        move || Some(self.cache.alloc(self.func.borrow_mut()(index)))
    }

    fn get_range<'a>(&'a self, index: Range<usize>) -> Option<impl Iterator<Item=&'a T>>
    where
        T: 'a,
    {
        Some(index.into_iter().map(move |i| &*self.cache.alloc(self.func.borrow_mut()(i))))
    }
}

/// Implemented for all numerical types (including char). This is an implementation detail of some
/// array creation functions, such as [`Arr::new_zeroed`](crate::arr::Arr::new_zeroed) and
/// [`Arr::new_ones`](crate::arr::Arr::new_ones)
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
    fn new_random<R: Rng + ?Sized, D: Distribution<Self>>(rng: &mut R, distr: D) -> Self
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

impl NumericalValue for usize {
    #[cfg(target_pointer_width = "32")]
    const BIT_WIDTH: Self = 32;
    #[cfg(target_pointer_width = "64")]
    const BIT_WIDTH: Self = 64;
    const ZERO: Self = 0;
    const ONE: Self = 1;

    #[cfg(feature = "rand")]
    fn new_random<R: Rng + ?Sized, D: Distribution<Self>>(rng: &mut R, distr: D) -> Self
    where
        Self: SampleUniform,
    {
        rng.sample(distr)
    }
}

#[cfg(test)]
mod test {
    use crate::backend::{Backend, ConstantBackend, FnBackend};

    #[test]
    fn constant_backend() {
        let backend = ConstantBackend::new(&7);
        let mut iter = backend.get_range(0..4).unwrap();
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn fn_backend() {
        let backend = FnBackend::new(|i: usize| 2 * i);
        let mut iter = backend.get_range(0..4).unwrap();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }


    #[test]
    fn fn_backend_noncopy() {
        #[derive(Debug, PartialEq)]
        struct NonCopy(usize);
        let backend = FnBackend::new(|i: usize| NonCopy(2 * i));
        let mut iter = backend.get_range(0..4).unwrap();
        assert_eq!(iter.next(), Some(&NonCopy(0)));
        assert_eq!(iter.next(), Some(&NonCopy(2)));
        assert_eq!(iter.next(), Some(&NonCopy(4)));
        assert_eq!(iter.next(), Some(&NonCopy(6)));
        assert_eq!(iter.next(), None);
    }
}