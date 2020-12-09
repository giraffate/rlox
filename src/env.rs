use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::lox_value::LoxValue;

#[derive(Clone, Debug)]
pub struct Env {
    values: HashMap<String, LoxValue>,
    pub enclosing: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn define(&mut self, k: String, v: LoxValue) -> Option<LoxValue> {
        self.values.insert(k, v)
    }

    pub fn assign(&mut self, k: String, v: LoxValue) -> Option<LoxValue> {
        if self.values.contains_key(&k) {
            self.values.insert(k, v)
        } else {
            match self.enclosing {
                Some(ref mut parent) => parent.borrow_mut().assign(k, v),
                None => None,
            }
        }
    }

    pub fn get(&self, k: &String) -> Option<LoxValue> {
        match self.values.get(k) {
            Some(v) => Some(v.clone()),
            None => match &self.enclosing {
                Some(parent) => parent.borrow().get(k),
                None => None,
            },
        }
    }
}
