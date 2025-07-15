//! Procedual macros for crate `dyn_compatible`.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]

use dyn_compatible_impl::parse_dyn_compatible;
use proc_macro as pm;
use proc_macro2::TokenStream;

mod dyn_compatible_impl;

/// `dyn_compatible` assertion attribute.
///
/// Adding this attribute to trait confirms dyn-compatibility.
/// If trait violates this specification, compile error will occur.
///
/// # Examples
///
/// ```
/// # use dyn_compatible_macro::dyn_compatible;
/// #[dyn_compatible(true)]
/// pub trait MyTrait {
///     fn some_method(&self);
/// }
/// ```
#[proc_macro_attribute]
pub fn dyn_compatible(attr: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    let attr = TokenStream::from(attr);
    let item = TokenStream::from(item);
    let ret = parse_dyn_compatible(attr, item);
    pm::TokenStream::from(ret)
}
