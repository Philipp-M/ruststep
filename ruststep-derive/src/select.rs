use super::*;
use inflector::Inflector;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

struct Input {
    name: String,
    ident: syn::Ident,
    holder_ident: syn::Ident,
    holder_visitor_ident: syn::Ident,
    variants: Vec<syn::Ident>,
    variant_names: Vec<String>,
    holder_types: Vec<syn::Type>,
}

impl Input {
    fn parse(ident: &syn::Ident, e: &syn::DataEnum) -> Self {
        let name = ident.to_string().to_screaming_snake_case();
        let holder_ident = as_holder_ident(ident);
        let holder_visitor_ident = as_visitor_ident(&holder_ident);
        let variants: Vec<syn::Ident> = e
            .variants
            .iter()
            .map(|var| var.ident.clone().into())
            .collect();
        let variant_names: Vec<_> = variants
            .iter()
            .map(|id| id.to_string().to_screaming_snake_case())
            .collect();
        let ty: Vec<&syn::Type> = e
            .variants
            .iter()
            .map(|var| {
                assert_eq!(var.fields.len(), 1);
                var.fields.iter().map(|f| decompose_box_ty(&f.ty))
            })
            .flatten()
            .collect();
        let holder_types: Vec<_> = ty.iter().map(|t| as_holder_path(t)).collect();

        Input {
            name,
            ident: ident.clone().into(),
            holder_ident,
            holder_visitor_ident,
            variants,
            variant_names,
            holder_types,
        }
    }

    fn def_holder(&self) -> TokenStream2 {
        let Input {
            holder_ident,
            variants,
            holder_types,
            ..
        } = self;
        quote! {
            #[derive(Clone, Debug, PartialEq)]
            enum #holder_ident {
                #(#variants(Box<#holder_types>)),*
            }
        } // quote!
    }

    fn impl_holder(&self) -> TokenStream2 {
        let Input {
            name,
            ident,
            holder_ident,
            variants,
            ..
        } = self;
        quote! {
            impl Holder for #holder_ident {
                type Owned = #ident;
                type Table = Table;
                fn into_owned(self, table: &Table) -> Result<Self::Owned> {
                    Ok(match self {
                        #(#holder_ident::#variants(sub) => #ident::#variants(Box::new(sub.into_owned(table)?))),*
                    })
                }
                fn name() -> &'static str {
                    #name
                }
                fn attr_len() -> usize {
                    0
                }
            }
        } // quote!
    }

    fn impl_deserialize(&self) -> TokenStream2 {
        let Input {
            name,
            holder_ident,
            holder_visitor_ident,
            ..
        } = self;
        quote! {
            impl<'de> de::Deserialize<'de> for #holder_ident {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: de::Deserializer<'de>,
                {
                    deserializer.deserialize_tuple_struct(#name, 0, #holder_visitor_ident {})
                }
            }
        } // quote!
    }

    fn def_visitor(&self) -> TokenStream2 {
        let Input {
            holder_ident,
            holder_types,
            holder_visitor_ident,
            name,
            variants,
            variant_names,
            ..
        } = self;
        quote! {
            struct #holder_visitor_ident;

            impl<'de> ::serde::de::Visitor<'de> for #holder_visitor_ident {
                type Value = #holder_ident;
                fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(formatter, #name)
                }

                // Entry point for Record or Parameter::Typed
                fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
                where
                    A: ::serde::de::MapAccess<'de>,
                {
                    let key: String = map
                        .next_key()?
                        .expect("Empty map cannot be accepted as ruststep Holder"); // this must be a bug, not runtime error
                    match key.as_str() {
                        #(
                        #variant_names => {
                            let value: #holder_types = map.next_value()?;
                            return Ok(#holder_ident::#variants(Box::new(value)));
                        }
                        )*
                        _ => {
                            use ::serde::de::{Error, Unexpected};
                            return Err(A::Error::invalid_value(Unexpected::Other(&key), &self));
                        }
                    }
                }
            }

            impl WithVisitor for #holder_ident {
                type Visitor = #holder_visitor_ident;
                fn visitor_new() -> Self::Visitor {
                    #holder_visitor_ident {}
                }
            }
        } // quote!
    }
}

fn decompose_box_ty(ty: &syn::Type) -> &syn::Type {
    if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
        assert_eq!(path.segments.len(), 1);
        let box_path = &path.segments[0];
        if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
            args,
            ..
        }) = &box_path.arguments
        {
            assert_eq!(args.len(), 1);
            if let syn::GenericArgument::Type(ty) = &args[0] {
                return ty;
            }
        }
    }
    unreachable!("Not Box<T>")
}

pub fn derive_holder(ident: &syn::Ident, e: &syn::DataEnum) -> TokenStream2 {
    let input = Input::parse(ident, e);
    let def_holder_tt = input.def_holder();
    let impl_holder_tt = input.impl_holder();
    let impl_deserialize_tt = input.impl_deserialize();
    let def_visitor_tt = input.def_visitor();

    quote! {
        #def_holder_tt
        #impl_holder_tt
        #impl_deserialize_tt
        #def_visitor_tt
    } // quote!
}

pub fn derive_deserialize(_ident: &syn::Ident, _e: &syn::DataEnum) -> TokenStream2 {
    unimplemented!()
}
