use crate::token::TokenType::*;
use crate::token::{Literal, Token, TokenType};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("and", And);
        map.insert("class", Class);
        map.insert("else", Else);
        map.insert("false", False);
        map.insert("for", For);
        map.insert("fun", Fun);
        map.insert("if", If);
        map.insert("nil", Nil);
        map.insert("or", Or);
        map.insert("pring", Print);
        map.insert("return", Return);
        map.insert("super", Super);
        map.insert("this", This);
        map.insert("true", True);
        map.insert("var", Var);
        map.insert("while", While);
        map
    };
}

pub struct Scanner {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            source: Vec::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            lit: None,
            line: self.line,
        });
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen, None),
            ')' => self.add_token(RightParen, None),
            '{' => self.add_token(LeftBrace, None),
            '}' => self.add_token(RightBrace, None),
            ',' => self.add_token(Comma, None),
            '.' => self.add_token(Dot, None),
            '-' => self.add_token(Minus, None),
            '+' => self.add_token(Plus, None),
            ';' => self.add_token(Semicolon, None),
            '*' => self.add_token(Star, None),
            '!' => {
                let token_type = if self.is_match('=') { BangEqual } else { Bang };
                self.add_token(token_type, None);
            }
            '=' => {
                let token_type = if self.is_match('=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.is_match('=') { LessEqual } else { Less };
                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.is_match('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(token_type, None);
            }
            '/' => {
                if self.is_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    panic!("unexpected character");
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn add_token(&mut self, token_type: TokenType, lit: Option<Literal>) {
        let token = Token {
            token_type: token_type,
            lexeme: self.source[self.start..self.current].iter().collect(),
            lit: lit,
            line: self.line,
        };
        self.tokens.push(token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("unterminated string")
        }

        // The closing "
        self.advance();

        // Trim the surrounded quotes
        let s = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token(Str, Some(Literal::Str(s)));
    }

    fn number(&mut self) {
        while is_digit(self.peek()) && !self.is_at_end() {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            // Consume .
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let n: f64 = self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();
        self.add_token(Number, Some(Literal::Number(n)));
    }

    fn identifier(&mut self) {
        while is_alpha_number(self.peek_next()) && !self.is_at_end() {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type = KEYWORDS.get(text.as_str()).unwrap_or(&Identifier);
        self.add_token(*token_type, None);

        self.add_token(Identifier, None);
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alpha_number(c: char) -> bool {
    is_digit(c) || is_alpha_number(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance() {
        let source: Vec<char> = "Hello world!".to_string().chars().collect();
        let mut scanner = Scanner {
            source: source,
            ..Default::default()
        };
        assert_eq!(scanner.advance(), 'H');
        assert_eq!(scanner.advance(), 'e');
    }

    #[test]
    fn test_is_at_end() {
        let source: Vec<char> = "a".to_string().chars().collect();
        let mut scanner = Scanner {
            source: source,
            ..Default::default()
        };
        assert_eq!(scanner.is_at_end(), false);
        scanner.advance();
        assert_eq!(scanner.is_at_end(), true);
    }

    #[test]
    fn test_is_match() {
        let source: Vec<char> = "a".to_string().chars().collect();
        let mut scanner = Scanner {
            source: source,
            ..Default::default()
        };
        assert_eq!(scanner.is_match('b'), false);
        assert_eq!(scanner.is_match('a'), true);
        assert_eq!(scanner.is_match('a'), false);
    }

    #[test]
    fn test_peek() {
        let source: Vec<char> = "a".to_string().chars().collect();
        let mut scanner = Scanner {
            source: source,
            ..Default::default()
        };
        assert_eq!(scanner.peek(), 'a');
        scanner.advance();
        assert_eq!(scanner.peek(), '\0');
    }

    #[test]
    fn test_peek_next() {
        let source: Vec<char> = "ab".to_string().chars().collect();
        let mut scanner = Scanner {
            source: source,
            ..Default::default()
        };
        assert_eq!(scanner.peek_next(), 'b');
        scanner.advance();
        assert_eq!(scanner.peek_next(), '\0');
    }

    #[test]
    fn test_string() {
        let source: Vec<char> = "\"Hello \n world!\"".to_string().chars().collect();
        let mut scanner = Scanner {
            source: source,
            ..Default::default()
        };
        scanner.advance();
        scanner.string();
        let token = &scanner.tokens[0];
        assert_eq!(
            token.lit.as_ref().unwrap(),
            &Literal::Str("Hello \n world!".to_string())
        );
        assert_eq!(scanner.line, 2);
    }

    #[test]
    fn test_number_int() {
        let source: Vec<char> = "123".to_string().chars().collect();
        let mut scanner = Scanner {
            source: source,
            ..Default::default()
        };
        scanner.advance();
        scanner.number();
        let token = &scanner.tokens[0];
        assert_eq!(token.lit.as_ref().unwrap(), &Literal::Number(123f64));
    }

    #[test]
    fn test_number_float() {
        let source: Vec<char> = "123.456".to_string().chars().collect();
        let mut scanner = Scanner {
            source: source,
            ..Default::default()
        };
        scanner.advance();
        scanner.number();
        let token = &scanner.tokens[0];
        assert_eq!(token.lit.as_ref().unwrap(), &Literal::Number(123.456f64));
    }

    #[test]
    fn test_is_digit() {
        assert_eq!(is_digit('0'), true);
        assert_eq!(is_digit('a'), false);
    }

    #[test]
    fn test_simple_scan_tokens() {
        let s = "1 + 2";
        let mut scanner = Scanner {
            source: s.chars().collect(),
            ..Default::default()
        };
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[1].token_type, TokenType::Plus);
        assert_eq!(tokens[2].token_type, TokenType::Number);
    }
}
