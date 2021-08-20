use lsp_types::{CodeLens, Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location};
use serde_json::Value;
use crate::frontend::rule::from_ast::GrammarContext;
use std::ops::Range;

pub use code_lens::*;
pub use diagnostic::*;

mod code_lens;
mod diagnostic;
