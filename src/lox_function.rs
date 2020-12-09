use std::cell::RefCell;
use std::rc::Rc;

use crate::callable::Callable;
use crate::env::Env;
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::lox_value::LoxValue;
use crate::stmt::{walk_stmt, Stmt};
use crate::token::Token;

#[derive(Debug)]
pub struct LoxFunction {
    pub name: Token,
    pub args: Vec<Token>,
    pub body: Stmt,
    pub closure: Rc<RefCell<Env>>,
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        self.args.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<LoxValue>) -> Result<LoxValue, Error> {
        let closure = self.closure.clone();
        let env = interpreter.env.clone();
        closure.borrow_mut().enclosing = Some(env.clone());

        for i in 0..self.args.len() {
            closure
                .borrow_mut()
                .define(self.args[i].lexeme.clone(), args[i].clone());
        }
        interpreter.env = closure.clone();
        let ret = walk_stmt(interpreter, &self.body);
        interpreter.env = env;

        match ret {
            Ok(LoxValue::Return(value)) => Ok(*value),
            _ => ret,
        }
    }
}
