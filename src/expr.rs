use std::fmt;

use crate::litteral::Literal;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Error,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", print_ast(self))
    }
}

fn parenthesize(name: &str, exprs: Vec<&Expr>) -> String {
    let mut result = "(".to_string() + name;
    for expr in exprs {
        result += " ";
        result += print_ast(expr).as_str();
    }
    result += ")";

    result
}

pub fn print_ast(expr: &Expr) -> String {
    match expr {
        Expr::Binary { left, op, right } => {
            return parenthesize(&op.lexeme, vec![left, right]);
        }
        Expr::Grouping(inside) => {
            return parenthesize("group", vec![inside]);
        }
        Expr::Literal(literal) => match literal {
            Literal::String(v) => v.to_string(),
            Literal::Num(v) => v.to_string(),
            Literal::Bool(v) => v.to_string(),
            Literal::Nil => "Nil".to_string(),
        },
        Expr::Unary { op, right } => {
            return parenthesize(&op.lexeme, vec![right]);
        }
        Expr::Error => {
            return "Error".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_print() {
        let expression = Expr::Binary {
            left: Box::new(Expr::Unary {
                op: Token {
                    kind: TokenType::Minus,
                    lexeme: "-".to_string(),
                    literal: None,
                    line: 1,
                },
                right: Box::new(Expr::Literal(Literal::Num(123.0))),
            }),
            op: Token {
                kind: TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Num(45.67))))),
        };

        let result = print_ast(&expression);

        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
