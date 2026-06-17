use std::collections::HashMap;

use crate::token::Token;
use crate::value::Value;

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> bool {
        if !self.values.contains_key(&name.lexeme) {
            return false;
        }

        self.values.insert(name.lexeme.clone(), value);

        true
    }

    pub fn define(&mut self, name: &Token, value: Value) {
        self.values.insert(name.lexeme.clone(), value);
    }

    pub fn get(&self, name: &Token) -> Option<Value> {
        self.values.get(&name.lexeme).cloned()
    }
}
