use crate::error::Error;
use crate::expr::{walk_expr, Expr};
use crate::lox_value::LoxValue;
use crate::stmt::{walk_stmt, Stmt};
use crate::token::{Literal, Token};

use std::cell::Cell;
use std::rc::Rc;

pub trait Visitor {
    fn visit_expr(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        walk_expr(self, expr)
    }

    fn visit_assign(
        &mut self,
        left: &Token,
        right: &Expr,
        distance: Rc<Cell<i32>>,
    ) -> Result<LoxValue, Error>;
    fn visit_binary(&mut self, left: &Expr, op: &Token, right: &Expr) -> Result<LoxValue, Error>;
    fn visit_grouping(&mut self, expr: &Expr) -> Result<LoxValue, Error>;
    fn visit_literal(&mut self, lit: &Literal) -> Result<LoxValue, Error>;
    fn visit_logical(&mut self, left: &Expr, op: &Token, right: &Expr) -> Result<LoxValue, Error>;
    fn visit_unary(&mut self, token: &Token, expr: &Expr) -> Result<LoxValue, Error>;
    fn visit_var_expr(&mut self, token: &Token, distance: Rc<Cell<i32>>)
        -> Result<LoxValue, Error>;
    fn visit_call(
        &mut self,
        callee: &Expr,
        paren: &Token,
        args: Vec<Expr>,
    ) -> Result<LoxValue, Error>;
    fn visit_get(&mut self, expr: &Expr, name: &Token) -> Result<LoxValue, Error>;
    fn visit_set(&mut self, expr: &Expr, name: &Token, value: &Expr) -> Result<LoxValue, Error>;
    fn visit_this(&mut self, token: &Token, distance: Rc<Cell<i32>>) -> Result<LoxValue, Error>;
    fn visit_super(
        &mut self,
        keyword: &Token,
        method: &Token,
        distance: Rc<Cell<i32>>,
    ) -> Result<LoxValue, Error>;

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<LoxValue, Error> {
        walk_stmt(self, stmt)
    }

    fn visit_class(
        &mut self,
        name: &Token,
        superclass: Option<Expr>,
        methods: Vec<Stmt>,
    ) -> Result<LoxValue, Error>;
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<LoxValue, Error>;
    fn visit_print(&mut self, expr: &Expr) -> Result<LoxValue, Error>;
    fn visit_block(&mut self, stmts: Vec<Stmt>) -> Result<LoxValue, Error>;
    fn visit_func(
        &mut self,
        name: &Token,
        args: Vec<Token>,
        body: &Stmt,
    ) -> Result<LoxValue, Error>;
    fn visit_if(
        &mut self,
        cond: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> Result<LoxValue, Error>;
    fn visit_return(&mut self, keyword: &Token, value: Option<&Expr>) -> Result<LoxValue, Error>;
    fn visit_while(&mut self, cond: &Expr, body: &Stmt) -> Result<LoxValue, Error>;
    fn visit_var_stmt(&mut self, token: &Token, expr: Option<&Expr>) -> Result<LoxValue, Error>;
}
