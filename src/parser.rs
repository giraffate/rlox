use crate::token::{Token, TokenType};
use crate::expr::Expr;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.is_match(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.is_match(vec![TokenType::GreaterEqual, TokenType::Greater, TokenType::LessEqual, TokenType::Less]) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.is_match(vec![TokenType::Plus, TokenType::Minus]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.is_match(vec![TokenType::Star, TokenType::Slash]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.is_match(vec![TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary();
            Expr::Unary(op, Box::new(right))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.is_match(vec![TokenType::False]) {
            Expr::Literal("false".to_string())
        } else if self.is_match(vec![TokenType::True]) {
            Expr::Literal("true".to_string())
        } else if self.is_match(vec![TokenType::Nil]) {
            Expr::Literal("nil".to_string())
        } else if self.is_match(vec![TokenType::Number, TokenType::Str]) {
            Expr::Literal(self.previous().lit.unwrap())
        } else if self.is_match(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.".to_string());
            Expr::Grouping(Box::new(expr))
        } else {
            panic!("not to land here");
        }
    }

    fn consume(&mut self, token_type: TokenType, s: String) -> Token {
        if self.check(token_type) {
            self.advance()
        } else {
            panic!(s);
        }
    }

    fn is_match(&self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types.iter() {
            if self.check(*token_type) {
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}



