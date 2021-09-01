use self::YggdrasilErrorKind::*;
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    ops::Range,
    path::Path,
};
use url::Url;

pub(crate) mod diagnostic;
pub(crate) mod error_3rd;
#[cfg(feature = "lsp-types")]
pub(crate) mod error_lsp;
pub(crate) mod error_std;

/// Result types for all errors about yggdrasil framework
pub type Result<T> = std::result::Result<T, YggdrasilError>;

///
#[derive(Debug)]
pub struct YggdrasilError {
    pub kind: YggdrasilErrorKind,
    pub file: Option<Url>,
    pub range: Option<Range<usize>>,
}

///
#[derive(Debug)]
pub enum YggdrasilErrorKind {
    /// Wrapper of [`std::io::Error`]
    IOError(std::io::Error),
    /// Wrapper of [`std::fmt::Error`]
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
    /// Setter of [`YggdrasilError.file`]
    pub fn set_url(mut self, url: Url) -> Self {
        self.file = Some(url);
        return self;
    }
    /// Setter of [`YggdrasilError.file`]
    #[cfg(any(unix, windows, target_os = "redox"))]
    pub fn set_path(self, path: &Path) -> Self {
        match Url::from_file_path(path) {
            Ok(o) => self.set_url(o),
            Err(_) => self,
        }
    }
    /// Setter of [`YggdrasilError.range`]
    pub fn set_range(mut self, range: Range<usize>) -> Self {
        self.range = Some(range);
        return self;
    }
    /// Setter of [`YggdrasilError.range`]
    pub fn set_range_pair(mut self, start: usize, end: usize) -> Self {
        self.range = Some(Range { start, end });
        return self;
    }
    /// Checker of [`YggdrasilErrorKind::Unwinding`]
    #[inline]
    pub fn is_unwinding(&self) -> bool {
        matches!(self.kind, Unwinding)
    }
}

impl YggdrasilError {
    /// Constructor of [`YggdrasilErrorKind::StructureError`]
    #[inline]
    pub fn structure_error(msg: impl Into<String>) -> Self {
        Self { kind: StructureError(msg.into()), file: None, range: None }
    }
    /// Constructor of [`YggdrasilErrorKind::UnexpectedToken`]
    #[inline]
    pub fn unexpected_token(msg: impl Into<String>) -> Self {
        Self { kind: UnexpectedToken(msg.into()), file: None, range: None }
    }
    /// Constructor of [`YggdrasilErrorKind::LanguageError`]
    #[inline]
    pub fn language_error(msg: impl Into<String>) -> Self {
        Self { kind: LanguageError(msg.into()), file: None, range: None }
    }
    /// Constructor of [`YggdrasilErrorKind::Unreachable`]
    #[inline]
    pub fn unreachable() -> Self {
        Self { kind: Unreachable, file: None, range: None }
    }
    /// Constructor of [`YggdrasilErrorKind::Unwinding`]
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
