pub trait Dimension {
    const DIMS: &'static [usize];
}

pub struct Transpose<const D: &'static [usize]>;

const TRANSPOSE<const D: &'static [usize]>: &[usize] = &[D[1], D[0]];

impl<const D: &'static [usize]> Dimension for Transpose<D> {
    const DIMS: &'static [usize] = TRANSPOSE::<D>;
}

const LEN<const OF: &'static [usize]>: usize = OF.len();
const ADD<const X: usize, const Y: isize>: usize = (X.cast_signed() + Y).cast_unsigned();
const DUMMY<const D: &'static [usize]>: [usize; LEN::<D>] = [0usize; LEN::<D>];
const SQUEEZE1<const D: &'static [usize]>: ([usize; LEN::<D>], usize) = {
    let mut out = DUMMY::<D>;
    let mut i = 0;
    let mut j = 0;
    let mut since_last = 0;
    while i < LEN::<D> {
        if D[i] != 1 {
            since_last += 1;
            out[j] = D[i];
            j += 1;
        }
        i += 1;
    }
    (out, since_last)
};
const SQUEEZE<const D: &'static [usize]>: &[usize] = &SQUEEZE1::<D>.0[0..SQUEEZE1::<D>.1];

pub struct Squeeze<const D: &'static [usize]>;
impl<const D: &'static [usize]> Dimension for Squeeze<D> {
    const DIMS: &'static [usize] = SQUEEZE::<D>;
}

const UNSQUEEZE<const D: &'static [usize], const AT: usize>: [usize; ADD::<{ LEN::<D> }, 1>] = {
    assert!(D.len() + 1 > AT);
    let mut out = [0usize; ADD::<{ LEN::<D> }, 1>];
    let mut i = 0;
    let mut j = 0;
    while i < LEN::<D> + 1{
        if i == AT {
            out[i] = 1;

        } else {
            out[i] = D[j];
            j += 1;
        }
        i += 1;
    }
out
};

pub struct Unsqueeze<const D: &'static [usize], const AT: usize>;
impl<const D: &'static [usize], const AT: usize> Dimension for Unsqueeze<D, AT> {
    const DIMS: &'static [usize] = &UNSQUEEZE::<D, AT>;
}

pub(crate) const INDEX_ARR<const D: &'static [usize], const AT: &'static [usize]>: &[usize] = {
    &D[AT.len()..]
};
