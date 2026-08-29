use crate::arr::Arr;
use crate::backend::{Backend, BackendOps};
use crate::dim::{Dimension, Squeeze, Transpose, Unsqueeze};
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
    pub fn squeeze(self) -> Arr<B, T, { <Squeeze<D> as Dimension>::DIMS }> {
        // Just a reshape to specific dims.
        self.reshape()
    }

    pub fn reshape<const D2: &'static [usize]>(self) -> Arr<B, T, D2> {
        // make sure total of dimensions is the same
        const {
            let mut d1 = 0;
            let mut d2 = 0;
            let mut i = 0;
            while i < D.len() {
                d1 *= D[i];
                i += 1;
            }
            i = 0;
            while i < D2.len() {
                d2 *= D2[i];
                i += 1;
            }
            assert!(d1 == d2);
        }
        Arr {
            storage: self.storage,
            _phantom: PhantomData,
        }
    }

    pub fn unsqueeze<const AT: usize>(
        self,
    ) -> Arr<B, T, { <Unsqueeze<D, AT> as Dimension>::DIMS }> {
        self.reshape()
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
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Arr::<_, _, { &[3, 2] }>::from([3, 1, 2, 2, 3, -2]);
        let transposed = arr1.transpose();
        assert_eq!(transposed, expected);
    }

    #[test]
    fn squeeze_1() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let squeezed = arr1.squeeze();
        assert_eq!(squeezed, expected);
    }
    #[test]
    fn squeeze_2() {
        let arr1 = Arr::<_, _, { &[1, 2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let squeezed = arr1.squeeze();
        assert_eq!(squeezed, expected);
    }
    #[test]
    fn squeeze_3() {
        let arr1 = Arr::<_, _, { &[1, 2, 1, 3, 1] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let squeezed = arr1.squeeze();
        assert_eq!(squeezed, expected);
    }

    #[test]
    fn reshape() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Arr::<_, _, { &[1, 6] }>::from([3, 2, 1, 3, 2, -2]);
        let reshaped = arr1.reshape();
        assert_eq!(reshaped, expected);
    }

    #[test]
    fn unsqueeze_1() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Arr::<_, _, { &[1, 2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let unsqueezed = arr1.unsqueeze::<0>();
        assert_eq!(unsqueezed, expected);
    }

    #[test]
    fn unsqueeze_2() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Arr::<_, _, { &[2, 3, 1] }>::from([3, 2, 1, 3, 2, -2]);
        let unsqueezed = arr1.unsqueeze::<2>();
        assert_eq!(unsqueezed, expected);
    }

    #[test]
    fn unsqueeze_3() {
        let arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let expected = Arr::<_, _, { &[2, 1, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let unsqueezed = arr1.unsqueeze::<1>();
        assert_eq!(unsqueezed, expected);
    }
}
