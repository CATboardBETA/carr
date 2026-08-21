use std::ops::Deref;

pub trait Size {
    const SIZE: usize;
}

pub struct S<const N: usize>;

impl<const N: usize> Deref for S<N> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &N
    }
}

impl<const N: usize> Size for S<N> {
    const SIZE: usize = N;
}
