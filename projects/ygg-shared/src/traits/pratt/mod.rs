use crate::{Error, Result};
use std::iter::Peekable;

pub type PrecedenceNumber = u16;

#[derive(Copy, Clone)]
pub enum Associativity {
    Left,
    Right,
    Neither,
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Precedence(PrecedenceNumber);

impl Precedence {
    pub const fn new(i: PrecedenceNumber) -> Self {
        Self(i)
    }

    const fn raise(mut self) -> Precedence {
        self.0 += 1;
        self
    }
    const fn lower(mut self) -> Precedence {
        self.0 -= 1;
        self
    }
    const fn normalize(mut self) -> Precedence {
        self.0 *= 10;
        self
    }
    const fn min() -> Precedence {
        Precedence(PrecedenceNumber::MIN)
    }
    const fn max() -> Precedence {
        Precedence(PrecedenceNumber::MAX)
    }
}

#[derive(Copy, Clone)]
pub enum Affix {
    Infix(Precedence, Associativity),
    Prefix(Precedence),
    Suffix(Precedence),
    None,
}

impl Affix {
    pub fn infix(p: PrecedenceNumber, a: Associativity) -> Self {
        Self::Infix(Precedence(p), a)
    }
    pub fn suffix(p: PrecedenceNumber) -> Self {
        Self::Prefix(Precedence(p))
    }
    pub fn prefix(p: PrecedenceNumber) -> Self {
        Self::Prefix(Precedence(p))
    }
}

pub trait PrattParser<Inputs>
where
    Inputs: Iterator<Item = Self::Input>,
{
    type Input;
    type Output: Sized;

    fn query(&mut self, input: &Self::Input) -> Result<Affix>;

    fn primary(&mut self, input: Self::Input) -> Result<Self::Output>;

    fn infix(&mut self, lhs: Self::Output, op: Self::Input, rhs: Self::Output) -> Result<Self::Output>;

    fn prefix(&mut self, op: Self::Input, rhs: Self::Output) -> Result<Self::Output>;

    fn suffix(&mut self, lhs: Self::Output, op: Self::Input) -> Result<Self::Output>;

    fn parse(&mut self, inputs: &mut Inputs) -> Result<Self::Output> {
        self.parse_input(&mut inputs.peekable(), Precedence(0))
    }

    fn parse_input(&mut self, tail: &mut Peekable<&mut Inputs>, rbp: Precedence) -> Result<Self::Output> {
        if let Some(head) = tail.next() {
            let info = self.query(&head)?;
            let mut nbp = self.nbp(info);
            let mut node = self.nud(head, tail, info);
            while let Some(head) = tail.peek() {
                let info = self.query(head)?;
                let lbp = self.lbp(info);
                if rbp < lbp && lbp < nbp {
                    let head = tail.next().unwrap();
                    nbp = self.nbp(info);
                    node = self.led(head, tail, info, node?);
                }
                else {
                    break;
                }
            }
            node
        }
        else {
            Err(YggdrasilError::unexpected_token("EmptyInput", None, None))
        }
    }

    /// Null-Denotation
    fn nud(&mut self, head: Self::Input, tail: &mut Peekable<&mut Inputs>, info: Affix) -> Result<Self::Output> {
        match info {
            Affix::Prefix(precedence) => {
                let rhs = self.parse_input(tail, precedence.normalize().lower());
                self.prefix(head, rhs?)
            }
            Affix::None => self.primary(head),
            Affix::Suffix(_) => Err(YggdrasilError::unexpected_token("Unexpected Postfix", None, None)),
            Affix::Infix(_, _) => Err(YggdrasilError::unexpected_token("Unexpected Infix", None, None)),
        }
    }

    /// Left-Denotation
    fn led(
        &mut self,
        head: Self::Input,
        tail: &mut Peekable<&mut Inputs>,
        info: Affix,
        lhs: Self::Output,
    ) -> Result<Self::Output> {
        match info {
            Affix::Infix(precedence, associativity) => {
                let precedence = precedence.normalize();
                let rhs = match associativity {
                    Associativity::Left => self.parse_input(tail, precedence),
                    Associativity::Right => self.parse_input(tail, precedence.lower()),
                    Associativity::Neither => self.parse_input(tail, precedence.raise()),
                };
                self.infix(lhs, head, rhs?)
            }
            Affix::Suffix(_) => self.suffix(lhs, head),
            Affix::None => Err(YggdrasilError::unexpected_token("Unexpected NilFix", None, None)),
            Affix::Prefix(_) => Err(YggdrasilError::unexpected_token("Unexpected Prefix", None, None)),
        }
    }

    //         <lbp>  <rbp>  <nbp> <kind>
    // Nilfix:  MIN |  MIN |  MAX | nud
    // Prefix:  MIN |   bp |  MAX | nud
    // Postfix:  bp |  MIN |  MAX | led
    // InfixL:   bp |   bp | bp+1 | led
    // InfixR:   bp | bp-1 | bp+1 | led
    // InfixN:   bp |   bp |   bp | led

    /// Left-Binding-Power
    fn lbp(&mut self, info: Affix) -> Precedence {
        match info {
            Affix::None => Precedence::min(),
            Affix::Prefix(_) => Precedence::min(),
            Affix::Suffix(precedence) => precedence.normalize(),
            Affix::Infix(precedence, _) => precedence.normalize(),
        }
    }

    /// Next-Binding-Power
    fn nbp(&mut self, info: Affix) -> Precedence {
        match info {
            Affix::None => Precedence::max(),
            Affix::Prefix(_) => Precedence::max(),
            Affix::Suffix(_) => Precedence::max(),
            Affix::Infix(precedence, Associativity::Left) => precedence.normalize().raise(),
            Affix::Infix(precedence, Associativity::Right) => precedence.normalize().raise(),
            Affix::Infix(precedence, Associativity::Neither) => precedence.normalize(),
        }
    }
}
