//! Operations applied to arrays. These are implemented using traits in [`std::ops`] whenever
//! possible.
//!
//! All actual implementations are in private submodules.

mod bin_ops;
mod connect_ops;
mod shape_ops;
mod view_ops;
