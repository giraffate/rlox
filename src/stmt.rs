use crate::error::Error;
use crate::expr::Expr;
use crate::lox_value::LoxValue;
use crate::token::Token;
use crate::visitor::Visitor;

#[derive(Debug)]
pub enum Stmt {
    Block(Vec<Expr>),
    Class(Token, Expr, Vec<Box<Stmt>>),
    Expr(Expr),
    Func(Token, Vec<Token>, Vec<Box<Stmt>>),
    If(Expr, Box<Stmt>, Box<Stmt>),
    Print(Expr),
    Return(Token, Expr),
    Var(Token, Option<Expr>),
    While(Expr, Box<Stmt>),
}

pub fn walk_stmt<V: Visitor + ?Sized>(visitor: &mut V, stmt: &Stmt) -> Result<LoxValue, Error> {
    match stmt {
        Stmt::Expr(expr) => visitor.visit_expr(expr),
        Stmt::Print(expr) => {
            let v = visitor.visit_expr(expr)?;
            println!("{}", v);
            Ok(LoxValue::Nil)
        }
        Stmt::Var(name, init) => visitor.visit_var_stmt(name, init.as_ref()),
        _ => Err(Error {
            kind: "runtime error".to_string(),
            msg: "unreachable".to_string(),
        }),
    }
}
