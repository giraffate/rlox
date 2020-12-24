use std::rc::Rc;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

use crate::callable::Callable;
use crate::env::Env;
use crate::error::Error;
use crate::expr::{walk_expr, Expr};
use crate::lox_class::LoxClass;
use crate::lox_function::LoxFunction;
use crate::lox_value::LoxValue;
use crate::native_fn::ClockFn;
use crate::stmt::{walk_stmt, Stmt};
use crate::token::{Literal, Token, TokenType};
use crate::visitor::Visitor;

pub struct Interpreter {
    pub env: Rc<RefCell<Env>>,
    pub globals: Rc<RefCell<Env>>,
}

impl Visitor for Interpreter {
    fn visit_assign(
        &mut self,
        left: &Token,
        right: &Expr,
        distance: Rc<Cell<i32>>,
    ) -> Result<LoxValue, Error> {
        let value = walk_expr(self, right)?;
        let distance = distance.get();
        if distance < 0 {
            self.globals.borrow_mut().assign(left.lexeme.clone(), value);
        } else {
            self.env
                .borrow_mut()
                .assign_at(distance, left.lexeme.clone(), value);
        }
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

    fn visit_get(&mut self, expr: &Expr, name: &Token) -> Result<LoxValue, Error> {
        let expr = walk_expr(self, expr)?;
        match expr {
            LoxValue::Instance(instance) => instance.borrow().get(name),
            _ => Err(Error {
                kind: "runtime error".to_string(),
                msg: "only instances have properties".to_string(),
            }),
        }
    }

    fn visit_set(&mut self, expr: &Expr, name: &Token, value: &Expr) -> Result<LoxValue, Error> {
        let expr = walk_expr(self, expr)?;
        match expr {
            LoxValue::Instance(instance) => {
                let value = walk_expr(self, value)?;
                instance.borrow_mut().set(name, value)
            }
            _ => Err(Error {
                kind: "runtime error".to_string(),
                msg: "only instances have fields".to_string(),
            }),
        }
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

    fn visit_var_expr(
        &mut self,
        token: &Token,
        distance: Rc<Cell<i32>>,
    ) -> Result<LoxValue, Error> {
        self.lookup_variable(token, distance.get())
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
                            "wrong number of arguments in `{}`\nexpected: {}\ngot: {}",
                            callee.name(),
                            callee.arity(),
                            args.len()
                        ),
                    });
                }
                callee.call(self, args)
            }
            LoxValue::Class(callee) => callee.call(self, args),
            _ => Err(Error {
                kind: "runtime error".to_string(),
                msg: "couldn't find the function".to_string(),
            }),
        }
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        walk_expr(self, expr)
    }

    fn visit_print(&mut self, expr: &Expr) -> Result<LoxValue, Error> {
        let v = walk_expr(self, expr)?;
        println!("{}", v);
        Ok(LoxValue::Nil)
    }

    fn visit_block(&mut self, stmts: Vec<Stmt>) -> Result<LoxValue, Error> {
        let mut child = Env::new();
        let parent = self.env.clone();
        child.enclosing = Some(parent.clone());
        self.env = Rc::new(RefCell::new(child));

        let mut return_value = None;
        for stmt in stmts.iter() {
            let value = walk_stmt(self, stmt)?;
            match value {
                LoxValue::Return(_) => {
                    return_value = Some(value);
                    break;
                }
                _ => {}
            }
        }
        self.env = parent;

        match return_value {
            Some(return_value) => Ok(return_value),
            _ => Ok(LoxValue::Nil),
        }
    }

    fn visit_func(
        &mut self,
        name: &Token,
        args: Vec<Token>,
        body: &Stmt,
    ) -> Result<LoxValue, Error> {
        let function = LoxFunction {
            name: name.clone(),
            args,
            body: body.clone(),
            closure: self.env.clone(),
        };
        self.env
            .borrow_mut()
            .define(name.lexeme.clone(), LoxValue::Fn(Rc::new(function)));
        Ok(LoxValue::Nil)
    }

    fn visit_class(&mut self, name: &Token, methods: Vec<Stmt>) -> Result<LoxValue, Error> {
        self.env
            .borrow_mut()
            .define(name.lexeme.clone(), LoxValue::Nil);
        let mut class_methods = HashMap::new();
        for method in methods {
            match method {
                Stmt::Func(name, args, body) => {
                    let function = LoxFunction {
                        name: name.clone(),
                        args,
                        body: *body.clone(),
                        closure: self.env.clone(),
                    };
                    class_methods.insert(name.lexeme.clone(), function);
                }
                _ => {
                    return Err(Error {
                        kind: "runtime error".to_string(),
                        msg: "function only in class'es methods".to_string(),
                    })
                }
            }
        }
        let klass = LoxClass::new(name.lexeme.clone(), class_methods);
        self.env
            .borrow_mut()
            .assign(name.lexeme.clone(), LoxValue::Class(Rc::new(klass)));
        Ok(LoxValue::Nil)
    }

    fn visit_return(&mut self, _token: &Token, value: Option<&Expr>) -> Result<LoxValue, Error> {
        let return_value = match value {
            Some(value) => walk_expr(self, value)?,
            None => return Ok(LoxValue::Nil),
        };
        Ok(LoxValue::Return(Box::new(return_value)))
    }

    fn visit_if(
        &mut self,
        cond: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> Result<LoxValue, Error> {
        let cond_value = walk_expr(self, cond)?;
        match cond_value {
            LoxValue::Bool(true) => walk_stmt(self, then_branch),
            _ => match else_branch {
                Some(else_branch_inside) => walk_stmt(self, else_branch_inside),
                None => Ok(LoxValue::Nil),
            },
        }
    }

    fn visit_while(&mut self, cond: &Expr, body: &Stmt) -> Result<LoxValue, Error> {
        let mut return_value = None;
        while walk_expr(self, cond)?.truthy()? == LoxValue::Bool(true) {
            let value = walk_stmt(self, body)?;
            match value {
                LoxValue::Return(_) => {
                    return_value = Some(value);
                    break;
                }
                _ => {}
            }
        }

        match return_value {
            Some(return_value) => Ok(return_value),
            _ => Ok(LoxValue::Nil),
        }
    }

    fn visit_var_stmt(&mut self, name: &Token, init: Option<&Expr>) -> Result<LoxValue, Error> {
        let value = if let Some(expr) = init {
            walk_expr(self, expr)?
        } else {
            LoxValue::Nil
        };
        self.env.borrow_mut().define(name.lexeme.clone(), value);
        Ok(LoxValue::Nil)
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut globals = Env::new();

        let clock_fn = ClockFn {};
        globals.define("clock".to_string(), LoxValue::Fn(Rc::new(clock_fn)));
        let globals = Rc::new(RefCell::new(globals));
        Interpreter {
            env: globals.clone(),
            globals: globals.clone(),
        }
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

    fn lookup_variable(&mut self, token: &Token, distance: i32) -> Result<LoxValue, Error> {
        if distance < 0 {
            match self.globals.borrow().get(&token.lexeme) {
                Some(value) => Ok(value.clone()),
                None => Err(Error {
                    kind: "runtime error".to_string(),
                    msg: format!("{} is not initialized", token.lexeme),
                }),
            }
        } else {
            match self.env.borrow().get_at(token.lexeme.clone(), distance) {
                Some(value) => Ok(value.clone()),
                None => Err(Error {
                    kind: "runtime error".to_string(),
                    msg: format!("{} is not initialized", token.lexeme),
                }),
            }
        }
    }
}
