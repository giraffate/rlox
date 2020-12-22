use std::rc::Rc;

use crate::lox_class::{LoxClass, LoxClassInner};

#[derive(Debug)]
pub struct LoxInstance {
    klass: Rc<LoxClassInner>,
}

impl LoxInstance {
    pub fn new(klass: &LoxClass) -> LoxInstance {
        LoxInstance {
            klass: klass.inner.clone(),
        }
    }
}
