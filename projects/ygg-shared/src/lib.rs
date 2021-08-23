#![allow(clippy::needless_return)]

pub mod macros;
pub mod records;
pub mod traits;

mod errors;

pub use self::errors::{Result, YggdrasilError, YggdrasilErrorKind};
