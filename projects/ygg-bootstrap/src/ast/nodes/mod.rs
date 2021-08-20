use std::ops::Range;

mod custom_debug;
mod custom_methods;
mod custom_trait;

// pub use self::nodes_resolver::*;

#[derive(Clone)]
pub struct Program {
    pub statement: Vec<Statement>,
    pub range: Range<usize>,
}

#[derive(Clone)]
pub enum Statement {
    Grammar(Box<GrammarStatement>),
    Import(Box<ImportStatement>),
    Assign(Box<AssignStatement>),
    Ignore(Box<IgnoreStatement>),
    Fragment(Box<FragmentStatement>),
    MacroCall(Box<MacroCall>),
    CommentDocument(Box<CommentDocument>),
}

#[derive(Clone, Debug)]
pub struct GrammarStatement {
    pub id: Symbol,
    pub ext: Vec<StringLiteral>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct FragmentStatement {
    pub id: Symbol,
    pub range: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct AssignStatement {
    pub id: Symbol,
    pub ty: Option<Symbol>,
    pub eq: String,
    pub rhs: Expression,
    pub range: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct IgnoreStatement {
    pub rules: Vec<Symbol>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct ImportStatement {
    pub path: StringLiteral,
    pub symbol_alias: Vec<SymbolAlias>,
    pub range: Range<usize>,
}

#[derive(Clone)]
pub struct SymbolAlias {
    pub from: Symbol,
    pub into: Symbol,
    pub range: Range<usize>,
}

#[derive(Clone)]
pub enum Expression {
    Data(Data),
    /// lhs ~ rhs
    Concat {
        is_soft: bool,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    /// lhs | rhs
    Choice {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    /// lhs <- rhs
    MarkNode {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    /// ^rhs
    MarkNodeShort(Box<Expression>),
    /// lhs: rhs
    MarkType {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    /// lhs #name
    MarkBranch {
        base: Box<Expression>,
        kind: Option<char>,
        name: Symbol,
    },
    /// !rhs
    MustNot(Box<Expression>),
    MustOne(Box<Expression>),
    Maybe(Box<Expression>),
    Many(Box<Expression>),
    ManyNonNull(Box<Expression>),
}

#[derive(Debug)]
pub enum Term {
    Prefix(char),
    Suffix(char),
    Infix(char),
    Atom(Expression),
}

#[derive(Clone, Debug)]
pub enum TermNext {
    Suffix(char),
    Slice(Slice),
    Branch(BranchTag),
}

#[derive(Clone, Debug)]
pub struct Prefix {
    pub data: char,
    pub range: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct Suffix {
    pub data: char,
    pub range: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct Slice {
    pub start: Option<Integer>,
    pub end: Option<Integer>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct BranchTag {
    pub kind: Option<char>,
    pub symbol: Symbol,
    pub range: Range<usize>,
}

#[derive(Clone)]
pub enum Data {
    Symbol(SymbolPath),
    Integer(Integer),
    String(StringLiteral),
    Macro(MacroCall),
    Regex,
}

#[derive(Clone, Debug)]
pub struct MacroCall {
    pub symbol: SymbolPath,
    pub range: Range<usize>,
}

#[derive(Clone)]
pub struct SymbolPath {
    pub symbol: Vec<Symbol>,
    pub range: Range<usize>,
}

#[derive(Clone)]
pub struct Symbol {
    pub data: String,
    pub range: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct Integer {
    pub data: isize,
    pub range: Range<usize>,
}

#[derive(Clone)]
pub struct StringLiteral {
    pub data: String,
    pub range: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct Eos {
    pub data: bool,
    pub range: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct CommentDocument {
    pub doc: String,
    pub range: Range<usize>,
}
