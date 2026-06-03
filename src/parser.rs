use crate::expr::Expr;
use crate::token::{Token, TokenType};
use crate::types::Literal;

struct ParserError {
    line: usize,
    msg: String,
    token: Token,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn check(&self, kind: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().kind == kind
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.matches(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();

            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn consume(&mut self, kind: TokenType) -> bool {
        if self.check(kind) {
            self.advance();
            return true;
        }

        false
    }

    fn error(&mut self, msg: String, token: Token) {
        self.errors.push(ParserError {
            line: token.line,
            msg: msg,
            token: token,
        });
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();

            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.matches(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    fn matches(&mut self, kinds: Vec<TokenType>) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn primary(&mut self) -> Expr {
        if self.matches(vec![TokenType::False]) {
            return Expr::Literal(Literal::Bool(false));
        }
        if self.matches(vec![TokenType::True]) {
            return Expr::Literal(Literal::Bool(true));
        }
        if self.matches(vec![TokenType::Nil]) {
            return Expr::Literal(Literal::Nil);
        }

        if self.matches(vec![TokenType::Number, TokenType::String]) {
            return Expr::Literal(self.previous().literal.unwrap());
        }

        if self.matches(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            if !self.consume(TokenType::RightParen) {
                let token = self.peek();
                let msg = format!("[Line {}] Expected ')', found {}", token.line, token.lexeme);
                self.error(msg, token);

                self.synchronize();

                return Expr::Error;
            }

            return Expr::Grouping(Box::new(expr));
        }

        let token = self.peek();
        let msg = format!(
            "[Line {}] Expected expression, found {}",
            token.line, token.lexeme
        );
        self.error(msg, token);
        self.synchronize();

        return Expr::Error;
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenType::Semicolon {
                return;
            }

            match self.peek().kind {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.matches(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();

            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();

            return Expr::Unary {
                op: operator,
                right: Box::new(right),
            };
        }

        self.primary()
    }
}
