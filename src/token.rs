use std::fmt;

use crate::lox_value::LoxValue;

#[allow(unused)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    Str,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub lit: Option<Literal>,
    pub line: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl Literal {
    pub fn value(&self) -> LoxValue {
        match self {
            Literal::Number(n) => LoxValue::Number(*n),
            Literal::Str(s) => LoxValue::Str(s.to_string()),
            Literal::Bool(b) => LoxValue::Bool(*b),
            Literal::Nil => LoxValue::Nil,
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(number) => write!(f, "{}", number),
            Literal::Str(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}
