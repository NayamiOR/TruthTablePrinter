use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::{Literal, Token};
use crate::token_type::TokenType;
use crate::token_type::TokenType::Semicolon;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, &'static str> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, &'static str> {
        self.assignment()
    }

    fn statement(&mut self) -> Result<Stmt, &'static str> {
        if self.check(TokenType::Identifier) && self.check_next(TokenType::Eq) {
            return self.assign_statement();
        }
        self.expression_statement()
    }

    fn assign_statement(&mut self) -> Result<Stmt, &'static str> {
        let name = self.advance().lexeme.clone();
        self.advance();  // consume '='
        let value = self.expression();
        let Ok(v) = value else { return Err("Expect value."); };
        self.consume(Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Assign { name, value: v })
    }

    fn expression_statement(&mut self) -> Result<Stmt, &'static str> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression { value: expr })
    }

    fn assignment(&mut self) -> Result<Expr, &'static str> {
        let expr = self.term()?;
        if self.match_token(vec![TokenType::Eq]) {
            let value = self.assignment()?;

            return if let Expr::Variable(name) = expr {
                Ok(Expr::Assign(name, Box::new(value)))
            } else {
                Err("Invalid assignment target.")
            }
        }
        Ok(expr)
    }
    fn term(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.factor()?;
        while self.match_token(vec![TokenType::Or]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.unary()?;
        while self.match_token(vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, &'static str> {
        if self.check(TokenType::PreNot) {
            let operator = self.advance();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        let mut expr = self.primary()?;
        while self.match_token(vec![TokenType::Not]) {
            let operator = self.previous();
            expr = Expr::Unary(operator, Box::new(expr));
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![TokenType::False]) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }
        if self.match_token(vec![TokenType::True]) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }
        if self.match_token(vec![TokenType::Num]) {
            return Ok(Expr::Literal(Literal::Number(self.previous().lexeme.parse().unwrap())));
        }

        if self.match_token(vec![TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous()));
        }

        if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err("Expect expression.")
    }

    fn match_token(&mut self, token_type: Vec<TokenType>) -> bool {
        for token in token_type {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &'static str) -> Result<Token, &'static str> {
        if !self.check(token_type) {
            return Err(message);
        }
        Ok(self.advance())
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn check_next(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek_next().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn peek_next(&self) -> Token {
        if self.is_at_end() {
            return Token::new(TokenType::Eof, "".to_string());
        }
        self.tokens[self.current + 1].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}