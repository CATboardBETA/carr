use itertools::Itertools;
#[cfg(feature = "rand")]
use rand::RngExt;
#[cfg(feature = "rand")]
use rand::distr::uniform::SampleUniform;
use std::ops::{Index, IndexMut, Range};

pub trait Backend<T>:
    Index<usize, Output = T>
    + IndexMut<usize, Output = T>
    + Index<Range<usize>, Output = [T]>
    + IndexMut<Range<usize>, Output = [T]>
    + Sized
{
    #[must_use]
    fn new_bck() -> Self;
    #[must_use]
    fn from_vec(v: Vec<T>) -> Option<Self>;
    #[must_use]
    fn length(&self) -> usize;
    #[must_use]
    fn into_vec(self) -> Vec<T>;
    #[must_use]
    fn as_vec(&self) -> Vec<&T>;
}

pub trait BackendOps<T>: Backend<T> + IntoIterator<Item = T> {
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
impl<T: Default, const N: usize> Backend<T> for [T; N] {
    fn new_bck() -> Self {
        std::array::from_fn(|_| T::default())
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

impl<T: Default, const N: usize> BackendOps<T> for [T; N] {
    fn apply_ops<B2, B3, F>(self, other: B2, op: F) -> B3
    where
        B2: Backend<T> + BackendOps<T>,
        B3: Backend<T> + BackendOps<T>,
        F: FnMut((T, T)) -> T,
    {
        B3::from_vec(self.into_iter().zip(other).map(op).collect::<Vec<_>>()).unwrap()
    }
}

impl<T> Backend<T> for Vec<T> {
    fn new_bck() -> Self {
        vec![]
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

impl<T> BackendOps<T> for Vec<T> {
    fn apply_ops<B2, B3, F>(self, other: B2, op: F) -> B3
    where
        B2: Backend<T> + BackendOps<T>,
        B3: Backend<T> + BackendOps<T>,
        F: FnMut((T, T)) -> T,
    {
        B3::from_vec(self.into_iter().zip(other).map(op).collect::<Self>()).unwrap()
    }
}

pub trait NumericalValue: Sized {
    const BIT_WIDTH: usize;

    const ZERO: Self;
    const ONE: Self;

    const MIN: Self;
    const MAX: Self;
    #[cfg(feature = "rand")]
    fn new_random<R: rand::Rng + ?Sized, D: rand::distr::Distribution<Self>>(rng: &mut R, distr: D) -> Self
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
            const MIN: Self = Self::MIN;
            const MAX: Self = Self::MAX;
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
