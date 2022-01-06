#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

pub(crate) mod macros;
pub(crate) mod records;
pub(crate) mod traits;

pub(crate) mod errors;
pub use self::{
    errors::{diagnostic::DiagnosticLevel, Result, YggdrasilError, YggdrasilErrorKind},
    records::{builder::ASTBuilder, cst_node::CSTNode, text_index::*, text_store::TextStore},
};
#[cfg(feature = "lsp")]
pub use lsp_types;
pub use url::Url;
