use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

mod error_custom;
mod error_lsp;

pub type Result<T> = std::result::Result<T, Error>;

#[rustfmt::skip]
#[derive(Error, Debug)]
pub enum Error {
    IOError { #[from] source: std::io::Error },
    FormatError { #[from] source: std::fmt::Error },
    // PestError { #[from] source: pest::error::Error<crate::cst::Rule> },
    LanguageError { error: String },
    StructureError {
        error: String,
        start: Option<usize>,
        end: Option<usize>,
    },
    UnexpectedToken {
        error: String,
        start: Option<usize>,
        end: Option<usize>,
    },
    InfoMissing { error: String },
    /// Some nodes failed to resolve and are being rolled back
    Unwinding,
    /// A forbidden cst_node encountered
    Unreachable,
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
}

impl Error {
    pub fn structure_error(msg: impl Into<String>, start: Option<usize>, end: Option<usize>) -> Error {
        Self::StructureError { error: msg.into(), start, end }
    }
    ///
    pub fn unexpected_token(msg: impl Into<String>, start: Option<usize>, end: Option<usize>) -> Error {
        Self::UnexpectedToken { error: msg.into(), start, end }
    }
    ///
    pub fn language_error(msg: impl Into<String>) -> Error {
        Self::LanguageError { error: msg.into() }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
