mod nodes;
mod parse;

use crate::cst::CSTBuilder;
use crate::cst::Rule;
use yggdrasil_shared::records::ASTBuilder;
use yggdrasil_shared::records::CSTNode;
use yggdrasil_shared::traits::ASTNode;
use yggdrasil_shared::Result;
use yggdrasil_shared::{string_node, YggdrasilError};

pub use self::nodes::*;

pub struct YggParser {
    cst: CSTBuilder,
    ast: ASTBuilder,
    error: Vec<YggdrasilError>,
}

impl Default for YggParser {
    fn default() -> Self {
        Self {
            cst: Default::default(),
            ast: Default::default(),
            error: vec![],
        }
    }
}

impl YggParser {
    /// parse_program
    pub fn parse_program(&mut self, input: &str) -> Result<Program> {
        let cst = self.parse_cst(input)?;
        // println!("{:#?}", cst);
        let program = Program::parse(cst, &mut self.ast)?;
        // println!("{:#?}", program);
        self.error.extend(std::mem::take(&mut self.cst.error).into_iter());
        self.error.extend(std::mem::take(&mut self.ast.error).into_iter());
        Ok(program)
    }
    pub fn parse_cst(&mut self, input: &str) -> Result<CSTNode<Rule>> {
        self.init(input);
        let cst = self.cst.parse(&self.ast.input)?;
        return Ok(cst);
    }

    pub fn errors(&self) -> &[YggdrasilError] {
        self.error.as_slice()
    }
    fn init(&mut self, input: &str) {
        self.cst.error.clear();
        self.ast.error.clear();
        self.error.clear();
        self.ast.input = String::from(input)
    }
}
