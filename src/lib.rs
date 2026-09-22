#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![allow(incomplete_features)]
#![feature(
    coverage_attribute,
    const_trait_impl,
    const_index,
    min_generic_const_args,
    generic_const_args,
    macroless_generic_const_args,
    unsized_const_params,
    adt_const_params,
    inherent_associated_types,
    generic_const_items,
    vec_from_fn
)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
#![warn(missing_docs)]

pub mod arr;
pub mod backend;
pub mod dim;
pub mod prelude;
