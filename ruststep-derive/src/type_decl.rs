use inflector::Inflector;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use std::convert::*;

use super::*;

pub fn derive_holder(ident: &syn::Ident, st: &syn::DataStruct, attr: &HolderAttr) -> TokenStream2 {
    //let name = ident.to_string().to_screaming_snake_case();
    //let holder_ident = as_holder_ident(ident);
    let def_holder_tt = def_holder(ident, st);
    let impl_holder_tt = impl_holder(ident, attr, st);
    let impl_entity_table_tt = impl_entity_table(ident, attr);
    quote! {
        #def_holder_tt
        #impl_holder_tt
        #impl_entity_table_tt
    }
}

pub fn def_holder(ident: &syn::Ident, st: &syn::DataStruct) -> TokenStream2 {
    let holder_ident = as_holder_ident(ident);
    let FieldEntries { holder_types, .. } = FieldEntries::parse(st);
    quote! {
        /// Auto-generated by `#[derive(Holder)]`
        #[derive(Debug, Clone, PartialEq)]
        pub struct #holder_ident(#(#holder_types),*);
    }
}

pub fn impl_holder(ident: &syn::Ident, table: &HolderAttr, st: &syn::DataStruct) -> TokenStream2 {
    let name = ident.to_string().to_screaming_snake_case();
    let holder_ident = as_holder_ident(ident);
    let FieldEntries {
        holder_types,
        into_owned,
    } = FieldEntries::parse(st);
    let HolderAttr { table, .. } = table;
    let tuple_len = holder_types.len();
    let table_arg = table_arg();
    let ruststep = ruststep_crate();

    quote! {
        #[automatically_derived]
        impl #ruststep::tables::Holder for #holder_ident {
            type Table = #table;
            type Owned = #ident;
            fn into_owned(self, #table_arg: &Self::Table) -> #ruststep::error::Result<Self::Owned> {
                Ok(#ident ( #(#into_owned),* ))
            }
            fn name() -> &'static str {
                #name
            }
            fn attr_len() -> usize {
                #tuple_len
            }
        }
    } // quote!
}

pub fn impl_entity_table(ident: &syn::Ident, table: &HolderAttr) -> TokenStream2 {
    let HolderAttr { table, field, .. } = table;
    let holder_ident = as_holder_ident(ident);
    let ruststep = ruststep_crate();

    quote! {
        #[automatically_derived]
        impl #ruststep::tables::EntityTable<#holder_ident> for #table {
            fn get_owned(&self, entity_id: u64) -> #ruststep::error::Result<#ident> {
                #ruststep::tables::get_owned(self, &self.#field, entity_id)
            }
            fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = #ruststep::error::Result<#ident>> + 'table> {
                #ruststep::tables::owned_iter(self, &self.#field)
            }
        }
    }
}

struct FieldEntries {
    holder_types: Vec<syn::Type>,
    into_owned: Vec<TokenStream2>,
}

impl FieldEntries {
    fn parse(st: &syn::DataStruct) -> Self {
        let table_arg = table_arg();

        let mut holder_types = Vec::new();
        let mut into_owned = Vec::new();

        for (i, field) in st.fields.iter().enumerate() {
            let ft: FieldType = field.ty.clone().try_into().unwrap();
            let index = syn::Index {
                index: i as u32,
                span: Span::call_site(),
            };

            let HolderAttr { place_holder, .. } = HolderAttr::parse(&field.attrs);
            if place_holder {
                match &ft {
                    FieldType::Path(_) => {
                        into_owned.push(quote! { self.#index.into_owned(#table_arg)? });
                    }
                    FieldType::Optional(_) => {
                        into_owned.push(
							quote! { self.#index.map(|holder| holder.into_owned(#table_arg)).transpose()? },
						);
                    }
                    FieldType::List(_) => into_owned.push(quote! {
                        self.#i
                            .into_iter()
                            .map(|v| v.into_owned(#table_arg))
                            .collect::<::std::result::Result<Vec<_>, _>>()?
                    }),
                    FieldType::Boxed(_) => abort_call_site!("Unexpected Box<T>"),
                }
                holder_types.push(ft.as_holder().as_place_holder().into());
            } else {
                into_owned.push(quote! { self.#index });
                holder_types.push(ft.into());
            }
        }
        FieldEntries {
            holder_types,
            into_owned,
        }
    }
}

/// This must be same between codegens
fn table_arg() -> syn::Ident {
    syn::Ident::new("table", Span::call_site())
}
