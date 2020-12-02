use crate::callable::Callable;
use crate::stmt::{walk_stmt, Stmt};
use crate::token::Token;
use crate::interpreter::Interpreter;
use crate::lox_value::LoxValue;
use crate::error::Error;
use crate::env::Env;

#[derive(Debug)]
pub struct LoxFunction {
    pub name: Token,
    pub args: Vec<Token>,
    pub body: Stmt,
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        self.args.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<LoxValue>) -> Result<LoxValue, Error> {
        let mut child = Env::new();
        child.enclosing = Some(Box::new(interpreter.env.clone()));

        for i in 0..self.args.len() {
            child.define(self.args[i].lexeme.clone(), args[i].clone());
        }
        interpreter.env = child;

        let ret = walk_stmt(interpreter, &self.body);
        interpreter.env = *(interpreter.env.enclosing.as_ref().unwrap()).clone();

        ret
    }
}