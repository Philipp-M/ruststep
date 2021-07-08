//! proc-macro for ruststep
//!
//! ```
//! pub struct Table; // moc table struct
//!
//! #[derive(Debug, Clone, PartialEq, ruststep_derive::Holder)]
//! #[holder(table = Table, field = a)]
//! pub struct A {
//!     pub x: f64,
//!     pub y: f64,
//! }
//! ```
//!
//! `#[derive(Holder)]` generates followings:
//!
//! - `AHolder` struct
//!   - naming rule is `{}Holder`
//! - `impl Holder for AHolder`
//!   - `#[holder(table = ...)]` is consumed here
//! - `impl Deserialize for AHolder`
//! - `AHolderVisitor` struct for implementing `Deserialize`
//!   - naming rule is `{}HolderVisitor`
//!   - This struct is usually generated by [serde::Deserialize] proc-macro,
//!     but their definition does not match for our cases.
//!
//! ```
//! pub struct Table; // moc table struct
//!
//! impl ::ruststep::tables::EntityTable<AHolder> for Table {
//!     fn get_owned(&self, entity_id: u64) -> ::ruststep::error::Result<A> {
//!         todo!()
//!     }
//!     fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = ::ruststep::error::Result<A>> + 'table> {
//!         todo!()
//!     }
//! }
//!
//! impl ::ruststep::tables::EntityTable<BHolder> for Table {
//!     fn get_owned(&self, entity_id: u64) -> ::ruststep::error::Result<B> {
//!         todo!()
//!     }
//!     fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = ::ruststep::error::Result<B>> + 'table> {
//!         todo!()
//!     }
//! }
//!
//! #[derive(Debug, Clone, PartialEq, ruststep_derive::Holder)]
//! #[holder(table = Table, field = a)]
//! pub struct A {
//!     pub x: f64,
//!     pub y: f64,
//! }
//!
//! #[derive(Debug, Clone, PartialEq, ruststep_derive::Holder)]
//! #[holder(table = Table, field = b)]
//! pub struct B {
//!     pub z: f64,
//!     #[holder(use_place_holder)]
//!     pub a: A,
//! }
//! ```

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

mod for_struct;
mod holder_attr;
use holder_attr::*;

#[proc_macro_derive(Holder, attributes(holder))]
pub fn derive_holder_entry(input: TokenStream) -> TokenStream {
    derive_holder(&syn::parse(input).unwrap()).into()
}

fn derive_holder(ast: &syn::DeriveInput) -> TokenStream2 {
    let TableAttr { table_name, .. } = parse_table_attr(ast);
    let ident = &ast.ident;
    match &ast.data {
        syn::Data::Struct(st) => {
            let def_holder_tt = for_struct::def_holder(ident, st);
            let def_visitor_tt = for_struct::def_visitor(ident, st);
            let impl_deserialize_tt = for_struct::impl_deserialize(ident);
            let impl_holder_tt = for_struct::impl_holder(ident, &table_name, st);
            quote! {
                #def_holder_tt
                #def_visitor_tt
                #impl_deserialize_tt
                #impl_holder_tt
            }
        }
        _ => unimplemented!("Only struct is supprted currently"),
    }
}
