// use std::fmt;

use crate::token::{Token, TokenType};

pub enum Object {
    Num(f64),
    String(String),
    Bool(bool),
    Nil,
}

// impl fmt::Display for Token {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Token {:?} ({})", self.kind, self.lexeme)
//     }
// }

pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Object),
    Unary {
        op: Token,
        right: Box<Expr>,
    },
}

// pub trait Visitor<R> {
//     fn visit_expr(&self, expr: Expr) -> R;
// }

fn parenthesize(name: String, exprs: Vec<Expr>) -> String {
    let mut result = "(".to_string() + name.as_str();
    for expr in exprs {
        result += " ";
        result += print_ast(expr).as_str();
    }
    result += ")";

    result
}

pub fn print_ast(expr: Expr) -> String {
    match expr {
        Expr::Binary { left, op, right } => {
            return parenthesize(op.lexeme, vec![*left, *right]);
        }
        Expr::Grouping(inside) => {
            return parenthesize("group".to_string(), vec![*inside]);
        }
        Expr::Literal(object) => match object {
            Object::String(v) => v,
            Object::Num(v) => v.to_string(),
            Object::Bool(v) => v.to_string(),
            Object::Nil => "Nil".to_string(),
        },
        Expr::Unary { op, right } => {
            return parenthesize(op.lexeme, vec![*right]);
        }
    }
}

fn rpn_parenthesize(name: String, exprs: Vec<Expr>) -> String {
    let mut result = "".to_string();
    for expr in exprs {
        result += print_rpn_ast(expr).as_str();
        result += " ";
    }
    result += name.as_str();

    result
}

pub fn print_rpn_ast(expr: Expr) -> String {
    match expr {
        Expr::Binary { left, op, right } => {
            return rpn_parenthesize(op.lexeme, vec![*left, *right]);
        }
        Expr::Grouping(inside) => {
            // return rpn_parenthesize("".to_string(), vec![*inside]);
            return print_rpn_ast(*inside);
        }
        Expr::Literal(object) => match object {
            Object::String(v) => v,
            Object::Num(v) => v.to_string(),
            Object::Bool(v) => v.to_string(),
            Object::Nil => "Nil".to_string(),
        },
        Expr::Unary { op, right } => {
            if op.kind == TokenType::Minus {
                return rpn_parenthesize("neg".to_string(), vec![*right]);
            }
            return rpn_parenthesize(op.lexeme, vec![*right]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let expression = Expr::Binary {
            left: Box::new(Expr::Unary {
                op: Token {
                    kind: TokenType::Minus,
                    lexeme: "-".to_string(),
                    line: 1,
                },
                right: Box::new(Expr::Literal(Object::Num(123.0))),
            }),
            op: Token {
                kind: TokenType::Star,
                lexeme: "*".to_string(),
                line: 1,
            },
            right: Box::new(Expr::Grouping(Box::new(Expr::Literal(Object::Num(45.67))))),
        };

        let result = print_ast(expression);

        assert_eq!(result, "(* (- 123) (group 45.67))");
    }

    #[test]
    fn test_rpn_print() {
        let expression = Expr::Binary {
            left: Box::new(Expr::Grouping(Box::new(Expr::Binary {
                left: Box::new(Expr::Literal(Object::Num(1.0))),
                op: Token {
                    kind: TokenType::Plus,
                    lexeme: "+".to_string(),
                    line: 1,
                },
                right: Box::new(Expr::Literal(Object::Num(2.0))),
            }))),
            op: Token {
                kind: TokenType::Star,
                lexeme: "*".to_string(),
                line: 1,
            },
            right: Box::new(Expr::Grouping(Box::new(Expr::Binary {
                left: Box::new(Expr::Literal(Object::Num(3.0))),
                op: Token {
                    kind: TokenType::Plus,
                    lexeme: "-".to_string(),
                    line: 1,
                },
                right: Box::new(Expr::Literal(Object::Num(4.0))),
            }))),
        };

        let result = print_rpn_ast(expression);

        assert_eq!(result, "1 2 + 3 4 - *");
    }
}
