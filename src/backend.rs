pub trait Backend<T> {
    #[must_use]
    fn new_bck() -> Self;
}

pub trait BackendOps<T>: Backend<T> + IntoIterator<Item = T> {
    #[must_use]
    fn apply_ops<O: Backend<T> + BackendOps<T>, F: FnMut((T, T)) -> T>(
        self,
        other: O,
        op: F,
    ) -> Self;
}

/// TODO: Remove bound on `T`. `MaybeUninit` perhaps?
impl<T: Default, const N: usize> Backend<T> for [T; N] {
    fn new_bck() -> Self {
        std::array::from_fn(|_| T::default())
    }
}

impl<T: Default, const N: usize> BackendOps<T> for [T; N] {
    fn apply_ops<O, F>(self, other: O, op: F) -> Self
    where
        O: Backend<T> + BackendOps<T>,
        F: FnMut((T, T)) -> T,
    {
        self.into_iter()
            .zip(other)
            .map(op)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ())
            .unwrap()
    }
}

impl<T> Backend<T> for Vec<T> {
    fn new_bck() -> Self {
        vec![]
    }
}

impl<T> BackendOps<T> for Vec<T> {
    fn apply_ops<O, F>(self, other: O, op: F) -> Self
    where
        O: Backend<T> + BackendOps<T>,
        F: FnMut((T, T)) -> T,
    {
        self.into_iter().zip(other).map(op).collect()
    }
}
