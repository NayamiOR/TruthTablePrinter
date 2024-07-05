use std::fmt::Display;
use crate::token_type::TokenType;

#[derive(Debug,Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Token{
    pub fn new(token_type: TokenType,lexeme: String) -> Self{
        Token{
            token_type,
            lexeme,
        }
    }
}

#[derive(Debug,Clone)]
pub enum Literal{
    Boolean(bool),
    Number(u64),
}

impl Literal{
    pub fn evaluate(&self) -> bool{
        match self{
            Literal::Boolean(b) => *b,
            Literal::Number(n) => *n != 0,
        }
    }
}

impl Display for Literal{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Literal::Boolean(b) => write!(f,"{}",b),
            Literal::Number(n) => write!(f,"{}",n),
        }
    }
}