use crate::environment::Environment;
use crate::expr::{Expr, ExprVisitor};
use crate::stmt::Stmt;
use crate::token::{Literal, Token};

pub struct AssignParser {
    pub environment: Environment,
    pub dependent_var: String,
    pub independent_vars: Vec<String>,
}

pub struct AnalysisResult {
    pub dependent_var: String,
    pub independent_vars: Vec<String>,
    pub environment: Environment,
    pub expr: Expr,
}

impl AssignParser {
    pub fn new() -> Self {
        let dependent_vars = Vec::new();
        let independent_var = "".to_string();
        AssignParser {
            environment: Environment::new(),
            dependent_var: independent_var,
            independent_vars: dependent_vars,
        }
    }
}

impl ExprVisitor<()> for AssignParser {
    fn visit_grouping(&mut self, expr: &Expr) {
        expr.accept(self);
    }
    fn visit_binary(&mut self, left: &Expr, _: &Token, right: &Expr) {
        left.accept(self);
        right.accept(self);
    }
    fn visit_unary(&mut self, _: &Token, right: &Expr) {
        right.accept(self);
    }
    fn visit_literal(&mut self, _: &Literal) {}
    fn visit_variable(&mut self, t: &Token) {
        if !self.independent_vars.contains(&t.lexeme) {
            self.environment.define(t.lexeme.clone(), Literal::Boolean(false));
            self.independent_vars.push(t.lexeme.clone());
        }
    }
    fn visit_assign(&mut self, _: &Token, e: &Expr) {
        e.accept(self);
    }
}

impl AssignParser {
    pub fn analyze(&mut self, stmt: Stmt) -> Result<AnalysisResult, &'static str> {
        let Stmt::Assign { name, value } = stmt
        else {
            return Err("Expected Assign statement");
        };

        value.accept(self);
        Ok(AnalysisResult {
            dependent_var: name.clone(),
            independent_vars: self.independent_vars.clone(),
            environment: self.environment.clone(),
            expr: value.clone(),
        })
    }
}