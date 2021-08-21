#![allow(clippy::needless_return)]

extern crate thiserror;
// pub mod records;

pub mod macros;
pub mod records;
pub mod traits;

mod errors;

pub use self::errors::{Result, YggdrasilError};
