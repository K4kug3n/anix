use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::token::Token;
use crate::token::TokenType;

#[derive(Clone)]
pub struct ScanError {
    pub line: usize,
    pub message: String,
}

pub struct Scanner {
    source: String,
    tokens: Vec<Result<Token, ScanError>>,

    start: usize,
    current: usize,
    line: usize,
}

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and".to_string(), TokenType::And);
        m.insert("class".to_string(), TokenType::Class);
        m.insert("else".to_string(), TokenType::Else);
        m.insert("false".to_string(), TokenType::False);
        m.insert("for".to_string(), TokenType::For);
        m.insert("fun".to_string(), TokenType::Fun);
        m.insert("if".to_string(), TokenType::If);
        m.insert("nil".to_string(), TokenType::Nil);
        m.insert("or".to_string(), TokenType::Or);
        m.insert("print".to_string(), TokenType::Print);
        m.insert("return".to_string(), TokenType::Return);
        m.insert("super".to_string(), TokenType::Super);
        m.insert("this".to_string(), TokenType::This);
        m.insert("true".to_string(), TokenType::True);
        m.insert("var".to_string(), TokenType::Var);
        m.insert("while".to_string(), TokenType::While);

        m
    };
}

impl Scanner {
    pub fn new(source: &String) -> Scanner {
        Scanner {
            source: source.clone(),
            tokens: Vec::new(),

            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn add_token(&mut self, kind: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Ok(Token::new(kind, text.to_string(), self.line)));
    }

    fn add_error(&mut self, message: String) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Err(ScanError {
            line: self.line,
            message: format!("{} {}", message, text),
        }));
    }

    fn advance(&mut self) -> char {
        let v = self.source.chars().nth(self.current);
        self.current += 1;

        match v {
            Some(c) => return c,
            None => panic!("Error: scanner::advance out-of-range"),
        }
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        match KEYWORDS.get(text) {
            Some(kind) => self.add_token(kind.clone()),
            _ => self.add_token(TokenType::Identifier(text.to_string())),
        }
    }

    fn is_alpha(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alphanumeric(c: char) -> bool {
        return Scanner::is_digit(c) || Scanner::is_alpha(c);
    }

    fn is_digit(c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }

        match self.source.chars().nth(self.current) {
            Some(c) => {
                if c != expected {
                    return false;
                }
                self.current += 1;
                return true;
            }
            None => return false,
        }
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let text = &self.source[self.start..self.current];
        let value = match text.parse() {
            Ok(v) => v,
            Err(_) => panic!("Error: scanner::number unable to parse"),
        };

        self.add_token(TokenType::Number(value));
    }

    fn peek(&self) -> char {
        if self.is_end() {
            return '\0';
        }

        match self.source.chars().nth(self.current) {
            Some(c) => return c,
            None => panic!("Error: scanner::peek out-of-range"),
        }
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.source.len() {
            return '\0';
        }

        match self.source.chars().nth(self.current + 1) {
            Some(c) => return c,
            None => panic!("Error: scanner::peek_next out-of-range"),
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.matches('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && self.is_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if Scanner::is_digit(c) {
                    self.number()
                } else if Scanner::is_alpha(c) {
                    self.identifier();
                } else {
                    self.add_error("Unexpected lexeme".to_string());
                }
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Result<Token, ScanError>> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Ok(Token::new(TokenType::Eof, "".to_string(), 0)));

        self.tokens.clone()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() {
            self.add_error("Unterminated string".to_string());
            return;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String(value.to_string()));
    }
}
