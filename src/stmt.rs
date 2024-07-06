use crate::expr::Expr;

#[derive(Debug,Clone)]
pub enum Stmt{
    Assign{
        name: String,
        value: Expr,
    },
    Expression{
        value: Expr,
    },
}