//! Legalize [SyntaxTree] into [IR]

pub mod entity;
pub mod namespace;
pub mod schema;
pub mod scope;
pub mod types;

pub use namespace::Namespace;
pub use schema::Schema;
pub use scope::Scope;

use crate::parser::SyntaxTree;
use proc_macro2::TokenStream;
use quote::*;

/// Semantic errors
#[derive(Debug)]
pub enum SemanticError {
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

impl Legalize for IR {
    type Input = SyntaxTree;
    fn legalize(_ns: &Namespace, _scope: &Scope, _syn: &SyntaxTree) -> Result<Self, SemanticError> {
        todo!()
    }
}

impl ToTokens for IR {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        todo!()
    }
}
