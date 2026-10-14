//! Compile-time evaluation of constant assertions.
//!
//! See [`Prop`] for an example.

/// Used to establish the truthiness of a condition at compile time.
///
/// Example:
/// ```
/// # #![expect(incomplete_features)]
/// # #![feature(generic_const_items, gca_min_const_items, gca_const_items, gca_macroless_args)]
/// use carr::prop::{Prop, PropTrue};
///
/// const IS_EVEN<const X: usize>: bool = (X % 2) == 0;
/// fn foo<const X: usize>()
/// where
///     Prop<{ IS_EVEN::<X> }>: PropTrue
/// {
///     // ...
/// }
/// ```
pub struct Prop<const COND: bool>;

/// Implemented for [`Prop<true>`]. Should not be implemented for any other types, or outside of
/// this crate.
pub trait PropTrue {}

impl PropTrue for Prop<true> {}

/// Implemented for [`Prop<false>`]. Should not be implemented for any other types, or outside of
/// this crate.
pub trait PropFalse {}

impl PropFalse for Prop<false> {}
