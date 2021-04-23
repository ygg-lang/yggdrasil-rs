#![feature(once_cell)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(trivial_bounds)]

pub mod codegen;
#[allow(dead_code)]
#[allow(unused)]
mod ygg;

mod manager;

pub use self::{
    manager::{GrammarManager, GRAMMAR_MANAGER},
    ygg::{ast, Result, SyntaxKind, YGGError},
};
pub use tree_sitter::{Parser, Tree};
pub use tree_sitter_ygg::language;
