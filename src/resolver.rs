use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;
use crate::expr::{walk_expr, Expr};
use crate::lox_value::LoxValue;
use crate::stmt::{walk_stmt, Stmt};
use crate::token::Token;
use crate::visitor::Visitor;

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    functoin_type: FunctionType,
    class_type: ClassType,
}

#[derive(Copy, Clone)]
enum FunctionType {
    Function,
    Method,
    Initializer,
    None,
}

#[derive(Clone, Copy)]
enum ClassType {
    Class,
    None,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            scopes: Vec::new(),
            functoin_type: FunctionType::None,
            class_type: ClassType::None,
        }
    }

    pub fn resolve_stmts(&mut self, stmts: Vec<Stmt>) -> Result<LoxValue, Error> {
        for stmt in stmts.iter() {
            self.resolve_stmt(stmt)?;
        }
        Ok(LoxValue::Nil)
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<LoxValue, Error> {
        walk_stmt(self, stmt)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        walk_expr(self, expr)
    }

    fn resolve_local(&mut self, distance: Rc<Cell<i32>>, name: &Token) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                distance.set(i as i32);
                return;
            }
        }
    }

    fn resolve_function(
        &mut self,
        args: Vec<Token>,
        body: &Stmt,
        function_type: FunctionType,
    ) -> Result<LoxValue, Error> {
        let encloging_function_type = self.functoin_type;
        self.functoin_type = function_type;
        self.begin_scope();
        for arg in args.iter() {
            self.declare(arg)?;
            self.define(arg);
        }
        self.resolve_stmt(body)?;
        self.end_scope();
        self.functoin_type = encloging_function_type;
        Ok(LoxValue::Nil)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) -> Result<LoxValue, Error> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                return Err(Error {
                    kind: "resolving error".to_string(),
                    msg: format!(
                        "variable `{}` that has the name already exists in this scope\nline: {}",
                        name.lexeme, name.line,
                    ),
                });
            }
            scope.insert(name.lexeme.clone(), false);
        }
        Ok(LoxValue::Nil)
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }
}

impl Visitor for Resolver {
    fn visit_block(&mut self, stmts: Vec<Stmt>) -> Result<LoxValue, Error> {
        self.begin_scope();
        self.resolve_stmts(stmts)?;
        self.end_scope();
        Ok(LoxValue::Nil)
    }

    fn visit_var_stmt(&mut self, token: &Token, expr: Option<&Expr>) -> Result<LoxValue, Error> {
        self.declare(token)?;
        if let Some(init) = expr {
            self.resolve_expr(init)?;
        }
        self.define(token);
        Ok(LoxValue::Nil)
    }

    fn visit_class(&mut self, name: &Token, methods: Vec<Stmt>) -> Result<LoxValue, Error> {
        let enclosing_class = self.class_type;
        self.class_type = ClassType::Class;
        self.declare(name)?;
        self.define(name);

        self.begin_scope();
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert("this".to_string(), true);
        }

        for method in methods.iter() {
            match method {
                Stmt::Func(name, args, body) => {
                    let function_type = if name.lexeme == "init".to_string() {
                        FunctionType::Initializer
                    } else {
                        FunctionType::Method
                    };
                    self.resolve_function(args.to_vec(), body, function_type)?;
                }
                _ => {
                    return Err(Error {
                        kind: "resolving error".to_string(),
                        msg: "function only in class'es methods".to_string(),
                    })
                }
            }
        }

        self.end_scope();

        self.class_type = enclosing_class;
        Ok(LoxValue::Nil)
    }

    fn visit_var_expr(
        &mut self,
        token: &Token,
        distance: Rc<Cell<i32>>,
    ) -> Result<LoxValue, Error> {
        if let Some(scope) = self.scopes.last() {
            if let Some(available) = scope.get(&token.lexeme) {
                if !available {
                    return Err(Error {
                        kind: "resolving error".to_string(),
                        msg: "can't read local variable in its own initializer".to_string(),
                    });
                }
            }
        }
        self.resolve_local(distance, token);
        Ok(LoxValue::Nil)
    }

    fn visit_assign(
        &mut self,
        left: &Token,
        right: &Expr,
        distance: Rc<Cell<i32>>,
    ) -> Result<LoxValue, Error> {
        self.resolve_expr(right)?;
        self.resolve_local(distance, left);
        Ok(LoxValue::Nil)
    }

    fn visit_binary(&mut self, left: &Expr, _op: &Token, right: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;
        Ok(LoxValue::Nil)
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(expr)?;
        Ok(LoxValue::Nil)
    }

    fn visit_literal(&mut self, _lit: &crate::token::Literal) -> Result<LoxValue, Error> {
        Ok(LoxValue::Nil)
    }

    fn visit_logical(&mut self, left: &Expr, _op: &Token, right: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;
        Ok(LoxValue::Nil)
    }

    fn visit_unary(&mut self, _token: &Token, expr: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(expr)?;
        Ok(LoxValue::Nil)
    }

    fn visit_call(
        &mut self,
        callee: &Expr,
        _paren: &Token,
        args: Vec<Expr>,
    ) -> Result<LoxValue, Error> {
        self.resolve_expr(callee)?;
        for arg in args.iter() {
            self.resolve_expr(arg)?;
        }
        Ok(LoxValue::Nil)
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(expr)?;
        Ok(LoxValue::Nil)
    }

    fn visit_print(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(expr)?;
        Ok(LoxValue::Nil)
    }

    fn visit_func(
        &mut self,
        name: &Token,
        args: Vec<Token>,
        body: &Stmt,
    ) -> Result<LoxValue, Error> {
        self.declare(name)?;
        self.define(name);
        self.resolve_function(args, body, FunctionType::Function)?;
        Ok(LoxValue::Nil)
    }

    fn visit_if(
        &mut self,
        cond: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> Result<LoxValue, Error> {
        self.resolve_expr(cond)?;
        self.resolve_stmt(then_branch)?;
        if let Some(else_branch) = else_branch {
            self.resolve_stmt(else_branch)?;
        }
        Ok(LoxValue::Nil)
    }

    fn visit_return(&mut self, keyword: &Token, value: Option<&Expr>) -> Result<LoxValue, Error> {
        if let FunctionType::None = self.functoin_type {
            return Err(Error {
                kind: "resolving error".to_string(),
                msg: format!("can't return from top-level code\nline: {}", keyword.line),
            });
        }

        if let Some(value) = value {
            match self.functoin_type {
                FunctionType::Initializer => {
                    return Err(Error {
                        kind: "resolving error".to_string(),
                        msg: "can't return a value from an initializer".to_string(),
                    })
                }
                _ => {}
            }
            self.resolve_expr(value)?;
        }
        Ok(LoxValue::Nil)
    }

    fn visit_while(&mut self, cond: &Expr, body: &Stmt) -> Result<LoxValue, Error> {
        self.resolve_expr(cond)?;
        self.resolve_stmt(body)?;
        Ok(LoxValue::Nil)
    }

    fn visit_expr(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        walk_expr(self, expr)
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<LoxValue, Error> {
        walk_stmt(self, stmt)
    }

    fn visit_get(&mut self, expr: &Expr, _name: &Token) -> Result<LoxValue, Error> {
        self.resolve_expr(expr)?;
        Ok(LoxValue::Nil)
    }

    fn visit_set(&mut self, expr: &Expr, _name: &Token, value: &Expr) -> Result<LoxValue, Error> {
        self.resolve_expr(expr)?;
        self.resolve_expr(value)?;
        Ok(LoxValue::Nil)
    }

    fn visit_this(&mut self, token: &Token, distance: Rc<Cell<i32>>) -> Result<LoxValue, Error> {
        match self.class_type {
            ClassType::Class => {}
            ClassType::None => {
                return Err(Error {
                    kind: "resolving error".to_string(),
                    msg: "can't use 'this' outside of a class".to_string(),
                })
            }
        }
        self.resolve_local(distance, token);
        Ok(LoxValue::Nil)
    }
}
