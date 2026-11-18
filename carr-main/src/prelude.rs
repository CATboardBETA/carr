//! Common types and functions are re-exported here.
//!
//! This module should be glob imported, i.e. `use carr::prelude::*;`

pub use crate::arr::{Arr, index::IndexConstSlice};
pub use crate::backend::{Backend, NumericalValue};
pub use crate::ix;
