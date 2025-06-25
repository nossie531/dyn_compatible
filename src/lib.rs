//! Dyn compatible marker.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

pub mod prelude;

/// `dyn_compatible` attribute macro.
///
/// # Examples
///
/// ```
/// # use dyn_compatible::prelude::*;
/// #[dyn_compatible(true)]
/// pub trait MyTrait {
///     fn some_method(&self);
/// }
/// ```
pub use dyn_compatible_macro::dyn_compatible;

pub use not_dyn::*;

mod not_dyn;

#[doc(hidden)]
#[path = "../tests_compile_fail/mod.rs"]
mod tests_compile_fail;
