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

    fn peek(&self) -> char {
        if self.is_end() {
            return '\0';
        }

        match self.source.chars().nth(self.current) {
            Some(c) => return c,
            None => panic!("Error: scanner::peek out-of-range"),
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
            _ => self.add_error("Unexpected lexeme".to_string()),
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
}
