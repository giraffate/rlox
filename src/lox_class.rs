use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::callable::Callable;
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::lox_function::LoxFunction;
use crate::lox_instance::LoxInstance;
use crate::lox_value::LoxValue;

#[derive(Debug)]
pub struct LoxClass {
    pub inner: Rc<LoxClassInner>,
}

#[derive(Debug)]
pub struct LoxClassInner {
    pub name: String,
    superclass: Option<Rc<LoxClass>>,
    methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(
        name: String,
        superclass: Option<Rc<LoxClass>>,
        methods: HashMap<String, LoxFunction>,
    ) -> LoxClass {
        LoxClass {
            inner: Rc::new(LoxClassInner::new(name, superclass, methods)),
        }
    }

    pub fn instantiate(&self) -> LoxInstance {
        let klass = LoxClass {
            inner: self.inner.clone(),
        };
        LoxInstance::new(&klass)
    }
}

impl LoxClassInner {
    fn new(
        name: String,
        superclass: Option<Rc<LoxClass>>,
        methods: HashMap<String, LoxFunction>,
    ) -> LoxClassInner {
        LoxClassInner {
            name,
            superclass,
            methods,
        }
    }

    pub fn find_method(&self, name: &String) -> Option<LoxFunction> {
        self.methods
            .get(name)
            .map(|v| v.clone())
            .or_else(|| match &self.superclass {
                Some(superclass) => superclass.inner.find_method(name),
                None => None,
            })
    }
}

impl Callable for LoxClass {
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn arity(&self) -> usize {
        match self.inner.find_method(&"init".to_string()) {
            Some(initializer) => initializer.arity(),
            None => 0,
        }
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<LoxValue>) -> Result<LoxValue, Error> {
        let instance = Rc::new(RefCell::new(self.instantiate()));
        match self.inner.find_method(&"init".to_string()) {
            Some(mut initializer) => {
                let instance = LoxValue::Instance(instance.clone());
                initializer.bind(instance).call(interpreter, args)?;
            }
            None => {}
        }
        Ok(LoxValue::Instance(instance))
    }
}
