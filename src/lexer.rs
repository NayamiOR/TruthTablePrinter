use crate::boolean::Boolean;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Lexer {
    input: String,
    start: usize,
    current: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            start: 0,
            current: 0,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "".to_string()));
        &self.tokens
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token_by_type(TokenType::LeftParen),
            ')' => self.add_token_by_type(TokenType::RightParen),
            '+' | '|' => self.add_token_by_type(TokenType::Or),
            '*' | '&' => self.add_token_by_type(TokenType::And),
            '\'' => self.add_token_by_type(TokenType::Not),
            '!' => self.add_token_by_type(TokenType::PreNot),
            ';' => self.add_token_by_type(TokenType::Semicolon),
            '=' => self.add_token_by_type(TokenType::Eq),
            ' ' | '\r' | '\t' | '\n' => (),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => Boolean::sim_error("Unexpected character."),
        }
    }
    
    pub fn add_token(&mut self, token:Token){
        self.tokens.push(token);
    }

    pub fn add_token_by_type(&mut self, token_type: TokenType) {
        let text = self.input[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text));
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.input.chars().nth(self.current - 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.input.chars().nth(self.current).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        let text = &self.input[self.start..self.current];
        self.add_token(Token::new(TokenType::Num, text.to_string()));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text=&self.input[self.start..self.current];
        match text {
            "true" => self.add_token_by_type(TokenType::True),
            "false" => self.add_token_by_type(TokenType::False),
            _ => self.add_token_by_type(TokenType::Identifier),
        }
    }
}
