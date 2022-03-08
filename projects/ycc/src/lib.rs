#![feature(try_trait_v2)]

mod json;
mod traits;

use chumsky::{
    combinator::{Map, Then},
    prelude::just,
    text, Parser,
};
use std::{
    fmt::{Display, Formatter},
    num::NonZeroUsize,
    ops::{ControlFlow, FromResidual, Try},
};
use text::digits;

use crate::IResult::{Failure, Success};

pub enum IResult<I, O> {
    Success(I, O),
    Failure(Failed),
}

impl<I, O> IResult<I, O> {
    pub fn finish(self) -> Result<O, ErrorMessage> {
        match self {
            Success(_, o) => Ok(o),
            Failure(Failed::Fail(e)) | Failure(Failed::Fatal(e)) => Err(e),
            Failure(Failed::Incomplete(_)) => {
                panic!(
                    "Cannot call `finish()` on `Err(Err::Incomplete(_))`: this result means that the parser does not have enough data to decide, you should gather more data and try to reapply  the parser instead"
                )
            }
        }
    }
}

pub enum Failed {
    Fail(ErrorMessage),
    Fatal(ErrorMessage),
    Incomplete(usize),
}

impl Display for Failed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Failed::Incomplete(0) => write!(f, "Parsing requires more data"),
            Failed::Incomplete(u) => write!(f, "Parsing requires {} bytes/chars", u),

            Failed::Fatal(c) => write!(f, "Parsing Failure: {:?}", c),
            Failed::Fail(c) => write!(f, "Parsing Error: {:?}", c),
        }
    }
}

#[derive(Debug)]
pub struct ErrorMessage {}

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
