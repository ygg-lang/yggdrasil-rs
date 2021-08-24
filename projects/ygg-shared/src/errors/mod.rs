use self::YggdrasilErrorKind::*;
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    ops::Range,
};
use url::Url;

pub(crate) mod error_3rd;
pub(crate) mod error_lsp;
pub(crate) mod error_std;
pub(crate) mod diagnostic;

pub type Result<T> = std::result::Result<T, YggdrasilError>;

#[derive(Debug)]
pub struct YggdrasilError {
    pub kind: YggdrasilErrorKind,
    pub file: Option<Url>,
    pub range: Option<Range<usize>>,
}

#[derive(Debug)]
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
    pub fn set_url(mut self, url: Url) -> Self {
        self.file = Some(url);
        return self;
    }
    pub fn set_path() {}
    pub fn set_range(mut self, range: Range<usize>) -> Self {
        self.range = Some(range);
        return self;
    }
    #[inline]
    pub fn get_kind(&self) -> &YggdrasilErrorKind {
        &self.kind
    }
    #[inline]
    pub fn is_unwinding(&self) -> bool {
        matches!(self.kind, Unwinding)
    }
}

impl YggdrasilError {
    #[inline]
    pub fn structure_error(msg: impl Into<String>) -> Self {
        Self { kind: StructureError(msg.into()), file: None, range: None }
    }
    ///
    #[inline]
    pub fn unexpected_token(msg: impl Into<String>) -> Self {
        Self { kind: UnexpectedToken(msg.into()), file: None, range: None }
    }
    ///
    #[inline]
    pub fn language_error(msg: impl Into<String>) -> Self {
        Self { kind: LanguageError(msg.into()), file: None, range: None }
    }
    #[inline]
    pub fn unreachable() -> Self {
        Self { kind: Unreachable, file: None, range: None }
    }
    #[inline]
    pub fn unwinding() -> Self {
        Self { kind: Unwinding, file: None, range: None }
    }
}

impl Error for YggdrasilError {}

impl Display for YggdrasilError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let path = match &self.file {
            Some(s) => s.path(),
            None => "<Anonymous>",
        };
        match &self.range {
            Some(s) => {
                writeln!(f, "at ({}, {}) of {}", s.start, s.end, path)?;
            }
            None => {
                writeln!(f, "at {}", path)?;
            }
        }
        write!(f, "{:indent$}{}", self.kind, indent = 4)
    }
}

impl Display for YggdrasilErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IOError(e) => f.write_str(&e.to_string()),
            FormatError(e) => f.write_str(&e.to_string()),
            LanguageError(e) => f.write_str(e),
            StructureError(e) => f.write_str(e),
            UnexpectedToken(e) => f.write_str(e),
            InfoMissing(e) => f.write_str(e),
            Unwinding => {
                unimplemented!()
            }
            Unreachable => {
                unimplemented!()
            }
            UnknownError(e) => f.write_str(&e.to_string()),
        }
    }
}
