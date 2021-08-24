//! Procedural macros for ruststep
//!
//! ```
//! use ruststep_derive::{as_holder, Holder};
//! use std::collections::HashMap;
//!
//! pub struct Table {
//!     a: HashMap<u64, as_holder!(A)>,
//!     b: HashMap<u64, as_holder!(B)>,
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Holder)]
//! #[holder(table = Table)]
//! #[holder(field = a)]
//! pub struct A {
//!     pub x: f64,
//!     pub y: f64,
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Holder)]
//! #[holder(table = Table)]
//! #[holder(field = b)]
//! pub struct B {
//!     pub z: f64,
//!     #[holder(use_place_holder)]
//!     pub a: A,
//! }
//! ```
//!
//! `#[derive(Holder)]` generates followings:
//!
//! - `AHolder` struct
//!   - naming rule is `{}Holder`
//!   - This name is obtained by `as_holder!(A)`
//! - `impl Holder for AHolder`
//!

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use std::convert::*;

mod entity;
mod field_type;
mod holder_attr;
mod select;

use field_type::*;
use holder_attr::*;

/// Generate `impl Deserialize` for entity structs
#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize_entry(input: TokenStream) -> TokenStream {
    derive_deserialize(&syn::parse(input).unwrap()).into()
}

fn derive_deserialize(ast: &syn::DeriveInput) -> TokenStream2 {
    let ident = &ast.ident;
    match &ast.data {
        syn::Data::Struct(st) => entity::derive_deserialize(ident, st),
        syn::Data::Enum(e) => select::derive_deserialize(ident, e),
        _ => unimplemented!("Only struct is supprted currently"),
    }
}

#[proc_macro_derive(Holder, attributes(holder))]
pub fn derive_holder_entry(input: TokenStream) -> TokenStream {
    derive_holder(&syn::parse(input).unwrap()).into()
}

fn derive_holder(ast: &syn::DeriveInput) -> TokenStream2 {
    let table_attr = HolderAttr::parse(&ast.attrs);
    let ident = &ast.ident;
    match &ast.data {
        syn::Data::Struct(st) => entity::derive_holder(ident, st, &table_attr),
        syn::Data::Enum(e) => select::derive_holder(ident, e),
        _ => unimplemented!("Only struct is supprted currently"),
    }
}

/// Resolve Holder struct from owned type, e.g. `A` to `AHolder`
#[proc_macro]
pub fn as_holder(input: TokenStream) -> TokenStream {
    let path = as_holder_path(&syn::parse(input).unwrap());
    let ts = quote! { #path };
    ts.into()
}

fn as_holder_ident(input: &syn::Ident) -> syn::Ident {
    format_ident!("{}Holder", input)
}

fn as_holder_path(input: &syn::Type) -> syn::Type {
    let ft: FieldType = input
        .clone()
        .try_into()
        .expect("as_holder! only accepts espr-generated type");
    ft.as_holder().into()
}

fn as_visitor_ident(input: &syn::Ident) -> syn::Ident {
    format_ident!("{}Visitor", input)
}

/// Returns `crate` or `::ruststep` as in ruststep crate or not
fn ruststep_crate() -> syn::Path {
    let path = crate_name("ruststep").unwrap();
    match path {
        FoundCrate::Itself => match std::env::var("CARGO_TARGET_TMPDIR") {
            Ok(_) => {
                // For tests and benches in ruststep crate
                //
                // https://doc.rust-lang.org/cargo/reference/environment-variables.html
                // > CARGO_TARGET_TMPDIR — Only set when building integration test or benchmark code.
                let mut segments = syn::punctuated::Punctuated::new();
                segments.push(syn::PathSegment {
                    ident: syn::Ident::new("ruststep", Span::call_site()),
                    arguments: syn::PathArguments::None,
                });
                syn::Path {
                    leading_colon: Some(syn::token::Colon2::default()),
                    segments,
                }
            }
            Err(_) => {
                let mut segments = syn::punctuated::Punctuated::new();
                segments.push(syn::PathSegment {
                    ident: syn::Ident::new("crate", Span::call_site()),
                    arguments: syn::PathArguments::None,
                });
                syn::Path {
                    leading_colon: None,
                    segments,
                }
            }
        },
        FoundCrate::Name(name) => {
            let mut segments = syn::punctuated::Punctuated::new();
            segments.push(syn::PathSegment {
                ident: syn::Ident::new(&name, Span::call_site()),
                arguments: syn::PathArguments::None,
            });
            syn::Path {
                leading_colon: Some(syn::token::Colon2::default()),
                segments,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holder_path() {
        let path = syn::parse_str("::some::Struct").unwrap();
        let holder = as_holder_path(&path);
        let ans = syn::parse_str("::some::StructHolder").unwrap();
        assert_eq!(holder, ans);
    }

    #[test]
    fn optional_holder_path() {
        let path = syn::parse_str("Option<::some::Struct>").unwrap();
        let holder = as_holder_path(&path);
        let ans = syn::parse_str("Option<::some::StructHolder>").unwrap();
        assert_eq!(holder, ans);
    }
}
