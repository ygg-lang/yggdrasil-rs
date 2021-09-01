#![allow(clippy::needless_return)]
#![forbid(missing_copy_implementations)]
#![forbid(missing_debug_implementations)]
// #![forbid(missing_docs)]
#![doc = include_str!("../Readme.md")]

#[cfg(feature = "lsp-types")]
pub extern crate lsp_types;

pub(crate) mod errors;
pub(crate) mod macros;
pub(crate) mod records;
pub(crate) mod traits;

pub use self::{
    errors::{diagnostic::DiagnosticLevel, Result, YggdrasilError, YggdrasilErrorKind},
    records::{
        builder::ASTBuilder,
        cst_node::CSTNode,
        text_index::{line_column::TextAdaptor, Position, TextChange, TextIndex},
    },
    traits::{
        ast_node::ASTNode,
        pratt::{Affix, Associativity, PrattParser, Precedence},
    },
};

#[cfg(feature = "lsp-types")]
pub use {self::records::text_index::lsp::LspTextAdaptor, lsp_types::Position as LspPosition, lsp_types::Range as LspRange};

pub use url::Url;
