use super::*;

impl ExpressionNode {
    #[inline]
    pub fn choice(lhs: Expression, rhs: Expression) -> Self {
        let mut base = ExpressionNode::from(lhs).as_choice();
        base.get_choice_mut().map(|e| e.add_assign(ExpressionNode::from(rhs)));
        return base;
    }
    fn as_choice(self) -> Self {
        if let Some(_) = self.get_choice() {
            return self;
        }
        return Self { inline_token: false, ty: None, branch_tag: None, node_tag: None, node: RefinedExpression::concat(self) };
    }
}

impl RefinedExpression {
    pub fn choice(base: ExpressionNode) -> Self {
        let mut inner = IndexSet::default();
        inner.insert(base);
        Self::Choice(Box::new(RefinedChoice { inner }))
    }
}

impl From<Expression> for RefinedChoice {
    fn from(e: Expression) -> Self {
        Self::from(ExpressionNode::from(e))
    }
}

impl From<ExpressionNode> for RefinedChoice {
    fn from(e: ExpressionNode) -> Self {
        match e.get_choice() {
            Some(s) => Self { inner: s.inner },
            None => {
                let mut inner = IndexSet::new();
                inner.insert(e);
                Self { inner }
            }
        }
    }
}

impl AddAssign<ExpressionNode> for RefinedChoice {
    fn add_assign(&mut self, rhs: ExpressionNode) {
        match rhs.get_choice() {
            Some(c) => c.inner.into_iter().for_each(|e| {
                self.inner.insert(e);
            }),
            None => {
                self.inner.insert(rhs);
            }
        };
    }
}
