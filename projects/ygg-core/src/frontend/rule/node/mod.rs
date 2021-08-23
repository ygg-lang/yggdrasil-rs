use crate::frontend::{Rule};
use std::{
    fmt::{Debug, Formatter},
    ops::{AddAssign, BitAndAssign},
};
use yggdrasil_bootstrap::ast::*;
use indexmap::set::IndexSet;

mod choice;
mod concat;
mod debug;
mod expr;
mod unary;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ExpressionNode {
    pub inline_token: bool,
    pub ty: Option<Symbol>,
    pub branch_tag: Option<ExpressionTag>,
    pub node_tag: Option<Symbol>,
    pub node: RefinedExpression,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ExpressionTag {
    pub name: Symbol,
    pub mode: String,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum RefinedExpression {
    Data(Box<RefinedData>),
    Unary(Box<RefinedUnary>),
    Choice(Box<RefinedChoice>),
    Concat(Box<RefinedConcat>),
}

#[derive(Clone, Eq, PartialEq)]
pub struct RefinedChoice {
    pub inner: IndexSet<ExpressionNode>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RefinedConcat {
    pub base: ExpressionNode,
    pub rest: Vec<(bool, ExpressionNode)>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RefinedUnary {
    pub base: ExpressionNode,
    pub ops: Vec<Operator>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Operator {
    /// e?
    Optional,
    /// e*
    Repeats,
    /// e+
    Repeats1,
    /// e+
    RepeatsBetween(Option<u8>, Option<u8>),
    /// ^e
    Mark,
    /// *e
    Recursive,
}

#[derive(Clone)]
pub enum RefinedData {
    Symbol(SymbolPath),
    String(String),
    Regex(String),
    Integer(isize),
}
