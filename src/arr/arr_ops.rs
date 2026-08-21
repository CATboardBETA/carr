use crate::arr::Arr;
use crate::backend::{Backend, BackendOps};
use crate::dimension::{Dimensions, Transmute};
use std::ops::{Add, Div, Mul, Rem, Sub};

impl<B, T, D, const N_DIMS: usize> Add for Arr<B, T, D, N_DIMS>
where
    B: Backend<T> + BackendOps<T>,
    T: Add<Output = T>,
    D: Dimensions<N_DIMS>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, |(x, y)| x + y)
    }
}

impl<B, T, D, const N_DIMS: usize> Sub for Arr<B, T, D, N_DIMS>
where
    B: Backend<T> + BackendOps<T>,
    T: Sub<Output = T>,
    D: Dimensions<N_DIMS>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, |(x, y)| x - y)
    }
}

impl<B, T, D, const N_DIMS: usize> Mul for Arr<B, T, D, N_DIMS>
where
    B: Backend<T> + BackendOps<T>,
    T: Mul<Output = T>,
    D: Dimensions<N_DIMS>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, |(x, y)| x * y)
    }
}

impl<B, T, D, const N_DIMS: usize> Div for Arr<B, T, D, N_DIMS>
where
    B: Backend<T> + BackendOps<T>,
    T: Div<Output = T>,
    D: Dimensions<N_DIMS>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, |(x, y)| x / y)
    }
}

impl<B, T, D, const N_DIMS: usize> Rem for Arr<B, T, D, N_DIMS>
where
    B: Backend<T> + BackendOps<T>,
    T: Rem<Output = T>,
    D: Dimensions<N_DIMS>,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, |(x, y)| x % y)
    }
}

impl<B, T, D> Arr<B, T, D, 2>
where
    B: Backend<T> + BackendOps<T>,
    T: Add<Output = T>,
    D: Dimensions<2>,
{
    pub fn transmute(&self) -> Arr<B, T, Transmute<D>, 2> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dimension::D2;
    use crate::size::S;

    #[test]
    fn add() {
        type Dims = D2<S<2>, S<3>>;
        let arr1 = Arr::<_, _, Dims, _>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([4, 5, 9, 0, -2, -4]);
        assert_eq!(arr1 + arr2, expected);
    }

    #[test]
    fn sub() {
        type Dims = D2<S<2>, S<3>>;
        let arr1 = Arr::<_, _, Dims, _>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([2, -1, -7, 6, 6, 0]);
        assert_eq!(arr1 - arr2, expected);
    }

    #[test]
    fn mul() {
        type Dims = D2<S<2>, S<3>>;
        let arr1 = Arr::<_, _, Dims, _>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([3, 6, 8, -9, -8, 4]);
        assert_eq!(arr1 * arr2, expected);
    }

    #[test]
    fn div() {
        type Dims = D2<S<2>, S<3>>;
        let arr1 = Arr::<_, _, Dims, _>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([3, 2 / 3, 1 / 8, 3 / -3, 2 / (-4), (-2) / (-2)]);
        assert_eq!(arr1 / arr2, expected);
    }

    #[test]
    fn rem() {
        type Dims = D2<S<2>, S<3>>;
        let arr1 = Arr::<_, _, Dims, _>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([0, 2, 1, 3 % (-3), 2, (-2) % (-2)]);
        assert_eq!(arr1 % arr2, expected);
    }
}
