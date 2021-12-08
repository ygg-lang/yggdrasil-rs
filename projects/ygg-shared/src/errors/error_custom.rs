use super::*;
use std::sync::PoisonError;

impl From<std::str::Utf8Error> for YggdrasilError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::language_error(e.to_string())
    }
}

impl From<std::num::ParseIntError> for YggdrasilError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::language_error(e.to_string())
    }
}

impl From<std::num::ParseFloatError> for YggdrasilError {
    fn from(e: std::num::ParseFloatError) -> Self {
        Self::language_error(e.to_string())
    }
}

impl From<url::ParseError> for YggdrasilError {
    fn from(_: url::ParseError) -> Self {
        Self { kind: IOError(std::io::Error::from_raw_os_error(10022)), file: None, range: None }
    }
}

impl<T> From<std::sync::PoisonError<T>> for YggdrasilError {
    fn from(_: PoisonError<T>) -> Self {
        Self::unreachable()
    }
}

impl From<ropey::Error> for YggdrasilError {
    fn from(e: ropey::Error) -> Self {
        Self::language_error(e.to_string())
    }
}

impl From<()> for YggdrasilError {
    fn from(_: ()) -> Self {
        Self::unreachable()
    }
}
