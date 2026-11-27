//! Operations applied to arrays. These are implemented using traits in [`std::ops`] whenever
//! possible.

use crate::arr::Arr;
use crate::backend::{Backend, InMemBackend};
use crate::dim::{CONCAT, INDEX_ARR, TOTAL_DIM};
use crate::dim::{SQUEEZE, TRANSPOSE, UNIT_ARR, UNSQUEEZE};
use crate::ix;
use crate::prop::{Prop, PropTrue};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

macro_rules! ops {
    ($($Trait:ident, $TraitAssign:ident, $f:ident, $f_assign:ident, $tt:tt);* $(;)?) => {
        $(
        impl<B1, B2, T, const D: &'static [usize]> $Trait<Arr<B2, T, D>> for Arr<B1, T, D>
        where
            B1: Backend<T>,
            B2: Backend<T>,
            T: $Trait<Output = T>,
        {
            type Output = Arr<Vec<T>, T, D>;

            fn $f(self, rhs: Arr<B2, T, D>) -> Self::Output {
                self.into_iter().zip(rhs).map(|(x, y)| x $tt y).collect()
            }
        }

        impl<B, T, const D: &'static [usize]> $Trait<T> for Arr<B, T, D>
        where
            B: Backend<T>,
            T: $Trait<Output = T> + Clone,
        {
            type Output = Arr<Vec<T>, T, D>;

            fn $f(self, rhs: T) -> Self::Output {
                self.into_iter().map(|x| x $tt rhs.clone()).collect()
            }
        }

        impl<B1, B2, T, const D: &'static [usize]> $TraitAssign<Arr<B2, T, D>> for Arr<B1, T, D>
        where
            B1: Backend<T> + InMemBackend<T>,
            B2: Backend<T>,
            for<'a> &'a T: $Trait<Output = T>,
        {
            fn $f_assign(&mut self, rhs: Arr<B2, T, D>) {
                *self.storage_mut() = B1::from_vec(
                    self.iter()
                        .zip(rhs.iter())
                        .map(|(l, r)| l $tt r)
                        .collect::<Vec<_>>(),
                )
                .unwrap();
            }
        }

        impl<B, T, const D: &'static [usize]> $TraitAssign<T> for Arr<B, T, D>
        where
            B: Backend<T> + InMemBackend<T>,
            for<'a> &'a T: $Trait<Output = T>,
        {
            fn $f_assign(&mut self, rhs: T) {
                *self.storage_mut() = B::from_vec(
                    self.iter()
                        .map(|l| l $tt &rhs)
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


impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T>,
    T: Clone,
{
    /// Transpose an array, reflecting along the primary diagonal.
    pub fn transpose(self) -> Arr<Vec<T>, T, { TRANSPOSE::<D> }> {
        const {
            assert!(D.len() == 2);
        }
        let &[height, width] = D else { unreachable!() };
        let mut out = Vec::with_capacity(height * width);
        for c in 0..height {
            for r in 0..width {
                out.push(self.storage.index(r * height + c).clone());
            }
        }
        out.into_iter().collect()
    }

    /// Squeeze an array - equivalent to removing all extra dimension (where `D[n] == 1`)
    pub fn squeeze(self) -> Arr<B, T, { SQUEEZE::<D> }> {
        // Just a reshape to specific dims.
        self.reshape()
    }

    /// Reshape an array to any other array with equivalent number of positions in the array.
    ///
    /// This method is the basis behind [`Arr::squeeze`], [`Arr::unsqueeze`], and more.
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

    /// Adds an extra dimension at `AT`, with 1 position.
    pub fn unsqueeze<const AT: usize>(self) -> Arr<B, T, { UNSQUEEZE::<D, AT> }> {
        self.reshape()
    }

    /// Views specific dimensions of an array; returns a [`Arr`] of references. This is equivalent to
    /// immutable usage of the [`ix`] macro.
    pub fn view<const V_D: &'static [usize]>(&self) -> Arr<Vec<&T>, &T, { INDEX_ARR::<D, V_D> }> {
        ix!(self => V_D)
    }

    /// Returns a mutable view into `self`. This is equivalent to mutable usage of the [`ix`] macro.
    pub fn view_mut<const V_D: &'static [usize]>(
        &mut self,
    ) -> Arr<Vec<&mut T>, &mut T, { INDEX_ARR::<D, V_D> }>
    where
        B: InMemBackend<T>,
    {
        ix!(self => mut V_D)
    }

    /// Concatenates two arrays, along the `AXIS` axis.
    pub fn concat<const AXIS: usize, B2, const D2: &'static [usize]>(
        &self,
        other: &Arr<B2, T, D2>,
    ) -> Arr<Vec<T>, T, { CONCAT::<AXIS, D, D2> }>
    where
        B2: Backend<T>,
    {
        let outer_size: usize = D[(AXIS + 1)..].iter().product();
        let chunk_a: usize = D[..=AXIS].iter().product();
        let chunk_b: usize = D2[..=AXIS].iter().product();

        let mut new = Vec::with_capacity(TOTAL_DIM::<D> + TOTAL_DIM::<D2>);

        for i in 0..outer_size {
            let start_a = i * chunk_a;
            new.extend_from_slice(&self.storage.index_range(start_a..start_a + chunk_a).cloned().collect::<Vec<_>>());

            let start_b = i * chunk_b;
            new.extend_from_slice(&other.storage.index_range(start_b..start_b + chunk_b).cloned().collect::<Vec<_>>());
        }

        Arr {
            storage: new,
            _phantom: PhantomData,
        }
    }
}

impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T>,
    Prop<{ UNIT_ARR::<D> }>: PropTrue,
{
    /// In a one-element array, returns the single element. This works for arrays with any number of
    /// dimensions, so long as each dimension has only one element.
    pub fn item(self) -> T {
        self.into_iter().next().unwrap()
    }

    /// In a one-element array, returns a reference to the single element. This works for arrays
    /// with any number of dimensions, so long as each dimension has only one element.
    pub fn item_ref(&self) -> &T {
        self.iter().next().unwrap()
    }

    /// In a one-element array, returns a mutable reference to the single element. This works for
    /// arrays with any number of dimensions, so long as each dimension has only one element.
    pub fn item_mut(&mut self) -> &mut T
    where
        B: InMemBackend<T>,
    {
        self.iter_mut().next().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn unit_1() {
        let arr1 = Arr::<_, _, { &[] }>::from([42]);
        assert_eq!(arr1.item(), 42);
    }

    #[test]
    fn unit_2() {
        let arr1 = Arr::<_, _, { &[1] }>::from([42]);
        assert_eq!(arr1.item(), 42);
    }

    #[test]
    fn unit_3() {
        let arr1 = Arr::<_, _, { &[1, 1, 1, 1, 1, 1, 1, 1] }>::from([42]);
        assert_eq!(arr1.item(), 42);
    }

    #[test]
    fn concat_1() {
        // [
        //  [1, 1],
        //  [1, 1],
        //  [1, 1]
        // ]
        let arr1 = Arr::<Vec<i32>, i32, { &[2, 3] }>::new_ones();
        // [
        //  [0, 0],
        // ]
        let arr2 = Arr::<Vec<i32>, i32, { &[2, 1] }>::new_zeroed();
        let expected = Arr::<_, i32, { &[2, 4] }>::from([1, 1, 1, 1, 1, 1, 0, 0]);
        assert_eq!(arr1.concat::<1, _, _>(&arr2), expected);
    }

    #[test]
    fn concat_2() {
        // [
        //  [1, 1, 1],
        //  [1, 1, 1],
        // ]
        let arr1 = Arr::<Vec<i32>, i32, { &[3, 2] }>::new_ones();
        // [
        //  [0],
        //  [0],
        // ]
        let arr2 = Arr::<Vec<i32>, i32, { &[1, 2] }>::new_zeroed();
        let expected = Arr::<_, i32, { &[4, 2] }>::from([1, 1, 1, 0, 1, 1, 1, 0]);
        assert_eq!(arr1.concat::</* axis = */ 0, _, _>(&arr2), expected);
    }

    #[test]
    fn concat_3() {
        // [
        //  [[1, 1], [1, 1], [1, 1]],
        //  [[1, 1], [1, 1], [1, 1]],
        // ]
        let arr1 = Arr::<Vec<i32>, i32, { &[2, 3, 2] }>::new_ones();
        // [
        //  [[0, 0], [0, 0]],
        //  [[0, 0], [0, 0]],
        // ]
        let arr2 = Arr::<Vec<i32>, i32, { &[2, 2, 2] }>::new_zeroed();
        let expected = Arr::<_, i32, { &[2, 5, 2] }>::from([
            1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0,
        ]);
        assert_eq!(arr1.concat::</* axis = */ 1, _, _>(&arr2), expected);
    }

    #[test]
    fn concat_many() {
        // [
        //  [1, 1],
        //  [1, 1],
        //  [1, 1]
        // ]
        let arr1 = Arr::<Vec<i32>, i32, { &[2, 3] }>::new_ones();
        // [
        //  [0, 0],
        // ]
        let arr2 = Arr::<Vec<i32>, i32, { &[2, 1] }>::new_zeroed();
        let arr3 = arr1.clone();
        let arr4 = arr2.clone();
        let expected =
            Arr::<_, i32, { &[2, 8] }>::from([1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0]);
        assert_eq!(
            arr1.concat::<1, _, _>(&arr2)
                .concat::<1, _, _>(&arr3)
                .concat::<1, _, _>(&arr4),
            expected
        );
    }
}
