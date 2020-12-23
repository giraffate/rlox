use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;
use crate::lox_class::{LoxClass, LoxClassInner};
use crate::lox_value::LoxValue;
use crate::token::Token;

#[derive(Debug)]
pub struct LoxInstance {
    klass: Rc<LoxClassInner>,
    field: HashMap<String, LoxValue>,
}

impl LoxInstance {
    pub fn new(klass: &LoxClass) -> LoxInstance {
        LoxInstance {
            klass: klass.inner.clone(),
            field: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<LoxValue, Error> {
        match self.field.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => Err(Error {
                kind: "runtime error".to_string(),
                msg: format!("undefined property: {}", name.lexeme),
            }),
        }
    }

    pub fn set(&mut self, name: &Token, value: LoxValue) -> Result<LoxValue, Error> {
        self.field.insert(name.lexeme.clone(), value);
        Ok(LoxValue::Nil)
    }
}
