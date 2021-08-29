#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

#[cfg(feature = "lsp")]
pub extern crate lsp_types;

pub(crate) mod macros;
pub(crate) mod records;
pub(crate) mod traits;

pub(crate) mod errors;
pub use self::{
    errors::{diagnostic::DiagnosticLevel, Result, YggdrasilError, YggdrasilErrorKind},
    records::{
        builder::ASTBuilder,
        cst_node::CSTNode,
        text_index::{LineColumn, TextChange, TextIndex},
    },
};

#[cfg(feature = "lsp")]
pub use self::records::text_index::lsp::TextAdapter;

pub use url::Url;
