use std::fmt::Debug;

use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::lox_value::LoxValue;

pub trait Callable: Debug {
    fn name(&self) -> String;
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<LoxValue>) -> Result<LoxValue, Error>;
}
