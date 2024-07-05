use crate::token::{Literal, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Grouping(Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Literal(Literal),
    Variable(Token),
    Assign(Token, Box<Expr>),
}

pub trait ExprVisitor<R> {
    fn visit_grouping(&mut self, expr: &Expr) -> R;
    fn visit_binary(&mut self, left: &Expr, op: &Token, right: &Expr) -> R;
    fn visit_unary(&mut self, op: &Token, right: &Expr) -> R;
    fn visit_literal(&mut self, l: &Literal) -> R;
    fn visit_variable(&mut self, t: &Token) -> R;
    fn visit_assign(&mut self, t: &Token, e: &Expr) -> R;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut impl ExprVisitor<R>) -> R {
        match self {
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
            Expr::Binary(left, op, right) => visitor.visit_binary(left, op, right),
            Expr::Unary(op, right) => visitor.visit_unary(op, right),
            Expr::Literal(l) => visitor.visit_literal(l),
            Expr::Variable(t) => visitor.visit_variable(t),
            Expr::Assign(t, e) => visitor.visit_assign(t, e),
        }
    }
}