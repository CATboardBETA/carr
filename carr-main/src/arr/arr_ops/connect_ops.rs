use crate::arr::Arr;
use crate::backend::Backend;
use crate::dim::{CONCAT, TOTAL_DIM};
use std::marker::PhantomData;

impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T>,
    T: Clone,
{
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
            new.extend_from_slice(
                &self
                    .storage
                    .index_range(start_a..start_a + chunk_a)
                    .cloned()
                    .collect::<Vec<_>>(),
            );

            let start_b = i * chunk_b;
            new.extend_from_slice(
                &other
                    .storage
                    .index_range(start_b..start_b + chunk_b)
                    .cloned()
                    .collect::<Vec<_>>(),
            );
        }

        Arr {
            storage: new,
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::arr::Arr;

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
