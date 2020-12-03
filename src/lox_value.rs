use std::cmp::PartialEq;
use std::fmt;
use std::rc::Rc;
use std::time::SystemTime;

use crate::callable::Callable;
use crate::error::Error;

#[derive(Clone, Debug)]
pub enum LoxValue {
    Number(f64),
    Str(String),
    Bool(bool),
    Time(SystemTime),
    Fn(Rc<dyn Callable>),
    Return(Box<LoxValue>),
    Nil,
}

impl PartialEq for LoxValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LoxValue::Number(l), LoxValue::Number(r)) => *l == *r,
            (LoxValue::Str(l), LoxValue::Str(r)) => *l == *r,
            (LoxValue::Bool(l), LoxValue::Bool(r)) => *l == *r,
            (LoxValue::Nil, LoxValue::Nil) => true,
            (LoxValue::Time(l), LoxValue::Time(r)) => *l == *r,
            _ => false,
        }
    }
}

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxValue::Number(n) => write!(f, "{}", n),
            LoxValue::Str(s) => write!(f, "{}", s),
            LoxValue::Bool(b) => write!(f, "{}", b),
            LoxValue::Nil => write!(f, "nil"),
            LoxValue::Time(t) => write!(f, "{:?}", t),
            LoxValue::Fn(callable) => write!(f, "{:?}", callable),
            LoxValue::Return(value) => write!(f, "{:?}", value),
        }
    }
}

impl LoxValue {
    pub fn negate_number(&self) -> Result<LoxValue, Error> {
        match self {
            LoxValue::Number(n) => Ok(LoxValue::Number(-1f64 * *n)),
            _ => Err(Error {
                kind: "negate type error".to_string(),
                msg: "not number".to_string(),
            }),
        }
    }

    pub fn negate(&self) -> Result<LoxValue, Error> {
        match self {
            LoxValue::Bool(b) => Ok(LoxValue::Bool(!b)),
            LoxValue::Nil => Ok(LoxValue::Bool(true)),
            _ => Ok(LoxValue::Bool(false)),
        }
    }

    pub fn subtract(&self, v: LoxValue) -> Result<LoxValue, Error> {
        match (self, v) {
            (LoxValue::Number(left), LoxValue::Number(right)) => Ok(LoxValue::Number(left - right)),
            _ => Err(Error {
                kind: "subtract type error".to_string(),
                msg: "not number".to_string(),
            }),
        }
    }

    pub fn multiply(&self, v: LoxValue) -> Result<LoxValue, Error> {
        match (self, v) {
            (LoxValue::Number(left), LoxValue::Number(right)) => Ok(LoxValue::Number(left * right)),
            _ => Err(Error {
                kind: "multiply type error".to_string(),
                msg: "not number".to_string(),
            }),
        }
    }

    pub fn plus(&self, v: LoxValue) -> Result<LoxValue, Error> {
        match (self, v) {
            (LoxValue::Number(left), LoxValue::Number(right)) => Ok(LoxValue::Number(left + right)),
            (LoxValue::Str(left), LoxValue::Str(right)) => {
                Ok(LoxValue::Str(left.clone() + &right[..]))
            }
            _ => Err(Error {
                kind: "plus type error".to_string(),
                msg: "not number".to_string(),
            }),
        }
    }

    pub fn divide(&self, v: LoxValue) -> Result<LoxValue, Error> {
        match (self, v) {
            (LoxValue::Number(_), LoxValue::Number(n)) if n == 0f64 => Err(Error {
                kind: "runtime error".to_string(),
                msg: "divided by zero".to_string(),
            }),
            (LoxValue::Number(left), LoxValue::Number(right)) => Ok(LoxValue::Number(left / right)),
            _ => Err(Error {
                kind: "divide type error".to_string(),
                msg: "not number".to_string(),
            }),
        }
    }

    pub fn greater(&self, v: LoxValue) -> Result<LoxValue, Error> {
        match (self, v) {
            (LoxValue::Number(left), LoxValue::Number(right)) => Ok(LoxValue::Bool(left > &right)),
            _ => Err(Error {
                kind: "greater type error".to_string(),
                msg: "not number".to_string(),
            }),
        }
    }

    pub fn greater_equal(&self, v: LoxValue) -> Result<LoxValue, Error> {
        match (self, v) {
            (LoxValue::Number(left), LoxValue::Number(right)) => Ok(LoxValue::Bool(left >= &right)),
            _ => Err(Error {
                kind: "greater equal type error".to_string(),
                msg: "not number".to_string(),
            }),
        }
    }

    pub fn less(&self, v: LoxValue) -> Result<LoxValue, Error> {
        match (self, v) {
            (LoxValue::Number(left), LoxValue::Number(right)) => Ok(LoxValue::Bool(left < &right)),
            _ => Err(Error {
                kind: "less type error".to_string(),
                msg: "not number".to_string(),
            }),
        }
    }

    pub fn less_equal(&self, v: LoxValue) -> Result<LoxValue, Error> {
        match (self, v) {
            (LoxValue::Number(left), LoxValue::Number(right)) => Ok(LoxValue::Bool(left <= &right)),
            _ => Err(Error {
                kind: "less equal type error".to_string(),
                msg: "not number".to_string(),
            }),
        }
    }

    pub fn equal_equal(&self, v: LoxValue) -> Result<LoxValue, Error> {
        match (self, v) {
            (LoxValue::Number(left), LoxValue::Number(right)) => Ok(LoxValue::Bool(left == &right)),
            (LoxValue::Str(left), LoxValue::Str(right)) => Ok(LoxValue::Bool(left == &right)),
            (LoxValue::Bool(left), LoxValue::Bool(right)) => Ok(LoxValue::Bool(left == &right)),
            (LoxValue::Nil, LoxValue::Nil) => Ok(LoxValue::Bool(true)),
            _ => Err(Error {
                kind: "equal equal type error".to_string(),
                msg: "not number".to_string(),
            }),
        }
    }

    pub fn bang_equal(&self, v: LoxValue) -> Result<LoxValue, Error> {
        match (self, v) {
            (LoxValue::Number(left), LoxValue::Number(right)) => Ok(LoxValue::Bool(left != &right)),
            (LoxValue::Str(left), LoxValue::Str(right)) => Ok(LoxValue::Bool(left != &right)),
            (LoxValue::Bool(left), LoxValue::Bool(right)) => Ok(LoxValue::Bool(left != &right)),
            (LoxValue::Nil, LoxValue::Nil) => Ok(LoxValue::Bool(false)),
            _ => Err(Error {
                kind: "bang equal type error".to_string(),
                msg: "not number".to_string(),
            }),
        }
    }

    pub fn truthy(&self) -> Result<LoxValue, Error> {
        match self {
            LoxValue::Bool(false) | LoxValue::Nil => Ok(LoxValue::Bool(false)),
            _ => Ok(LoxValue::Bool(true)),
        }
    }
}
