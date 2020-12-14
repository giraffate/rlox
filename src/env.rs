use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;
use crate::lox_value::LoxValue;

#[derive(Clone, Debug)]
pub struct Env {
    pub values: HashMap<String, LoxValue>,
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

    pub fn assign_at(&mut self, distance: i32, k: String, v: LoxValue) -> Option<LoxValue> {
        let mut ret_env = match self.enclosing.clone() {
            Some(parent_env) => parent_env.clone(),
            _ => return None,
        };
        for _ in 1..distance {
            let new_env = match ret_env.borrow().enclosing {
                Some(ref parent_env) => parent_env.clone(),
                None => return None,
            };
            ret_env = new_env;
        }
        let ret = ret_env.borrow_mut().assign(k, v);
        ret
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

    pub fn get_at(&self, name: String, distance: i32) -> Option<LoxValue> {
        let mut ret_env = match self.enclosing.clone() {
            Some(parent_env) => parent_env.clone(),
            _ => return None,
        };
        for _ in 1..distance {
            let new_env = match ret_env.borrow().enclosing {
                Some(ref parent_env) => parent_env.clone(),
                None => return None,
            };
            ret_env = new_env;
        }
        let ret = ret_env.borrow().get(&name);
        ret
    }
}
