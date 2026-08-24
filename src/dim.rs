pub trait Dimension {
    const DIMS: &'static [usize];
}

pub struct Transpose<const D: &'static [usize]>;

const TRANSPOSE<const D: &'static [usize]>: &[usize] = &[D[1], D[0]];

impl<const D: &'static [usize]> Dimension for Transpose<D> {
    const DIMS: &'static [usize] = TRANSPOSE::<D>;
}
