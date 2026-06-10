use crate::{expr::Expr, stmt::Stmt, token::TokenType, types::Literal};

use std::fmt::{self, format};

#[derive(PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

fn is_equal(x: &Value, y: &Value) -> bool {
    return x == y;
}

pub enum RuntimeError {
    TypeError { message: String },
    OperationError { message: String },
    ParserError,
}

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    fn execute(&self, stmt: &Stmt) -> Option<RuntimeError> {
        match stmt {
            Stmt::Expr(expr) => self.visit_expr_stmt(expr),
            Stmt::Print(expr) => self.visit_print_stmt(expr),
        }
    }

    fn evaluate_binary(
        &self,
        left: &Expr,
        op: &TokenType,
        right: &Expr,
    ) -> Result<Value, RuntimeError> {
        let l_value = self.evaluate(left)?;
        let r_value = self.evaluate(right)?;

        match op {
            TokenType::Plus => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
                (Value::String(x), Value::String(y)) => Ok(Value::String(format!("{}{}", x, y))),
                _ => Err(RuntimeError::TypeError {
                    message: format!(
                        "Operands should be Number or String, got {} and {}",
                        l_value, r_value
                    ),
                }),
            },
            TokenType::Minus => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x - y)),
                _ => Err(RuntimeError::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                }),
            },
            TokenType::Star => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x * y)),
                _ => Err(RuntimeError::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                }),
            },
            TokenType::Slash => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => {
                    if *y == 0.0 {
                        return Err(RuntimeError::OperationError {
                            message: "Division by 0.0".to_string(),
                        });
                    }

                    return Ok(Value::Number(x / y));
                }
                _ => Err(RuntimeError::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                }),
            },
            TokenType::Greater => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x > y)),
                _ => Err(RuntimeError::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                }),
            },
            TokenType::GreaterEqual => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x >= y)),
                _ => Err(RuntimeError::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                }),
            },
            TokenType::Less => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x < y)),
                _ => Err(RuntimeError::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                }),
            },
            TokenType::LessEqual => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x <= y)),
                _ => Err(RuntimeError::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                }),
            },
            TokenType::EqualEqual => {
                if std::mem::discriminant(&l_value) == std::mem::discriminant(&r_value) {
                    return Ok(Value::Boolean(is_equal(&l_value, &r_value)));
                }

                Err(RuntimeError::TypeError {
                    message: format!(
                        "Operands should be of same type, got {} and {}",
                        l_value, r_value
                    ),
                })
            }
            TokenType::BangEqual => {
                if std::mem::discriminant(&l_value) == std::mem::discriminant(&r_value) {
                    return Ok(Value::Boolean(!is_equal(&l_value, &r_value)));
                }

                Err(RuntimeError::TypeError {
                    message: format!(
                        "Operands should be of same type, got {} and {}",
                        l_value, r_value
                    ),
                })
            }
            _ => Err(RuntimeError::ParserError),
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Binary { left, op, right } => self.evaluate_binary(left, &op.kind, right),
            Expr::Literal(litteral) => self.evaluate_litteral(litteral),
            Expr::Grouping(group) => self.evaluate(group),
            Expr::Unary { op, right } => self.evaluate_unary(&op.kind, right),
            Expr::Error => Err(RuntimeError::ParserError),
        }
    }

    fn evaluate_litteral(&self, litteral: &Literal) -> Result<Value, RuntimeError> {
        match litteral {
            Literal::Num(x) => Ok(Value::Number(*x)),
            Literal::Bool(b) => Ok(Value::Boolean(*b)),
            Literal::String(str) => Ok(Value::String(str.clone())),
            Literal::Nil => Ok(Value::Nil),
        }
    }

    fn evaluate_unary(&self, op: &TokenType, right: &Expr) -> Result<Value, RuntimeError> {
        let value = self.evaluate(right)?;

        match op {
            TokenType::Minus => {
                if let Value::Number(x) = value {
                    return Ok(Value::Number(-x));
                }

                return Err(RuntimeError::TypeError {
                    message: format!("Operand should be number, got {}", value),
                });
            }
            TokenType::Bang => {
                return Ok(Value::Boolean(!self.is_truthy(&value)));
            }
            _ => Err(RuntimeError::ParserError),
        }
    }

    pub fn interpret(&self, statments: &Vec<Stmt>) -> Option<RuntimeError> {
        for stmt in statments {
            self.execute(stmt)?;
        }

        None
    }

    fn is_truthy(&self, value: &Value) -> bool {
        if *value == Value::Nil {
            return false;
        }
        if let Value::Boolean(x) = value {
            return *x;
        }

        true
    }

    fn visit_expr_stmt(&self, expr: &Expr) -> Option<RuntimeError> {
        let value = self.evaluate(expr);

        match value {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    fn visit_print_stmt(&self, expr: &Expr) -> Option<RuntimeError> {
        let value = self.evaluate(expr);

        match value {
            Ok(x) => {
                println!("{}", x);
                None
            }
            Err(e) => Some(e),
        }
    }
}
