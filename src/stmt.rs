use std::fmt;

use crate::expr::{Expr, print_ast};

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", print_stmt(self))
    }
}

pub fn print_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Expr(expr) => print_ast(expr),
        Stmt::Print(expr) => {
            return "print(".to_string() + print_ast(expr).as_str() + ")";
        }
    }
}
