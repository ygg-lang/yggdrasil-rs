mod optimize;
pub mod rule;
pub mod typing;

pub use self::{
    rule::{GrammarContext, Translator},
    typing::GrammarType,
};

pub use self::remap::{Keys, Map, Set, Values};
use crate::frontend::rule::ExpressionNode;
use lsp_types::Url;
use yggdrasil_bootstrap::ast::{StringLiteral, Symbol, SymbolAlias};
use std::ops::Range;

// used for ide hint
#[cfg(debug_assertions)]
mod remap {
    pub use std::collections::hash_map::{Keys, Values};

    pub type Set<V> = std::collections::HashSet<V>;
    pub type Map<K, V> = std::collections::HashMap<K, V>;
}

#[cfg(not(debug_assertions))]
mod remap {
    pub use indexmap::map::{Keys, Values};

    pub type Set<V> = indexmap::IndexSet<V>;
    pub type Map<K, V> = indexmap::IndexMap<K, V>;
}

#[derive(Clone, Debug)]
pub struct GrammarInfo {
    pub(crate) url: Url,
    pub(crate) text: String,
    pub(crate) is_grammar: bool,
    pub(crate) name: Symbol,
    pub(crate) extensions: Vec<StringLiteral>,
    pub(crate) ignores: Vec<Symbol>,
    pub(crate) imports: Map<Url, Vec<SymbolAlias>>,
    pub(crate) rule_map: Map<String, Rule>,
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
    pub(crate) hash: Option<String>,
}

impl Default for RuleMethods {
    fn default() -> Self {
        Self { parser: None, debug: None, display: None, eq: false, eq_partial: None, hash: None }
    }
}
