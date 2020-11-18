use crate::error::Error;
use crate::expr::{walk_expr, Expr};
use crate::lox_value::LoxValue;
use crate::stmt::{walk_stmt, Stmt};
use crate::token::{Literal, Token};

pub trait Visitor {
    fn visit_expr(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        walk_expr(self, expr)
    }

    fn visit_assign(&mut self, left: &Token, right: &Expr) -> Result<LoxValue, Error>;
    fn visit_binary(&mut self, left: &Expr, op: &Token, right: &Expr) -> Result<LoxValue, Error>;
    fn visit_grouping(&mut self, expr: &Expr) -> Result<LoxValue, Error>;
    fn visit_literal(&mut self, lit: &Literal) -> Result<LoxValue, Error>;
    fn visit_unary(&mut self, token: &Token, expr: &Expr) -> Result<LoxValue, Error>;
    fn visit_var_expr(&mut self, expr: &Token) -> Result<LoxValue, Error>;

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<LoxValue, Error> {
        walk_stmt(self, stmt)
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<LoxValue, Error>;
    fn visit_print(&mut self, expr: &Expr) -> Result<LoxValue, Error>;
    fn visit_block(&mut self, stmts: Vec<Stmt>) -> Result<LoxValue, Error>;
    fn visit_var_stmt(&mut self, token: &Token, expr: Option<&Expr>) -> Result<LoxValue, Error>;
}