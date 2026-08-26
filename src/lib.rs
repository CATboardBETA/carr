#![forbid(unsafe_code)]
#![allow(incomplete_features)]
#![feature(
    const_trait_impl,
    const_index,
    min_generic_const_args,
    generic_const_args,
    macroless_generic_const_args,
    unsized_const_params,
    adt_const_params,
    inherent_associated_types,
    generic_const_items
)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

pub mod arr;
pub mod backend;
pub mod dim;
pub mod prelude;
