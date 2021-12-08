#![feature(once_cell)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(trivial_bounds)]
#![feature(async_closure)]
#![allow(clippy::needless_return)]

pub use self::manager::{FILE_MANAGER, HINT_MANAGER, PARSER_MANAGER};
pub use yggdrasil_bootstrap::{Result, YggdrasilError};

pub mod codegen;
pub mod frontend;
pub mod manager;

#[cfg(feature = "railroad")]
pub mod railroad {
    pub use crate::codegen::target_railroad::{Diagram, RailroadNode, VerticalGrid, DEFAULT_CSS};
}
