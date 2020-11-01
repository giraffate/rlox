use crate::error::Error;
use crate::lox_value::LoxValue;
use crate::stmt::Stmt;
use crate::visitor::Visitor;

pub struct Interpreter {}

impl Visitor for Interpreter {}

impl Interpreter {
    pub fn interpret(&self, stmts: Vec<Stmt>) -> Result<LoxValue, Error> {
        for stmt in stmts.iter() {
            self.execute(stmt)?;
        }
        Ok(LoxValue::Nil)
    }

    fn execute(&self, stmt: &Stmt) -> Result<LoxValue, Error> {
        self.visit_stmt(stmt)
    }
}
