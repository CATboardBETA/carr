use crate::arr::Arr;
use crate::backend::{Backend, InMemBackend};
use crate::dim::{INDEX_ARR, UNIT_ARR};
use crate::ix;
use crate::prop::{Prop, PropTrue};

impl<B, T, const D: &'static [usize]> Arr<B, T, D>
where
    B: Backend<T>
{
    /// Views specific dimensions of an array; returns a [`Arr`] of references. This is equivalent to
    /// immutable usage of the [`ix`] macro.
    pub fn view<const V_D: &'static [usize]>(&self) -> Arr<Vec<&T>, &T, { INDEX_ARR::<D, V_D> }> {
        ix!(self => V_D)
    }

    /// Views the entirety of an array, immutably.
    pub fn view_all(&self) -> Arr<Vec<&T>, &T, D> {
        self.iter().collect()
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


    /// Views the entirety of an array, mutably. Note that this only works on arrays with an
    /// in-memory backend, for functional backends are immutable.
    pub fn view_mut_all(&mut self) -> Arr<Vec<&mut T>, &mut T, D>
    where
        B: InMemBackend<T>
    {
        self.iter_mut().collect()
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
    use crate::arr::Arr;

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
}