use crate::{
    records::{ASTBuilder, CSTNode},
    Error, Result,
};
use std::collections::HashMap;
use std::ops::Range;

/// It's a cst_node contained in the Strongly Typed Abstract Syntax Tree
/// Implement the `parse` method to express how to become a typed cst_node
/// Semantic analysis, interpreter, LSP can start at this level
pub trait ASTNode<N>
where
    Self: Sized,
{
    /// This cst_node will appear multiple times
    /// If the child cst_node fails, it will be abandoned
    fn many(node: Vec<N>, builder: &mut ASTBuilder) -> Vec<Self> {
        let mut out = Vec::with_capacity(node.len());
        for pair in node {
            match Self::parse(pair, builder) {
                Ok(o) => out.push(o),
                Err(YggdrasilError::Unwinding) => (),
                Err(e) => builder.error.push(e),
            }
        }
        return out;
    }
    /// This cst_node is optional
    /// If the parsing fails, it will be abandoned and return null
    fn some(node: N, builder: &mut ASTBuilder) -> Option<Self> {
        match Self::parse(node, builder) {
            Ok(o) => return Some(o),
            Err(YggdrasilError::Unwinding) => (),
            Err(e) => builder.error.push(e),
        }
        return None;
    }
    /// This cst_node is required
    /// If it fails, it will start to roll back
    /// Destruct level by level, until a cst_node can tolerate this problem
    fn one(node: N, builder: &mut ASTBuilder) -> Result<Self> {
        match Self::parse(node, builder) {
            Ok(o) => Ok(o),
            Err(YggdrasilError::Unwinding) => Err(YggdrasilError::Unwinding),
            Err(e) => {
                builder.error.push(e);
                Err(YggdrasilError::Unwinding)
            }
        }
    }
    /// many
    fn named_many(map: &mut HashMap<&'static str, Vec<N>>, tag: &str, builder: &mut ASTBuilder) -> Vec<Self> {
        match map.remove(tag) {
            Some(s) => Self::many(s, builder),
            _ => vec![],
        }
    }
    /// some
    fn named_some(map: &mut HashMap<&'static str, Vec<N>>, tag: &str, builder: &mut ASTBuilder) -> Option<Self> {
        match map.remove(tag).as_mut().map(|v| v.remove(0)) {
            Some(s) => Self::some(s, builder),
            _ => None,
        }
    }
    /// one
    fn named_one(map: &mut HashMap<&'static str, Vec<N>>, tag: &str, builder: &mut ASTBuilder) -> Result<Self> {
        match map.remove(tag).as_mut().map(|v| v.remove(0)) {
            Some(s) => Self::one(s, builder),
            _ => Err(YggdrasilError::structure_error(format!("Missing Node: {tag}", tag = tag), None, None)),
        }
    }
    /// parse
    fn parse(node: N, builder: &mut ASTBuilder) -> Result<Self>;
}

macro_rules! ast_node_num {
    ($t:ty) => {
        impl<R> ASTNode<CSTNode<R>> for $t {
            fn parse(node: CSTNode<R>, builder: &mut ASTBuilder) -> Result<Self> {
                Ok(node.get_str(&builder.input).parse::<$t>()?)
            }
        }
    };
    ($($t:ty),+ $(,)?) => {
        $(ast_node_num!($t);)+
    };
}

ast_node_num![u8, u16, u32, u64, u128, usize];
ast_node_num![i8, i16, i32, i64, i128, isize];
ast_node_num![f32, f64];

impl<R> ASTNode<CSTNode<R>> for String {
    fn parse(node: CSTNode<R>, builder: &mut ASTBuilder) -> Result<Self> {
        Ok(node.get_str(&builder.input).to_string())
    }
}
impl<R> ASTNode<CSTNode<R>> for char {
    fn parse(node: CSTNode<R>, builder: &mut ASTBuilder) -> Result<Self> {
        let Range { start, end } = node.get_range();
        match node.get_str(&builder.input).chars().next() {
            Some(c) => Ok(c),
            None => Err(YggdrasilError::structure_error("Invalid `char`", Some(start), Some(end))),
        }
    }
}
