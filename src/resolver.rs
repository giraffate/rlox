use std::collections::HashMap;

use crate::error::Error;
use crate::expr::{walk_expr, Expr};
use crate::interpreter::Interpreter;
use crate::lox_value::LoxValue;
use crate::stmt::{walk_stmt, Stmt};
use crate::token::Token;
use crate::visitor::Visitor;

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn resolve_stmts(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts.iter() {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        walk_stmt(self, stmt);
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        walk_expr(self, expr);
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(&name.lexeme) {
                // TODO
                // interpreter.resolve
                return;
            }
        }
    }

    fn resolve_function(&mut self, args: Vec<Token>, body: &Stmt) {
        self.begin_scope();
        for arg in args.iter() {
            self.declare(arg);
            self.define(arg);
        }
        self.resolve_stmt(body);
        self.end_scope();
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn end_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let mut scope = self.scopes.pop().unwrap();
        scope.insert(name.lexeme.clone(), false);
        self.scopes.push(scope);
    }

    pub fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let mut scope = self.scopes.pop().unwrap();
        scope.insert(name.lexeme.clone(), true);
        self.scopes.push(scope);
    }
}

impl Visitor for Resolver {
    fn visit_block(&mut self, stmts: Vec<Stmt>) -> Result<LoxValue, Error> {
        self.begin_scope();
        self.resolve_stmts(stmts);
        self.end_scope();
        Ok(LoxValue::Nil)
    }

    fn visit_var_stmt(&mut self, token: &Token, expr: Option<&Expr>) -> Result<LoxValue, Error> {
        self.declare(token);
        if let Some(init) = expr {
            self.resolve_expr(init);
        }
        self.define(token);
        Ok(LoxValue::Nil)
    }

    fn visit_var_expr(&mut self, expr: &Token) -> Result<LoxValue, Error> {
        if !self.scopes.is_empty() && !self.scopes[self.scopes.len()].get(&expr.lexeme).unwrap() {
            return Err(Error {
                kind: "resolved error".to_string(),
                msg: "can't read local variable in its own initializer".to_string(),
            });
        }

        Ok(LoxValue::Nil)
    }

    fn visit_assign(&mut self, left: &Token, right: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(right);
        self.resolve_local(&Expr::Assign(left.clone(), Box::new(right.clone())), left);
        Ok(LoxValue::Nil)
    }

    fn visit_binary(&mut self, left: &Expr, _op: &Token, right: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(left);
        self.resolve_expr(right);
        Ok(LoxValue::Nil)
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(expr);
        Ok(LoxValue::Nil)
    }

    fn visit_literal(&mut self, _lit: &crate::token::Literal) -> Result<LoxValue, Error> {
        Ok(LoxValue::Nil)
    }

    fn visit_logical(&mut self, left: &Expr, _op: &Token, right: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(left);
        self.resolve_expr(right);
        Ok(LoxValue::Nil)
    }

    fn visit_unary(&mut self, _token: &Token, expr: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(expr);
        Ok(LoxValue::Nil)
    }

    fn visit_call(
        &mut self,
        callee: &Expr,
        _paren: &Token,
        args: Vec<Expr>,
    ) -> Result<LoxValue, Error> {
        self.resolve_expr(callee);
        for arg in args.iter() {
            self.resolve_expr(arg);
        }
        Ok(LoxValue::Nil)
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(expr);
        Ok(LoxValue::Nil)
    }

    fn visit_print(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(expr);
        Ok(LoxValue::Nil)
    }

    fn visit_func(
        &mut self,
        name: &Token,
        args: Vec<Token>,
        body: &Stmt,
    ) -> Result<LoxValue, Error> {
        self.declare(name);
        self.define(name);
        self.resolve_function(args, body);
        Ok(LoxValue::Nil)
    }

    fn visit_if(
        &mut self,
        cond: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> Result<LoxValue, Error> {
        self.resolve_expr(cond);
        self.resolve_stmt(then_branch);
        if let Some(else_branch) = else_branch {
            self.resolve_stmt(else_branch);
        }
        Ok(LoxValue::Nil)
    }

    fn visit_return(&mut self, _keyword: &Token, value: Option<&Expr>) -> Result<LoxValue, Error> {
        if let Some(value) = value {
            self.resolve_expr(value);
        }
        Ok(LoxValue::Nil)
    }

    fn visit_while(&mut self, cond: &Expr, body: &Stmt) -> Result<LoxValue, Error> {
        self.resolve_expr(cond);
        self.resolve_stmt(body);
        Ok(LoxValue::Nil)
    }

    fn visit_expr(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        walk_expr(self, expr)
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<LoxValue, Error> {
        walk_stmt(self, stmt)
    }
}
