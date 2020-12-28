use std::cell::RefCell;
use std::rc::Rc;

use crate::callable::Callable;
use crate::env::Env;
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::lox_value::LoxValue;
use crate::stmt::{walk_stmt, Stmt};
use crate::token::Token;

#[derive(Clone, Debug)]
pub struct LoxFunction {
    pub name: Token,
    pub args: Vec<Token>,
    pub body: Stmt,
    pub closure: Rc<RefCell<Env>>,
    pub is_initilizer: bool,
}

impl LoxFunction {
    pub fn bind(&mut self, instance: LoxValue) -> LoxFunction {
        let mut env = Env::new();
        env.enclosing = Some(self.closure.clone());
        env.values.insert("this".to_string(), instance);
        LoxFunction {
            name: self.name.clone(),
            args: self.args.clone(),
            body: self.body.clone(),
            closure: Rc::new(RefCell::new(env)),
            is_initilizer: self.is_initilizer,
        }
    }
}

impl Callable for LoxFunction {
    fn name(&self) -> String {
        self.name.lexeme.clone()
    }

    fn arity(&self) -> usize {
        self.args.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<LoxValue>) -> Result<LoxValue, Error> {
        let mut closure = Env::new();
        closure.enclosing = Some(self.closure.clone());
        let closure = Rc::new(RefCell::new(closure));
        let env = interpreter.env.clone();

        for i in 0..self.args.len() {
            closure
                .borrow_mut()
                .define(self.args[i].lexeme.clone(), args[i].clone());
        }
        interpreter.env = closure.clone();
        let ret = walk_stmt(interpreter, &self.body);
        interpreter.env = env;
        if self.is_initilizer {
            return closure.borrow().get_at("this".to_string(), 0).map_or(
                Err(Error {
                    kind: "runtime error".to_string(),
                    msg: "no initializer exists".to_string(),
                }),
                |v| Ok(v),
            );
        }
        match ret {
            Ok(LoxValue::Return(value)) => Ok(*value),
            _ => match closure.borrow().get_at("this".to_string(), 0) {
                Some(value) => Ok(value),
                None => ret,
            },
        }
    }
}
