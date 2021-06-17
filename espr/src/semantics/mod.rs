//! Legalize [SyntaxTree] into [IR]

mod entity;
mod namespace;
mod schema;
mod scope;
mod type_decl;
mod type_ref;

pub use entity::*;
pub use namespace::*;
pub use schema::*;
pub use scope::*;
pub use type_decl::*;
pub use type_ref::*;

use crate::ast::SyntaxTree;
use proc_macro2::TokenStream;
use quote::*;
use std::fmt;
use thiserror::Error;

/// Semantic errors
#[derive(Debug, Error)]
pub enum SemanticError {
    #[error("Type {name} not found in scope {scope}")]
    TypeNotFound { name: String, scope: Scope },
}

/// Legalize partial parsed input into corresponding intermediate representation
pub trait Legalize: Sized {
    type Input;
    fn legalize(
        namespace: &Namespace,
        scope: &Scope,
        syn: &Self::Input,
    ) -> Result<Self, SemanticError>;
}

/// Intermediate Representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IR {
    pub schemas: Vec<Schema>,
}

impl IR {
    pub fn from_syntax_tree(st: &SyntaxTree) -> Result<Self, SemanticError> {
        let ns = Namespace::new(&st)?;
        let ir = Self::legalize(&ns, &Scope::root(), &st)?;
        Ok(ir)
    }
}

impl fmt::Display for IR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#![allow(dead_code)]\n{}",
            self.to_token_stream().to_string()
        )
    }
}

impl Legalize for IR {
    type Input = SyntaxTree;
    fn legalize(ns: &Namespace, scope: &Scope, syn: &SyntaxTree) -> Result<Self, SemanticError> {
        let schemas = syn
            .schemas
            .iter()
            .map(|schema| Schema::legalize(ns, scope, schema))
            .collect::<Result<Vec<Schema>, SemanticError>>()?;
        Ok(IR { schemas })
    }
}

impl ToTokens for IR {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let schemas = &self.schemas;
        tokens.append_all(quote! {
            #(#schemas)*
        })
    }
}
