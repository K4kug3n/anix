use crate::expr::Expr;
use crate::litteral::Literal;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};

pub struct ParserError {
    pub line: usize,
    pub msg: String,
    pub token: Token,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub errors: Vec<ParserError>,
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

    fn assignment(&mut self) -> Expr {
        let expr = self.equality();
        if self.matches(vec![TokenType::Equal]) {
            let equal = self.previous();
            let value = self.assignment();

            if let Expr::Variable(name) = expr {
                return Expr::Assign {
                    name: name,
                    value: Box::new(value),
                };
            }

            let msg = format!("Invalid assignment target, found {}", expr);
            self.error(msg, equal.clone());

            self.synchronize();

            return Expr::Error(equal);
        }

        expr
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

    fn declaration(&mut self) -> Stmt {
        if self.matches(vec![TokenType::Var]) {
            return self.var_declaration();
        }

        return self.statement();
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
        self.assignment()
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        if !self.consume(TokenType::Semicolon) {
            let token = self.peek();
            let msg = format!("Expected ';' found {}", token.lexeme);
            self.error(msg, token.clone());

            self.synchronize();

            return Stmt::Expr(Expr::Error(token));
        }

        Stmt::Expr(expr)
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

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statments = Vec::new();
        while !self.is_at_end() {
            statments.push(self.declaration());
        }

        statments
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

        if self.matches(vec![TokenType::Identifier]) {
            return Expr::Variable(self.previous());
        }

        if self.matches(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            if !self.consume(TokenType::RightParen) {
                let token = self.peek();
                let msg = format!("Expected ')', found {}", token.lexeme);
                self.error(msg, token.clone());

                self.synchronize();

                return Expr::Error(token);
            }

            return Expr::Grouping(Box::new(expr));
        }

        let token = self.peek();
        let msg = format!("Expected expression, found {}", token.lexeme);
        self.error(msg, token.clone());
        self.synchronize();

        return Expr::Error(token);
    }

    fn print_statement(&mut self) -> Stmt {
        let expr = self.expression();
        if !self.consume(TokenType::Semicolon) {
            let token = self.peek();
            let msg = format!("Expected ';' found {}", token.lexeme);
            self.error(msg, token.clone());

            self.synchronize();

            return Stmt::Expr(Expr::Error(token));
        }

        Stmt::Print(expr)
    }

    fn statement(&mut self) -> Stmt {
        if self.matches(vec![TokenType::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
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

    fn var_declaration(&mut self) -> Stmt {
        if !self.consume(TokenType::Identifier) {
            let token = self.peek();

            let msg = format!("Expected variable name, found {}", token.lexeme);
            self.error(msg, token.clone());

            self.synchronize();

            return Stmt::Expr(Expr::Error(token));
        }

        let name = self.previous();
        let mut init = None;
        if self.matches(vec![TokenType::Equal]) {
            init = Some(self.expression());
        }

        if !self.consume(TokenType::Semicolon) {
            let token = self.peek();
            let msg = format!("Expected ';' found {}", token.lexeme);
            self.error(msg, token.clone());

            self.synchronize();

            return Stmt::Expr(Expr::Error(token));
        }

        return Stmt::Var { name, init };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_tokens(mut tokens: Vec<Token>) -> Vec<Stmt> {
        // Add EOF token for valid sequence
        tokens.push(Token::new(TokenType::Eof, "".to_string(), None, 0));

        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_number() {
        let value = 12.3;
        let num = Token::from_literal(TokenType::Number, Literal::Num(value), 0);
        let sc = Token::from_operand(TokenType::Semicolon, ";", 0);

        let tokens = vec![num, sc];
        let stmts = parse_tokens(tokens);

        assert_eq!(stmts.len(), 1);
        assert_eq!(stmts[0], Stmt::Expr(Expr::Literal(Literal::Num(value))))
    }

    #[test]
    fn test_number_product() {
        let value = 12.3;
        let op = Token::from_operand(TokenType::Star, "*", 0);
        let num = Token::from_literal(TokenType::Number, Literal::Num(value), 0);
        let sc = Token::from_operand(TokenType::Semicolon, ";", 0);

        let tokens = vec![num.clone(), op.clone(), num.clone(), sc];
        let stmts = parse_tokens(tokens);

        assert_eq!(stmts.len(), 1);
        assert_eq!(
            stmts[0],
            Stmt::Expr(Expr::Binary {
                left: Box::new(Expr::Literal(Literal::Num(value))),
                op: op,
                right: Box::new(Expr::Literal(Literal::Num(value))),
            })
        )
    }

    #[test]
    fn test_bool_gt() {
        let value = true;
        let boolean = Token::from_operand(TokenType::True, "true", 0);
        let op = Token::from_operand(TokenType::GreaterEqual, ">=", 0);
        let sc = Token::from_operand(TokenType::Semicolon, ";", 0);

        let tokens = vec![boolean.clone(), op.clone(), boolean.clone(), sc];
        let stmts = parse_tokens(tokens);

        assert_eq!(stmts.len(), 1);
        assert_eq!(
            stmts[0],
            Stmt::Expr(Expr::Binary {
                left: Box::new(Expr::Literal(Literal::Bool(value))),
                op,
                right: Box::new(Expr::Literal(Literal::Bool(value)))
            })
        );
    }

    #[test]
    fn test_unary_minus() {
        let value = 12.3;
        let num = Token::from_literal(TokenType::Number, Literal::Num(value), 0);
        let op = Token::from_operand(TokenType::Minus, "-", 0);
        let sc = Token::from_operand(TokenType::Semicolon, ";", 0);

        let tokens = vec![op.clone(), num, sc];
        let stmts = parse_tokens(tokens);

        assert_eq!(stmts.len(), 1);
        assert_eq!(
            stmts[0],
            Stmt::Expr(Expr::Unary {
                op,
                right: Box::new(Expr::Literal(Literal::Num(value)))
            })
        )
    }

    #[test]
    fn test_operator_precedence_product() {
        let value1 = 12.3;
        let num1 = Token::from_literal(TokenType::Number, Literal::Num(value1), 0);
        let op1 = Token::from_operand(TokenType::Plus, "+", 0);
        let value2 = 2.13;
        let num2 = Token::from_literal(TokenType::Number, Literal::Num(value2), 0);
        let op2 = Token::from_operand(TokenType::Star, "*", 0);
        let value3 = 31.2;
        let num3 = Token::from_literal(TokenType::Number, Literal::Num(value3), 0);
        let sc = Token::from_operand(TokenType::Semicolon, ";", 0);

        let tokens = vec![num1, op1.clone(), num2, op2.clone(), num3, sc];
        let stmts = parse_tokens(tokens);

        assert_eq!(stmts.len(), 1);
        assert_eq!(
            stmts[0],
            Stmt::Expr(Expr::Binary {
                left: Box::new(Expr::Literal(Literal::Num(value1))),
                op: op1,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal(Literal::Num(value2))),
                    op: op2,
                    right: Box::new(Expr::Literal(Literal::Num(value3))),
                })
            })
        );
    }

    #[test]
    fn test_grouping() {
        let l_par = Token::from_operand(TokenType::LeftParen, "(", 0);
        let value1 = 12.3;
        let num1 = Token::from_literal(TokenType::Number, Literal::Num(value1), 0);
        let op1 = Token::from_operand(TokenType::Plus, "+", 0);
        let value2 = 2.13;
        let num2 = Token::from_literal(TokenType::Number, Literal::Num(value2), 0);
        let r_par = Token::from_operand(TokenType::RightParen, ")", 0);
        let op2 = Token::from_operand(TokenType::Star, "*", 0);
        let value3 = 31.2;
        let num3 = Token::from_literal(TokenType::Number, Literal::Num(value3), 0);
        let sc = Token::from_operand(TokenType::Semicolon, ";", 0);

        let tokens = vec![l_par, num1, op1.clone(), num2, r_par, op2.clone(), num3, sc];
        let stmts = parse_tokens(tokens);

        assert_eq!(stmts.len(), 1);
        assert_eq!(
            stmts[0],
            Stmt::Expr(Expr::Binary {
                left: Box::new(Expr::Grouping(Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal(Literal::Num(value1))),
                    op: op1,
                    right: Box::new(Expr::Literal(Literal::Num(value2)))
                }))),
                op: op2,
                right: Box::new(Expr::Literal(Literal::Num(value3)))
            })
        );
    }

    #[test]
    fn test_print_stmt() {
        let print = Token::from_operand(TokenType::Print, "print", 0);
        let value1 = 12.3;
        let num1 = Token::from_literal(TokenType::Number, Literal::Num(value1), 0);
        let op = Token::from_operand(TokenType::Plus, "+", 0);
        let value2 = 2.13;
        let num2 = Token::from_literal(TokenType::Number, Literal::Num(value2), 0);
        let sc = Token::from_operand(TokenType::Semicolon, ";", 0);

        let tokens = vec![print, num1, op.clone(), num2, sc];
        let stmts = parse_tokens(tokens);

        assert_eq!(stmts.len(), 1);
        assert_eq!(
            stmts[0],
            Stmt::Print(Expr::Binary {
                left: Box::new(Expr::Literal(Literal::Num(value1))),
                op: op.clone(),
                right: Box::new(Expr::Literal(Literal::Num(value2)))
            })
        );
    }

    #[test]
    fn test_var_init_stmt() {
        let str = "var1";
        let name = Token::from_literal(TokenType::Identifier, Literal::String(str.to_string()), 0);
        let equal = Token::from_operand(TokenType::Equal, "=", 0);
        let value = 12.3;
        let num = Token::from_literal(TokenType::Number, Literal::Num(value), 0);

        let var = Token::from_operand(TokenType::Var, "var", 0);
        let sc = Token::from_operand(TokenType::Semicolon, ";", 0);

        let tokens = vec![var, name.clone(), equal, num, sc];
        let stmts = parse_tokens(tokens);

        assert_eq!(stmts.len(), 1);
        assert_eq!(
            stmts[0],
            Stmt::Var {
                name,
                init: Some(Expr::Literal(Literal::Num(value)))
            }
        );
    }

    #[test]
    fn test_var_uninit_stmt() {
        let str = "var1";
        let name = Token::from_literal(TokenType::Identifier, Literal::String(str.to_string()), 0);

        let var = Token::from_operand(TokenType::Var, "var", 0);
        let sc = Token::from_operand(TokenType::Semicolon, ";", 0);

        let tokens = vec![var, name.clone(), sc];
        let stmts = parse_tokens(tokens);

        assert_eq!(stmts.len(), 1);
        assert_eq!(stmts[0], Stmt::Var { name, init: None });
    }

    #[test]
    fn test_multiple_stmt() {
        let value1 = 12.3;
        let num1 = Token::from_literal(TokenType::Number, Literal::Num(value1), 0);
        let op1 = Token::from_operand(TokenType::Plus, "+", 0);
        let value2 = 2.13;
        let num2 = Token::from_literal(TokenType::Number, Literal::Num(value2), 0);

        let print = Token::from_operand(TokenType::Print, "print", 0);
        let value3 = 31.2;
        let num3 = Token::from_literal(TokenType::Number, Literal::Num(value3), 0);
        let op2 = Token::from_operand(TokenType::Star, "*", 0);
        let value4 = 23.1;
        let num4 = Token::from_literal(TokenType::Number, Literal::Num(value4), 0);

        let sc = Token::from_operand(TokenType::Semicolon, ";", 0);

        let tokens = vec![
            num1,
            op1.clone(),
            num2,
            sc.clone(),
            print,
            num3,
            op2.clone(),
            num4,
            sc,
        ];
        let stmts = parse_tokens(tokens);

        assert_eq!(stmts.len(), 2);
        assert_eq!(
            stmts[0],
            Stmt::Expr(Expr::Binary {
                left: Box::new(Expr::Literal(Literal::Num(value1))),
                op: op1,
                right: Box::new(Expr::Literal(Literal::Num(value2)))
            })
        );
        assert_eq!(
            stmts[1],
            Stmt::Print(Expr::Binary {
                left: Box::new(Expr::Literal(Literal::Num(value3))),
                op: op2,
                right: Box::new(Expr::Literal(Literal::Num(value4)))
            })
        );
    }
}
