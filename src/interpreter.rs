use crate::env::Env;
use crate::error::Error;
use crate::expr::{walk_expr, Expr};
use crate::lox_value::LoxValue;
use crate::stmt::Stmt;
use crate::token::{Literal, Token, TokenType};
use crate::visitor::Visitor;

pub struct Interpreter {
    pub env: Env,
}

impl Visitor for Interpreter {
    fn visit_binary(&self, left: &Expr, op: &Token, right: &Expr) -> Result<LoxValue, Error> {
        let left = walk_expr(self, left)?;
        let right = walk_expr(self, right)?;
        match op.token_type {
            TokenType::Minus => left.subtract(right),
            TokenType::Plus => left.plus(right),
            TokenType::Star => left.multiply(right),
            TokenType::Slash => left.divide(right),
            TokenType::Greater => left.greater(right),
            TokenType::GreaterEqual => left.greater_equal(right),
            TokenType::Less => left.less(right),
            TokenType::LessEqual => left.less_equal(right),
            TokenType::EqualEqual => left.equal_equal(right),
            TokenType::BangEqual => left.bang_equal(right),
            _ => Err(Error {
                kind: "syntax error".to_string(),
                msg: "invalid operator in binary".to_string(),
            }),
        }
    }

    fn visit_grouping(&self, expr: &Expr) -> Result<LoxValue, Error> {
        walk_expr(self, expr)
    }

    fn visit_literal(&self, lit: &Literal) -> Result<LoxValue, Error> {
        Ok(lit.value())
    }

    fn visit_unary(&self, token: &Token, expr: &Expr) -> Result<LoxValue, Error> {
        let right = walk_expr(self, expr)?;
        match token.token_type {
            TokenType::Minus => right.negate_number(),
            TokenType::Bang => right.negate(),
            _ => Err(Error {
                kind: "syntax error".to_string(),
                msg: "invalid operator in unary".to_string(),
            }),
        }
    }

    fn visit_var_expr(&self, name: &Token) -> Result<LoxValue, Error> {
        match self.env.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(Error {
                kind: "runtime error".to_string(),
                msg: format!("{} is not initialized", name.lexeme),
            }),
        }
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        self.visit_expr(expr)
    }

    fn visit_print(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        let v = self.visit_expr(expr)?;
        println!("{}", v);
        Ok(LoxValue::Nil)
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
