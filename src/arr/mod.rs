use crate::backend::{Backend, BackendOps, NumericalValue};
use crate::dim::TOTAL_DIM;
#[cfg(feature = "rand")]
use rand::distr::StandardUniform;
#[cfg(feature = "rand")]
use rand::distr::uniform::SampleUniform;
#[cfg(feature = "rand")]
use rand::distr::Uniform;
#[cfg(feature = "rand")]
use rand::RngExt;
use std::marker::PhantomData;

mod arr_ops;
mod conversions;
#[coverage(off)]
mod debug;
mod eq;
pub mod index;

#[derive(Eq)]
pub struct Arr<B, T, const DIMS: &'static [usize]>
where
    B: Backend<T>,
{
    storage: B,
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
    B: Backend<T>,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            storage: B::new_bck(),
            _phantom: PhantomData,
        }
    }

    pub const fn storage(&self) -> &B {
        &self.storage
    }

    pub const fn storage_mut(&mut self) -> &mut B {
        &mut self.storage
    }
}

impl<B, T, const D: &'static [usize]> Default for Arr<B, T, D>
where
    B: Backend<T>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T> + BackendOps<T>,
{
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
    T: NumericalValue + Clone,
{

    #[must_use]
    pub fn new_zeroed() -> Self {
        Self::from(B::from_vec(vec![T::ZERO; TOTAL_DIM::<D>]).unwrap())
    }

    #[must_use]
    pub fn new_ones() -> Self {
        Self::from(B::from_vec(vec![T::ONE; TOTAL_DIM::<D>]).unwrap())
    }

    #[cfg(feature = "rand")]
    #[must_use]
    pub fn new_random() -> Self
    where
        T: SampleUniform,
        StandardUniform: rand::distr::Distribution<T>
    {
        let mut rng = rand::rng();
        let distr = StandardUniform;
        let a: [T; TOTAL_DIM::<D>] = std::array::from_fn(|_| rng.sample(distr));
        Self::from(B::from_vec(a.to_vec()).unwrap())
    }

    #[cfg(feature = "rand")]
    #[must_use]
    pub fn new_random_rng<R: rand::Rng + ?Sized>(rng: &mut R) -> Self
    where
        T: SampleUniform,
        StandardUniform: rand::distr::Distribution<T>
    {
        let distr = StandardUniform;
        let a: [T; TOTAL_DIM::<D>] = std::array::from_fn(|_| rng.sample(distr));
        Self::from(B::from_vec(a.to_vec()).unwrap())
    }

    #[cfg(feature = "rand")]
    #[must_use]
    pub fn new_random_range(start: T, end: T) -> Self
    where
        T: SampleUniform,
    {
        let mut rng = rand::rng();
        let distr = &Uniform::new(start, end).unwrap();
        let a: [T; TOTAL_DIM::<D>] = std::array::from_fn(|_| rng.sample(distr));
        Self::from(B::from_vec(a.to_vec()).unwrap())
    }
    #[cfg(feature = "rand")]
    #[must_use]
    pub fn new_random_range_rng<R: rand::Rng + ?Sized>(rng: &mut R, start: T, end: T) -> Self
    where
        T: SampleUniform,
    {
        let distr = &Uniform::new(start, end).unwrap();
        let a: [T; TOTAL_DIM::<D>] = std::array::from_fn(|_| rng.sample(distr));
        Self::from(B::from_vec(a.to_vec()).unwrap())
    }
}

#[cfg(test)]
mod test {
    use std::array;
    use crate::arr::Arr;

    #[test]
    fn zeroed_test() {
        let arr = Arr::<Vec<_>, f32, { &[1, 2] }>::new_zeroed();
        let expected = Arr::<_, f32, { &[1, 2] }>::from([0.,0.]);
        assert_eq!(arr, expected);
    }
    #[test]
    fn ones_test() {
        let arr = Arr::<Vec<_>, f32, { &[5, 2, 1, 2] }>::new_ones();
        let expected = Arr::<_, f32, { &[5, 2, 1, 2] }>::from(array::repeat::<_, { 5 * 2 * 2}>(1.));
        assert_eq!(arr, expected);
    }
}