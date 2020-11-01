use crate::error::Error;
use crate::expr::{walk_expr, Expr};
use crate::lox_value::LoxValue;
use crate::stmt::{walk_stmt, Stmt};

pub trait Visitor {
    fn visit_expr(&self, expr: &Expr) -> Result<LoxValue, Error> {
        walk_expr(self, expr)
    }

    fn visit_stmt(&self, stmt: &Stmt) -> Result<LoxValue, Error> {
        walk_stmt(self, stmt)
    }
}
