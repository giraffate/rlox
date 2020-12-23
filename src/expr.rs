use crate::error::Error;
use crate::lox_value::LoxValue;
use crate::token::{Literal, Token};
use crate::visitor::Visitor;

use std::cell::Cell;
use std::fmt;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Assign(Token, Box<Expr>, Rc<Cell<i32>>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
    Get(Box<Expr>, Token),
    Grouping(Box<Expr>),
    Literal(Literal),
    Logical(Box<Expr>, Token, Box<Expr>),
    Set(Box<Expr>, Token, Box<Expr>),
    Super(Token, Token),
    This(Token),
    Unary(Token, Box<Expr>),
    Variable(Token, Rc<Cell<i32>>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Assign(token, expr, _) => write!(f, "assign: ({}, {})", token.lexeme, expr),
            Expr::Binary(left_expr, token, right_expr) => {
                write!(f, "binary: ({} {} {}", left_expr, token.lexeme, right_expr)
            }
            Expr::Call(expr, token, args) => {
                write!(f, "call: ({} {} {:?})", expr, token.lexeme, args)
            }
            Expr::Get(expr, token) => write!(f, "get: ({} {})", expr, token.lexeme),
            Expr::Grouping(expr) => write!(f, "grouping: ({})", expr),
            Expr::Literal(s) => write!(f, "literal: ({})", s),
            Expr::Logical(left_expr, token, right_expr) => write!(
                f,
                "logical: ({} {} {})",
                left_expr, token.lexeme, right_expr
            ),
            Expr::Set(left_expr, token, right_expr) => {
                write!(f, "set: ({} {} {})", left_expr, token.lexeme, right_expr)
            }
            Expr::Super(keyword, method) => {
                write!(f, "super: ({} {})", keyword.lexeme, method.lexeme)
            }
            Expr::This(token) => write!(f, "this: ({})", token.lexeme),
            Expr::Unary(token, expr) => write!(f, "unary: ({} {})", token.lexeme, expr),
            Expr::Variable(token, _) => write!(f, "variable: ({})", token.lexeme),
        }
    }
}

pub fn walk_expr<V: Visitor + ?Sized>(visitor: &mut V, expr: &Expr) -> Result<LoxValue, Error> {
    match expr {
        Expr::Assign(left, right, distance) => visitor.visit_assign(left, right, distance.clone()),
        Expr::Binary(left, op, right) => visitor.visit_binary(left, op, right),
        Expr::Call(callee, paren, args) => visitor.visit_call(callee, paren, args.to_vec()),
        Expr::Get(expr, name) => visitor.visit_get(expr, name),
        Expr::Grouping(expr) => visitor.visit_grouping(expr),
        Expr::Literal(lit) => visitor.visit_literal(lit),
        Expr::Logical(left, op, right) => visitor.visit_logical(left, op, right),
        Expr::Set(expr, name, value) => visitor.visit_set(expr, name, value),
        // Expr::Super(_, _) => {}
        // Expr::This(_) => {}
        Expr::Unary(token, expr) => visitor.visit_unary(token, expr),
        Expr::Variable(name, distance) => visitor.visit_var_expr(name, distance.clone()),
        _ => Err(Error {
            kind: "runtime error".to_string(),
            msg: "unreachable".to_string(),
        }),
    }
}
