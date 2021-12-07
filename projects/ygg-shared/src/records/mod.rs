mod builder;
mod cst_node;

#[cfg(feature = "lsp")]
mod line_breaks;
#[cfg(feature = "lsp")]
pub use self::{line_breaks::LineBreaks};

pub use self::{builder::ASTBuilder, cst_node::CSTNode};
