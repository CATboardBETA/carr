//! `carr`: ***C***onstant-sized ***Arr***ays
//!
//! # Installation
//! To install the latest version, simply run
//! ```sh
//! cargo add carr
//! ```
//! Or, for manual installation, add `carr="0.3"` to your `Cargo.toml` under the dependencies table.
//! # Usage
//! todo
//! # Examples
//! todo
#![forbid(unsafe_code)]
#![allow(incomplete_features)]
#![feature(
    coverage_attribute,
    const_trait_impl,
    const_index,
    gca_min_const_items,
    gca_const_items,
    gca_macroless_args,
    unsized_const_params,
    adt_const_params,
    inherent_associated_types,
    generic_const_items,
    impl_trait_in_assoc_type
)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
#![warn(missing_docs)]

use crate::arr::Arr;

pub mod arr;
pub mod backend;
pub mod dim;
#[cfg(any(feature = "image", feature = "ndarray"))]
pub mod integrations;
pub mod prelude;
pub mod prop;

/// An owned array. This is just a type alias for an array with [`Vec<T>`] as the backend.
pub type ArrOwn<T, const D: &'static [usize]> = Arr<Vec<T>, T, D>;

/// An immutably borrowed array. This is just a type alias for an array with [`Vec<&T>`] as the
/// backend.
pub type ArrRef<'a, T, const D: &'static [usize]> = Arr<Vec<&'a T>, &'a T, D>;

/// A mutably borrowed array. This is just a type alias for an array with [`Vec<&mut T>`](Vec<T>) as
/// the backend
pub type ArrMut<'a, T, const D: &'static [usize]> = Arr<Vec<&'a mut T>, &'a mut T, D>;
