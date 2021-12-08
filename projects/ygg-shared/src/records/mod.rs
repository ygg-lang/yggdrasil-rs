mod builder;
mod cst_node;
mod text_store;

#[cfg(feature = "lsp")]
mod text_index;

#[cfg(feature = "lsp")]
pub use self::text_index::*;


pub use self::text_store::TextStore;
pub use self::{builder::ASTBuilder, cst_node::CSTNode};
