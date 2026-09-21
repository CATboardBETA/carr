//! Dimension structs. 
//! 
//! These work by implementing [`Dimension`], which has an associated constant
//! describing the final dimensions, given an input. See [`Dimension`] for more information and
//! an example implementation.

/// Implemented by all non-trivial Dimension structs.
///
/// To implement yourself, create a const with a const generic argument of type `&'static [usize]`.
/// This represents the dimensions in. The const should return the output dimensions. Create a
/// fieldless struct with a const generic parameter with type `&'static [usize]`, then implement
/// [`Dimension`] on it.
///
/// Simple example:
/// ```
/// # #![feature(min_adt_const_params, adt_const_params, unsized_const_params, generic_const_items)]
/// # #![expect(incomplete_features)]
/// use carr::dim::Dimension;
///
/// const TRANSPOSE<const D: &'static [usize]>: &[usize] = &[D[1], D[0]];
/// pub struct Transpose<const D: &'static [usize]>;
///
/// impl<const D: &'static [usize]> Dimension for Transpose<D> {
///     const DIMS: &'static [usize] = TRANSPOSE::<D>;
/// }
/// ```
pub trait Dimension {
    /// Output dimensions of the implementor.
    const DIMS: &'static [usize];
}

/// Dimension struct for transposed 2D arrays. Simply flips first and second dimension.
pub struct Transpose<const D: &'static [usize]>;

const TRANSPOSE<const D: &'static [usize]>: &[usize] = &[D[1], D[0]];

impl<const D: &'static [usize]> Dimension for Transpose<D> {
    const DIMS: &'static [usize] = TRANSPOSE::<D>;
}

const LEN<const OF: &'static [usize]>: usize = OF.len();
const ADD<const X: usize, const Y: usize>: usize = X + Y;
const SUB<const X: usize, const Y: usize>: usize = X - Y;
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
const SQUEEZE<const D: &'static [usize]>: &[usize] = &SQUEEZE1::<D>.0[0..SQUEEZE1::<D>.1];

/// Dimension struct for an array that has been squeezed. Simply removes all dimensions that have
/// only one element.
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

/// Dimension struct for unsqueezed arrays. Adds a dimension at `AT`, with one element.
pub struct Unsqueeze<const D: &'static [usize], const AT: usize>;
impl<const D: &'static [usize], const AT: usize> Dimension for Unsqueeze<D, AT> {
    const DIMS: &'static [usize] = &UNSQUEEZE::<D, AT>;
}

const INDEX_ARR1<const D: &'static [usize], const AT: &'static [usize]>: [usize; SUB::<{ LEN::<D> }, { LEN::<AT> }> ] = {
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

pub(crate) const INDEX_ARR<const D: &'static [usize], const AT: &'static [usize]>: &[usize] = &INDEX_ARR1::<D, AT>;
