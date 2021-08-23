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

impl From<std::io::Error> for YggdrasilError {
    fn from(e: std::io::Error) -> Self {
        Self { kind: IOError(e), file: None, range: None }
    }
}

impl From<std::fmt::Error> for YggdrasilError {
    fn from(e: std::fmt::Error) -> Self {
        Self { kind: FormatError(e), file: None, range: None }
    }
}

impl<T> From<std::sync::PoisonError<T>> for YggdrasilError {
    fn from(_: PoisonError<T>) -> Self {
        Self::unreachable()
    }
}

impl From<()> for YggdrasilError {
    fn from(_: ()) -> Self {
        Self::unreachable()
    }
}
