use std::rc::Rc;

use crate::env::Env;
use crate::error::Error;
use crate::expr::{walk_expr, Expr};
use crate::lox_value::LoxValue;
use crate::native_fn::ClockFn;
use crate::stmt::{walk_stmt, Stmt};
use crate::token::{Literal, Token, TokenType};
use crate::visitor::Visitor;

pub struct Interpreter {
    pub env: Env,
}

impl Visitor for Interpreter {
    fn visit_assign(&mut self, left: &Token, right: &Expr) -> Result<LoxValue, Error> {
        let value = walk_expr(self, right)?;
        self.env.assign(left.lexeme.clone(), value);
        Ok(LoxValue::Nil)
    }

    fn visit_binary(&mut self, left: &Expr, op: &Token, right: &Expr) -> Result<LoxValue, Error> {
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

    fn visit_grouping(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        walk_expr(self, expr)
    }

    fn visit_literal(&mut self, lit: &Literal) -> Result<LoxValue, Error> {
        Ok(lit.value())
    }

    fn visit_logical(
        &mut self,
        left_expr: &Expr,
        op: &Token,
        right_expr: &Expr,
    ) -> Result<LoxValue, Error> {
        let left = walk_expr(self, left_expr)?;
        match op.token_type {
            TokenType::Or => match left.truthy() {
                Ok(LoxValue::Bool(true)) => return Ok(left),
                _ => {}
            },
            _ => match left.truthy() {
                Ok(LoxValue::Bool(false)) => return Ok(left),
                _ => {}
            },
        };
        walk_expr(self, right_expr)
    }

    fn visit_unary(&mut self, token: &Token, expr: &Expr) -> Result<LoxValue, Error> {
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

    fn visit_var_expr(&mut self, name: &Token) -> Result<LoxValue, Error> {
        match self.env.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(Error {
                kind: "runtime error".to_string(),
                msg: format!("{} is not initialized", name.lexeme),
            }),
        }
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        walk_expr(self, expr)
    }

    fn visit_call(
        &mut self,
        callee: &Expr,
        _paren: &Token,
        args: Vec<Expr>,
    ) -> Result<LoxValue, Error> {
        let callee = walk_expr(self, callee)?;
        let args = {
            let mut v = Vec::new();
            for arg in args.iter() {
                let arg = walk_expr(self, arg)?;
                v.push(arg);
            }
            v
        };
        match callee {
            LoxValue::Fn(callee) => {
                if args.len() != callee.arity() {
                    return Err(Error {
                        kind: "runtime error".to_string(),
                        msg: format!(
                            "wrong number of arguments\nexpected: {}\ngot: {}",
                            callee.arity(),
                            args.len()
                        ),
                    });
                }
                callee.call(self, args)
            }
            _ => Err(Error {
                kind: "runtime error".to_string(),
                msg: "couldn't find the function".to_string(),
            }),
        }
    }

    fn visit_print(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        let v = walk_expr(self, expr)?;
        println!("{}", v);
        Ok(LoxValue::Nil)
    }

    fn visit_block(&mut self, stmts: Vec<Stmt>) -> Result<LoxValue, Error> {
        let mut child = Env::new();
        child.enclosing = Some(Box::new(self.env.clone()));
        self.env = child;

        for stmt in stmts.iter() {
            walk_stmt(self, stmt)?;
        }
        self.env = *(self.env.enclosing.as_ref().unwrap()).clone();
        Ok(LoxValue::Nil)
    }

    fn visit_if(
        &mut self,
        cond: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> Result<LoxValue, Error> {
        let cond_value = walk_expr(self, cond)?;
        match cond_value {
            LoxValue::Bool(true) => walk_stmt(self, then_branch)?,
            _ => match else_branch {
                Some(else_branch_inside) => walk_stmt(self, else_branch_inside)?,
                None => LoxValue::Nil,
            },
        };
        Ok(LoxValue::Nil)
    }

    fn visit_while(&mut self, cond: &Expr, body: &Stmt) -> Result<LoxValue, Error> {
        while walk_expr(self, cond)?.truthy()? == LoxValue::Bool(true) {
            walk_stmt(self, body)?;
        }
        Ok(LoxValue::Nil)
    }

    fn visit_var_stmt(&mut self, name: &Token, init: Option<&Expr>) -> Result<LoxValue, Error> {
        let value = if let Some(expr) = init {
            walk_expr(self, expr)?
        } else {
            LoxValue::Nil
        };
        self.env.define(name.lexeme.clone(), value);
        Ok(LoxValue::Nil)
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut globals = Env::new();

        let clock_fn = ClockFn {};
        globals.define("clock".to_string(), LoxValue::Fn(Rc::new(clock_fn)));
        Interpreter { env: globals }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<LoxValue, Error> {
        for stmt in stmts.iter() {
            self.execute(stmt)?;
        }
        Ok(LoxValue::Nil)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<LoxValue, Error> {
        walk_stmt(self, stmt)
    }
}
