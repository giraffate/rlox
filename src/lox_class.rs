use std::rc::Rc;

use crate::callable::Callable;
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::lox_instance::LoxInstance;
use crate::lox_value::LoxValue;

#[derive(Debug)]
pub struct LoxClass {
    pub inner: Rc<LoxClassInner>,
}

#[derive(Debug)]
pub struct LoxClassInner {
    name: String,
}

impl LoxClass {
    pub fn new(name: String) -> LoxClass {
        LoxClass {
            inner: Rc::new(LoxClassInner::new(name)),
        }
    }

    pub fn instantiate(&self) -> LoxClass {
        LoxClass {
            inner: self.inner.clone(),
        }
    }
}

impl LoxClassInner {
    fn new(name: String) -> LoxClassInner {
        LoxClassInner { name }
    }
}

impl Callable for LoxClass {
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _args: Vec<LoxValue>,
    ) -> Result<LoxValue, Error> {
        let instance = self.instantiate();
        Ok(LoxValue::Instance(Rc::new(LoxInstance::new(&instance))))
    }
}
