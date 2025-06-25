//! `dyn_compatible` attribute implementation.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{self, Error};

/// Parse `dyn_compatible` attribute and its item.
pub fn parse_dyn_compatible(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_attr(attr);
    let item = parse_item(item);
    let errs = [attr.as_ref().err(), item.as_ref().err()];
    let errs = errs.iter().filter_map(|x| *x).collect::<Vec<_>>();
    if !errs.is_empty() {
        let mut ret = TokenStream::new();
        ret.extend(errs.iter().map(|&x| x.clone()));
        return ret;
    }

    let is_dyn = attr.unwrap();
    let target = item.unwrap();
    let remake = if is_dyn { dyn_trait } else { not_dyn_trait };
    remake(target)
}

/// Parse attribute.
fn parse_attr(input: TokenStream) -> Result<bool, TokenStream> {
    match syn::parse2::<syn::Lit>(input.clone()) {
        Err(x) => Err(x.into_compile_error()),
        Ok(syn::Lit::Bool(x)) => Ok(x.value),
        _ => {
            let err = Error::new_spanned(input, msg::BOOL_ONLY);
            let err = err.into_compile_error();
            Err(err)
        }
    }
}

/// Parse item.
fn parse_item(input: TokenStream) -> Result<syn::ItemTrait, TokenStream> {
    match syn::parse2::<syn::Item>(input.clone()) {
        Err(_) => Err(input),
        Ok(syn::Item::Trait(x)) => Ok(x),
        _ => {
            let err = Error::new_spanned(input.clone(), msg::TRAIT_ONLY);
            let err = err.into_compile_error();
            Err(err)
        }
    }
}

/// Creates `dyn` compatible trait from raw trait.
fn dyn_trait(item_trait: syn::ItemTrait) -> TokenStream {
    let ident = item_trait.ident.clone();
    let generics = item_trait.generics.clone();
    let generic_args = generic_args(&item_trait.generics);
    quote! {
        #item_trait
        impl #generics dyn #ident #generic_args {}
    }
}

/// Creates not `dyn` compatible trait from raw trait.
fn not_dyn_trait(mut item_trait: syn::ItemTrait) -> TokenStream {
    let not_dyn = quote! {dyn_compatible::NotDyn};
    item_trait
        .supertraits
        .push(syn::TypeParamBound::Verbatim(not_dyn));
    item_trait.into_token_stream()
}

/// Creates generic arguments from generics source.
fn generic_args(generics: &syn::Generics) -> TokenStream {
    let mut ret = TokenStream::new();
    let idents = generics
        .params
        .iter()
        .map(|x| match x {
            syn::GenericParam::Lifetime(x) => x.lifetime.to_token_stream(),
            syn::GenericParam::Type(x) => x.ident.to_token_stream(),
            syn::GenericParam::Const(x) => x.ident.to_token_stream(),
        })
        .collect::<Vec<_>>();

    ret.extend(quote! {<});
    ret.extend(idents.iter().take(1).cloned());
    ret.extend(idents.iter().skip(1).map(|x| quote! {, #x}));
    ret.extend(quote! {>});
    ret
}

/// Messages.
mod msg {
    pub const BOOL_ONLY: &str = "`dyn_compatible` argument must be boolean type";
    pub const TRAIT_ONLY: &str = "`dyn_compatible` attribute can only be used on traits";
}
