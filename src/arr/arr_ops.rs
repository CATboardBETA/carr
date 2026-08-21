use crate::arr::Arr;
use crate::backend::{Backend, BackendOps};
use std::ops::Add;

impl<B, T, const N_DIMS: usize> Add for Arr<B, T, N_DIMS>
where
    B: Backend<T> + BackendOps<T>,
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let dims = self.dims;
        self.apply_ops(rhs, dims, |(x, y)| x + y)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dimension::Dimensions;

    #[test]
    fn add() {
        let dims = Dimensions::new([2, 3]);
        let arr1 = Arr::from([3, 2, 1, 3, 2, -2], dims);
        let arr2 = Arr::from([1, 3, 8, -3, -4, -2], dims);
        let expected = Arr::from([4, 5, 9, 0, -2, -4], dims);
        assert_eq!(arr1 + arr2, expected);
    }
}
