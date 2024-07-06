use crate::environment::Environment;
use crate::expr::{Expr, ExprVisitor};
use crate::token::{Literal, Token};
use crate::token_type::TokenType;

pub struct Interpreter {
    pub environment: Environment,
}

impl ExprVisitor<bool> for Interpreter {
    fn visit_grouping(&mut self, expr: &Expr) -> bool {
        expr.accept(self)
    }

    fn visit_binary(&mut self, left: &Expr, op: &Token, right: &Expr) -> bool {
        let left = left.accept(self);
        let right = right.accept(self);
        match op.token_type {
            TokenType::And => left && right,
            TokenType::Or => left || right,
            _ => false,
        }
    }

    fn visit_unary(&mut self, op: &Token, right: &Expr) -> bool {
        let right = right.accept(self);
        match op.token_type {
            TokenType::Not | TokenType::PreNot => !right,
            _ => false,
        }
    }

    fn visit_literal(&mut self, literal: &Literal) -> bool {
        literal.evaluate()
    }

    fn visit_variable(&mut self, t: &Token) -> bool {
        let var_expr = self.environment.get(t.lexeme.clone());
        match var_expr {
            Literal::Boolean(b) => b,
            _ => false,
        }
    }

    fn visit_assign(&mut self, t: &Token, e: &Expr) -> bool {
        let value = self.evaluate(e);
        self.environment.assign(t.lexeme.clone(), Literal::Boolean(value));
        value
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn update_environment(&mut self, environment: Environment) {
        self.environment = environment;
    }
    pub fn evaluate(&mut self, expr: &Expr) -> bool {
        expr.accept(self)
    }
}

