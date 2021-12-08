use super::Error;
use std::sync::PoisonError;

impl From<std::str::Utf8Error> for YggdrasilError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::LanguageError { error: e.to_string() }
    }
}

impl From<std::num::ParseIntError> for YggdrasilError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::LanguageError { error: e.to_string() }
    }
}

impl From<std::num::ParseFloatError> for YggdrasilError {
    fn from(e: std::num::ParseFloatError) -> Self {
        Self::LanguageError { error: e.to_string() }
    }
}

impl From<url::ParseError> for YggdrasilError {
    fn from(_: url::ParseError) -> Self {
        Self::IOError { source: std::io::Error::from_raw_os_error(10022) }
    }
}

impl<T> From<std::sync::PoisonError<T>> for YggdrasilError {
    fn from(_: PoisonError<T>) -> Self {
        Self::Unreachable
    }
}

impl From<ropey::Error> for YggdrasilError {
    fn from(e: ropey::Error) -> Self {
        Self::LanguageError { error: e.to_string() }
    }
}

impl From<()> for YggdrasilError {
    fn from(_: ()) -> Self {
        Self::Unreachable
    }
}
