use crate::environment::Environment;
use crate::expr::Expr;
use crate::litteral::Literal;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};
use crate::value::Value;

use std::fmt::{self, format};

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

pub enum RuntimeErrorType {
    TypeError { message: String },
    OperationError { message: String },
    UndefinedVariable,
    ParserError,
}

pub struct RuntimeError {
    pub kind: RuntimeErrorType,
    pub token: Token,
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expr(expr) => self.visit_expr_stmt(expr),
            Stmt::Print(expr) => self.visit_print_stmt(expr),
            Stmt::Var { name, init } => self.visit_var_stmt(name, init),
        }
    }

    fn evaluate_assign(&mut self, name: &Token, expr: &Expr) -> Result<Value, RuntimeError> {
        let value = self.evaluate(expr)?;
        if !self.environment.assign(name, value.clone()) {
            return Err(RuntimeError {
                kind: RuntimeErrorType::UndefinedVariable,
                token: name.clone(),
            });
        }

        Ok(value)
    }

    fn evaluate_binary(
        &mut self,
        left: &Expr,
        op: &Token,
        right: &Expr,
    ) -> Result<Value, RuntimeError> {
        let l_value = self.evaluate(left)?;
        let r_value = self.evaluate(right)?;

        let make_error = |kind: RuntimeErrorType| RuntimeError {
            token: op.clone(),
            kind: kind,
        };

        match op.kind {
            TokenType::Plus => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
                (Value::String(x), Value::String(y)) => Ok(Value::String(format!("{}{}", x, y))),
                _ => Err(make_error(RuntimeErrorType::TypeError {
                    message: format!(
                        "Operands should be Number or String, got {} and {}",
                        l_value, r_value
                    ),
                })),
            },
            TokenType::Minus => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x - y)),
                _ => Err(make_error(RuntimeErrorType::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                })),
            },
            TokenType::Star => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x * y)),
                _ => Err(make_error(RuntimeErrorType::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                })),
            },
            TokenType::Slash => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => {
                    if *y == 0.0 {
                        return Err(make_error(RuntimeErrorType::OperationError {
                            message: "Division by 0.0".to_string(),
                        }));
                    }

                    return Ok(Value::Number(x / y));
                }
                _ => Err(make_error(RuntimeErrorType::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                })),
            },
            TokenType::Greater => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x > y)),
                _ => Err(make_error(RuntimeErrorType::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                })),
            },
            TokenType::GreaterEqual => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x >= y)),
                _ => Err(make_error(RuntimeErrorType::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                })),
            },
            TokenType::Less => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x < y)),
                _ => Err(make_error(RuntimeErrorType::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                })),
            },
            TokenType::LessEqual => match (&l_value, &r_value) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x <= y)),
                _ => Err(make_error(RuntimeErrorType::TypeError {
                    message: format!("Operands should be Number, got {} and {}", l_value, r_value),
                })),
            },
            TokenType::EqualEqual => {
                if std::mem::discriminant(&l_value) == std::mem::discriminant(&r_value) {
                    return Ok(Value::Boolean(is_equal(&l_value, &r_value)));
                }

                Err(make_error(RuntimeErrorType::TypeError {
                    message: format!(
                        "Operands should be of same type, got {} and {}",
                        l_value, r_value
                    ),
                }))
            }
            TokenType::BangEqual => {
                if std::mem::discriminant(&l_value) == std::mem::discriminant(&r_value) {
                    return Ok(Value::Boolean(!is_equal(&l_value, &r_value)));
                }

                Err(make_error(RuntimeErrorType::TypeError {
                    message: format!(
                        "Operands should be of same type, got {} and {}",
                        l_value, r_value
                    ),
                }))
            }
            _ => Err(make_error(RuntimeErrorType::ParserError)),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Assign { name, value } => self.evaluate_assign(name, value),
            Expr::Binary { left, op, right } => self.evaluate_binary(left, &op, right),
            Expr::Literal(litteral) => self.evaluate_litteral(litteral),
            Expr::Grouping(group) => self.evaluate(group),
            Expr::Unary { op, right } => self.evaluate_unary(&op, right),
            Expr::Variable(name) => self.evaluate_variable(name),
            Expr::Error(token) => Err(RuntimeError {
                kind: RuntimeErrorType::ParserError,
                token: token.clone(),
            }),
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

    fn evaluate_unary(&mut self, op: &Token, right: &Expr) -> Result<Value, RuntimeError> {
        let value = self.evaluate(right)?;

        match op.kind {
            TokenType::Minus => {
                if let Value::Number(x) = value {
                    return Ok(Value::Number(-x));
                }

                return Err(RuntimeError {
                    kind: RuntimeErrorType::TypeError {
                        message: format!("Operand should be number, got {}", value),
                    },
                    token: op.clone(),
                });
            }
            TokenType::Bang => {
                return Ok(Value::Boolean(!self.is_truthy(&value)));
            }
            _ => Err(RuntimeError {
                kind: RuntimeErrorType::ParserError,
                token: op.clone(),
            }),
        }
    }

    fn evaluate_variable(&self, name: &Token) -> Result<Value, RuntimeError> {
        match self.environment.get(name) {
            Some(value) => Ok(value),
            None => Err(RuntimeError {
                kind: RuntimeErrorType::UndefinedVariable,
                token: name.clone(),
            }),
        }
    }

    pub fn interpret(&mut self, statments: &Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in statments {
            self.execute(stmt)?;
        }

        Ok(())
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

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        self.evaluate(expr)?;

        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        let value = self.evaluate(expr)?;
        println!("{}", value);

        Ok(())
    }

    fn visit_var_stmt(&mut self, name: &Token, init: &Option<Expr>) -> Result<(), RuntimeError> {
        let value = match init {
            Some(expr) => self.evaluate(&expr)?,
            None => Value::Nil,
        };

        self.environment.define(name, value);

        Ok(())
    }
}
