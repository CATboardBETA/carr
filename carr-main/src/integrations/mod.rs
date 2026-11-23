//! Holds a submodule for each integration
//!
//! Current integrations: [image](https://lib.rs/crates/image)

/// Errors specific to integrations, that can't occur anywhere else
// TODO: Add a broader error enum, used anywhere fallible
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum IntegrationError {
    /// Dimensional mismatch between input array-like structure and the target dimensions const
    /// generic
    DimMismatch,
    /// An empty array would be outputted from the conversion. (an invalid state for
    /// [`Arr`](crate::prelude::Arr)).
    EmptyArr,
    /// When a dimension overflows a type that it must be converted to for the integration.
    DimOverflow,
}

#[cfg(feature = "image")]
pub mod image;
