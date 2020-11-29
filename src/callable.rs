use crate::error::Error;
use crate::lox_value::LoxValue;
use crate::visitor::Visitor;

pub trait Callble {
    fn arity(&self) -> usize;
    fn call<V: Visitor + ?Sized>(&mut self, visitor: &mut V, args: Vec<LoxValue>) -> Result<LoxValue, Error>;
}