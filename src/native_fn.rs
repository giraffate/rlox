use std::time::SystemTime;

use crate::callable::Callable;
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::lox_value::LoxValue;

#[derive(Clone, Debug)]
pub struct ClockFn;

impl Callable for ClockFn {
    fn name(&self) -> String {
        "clock".to_string()
    }

    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _args: Vec<LoxValue>,
    ) -> Result<LoxValue, Error> {
        Ok(LoxValue::Time(SystemTime::now()))
    }
}
