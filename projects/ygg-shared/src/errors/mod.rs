use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;
use url::Url;
use std::ops::Range;

mod error_custom;
mod error_lsp;

pub type Result<T> = std::result::Result<T, YggdrasilError>;

pub struct YggdrasilError {
    kind: YggdrasilErrorKind,
    file: Option<Url>,
    range: Option<Range<usize>>
}

pub enum YggdrasilErrorKind {
    IOError(std::io::Error),
    FormatError(std::fmt::Error),
    // PestError { #[from] source: pest::error::Error<crate::cst::Rule> },
    LanguageError(String),
    StructureError(String),
    UnexpectedToken(String),
    InfoMissing(String),
    /// Some nodes failed to resolve and are being rolled back
    Unwinding,
    /// A forbidden cst_node encountered
    Unreachable,
    UnknownError(anyhow::Error),
}

impl YggdrasilError {
    pub fn structure_error(msg: impl Into<String>, start: Option<usize>, end: Option<usize>) -> YggdrasilError {
        Self::StructureError { error: msg.into(), start, end }
    }
    ///
    pub fn unexpected_token(msg: impl Into<String>, start: Option<usize>, end: Option<usize>) -> YggdrasilError {
        Self::UnexpectedToken { error: msg.into(), start, end }
    }
    ///
    pub fn language_error(msg: impl Into<String>) -> YggdrasilError {
        Self::LanguageError { error: msg.into() }
    }
}

impl Display for YggdrasilError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
