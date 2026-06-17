use std::fmt;

use crate::litteral::Literal;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,

    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TokenType::Eof => write!(f, "Token {:?}", self.kind),
            _ => write!(f, "Token {:?} ({})", self.kind, self.lexeme),
        }
    }
}

impl Token {
    pub fn new(kind: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Token {
        Token {
            kind: kind,
            lexeme: lexeme,
            literal: literal,

            line: line,
        }
    }

    pub fn from_literal(kind: TokenType, literal: Literal, line: usize) -> Token {
        Token::new(kind, literal.to_string(), Some(literal), line)
    }

    pub fn from_operand(kind: TokenType, lexeme: &str, line: usize) -> Token {
        Token::new(kind, lexeme.to_string(), None, line)
    }
}
