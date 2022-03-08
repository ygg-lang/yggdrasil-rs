#![feature(try_trait_v2)]

use std::{
    num::NonZeroUsize,
    ops::{ControlFlow, FromResidual, Try},
};

use crate::IResult::{Failure, Success};

pub enum IResult<I, O> {
    Success(I, O),
    Failure(Failed),
}

pub enum Failed {
    Fail(ErrorMessage),
    Fatal(ErrorMessage),
    Incomplete(Needed),
}

pub struct ErrorMessage {}

/// Contains information on needed data if a parser returned `Incomplete`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Needed {
    /// Needs more data, but we do not know how much
    Unknown,
    /// Contains the required data size in bytes
    Size(NonZeroUsize),
}

impl Needed {
    /// Creates `Needed` instance, returns `Needed::Unknown` if the argument is zero
    pub fn new(s: usize) -> Self {
        match NonZeroUsize::new(s) {
            Some(sz) => Needed::Size(sz),
            None => Needed::Unknown,
        }
    }

    /// Indicates if we know how many bytes we need
    pub fn is_known(&self) -> bool {
        *self != Needed::Unknown
    }

    /// Maps a `Needed` to `Needed` by applying a function to a contained `Size` value.
    #[inline]
    pub fn map<F: Fn(NonZeroUsize) -> usize>(self, f: F) -> Needed {
        match self {
            Self::Unknown => Needed::Unknown,
            Self::Size(n) => Needed::new(f(n)),
        }
    }
}

impl<I, O> FromResidual<(I, O)> for IResult<I, O> {
    fn from_residual(residual: (I, O)) -> Self {
        Self::Success(residual.0, residual.1)
    }
}

impl<I, O> FromResidual<Failed> for IResult<I, O> {
    fn from_residual(residual: Failed) -> Self {
        Self::Failure(residual)
    }
}

impl<I, O> Try for IResult<I, O> {
    type Output = (I, O);
    type Residual = Failed;

    fn from_output(output: Self::Output) -> Self {
        Success(output.0, output.1)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Success(i, o) => ControlFlow::Continue((i, o)),
            Failure(e) => ControlFlow::Break(e),
        }
    }
}

#[test]
fn test() {}
