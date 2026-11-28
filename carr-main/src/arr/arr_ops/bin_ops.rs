use crate::arr::Arr;
use crate::backend::{Backend, InMemBackend};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

macro_rules! ops {
    ($($Trait:ident, $TraitAssign:ident, $f:ident, $f_assign:ident, $tt:tt);* $(;)?) => {
        $(
        impl<B1, B2, T, T2, const D: &'static [usize]> $Trait<Arr<B2, T2, D>> for Arr<B1, T, D>
        where
            B1: Backend<T>,
            B2: Backend<T2>,
            T: $Trait<T2>,
        {
            type Output = Arr<Vec<<T as $Trait<T2>>::Output>, <T as $Trait<T2>>::Output, D>;

            fn $f(self, rhs: Arr<B2, T2, D>) -> Self::Output {
                self.into_iter().zip(rhs).map(|(x, y)| x $tt y).collect()
            }
        }

        impl<B1, B2, T, T2, const D: &'static [usize]> $TraitAssign<Arr<B2, T2, D>> for Arr<B1, T, D>
        where
            B1: Backend<T> + InMemBackend<T>,
            B2: Backend<T2>,
            for<'a> &'a T: $Trait<&'a T2, Output = T>,
        {
            fn $f_assign(&mut self, rhs: Arr<B2, T2, D>) {
                *self.storage_mut() = B1::from_vec(
                    self.iter()
                        .zip(rhs.iter())
                        .map(|(l, r)| l $tt r)
                        .collect::<Vec<_>>(),
                )
                .unwrap();
            }
        }
        )*
    };
}

ops!(
    Add, AddAssign, add, add_assign, +;
    Sub, SubAssign, sub, sub_assign, -;
    Mul, MulAssign, mul, mul_assign, *;
    Div, DivAssign, div, div_assign, /;
    Rem, RemAssign, rem, rem_assign, %;
);

#[cfg(test)]
mod test {
    use crate::arr::Arr;
    use crate::backend::ConstantBackend;

    #[test]
    fn ref_ops() {
        let mut arr1 = Arr::<_, _, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([4, 5, 9, 0, -2, -4]);
        assert_eq!(arr1.view_all() + arr2.view_all(), expected);
        assert_eq!(arr1.clone() + arr2.view_all(), expected);
        assert_eq!(arr1.view_all() + arr2.clone(), expected);
        arr1 += arr2;
        assert_eq!(arr1, expected);
    }

    #[test]
    fn op_diff_backends() {
        let arr = Arr::<_, i32, { &[2, 3] }>::from([3, 2, 1, 3, 2, -2]);
        let arr_sevens = Arr::<_, i32, { &[2, 3] }>::from([7, 7, 7, 7, 7, 7]);
        let const_arr = Arr::<_, i32, { &[2, 3] }>::from(ConstantBackend::new(&7));
        assert_eq!(arr.view_all() + arr_sevens, arr.view_all() + const_arr);
    }

    #[test]
    fn add() {
        let mut arr1 = Arr::<_, _, { &[2, 3][..] }>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([4, 5, 9, 0, -2, -4]);
        assert_eq!(arr1.clone() + arr2.clone(), expected);
        arr1 += arr2;
        assert_eq!(arr1, expected);
    }

    #[test]
    fn sub() {
        let mut arr1 = Arr::<_, _, { &[2, 3][..] }>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([2, -1, -7, 6, 6, 0]);
        assert_eq!(arr1.clone() - arr2.clone(), expected);
        arr1 -= arr2;
        assert_eq!(arr1, expected);
    }

    #[test]
    fn mul() {
        let mut arr1 = Arr::<_, _, { &[2, 3][..] }>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([3, 6, 8, -9, -8, 4]);
        assert_eq!(arr1.clone() * arr2.clone(), expected);
        arr1 *= arr2;
        assert_eq!(arr1, expected);
    }

    #[test]
    fn div() {
        let mut arr1 = Arr::<_, _, { &[2, 3][..] }>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([3, 2 / 3, 1 / 8, 3 / -3, 2 / (-4), (-2) / (-2)]);
        assert_eq!(arr1.clone() / arr2.clone(), expected);
        arr1 /= arr2;
        assert_eq!(arr1, expected);
    }

    #[test]
    fn rem() {
        let mut arr1 = Arr::<_, _, { &[2, 3][..] }>::from([3, 2, 1, 3, 2, -2]);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2]);
        let expected = Arr::from([0, 2, 1, 3 % (-3), 2, (-2) % (-2)]);
        assert_eq!(arr1.clone() % arr2.clone(), expected);
        arr1 %= arr2;
        assert_eq!(arr1, expected);
    }
}
