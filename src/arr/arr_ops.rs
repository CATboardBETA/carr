use crate::arr::Arr;
use crate::backend::{Backend, BackendOps};
use crate::dim::{Dimension, Transpose};
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Rem, Sub};

impl<B, T, const D: &'static [usize]> Add for Arr<B, T, D>
where
    B: Backend<T> + BackendOps<T>,
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, |(x, y)| x + y)
    }
}

impl<B, T, const D: &'static [usize]> Sub for Arr<B, T, D>
where
    B: Backend<T> + BackendOps<T>,
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, |(x, y)| x - y)
    }
}

impl<B, T, const D: &'static [usize]> Mul for Arr<B, T, D>
where
    B: Backend<T> + BackendOps<T>,
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, |(x, y)| x * y)
    }
}

impl<B, T, const D: &'static [usize]> Div for Arr<B, T, D>
where
    B: Backend<T> + BackendOps<T>,
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, |(x, y)| x / y)
    }
}

impl<B, T, const D: &'static [usize]> Rem for Arr<B, T, D>
where
    B: Backend<T> + BackendOps<T>,
    T: Rem<Output = T>,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, |(x, y)| x % y)
    }
}
impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T> + BackendOps<T>,
    T: Clone,
{
    pub fn transpose(self) -> Arr<B, T, { <Transpose<D> as Dimension>::DIMS }> {
        const {
            assert!(D.len() == 2);
        }
        let &[height, width] = D else { unreachable!() };
        let mut out = Vec::with_capacity(height * width);
        for c in 0..height {
            for r in 0..width {
                out.push(self.storage[r * height + c].clone());
            }
        }
        Arr {
            storage: B::from_vec(out).unwrap(),
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let arr1 = Arr::<_, _, { &[2, 3][..] }>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([4, 5, 9, 0, -2, -4]);
        assert_eq!(arr1 + arr2, expected);
    }

    #[test]
    fn sub() {
        let arr1 = Arr::<_, _, { &[2, 3][..] }>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([2, -1, -7, 6, 6, 0]);
        assert_eq!(arr1 - arr2, expected);
    }

    #[test]
    fn mul() {
        let arr1 = Arr::<_, _, { &[2, 3][..] }>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([3, 6, 8, -9, -8, 4]);
        assert_eq!(arr1 * arr2, expected);
    }

    #[test]
    fn div() {
        let arr1 = Arr::<_, _, { &[2, 3][..] }>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([3, 2 / 3, 1 / 8, 3 / -3, 2 / (-4), (-2) / (-2)]);
        assert_eq!(arr1 / arr2, expected);
    }

    #[test]
    fn rem() {
        let arr1 = Arr::<_, _, { &[2, 3][..] }>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([0, 2, 1, 3 % (-3), 2, (-2) % (-2)]);
        assert_eq!(arr1 % arr2, expected);
    }

    #[test]
    fn transpose() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3i32, 2, 1, 3, 2, -2]);
        let expected = Arr::<_, _, { &[3, 2] }>::from([3i32, 1, 2, 2, 3, -2]);
        let transposed = arr1.transpose();
        assert_eq!(transposed, expected);
    }
}
