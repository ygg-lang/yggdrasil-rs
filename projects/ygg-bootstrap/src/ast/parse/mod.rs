mod parse_custom;

use super::*;

type Node = CSTNode<Rule>;

impl ASTNode<Node> for Program {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_range();
        let mut map = node.get_tag_map();
        let statement = ASTNode::named_many(&mut map, "statement", builder);
        return Ok(Self { statement, range });
    }
}

impl ASTNode<Node> for Statement {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let branch = node.branch_tag;
        let mut map = node.get_tag_map();
        match branch {
            Some("Grammar") => Ok(Self::Grammar(Box::new(ASTNode::named_one(&mut map, "grammar_statement", builder)?))),
            Some("Fragment") => Ok(Self::Fragment(Box::new(ASTNode::named_one(&mut map, "fragment_statement", builder)?))),
            Some("Ignore") => Ok(Self::Ignore(Box::new(ASTNode::named_one(&mut map, "ignore_statement", builder)?))),
            Some("Import") => Ok(Self::Import(Box::new(ASTNode::named_one(&mut map, "import_statement", builder)?))),
            Some("Assign") => Ok(Self::Assign(Box::new(ASTNode::named_one(&mut map, "assign_statement", builder)?))),
            Some("MacroCall") => Ok(Self::Assign(Box::new(ASTNode::named_one(&mut map, "assign_statement", builder)?))),
            _ => {
                println!("{:#?}", map);
                unreachable!()
            }
        }
    }
}

impl ASTNode<Node> for GrammarStatement {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_range();
        let mut map = node.get_tag_map();
        let id = ASTNode::named_one(&mut map, "id", builder)?;
        let ext = ASTNode::named_many(&mut map, "ext", builder);
        return Ok(Self { id, ext, range });
    }
}

impl ASTNode<Node> for FragmentStatement {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_range();
        let mut map = node.get_tag_map();
        let id = ASTNode::named_one(&mut map, "id", builder)?;
        return Ok(Self { id, range });
    }
}

impl ASTNode<Node> for ImportStatement {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_range();
        let mut map = node.get_tag_map();
        let path = ASTNode::named_one(&mut map, "path", builder)?;
        let symbol_alias = ASTNode::named_many(&mut map, "symbol_alias", builder);
        return Ok(Self { path, symbol_alias, range });
    }
}

impl ASTNode<Node> for IgnoreStatement {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_range();
        let mut map = node.get_tag_map();
        let rules = ASTNode::named_many(&mut map, "rules", builder);
        return Ok(Self { rules, range });
    }
}

impl ASTNode<Node> for AssignStatement {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_range();
        let mut map = node.get_tag_map();
        let id = ASTNode::named_one(&mut map, "id", builder)?;
        let ty = ASTNode::named_some(&mut map, "ty", builder);
        let eq = ASTNode::named_one(&mut map, "eq", builder)?;
        let rhs = ASTNode::named_one(&mut map, "rhs", builder)?;
        Ok(Self { id, ty, eq, rhs, range })
    }
}

impl ASTNode<Node> for TermNext {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let branch = node.branch_tag;
        let mut map = node.get_tag_map();
        match branch {
            Some("Suffix") => Ok(Self::Suffix(ASTNode::named_one(&mut map, "suffix", builder)?)),
            Some("Slice") => unimplemented!("Slice"),
            Some("Branch") => Ok(Self::Branch(ASTNode::named_one(&mut map, "branch_tag", builder)?)),
            _ => unreachable!(),
        }
    }
}

impl ASTNode<Node> for BranchTag {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_range();
        let mut map = node.get_tag_map();
        let kind = ASTNode::named_some(&mut map, "kind", builder);
        let symbol = ASTNode::named_one(&mut map, "symbol", builder)?;
        Ok(Self { kind, symbol, range })
    }
}

impl ASTNode<Node> for Data {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let branch = node.branch_tag;
        let children = &mut node.get_tag_map();
        match branch {
            Some("Symbol") => Ok(Self::Symbol(ASTNode::named_one(children, "symbol_path", builder)?)),
            Some("Integer") => Ok(Self::Integer(ASTNode::named_one(children, "integer", builder)?)),
            Some("String") => Ok(Self::String(ASTNode::named_one(children, "string", builder)?)),
            Some("Macro") => Ok(Self::String(ASTNode::named_one(children, "macro", builder)?)),
            Some("Regex") => Ok(Self::String(ASTNode::named_one(children, "regex", builder)?)),
            Some("Range") => Ok(Self::String(ASTNode::named_one(children, "regex", builder)?)),
            Some(s) => {
                println!("{:#?}", s);
                unreachable!()
            }
            _ => return Err(YggdrasilError::structure_error("Data")),
        }
    }
}

impl ASTNode<Node> for Slice {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_range();
        let mut map = node.get_tag_map();
        let start = ASTNode::named_some(&mut map, "start", builder);
        let end = ASTNode::named_some(&mut map, "end", builder);
        Ok(Self { start, end, range })
    }
}

impl ASTNode<Node> for SymbolPath {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_range();
        let mut map = node.get_tag_map();
        let data = ASTNode::named_many(&mut map, "symbol", builder);
        Ok(Self { symbol: data, range })
    }
}

string_node!(Node, Prefix);
string_node!(Node, Suffix);
string_node!(Node, Symbol);
string_node!(Node, Integer);
