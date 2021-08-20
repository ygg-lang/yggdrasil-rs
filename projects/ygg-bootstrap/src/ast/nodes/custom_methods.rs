use super::*;

impl Expression {
    pub fn range(&self) -> Range<usize> {
        match self {
            Self::Data(e) => e.range(),
            Self::Concat { lhs, rhs, .. } => Range {
                start: lhs.range().start,
                end: rhs.range().end,
            },
            Self::Choice { lhs, rhs } => Range {
                start: lhs.range().start,
                end: rhs.range().end,
            },
            Self::MarkNode { lhs, rhs } => Range {
                start: lhs.range().start,
                end: rhs.range().end,
            },
            Self::MarkNodeShort(e) => Range {
                start: (e.range().start - 1),
                end: e.range().end,
            },
            Self::MarkType { lhs, rhs } => Range {
                start: lhs.range().start,
                end: rhs.range().end,
            },
            Self::MarkBranch { base, name, .. } => Range {
                start: (base.range().start),
                end: name.range.end
            },
            Self::MustNot(e) => Range {
                start: e.range().start - 1,
                end: e.range().end,
            },
            Self::MustOne(e) => Range {
                start: (e.range().start - 1 ),
                end: (e.range().end)
            },
            Self::Maybe(e) => Range {
                start: (e.range().start),
                end: (e.range().end + 1)
            },
            Self::Many(e) => Range {
                start: (e.range().start),
                end: ( e.range().end + 1)
            },
            Self::ManyNonNull(e) => Range {
                start: (e.range().start ),
                end: (e.range().end + 1)
            },
        }
    }
    pub fn as_data(self) -> Option<Data> {
        match self {
            Expression::Data(e) => Some(e),
            Expression::Maybe(e) | Expression::Many(e) | Expression::ManyNonNull(e) => e.as_data(),
            _ => None,
        }
    }
    pub fn as_symbol_path(self) -> Option<SymbolPath> {
        match self.as_data() {
            Some(Data::Symbol(s)) => Some(s),
            _ => None,
        }
    }
    pub fn as_symbol(self) -> Option<Symbol> {
        let mut symbols = self.as_symbol_path().map(|e| e.symbol).unwrap_or_default();
        match symbols.len() {
            1 => Some(symbols.remove(0)),
            _ => None,
        }
    }
}

impl Data {
    pub fn range(&self) -> Range<usize> {
        match self {
            Data::Symbol(e) => e.range.to_owned(),
            Data::Integer(e) => e.range.to_owned(),
            Data::String(e) => e.range.to_owned(),
            Data::Macro(e) => e.range.to_owned(),
            Data::Regex => Range::default(),
        }
    }
}
