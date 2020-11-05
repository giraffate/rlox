use crate::error::Error;
use crate::expr::{walk_expr, Expr};
use crate::lox_value::LoxValue;
use crate::stmt::{walk_stmt, Stmt};
use crate::token::{Literal, Token};

pub trait Visitor {
    fn visit_expr(&self, expr: &Expr) -> Result<LoxValue, Error> {
        walk_expr(self, expr)
    }

    fn visit_binary(&self, left: &Expr, op: &Token, right: &Expr) -> Result<LoxValue, Error>;
    fn visit_grouping(&self, expr: &Expr) -> Result<LoxValue, Error>;
    fn visit_literal(&self, lit: &Literal) -> Result<LoxValue, Error>;
    fn visit_unary(&self, token: &Token, expr: &Expr) -> Result<LoxValue, Error>;
    fn visit_var_expr(&self, expr: &Token) -> Result<LoxValue, Error>;

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<LoxValue, Error> {
        walk_stmt(self, stmt)
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<LoxValue, Error>;
    fn visit_print(&mut self, expr: &Expr) -> Result<LoxValue, Error>;
    fn visit_var_stmt(&mut self, token: &Token, expr: Option<&Expr>) -> Result<LoxValue, Error>;
}
