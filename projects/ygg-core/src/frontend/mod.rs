mod optimize;
pub mod rule;
pub mod typing;

pub use self::{
    rule::{GrammarContext, Translator},
    typing::GrammarType,
};

use crate::frontend::rule::ExpressionNode;
use lsp_types::Url;
use std::ops::Range;
use yggdrasil_bootstrap::ast::{StringLiteral, Symbol, SymbolAlias};
use indexmap::map::IndexMap;

#[derive(Clone, Debug)]
pub struct GrammarInfo {
    pub(crate) url: Url,
    pub(crate) text: String,
    pub(crate) is_grammar: bool,
    pub(crate) name: Symbol,
    pub(crate) extensions: Vec<StringLiteral>,
    pub(crate) ignores: Vec<Symbol>,
    pub(crate) imports: IndexMap<Url, Vec<SymbolAlias>>,
    pub(crate) rule_map: IndexMap<String, Rule>,
}

#[derive(Clone)]
pub struct Rule {
    ///
    pub(crate) name: Symbol,
    ///
    pub(crate) ty: Symbol,
    ///
    pub(crate) doc: String,
    ///
    pub(crate) custom_methods: RuleMethods,
    ///
    pub(crate) force_inline: bool,
    ///
    pub(crate) force_box: bool,
    /// Eliminate unnamed nodes
    /// ```ygg
    /// name <- expr
    /// ^expr
    /// ```
    pub(crate) already_inline: bool,
    pub(crate) eliminate_unmarked: bool,
    /// Eliminate unnamed nodes
    /// ```ygg
    /// "string"
    /// /regex/
    /// [0-9a-z]
    /// 012345
    /// ```
    pub(crate) eliminate_unnamed: bool,
    ///
    pub(crate) expression: ExpressionNode,
    /// position of all parts
    pub(crate) range: Range<usize>,
}

#[derive(Clone)]
pub struct RuleMethods {
    pub(crate) parser: Option<String>,
    pub(crate) debug: Option<String>,
    pub(crate) display: Option<String>,
    pub(crate) eq: bool,
    pub(crate) eq_partial: Option<String>,
    pub(crate) ord: bool,
    pub(crate) ord_partial: Option<String>,
    pub(crate) hash: Option<String>,
}

impl Default for RuleMethods {
    fn default() -> Self {
        Self { parser: None, debug: None, display: None, eq: false, eq_partial: None, ord: false, ord_partial: None, hash: None }
    }
}
