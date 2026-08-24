use std::ops::{Index, IndexMut};

pub trait Backend<T>: Index<usize, Output = T> + IndexMut<usize, Output = T> + Sized {
    #[must_use]
    fn new_bck() -> Self;
    #[must_use]
    fn from_vec(v: Vec<T>) -> Option<Self>;
}

pub trait BackendOps<T>: Backend<T> + IntoIterator<Item = T> {
    #[must_use]
    fn apply_ops<
        B2: Backend<T> + BackendOps<T>,
        B3: Backend<T> + BackendOps<T>,
        F: FnMut((T, T)) -> T,
    >(
        self,
        other: B2,
        op: F,
    ) -> B3;
}

/// TODO: Remove bound on `T`. `MaybeUninit` perhaps?
impl<T: Default, const N: usize> Backend<T> for [T; N] {
    fn new_bck() -> Self {
        std::array::from_fn(|_| T::default())
    }

    fn from_vec(v: Vec<T>) -> Option<[T; N]> {
        v.try_into().ok()
    }
}

impl<T: Default, const N: usize> BackendOps<T> for [T; N] {
    fn apply_ops<B2, B3, F>(self, other: B2, op: F) -> B3
    where
        B2: Backend<T> + BackendOps<T>,
        B3: Backend<T> + BackendOps<T>,
        F: FnMut((T, T)) -> T,
    {
        B3::from_vec(self.into_iter().zip(other).map(op).collect::<Vec<_>>()).unwrap()
    }
}

impl<T> Backend<T> for Vec<T> {
    fn new_bck() -> Self {
        vec![]
    }

    fn from_vec(v: Self) -> Option<Self> {
        Some(v)
    }
}

impl<T> BackendOps<T> for Vec<T> {
    fn apply_ops<B2, B3, F>(self, other: B2, op: F) -> B3
    where
        B2: Backend<T> + BackendOps<T>,
        B3: Backend<T> + BackendOps<T>,
        F: FnMut((T, T)) -> T,
    {
        B3::from_vec(self.into_iter().zip(other).map(op).collect::<Self>()).unwrap()
    }
}
