use std::collections::HashMap;

use crate::lox_value::LoxValue;

pub struct Env {
    values: HashMap<String, LoxValue>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            values: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: String, v: LoxValue) -> Option<LoxValue> {
        self.values.insert(k, v)
    }

    pub fn get(&self, k: &String) -> Option<&LoxValue> {
        self.values.get(k)
    }
}
