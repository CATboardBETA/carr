use crate::arr::Arr;
use crate::backend::Backend;
use std::marker::PhantomData;

pub struct ArrIter<'a, B: Backend<T>, T, const D: &'static [usize]> {
    inner: &'a Arr<B, T, D>,
    at: usize,
}

pub struct ArrIterMut<'a, T> {
    inner: Vec<&'a mut T>,
}

impl<'a, B: Backend<T>, T: 'a, const D: &'static [usize]> Iterator for ArrIter<'a, B, T, D> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.at < self.inner.storage().length() {
            Some(self.inner.storage().index(self.at))
        } else {
            None
        };
        self.at += 1;
        ret
    }
}

impl<'a, T: 'a> Iterator for ArrIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop()
    }
}

impl<'a, B: Backend<T>, T: 'a, const D: &'static [usize]> ExactSizeIterator
    for ArrIter<'a, B, T, D>
{
    fn len(&self) -> usize {
        self.inner.storage().length()
    }
}

impl<'a, B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T> + 'a,
    T: 'a,
{
    /// Iterate by reference over an [`Arr`]'s internal storage
    pub fn iter(&'a self) -> ArrIter<'a, B, T, D> {
        self.into_iter()
    }

    /// Iterate by exclusive (mutable) reference over an [`Arr`]'s internal storage.
    pub fn iter_mut(&'a mut self) -> ArrIterMut<'a, T> {
        self.into_iter()
    }
}

impl<'a, B, T, const D: &'static [usize]> IntoIterator for &'a Arr<B, T, D>
where
    B: Backend<T> + 'a,
    T: 'a,
{
    type Item = &'a T;
    type IntoIter = ArrIter<'a, B, T, D>;

    fn into_iter(self) -> Self::IntoIter {
        ArrIter { inner: self, at: 0 }
    }

}


impl<'a, B, T, const D: &'static [usize]> IntoIterator for &'a mut Arr<B, T, D>
where
    B: Backend<T> + 'a,
    T: 'a,
{
    type Item = &'a mut T;
    type IntoIter = ArrIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut inner = self
                .storage
                .as_vec_mut();
        inner.reverse();
        ArrIterMut {
            inner
        }
    }
}

impl<B, T, const D: &'static [usize]> FromIterator<T> for Arr<B, T, D>
where
    B: Backend<T>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            storage: B::from_vec(iter.into_iter().collect::<Vec<_>>()).unwrap(),
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::arr::Arr;

    #[test]
    fn iter_1() {
        let arr = Arr::<[i32; 0], _, { &[0] }>::new(&1);
        let mut iter = arr.iter();
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_2() {
        let arr = Arr::<[i32; 1], _, { &[1] }>::new_ones();
        let mut iter = arr.iter();
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_3() {
        let arr = Arr::<_, i32, { &[3] }>::from([1, 0, 1]);
        let mut iter = arr.iter();
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&0), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn from_iter() {
        let v = vec![1, 2, 3, 4, 5].into_iter();
        let expected = Arr::<_, _, { &[5] }>::from([1, 2, 3, 4, 5]);
        assert_eq!(expected, v.collect::<Arr<Vec<_>, _, { &[5] }>>());
    }

    #[test]
    fn iter_mut() {
        let mut arr = Arr::<_, _, { &[5] }>::from([1, 2, 3, 4, 5]);
        for x in &mut arr {
            *x = if *x > 3 {
                0
            } else {
                *x
            }
        }
        let expected = Arr::<_, _, { &[5] }>::from([1, 2, 3, 0, 0]);
        assert_eq!(arr, expected);
    }
}
