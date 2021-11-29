use crate::frontend::{rule::ExpressionNode, Map, Rule, RuleMethods};
use itertools::Itertools;
use std::{fmt, fmt::Formatter};
use yggdrasil_bootstrap::ast::Symbol;
mod write_nodes;

pub enum ASTWriter {
    Normal(SymbolCounted),
}

pub struct SymbolCounted {
    rule: Rule,
    map: Map<String, SymbolCount>,
}

pub enum SymbolCount {
    One(String, String),
    Some(String, String),
    Many(String, String),
}

impl From<Rule> for SymbolCounted {
    fn from(rule: Rule) -> Self {
        let mut out = Self { rule, map: Map::default() };
        out.count();
        return out;
    }
}

impl SymbolCounted {
    fn count(&mut self) {}
    fn one(&mut self, s: &Symbol) {}
    fn some(&mut self, s: &Symbol) {}
    fn many(&mut self, s: &Symbol) {}
}

impl ExpressionNode {
    fn count(&self, map: &mut SymbolCounted) {}
}

impl SymbolCounted {
    fn write_string_node_parsing(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "string_node!(Node, {});", self.rule.name.data)
    }
    pub fn write_parser(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(s) = &self.rule.custom_methods.parser {
            f.write_str(s)?;
            return Ok(());
        }

        writeln!(f, "impl ASTNode<Node> for {} {{", self.rule.ty.data)?;
        writeln!(f, "    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {{")?;
        writeln!(f, "        let range = node.get_span();")?;
        writeln!(f, "        let mut map = node.get_tag_map();")?;
        for symbol in self.map.values() {
            f.write_str("        ")?;
            symbol.write_parser(f)?;
        }
        f.write_str("        ")?;
        if self.map.is_empty() {
            f.write_str("Ok(Self { range })")?;
        }
        else {
            writeln!(f, "Ok(Self {{ {}, range }})", self.map.keys().join(", "))?;
        }
        writeln!(f, "    }}")?;
        writeln!(f, "}}")
    }
}

impl SymbolCount {
    fn write_parser(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::One(s, _) => {
                writeln!(f, r#"let {id} = ASTNode::named_one(&mut map, "{id}", builder)?;"#, id = s)
            }
            Self::Some(s, _) => {
                writeln!(f, r#"let {id} = ASTNode::named_some(&mut map, "{id}", builder);"#, id = s)
            }
            Self::Many(s, _) => {
                writeln!(f, r#"let {id} = ASTNode::named_many(&mut map, "{id}", builder);"#, id = s)
            }
        }
    }
}

impl RuleMethods {
    fn write_custom_derive(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut traits = vec![];
        if let Some(s) = &self.debug {
            traits.push(s)
        }
        if let Some(s) = &self.display {
            traits.push(s)
        }
        f.write_str(&traits.iter().join("\n\n"))
    }
}
