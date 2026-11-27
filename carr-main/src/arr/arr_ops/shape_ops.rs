use std::marker::PhantomData;
use crate::arr::Arr;
use crate::backend::Backend;
use crate::dim::{SQUEEZE, TRANSPOSE, UNSQUEEZE};

impl<B, T, const D: &'static [usize]> Arr<B, T, D> 
where
    B: Backend<T>
{
    // Not sure if this is the best module for transpose, but ig I can always change it later
    /// Transpose an array, reflecting along the primary diagonal.
    pub fn transpose(self) -> Arr<Vec<T>, T, { TRANSPOSE::<D> }> 
    where
        T: Clone
    {
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
}

#[cfg(test)]
mod test {
    use crate::arr::Arr;
    
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