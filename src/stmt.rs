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

pub trait StmtVisitor<R>{
    fn visit_assign_stmt(&mut self,name: &String,value: &Expr) -> R;
    fn visit_expression_stmt(&mut self,value: &Expr) -> R;
}

impl Stmt{
    pub fn accept<R>(&self,visitor: &mut impl StmtVisitor<R>) -> R{
        match self{
            Stmt::Assign{name,value} => visitor.visit_assign_stmt(name,value),
            Stmt::Expression{value} => visitor.visit_expression_stmt(value),
            _ => panic!("Invalid statement"),
        }
    }
}