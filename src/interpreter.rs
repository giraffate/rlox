use crate::env::Env;
use crate::error::Error;
use crate::expr::Expr;
use crate::lox_value::LoxValue;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::visitor::Visitor;

pub struct Interpreter {
    pub env: Env,
}

impl Visitor for Interpreter {
    fn visit_var_expr(&self, name: &Token) -> Result<LoxValue, Error> {
        match self.env.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(Error {
                kind: "runtime error".to_string(),
                msg: format!("{} is not initialized", name.lexeme),
            }),
        }
    }

    fn visit_var_stmt(&mut self, name: &Token, init: Option<&Expr>) -> Result<LoxValue, Error> {
        let value = if let Some(expr) = init {
            self.visit_expr(expr)?
        } else {
            LoxValue::Nil
        };
        self.env.insert(name.lexeme.clone(), value);
        Ok(LoxValue::Nil)
    }
}

impl Interpreter {
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<LoxValue, Error> {
        for stmt in stmts.iter() {
            self.execute(stmt)?;
        }
        Ok(LoxValue::Nil)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<LoxValue, Error> {
        self.visit_stmt(stmt)
    }
}
