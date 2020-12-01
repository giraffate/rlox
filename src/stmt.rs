use crate::error::Error;
use crate::expr::Expr;
use crate::lox_value::LoxValue;
use crate::token::Token;
use crate::visitor::Visitor;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Class(Token, Expr, Vec<Box<Stmt>>),
    Expr(Expr),
    Func(Token, Vec<Token>, Box<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Print(Expr),
    Return(Token, Expr),
    Var(Token, Option<Expr>),
    While(Expr, Box<Stmt>),
}

pub fn walk_stmt<V: Visitor + ?Sized>(visitor: &mut V, stmt: &Stmt) -> Result<LoxValue, Error> {
    match stmt {
        Stmt::Block(stmts) => visitor.visit_block(stmts.to_vec()),
        Stmt::Expr(expr) => visitor.visit_expr_stmt(expr),
        Stmt::If(cond, then_branch, else_branch) => {
            let else_branch_converted = match else_branch {
                Some(b) => Some(&**b),
                None => None,
            };
            visitor.visit_if(cond, then_branch, else_branch_converted)
        }
        Stmt::While(cond, body) => visitor.visit_while(cond, body),
        Stmt::Print(expr) => visitor.visit_print(expr),
        Stmt::Var(name, init) => visitor.visit_var_stmt(name, init.as_ref()),
        _ => Err(Error {
            kind: "runtime error".to_string(),
            msg: "unreachable".to_string(),
        }),
    }
}
