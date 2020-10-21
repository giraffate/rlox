use crate::token::{Literal, Token};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Assign(Token, Box<Expr>),
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
    Variable(Token),
}

pub fn walk_expr<V: Visitor + ?Sized>(visitor: &V, expr: &Expr) {}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Assign(token, expr) => write!(f, "assign: ({}, {})", token.lexeme, expr),
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
            Expr::Variable(token) => write!(f, "variable: ({})", token.lexeme),
        }
    }
}

pub trait Visitor {
    fn visit_expr(&self, expr: &Expr) {
        walk_expr(self, expr);
    }
}
