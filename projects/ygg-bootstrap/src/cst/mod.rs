#![allow(non_snake_case, non_camel_case_types)]
#![allow(unused_variables, dead_code)]

include!(concat!(env!("OUT_DIR"), "/ygg.rs"));

mod parse;

pub use self::ygg::{Node, Rule, PEG};
//pub use self::parse::{Node, Rule, PEG};
use std::ops::Range;
use yggdrasil_shared::CSTNode;
use yggdrasil_shared::{Result, YggdrasilError};

pub struct CSTBuilder {
    pub peg: PEG,
    pub error: Vec<YggdrasilError>,
}

impl Default for CSTBuilder {
    fn default() -> Self {
        Self { peg: PEG::new(), error: vec![] }
    }
}

impl CSTBuilder {
    pub fn parse(&mut self, input: &str) -> Result<CSTNode<Rule>> {
        let parsed = self.peg.parse(input);
        // println!("{:#?}", parsed);
        match parsed {
            Ok(o) => Ok(flatten(o)),
            Err(e) => Err(YggdrasilError::unexpected_token("CST UnexpectedToken").set_range(Range { start: e.0, end: e.1 })),
        }
    }
}

fn flatten(node: Node) -> CSTNode<Rule> {
    let mut buffer = vec![];
    for node in node.children {
        flatten_rec(node, &mut buffer)
    }
    CSTNode {
        rule: node.rule,
        range: Range { start: node.start, end: node.end },
        children: buffer,
        node_tag: node.label,
        branch_tag: node.alternative,
    }
}

fn flatten_rec(node: Node, buffer: &mut Vec<CSTNode<Rule>>) {
    if node.start == node.end {
        return;
    }
    match node.rule {
        // not important
        Rule::EOI | Rule::MustNotMatch => {}
        // flatten these nodes
        Rule::Any | Rule::List => {
            for node in node.children {
                flatten_rec(node, buffer)
            }
        }
        #[cfg(feature = "no-ignored")]
        Rule::IGNORE => {}
        #[cfg(feature = "no-unnamed")]
        Rule::Terminal => {}
        _ => buffer.push(flatten(node)),
    }
}
