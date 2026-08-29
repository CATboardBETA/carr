use crate::arr::Arr;
use crate::backend::Backend;

impl<B1, B2, T, const D: &'static [usize]> PartialEq<Arr<B2, T, D>> for Arr<B1, T, D>
where
    B1: Backend<T>,
    B2: Backend<T>,
    T: PartialEq,
{
    fn eq(&self, other: &Arr<B2, T, D>) -> bool {
        self.storage().as_vec() == other.storage().as_vec()
    }
}
