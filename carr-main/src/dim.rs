//! Dimension constants.
//!
//! These work by implementing a constant for each modification to dimensions
//! which creates a new static slice at compile time using const expressions.
//! Look at [`TRANSPOSE`] for an example on how this is implemented.

/// Dimension constant for transposed 2D arrays. Simply flips first and second dimension.
pub const TRANSPOSE<const D: &'static [usize]>: &[usize] = &[D[1], D[0]];

const LEN<const OF: &'static [usize]>: usize = OF.len();
const ADD<const X: usize, const Y: usize>: usize = X + Y;
const SUB<const X: usize, const Y: usize>: usize = X - Y;
pub(crate) const PROD<const A: usize, const B: usize>: usize = A * B;

pub(crate) const TOTAL_DIM<const D: &'static [usize]>: usize = {
    let mut i = 0;
    let mut total = 1;
    while i < D.len() {
        total *= D[i];
        i += 1;
    }
    total
};
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

/// Dimension constant for an array that has been squeezed. Simply removes all dimensions that have
/// only one element.
pub const SQUEEZE<const D: &'static [usize]>: &[usize] = &SQUEEZE1::<D>.0[0..SQUEEZE1::<D>.1];

const UNSQUEEZE1<const D: &'static [usize], const AT: usize>: [usize; ADD::<{ LEN::<D> }, 1>] = {
    assert!(D.len() + 1 > AT);
    let mut out = [0usize; ADD::<{ LEN::<D> }, 1>];
    let mut i = 0;
    let mut j = 0;
    while i < LEN::<D> + 1 {
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

/// Dimension constant for unsqueezed arrays. Adds a dimension at `AT`, with one element.
pub const UNSQUEEZE<const D: &'static [usize], const AT: usize>: &[usize] =
    &UNSQUEEZE1::<D, AT>;

const INDEX_ARR1<const D: &'static [usize], const AT: &'static [usize]>: [usize;
    SUB::<{ LEN::<D> }, { LEN::<AT> }>] = {
    if AT.is_empty() {
        *D.as_array().unwrap()
    } else {
        let mut out = [1usize; SUB::<{ LEN::<D> }, { LEN::<AT> }>];
        let mut i = 0;
        while i < (D.len() - AT.len()) {
            out[i] = D[D.len() - 1 - i];
            i += 1;
        }
        out
    }
};

pub(crate) const INDEX_ARR<const D: &'static [usize], const AT: &'static [usize]>: &[usize] =
    &INDEX_ARR1::<D, AT>;

const REMOVE_DIM1<const D: &'static [usize], const AT: usize>: ([usize; SUB::<{ LEN::<D> }, 1>], usize) = {
    let mut new_d = [0; SUB::<{ LEN::<D> }, 1>];
    let mut i = 0;
    let mut j = 0;
    let mut removed_d = 0;
    while i < D.len() {
        if i == AT {
            removed_d = D[j];
        } else {
            new_d[i] = D[j];
            j += 1;
        }
        i += 1;
    }
    (new_d, removed_d)
};

pub const REMOVE_DIM<const D: &'static [usize], const AT: usize>: &[usize] = &REMOVE_DIM1::<D, AT>.0;
pub const REMOVE_DIM_EXTRA<const D: &'static [usize], const AT: usize>: usize = REMOVE_DIM1::<D, AT>.1;
