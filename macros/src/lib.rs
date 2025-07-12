//! Procedual macros for crate `dyn_compatible`.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]

use dyn_compatible_impl::parse_dyn_compatible;
use proc_macro as pm;
use proc_macro2::TokenStream;

mod dyn_compatible_impl;

#[allow(missing_docs)]
#[proc_macro_attribute]
pub fn dyn_compatible(attr: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    let attr = TokenStream::from(attr);
    let item = TokenStream::from(item);
    let ret = parse_dyn_compatible(attr, item);
    pm::TokenStream::from(ret)
}
