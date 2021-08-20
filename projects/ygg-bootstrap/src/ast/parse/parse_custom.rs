use super::*;
use yggdrasil_shared::traits::{Affix, Associativity, PrattParser};

impl ASTNode<Node> for StringLiteral {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_range();
        let raw = unsafe { (&builder.input).get_unchecked((range.start + 1)..(range.end - 1)) };
        let data = unescape(raw, range.start)?;
        Ok(Self { data, range })
    }
}

fn unescape(raw: &str, offset: usize) -> Result<String> {
    let mut out = String::with_capacity(raw.len());
    let mut offset = offset;
    let mut chars = raw.chars();
    while let Some(c) = chars.next() {
        offset += 1;
        if c != '\\' {
            out.push(c);
            continue;
        };
        let c = chars.next().ok_or(YggdrasilError::unexpected_token("Missing character after \\", Some(offset), Some(offset + 1)))?;
        match c {
            'b' => out.push('\u{08}'),
            't' => out.push('\t'),
            'n' => out.push('\n'),
            'f' => out.push('\u{0C}'),
            'r' => out.push('\r'),
            'u' => out.push(parse_unicode(&mut chars, offset)?),
            _ => out.push(c),
        }
    }
    return Ok(out);
}

fn parse_unicode<I>(chars: &mut I, mut offset: usize) -> Result<char>
where
    I: Iterator<Item = char>,
{
    offset += 1;
    match chars.next() {
        Some('{') => {}
        _ => {
            return Err(YggdrasilError::unexpected_token("Missing unicode closing character }", Some(offset), Some(offset + 1)));
        }
    }
    let unicode_seq: String = chars.take_while(|&c| c != '}').collect();
    let n = u32::from_str_radix(&unicode_seq, 16).ok().and_then(|c| char::from_u32(c));
    match n {
        Some(c) => Ok(c),
        None => Err(YggdrasilError::unexpected_token("Invalid unicode token", Some(offset), Some(offset + 1))),
    }
}

impl ASTNode<Node> for SymbolAlias {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_range();
        let mut map = node.get_tag_map();
        let from = Symbol::named_one(&mut map, "from", builder)?;
        let into = Symbol::named_some(&mut map, "into", builder).unwrap_or(from.clone());
        Ok(Self { from, into, range })
    }
}

impl ASTNode<Node> for Expression {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let mut map = node.get_tag_map();
        let mut terms = vec![];
        terms.push(ASTNode::named_one(&mut map, "term", builder)?);
        for e in map.remove("expr_next").unwrap_or_default() {
            let branch = e.branch_tag;
            let mut map = e.get_tag_map();
            match branch {
                Some("Infix") => {
                    let infix = map.remove("infix").unwrap_or_default().remove(0);
                    if let Some(s) = infix.get_str(&builder.input).chars().next() {
                        terms.push(Term::Infix(s))
                    }
                }
                Some("Other") => terms.push(Term::Infix(' ')),
                _ => {}
            }
            terms.push(ASTNode::named_one(&mut map, "term", builder)?);
        }
        Term::build_expression(terms)
    }
}

impl ASTNode<Node> for Term {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let mut map = node.get_tag_map();
        let prefix = Prefix::named_many(&mut map, "prefix", builder);
        let suffix = TermNext::named_many(&mut map, "term_next", builder);
        let mut base = match Expression::named_one(&mut map, "expr", builder) {
            Ok(o) => o,
            Err(_) => Data::named_one(&mut map, "data", builder).map(Expression::Data)?,
        };
        if prefix.is_empty() && suffix.is_empty() {
            return Ok(Term::Atom(base));
        }
        base = Term::build_suffix(base, suffix);
        base = Term::build_prefix(base, prefix);
        return Ok(Term::Atom(base));
    }
}

impl Term {
    #[inline]
    pub fn build_expression(input: Vec<Term>) -> Result<Expression> {
        TermResolve.parse(&mut input.into_iter())
    }
    fn build_prefix(mut base: Expression, ops: Vec<Prefix>) -> Expression {
        for op in ops.into_iter().rev() {
            match op.data {
                '!' => base = Expression::MustNot(Box::new(base)),
                '&' => base = Expression::MustOne(Box::new(base)),
                '^' => base = Expression::MarkNodeShort(Box::new(base)),
                '%' => unimplemented!(),
                _ => unreachable!(),
            }
        }
        return base;
    }
    pub fn build_suffix(mut base: Expression, ops: Vec<TermNext>) -> Expression {
        for op in ops.into_iter() {
            match op {
                TermNext::Suffix('?') => base = Expression::Maybe(Box::new(base)),
                TermNext::Suffix('+') => base = Expression::ManyNonNull(Box::new(base)),
                TermNext::Suffix('*') => base = Expression::Many(Box::new(base)),
                TermNext::Slice(_) => {
                    unimplemented!()
                }
                TermNext::Branch(BranchTag { kind, symbol, .. }) => base = Expression::MarkBranch { base: Box::new(base), kind, name: symbol },
                _ => unreachable!(),
            }
        }
        return base;
    }
}

pub struct TermResolve;

impl<I> PrattParser<I> for TermResolve
where
    I: Iterator<Item = Term>,
{
    type Input = Term;
    type Output = Expression;

    fn query(&mut self, term: &Self::Input) -> Result<Affix> {
        let affix = match term {
            Term::Suffix('?') => Affix::suffix(300),
            Term::Suffix('+') => Affix::suffix(300),
            Term::Suffix('*') => Affix::suffix(300),

            Term::Prefix('!') => Affix::prefix(200),
            Term::Prefix('&') => Affix::prefix(200),
            Term::Prefix('^') => Affix::prefix(200),
            Term::Prefix('%') => Affix::prefix(200),

            Term::Infix(' ') => Affix::infix(100, Associativity::Left),
            Term::Infix('~') => Affix::infix(90, Associativity::Left),
            Term::Infix('|') => Affix::infix(80, Associativity::Left),
            Term::Infix('/') => Affix::infix(80, Associativity::Left),
            Term::Infix(':') => Affix::infix(70, Associativity::Left),
            Term::Infix('<') => Affix::infix(60, Associativity::Left),

            Term::Atom(_) => Affix::None,
            _ => unreachable!(),
        };
        Ok(affix)
    }

    fn primary(&mut self, term: Self::Input) -> Result<Self::Output> {
        match term {
            Term::Atom(e) => Ok(e),
            _ => unreachable!(),
        }
    }

    fn infix(&mut self, lhs: Self::Output, term: Self::Input, rhs: Self::Output) -> Result<Self::Output> {
        let out = match term {
            Term::Infix(' ') => Expression::Concat { is_soft: false, lhs: Box::new(lhs), rhs: Box::new(rhs) },
            Term::Infix('~') => Expression::Concat { is_soft: true, lhs: Box::new(lhs), rhs: Box::new(rhs) },
            Term::Infix('|') => Expression::Choice { lhs: Box::new(lhs), rhs: Box::new(rhs) },
            Term::Infix('/') => Expression::Choice { lhs: Box::new(lhs), rhs: Box::new(rhs) },
            Term::Infix(':') => Expression::MarkType { lhs: Box::new(lhs), rhs: Box::new(rhs) },
            Term::Infix('<') => Expression::MarkNode { lhs: Box::new(lhs), rhs: Box::new(rhs) },
            _ => unreachable!(),
        };
        return Ok(out);
    }

    fn prefix(&mut self, term: Self::Input, rhs: Self::Output) -> Result<Self::Output> {
        let _ = (term, rhs);
        unreachable!()
        // let out = match term {
        //     Term::Prefix('!') => Expression::MustNot(Box::new(rhs)),
        //     Term::Prefix('&') => Expression::MustOne(Box::new(rhs)),
        //     Term::Prefix('^') => Expression::MarkNodeShort(Box::new(rhs)),
        //     Term::Prefix('%') => unimplemented!(),
        //     _ => unreachable!(),
        // };
        // Ok(out)
    }

    fn suffix(&mut self, lhs: Self::Output, term: Self::Input) -> Result<Self::Output> {
        let _ = (lhs, term);
        unreachable!()
        // let out = match tree {
        //     Term::Suffix('?') => Expression::Maybe(Box::new(lhs)),
        //     Term::Suffix('+') => Expression::Maybe(Box::new(lhs)),
        //     Term::Suffix('*') => Expression::Maybe(Box::new(lhs)),
        //     _ => unreachable!(),
        // };
        // Ok(out)
    }
}
