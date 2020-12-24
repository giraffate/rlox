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

        match ret {
            Ok(LoxValue::Return(value)) => Ok(*value),
            _ => ret,
        }
    }
}
