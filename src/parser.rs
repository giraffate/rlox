use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::{Literal, Token, TokenType};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !self.is_at_end() {
            stmts.push(self.declaration());
        }
        stmts
    }

    fn declaration(&mut self) -> Stmt {
        if self.is_match(vec![TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self.consume(TokenType::Identifier, "expect variable name".to_string());
        let init = if self.is_match(vec![TokenType::Equal]) {
            Some(self.expression())
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.".to_string(),
        );
        Stmt::Var(name, init)
    }

    fn statement(&mut self) -> Stmt {
        if self.is_match(vec![TokenType::Print]) {
            self.print_statement()
        } else if self.is_match(vec![TokenType::LeftBrace]) {
            self.block_statement()
        } else if self.is_match(vec![TokenType::If]) {
            self.if_statement()
        } else if self.is_match(vec![TokenType::While]){
            self.while_statement()
        } else {
            self.expr_statement()
        }
    }

    fn print_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
        Stmt::Print(expr)
    }

    fn block_statement(&mut self) -> Stmt {
        let mut stmts = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let stmt = self.declaration();
            stmts.push(stmt);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.".to_string());
        Stmt::Block(stmts)
    }

    fn if_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.".to_string());
        let cond = self.expression();
        self.consume(
            TokenType::RightParen,
            "Expect ')' after 'if' condition.".to_string(),
        );
        let then_branch = Box::new(self.statement());
        let else_branch = if self.is_match(vec![TokenType::Else]) {
            Some(Box::new(self.statement()))
        } else {
            None
        };
        Stmt::If(cond, then_branch, else_branch)
    }

    fn while_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after while.".to_string());
        let cond = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after condition.".to_string());
        let body = self.statement();
        Stmt::While(cond, Box::new(body))
    }

    fn expr_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
        Stmt::Expr(expr)
    }

    pub fn expression(&mut self) -> Expr {
        self.assignment()
    }

    pub fn assignment(&mut self) -> Expr {
        let mut expr = self.or();

        if self.is_match(vec![TokenType::Equal]) {
            let value = self.assignment();
            expr = match expr {
                Expr::Variable(token) => Expr::Assign(token, Box::new(value)),
                _ => panic!("invalid assignment target"),
            }
        }

        expr
    }

    pub fn or(&mut self) -> Expr {
        let mut expr = self.and();

        if self.is_match(vec![TokenType::Or]) {
            let op = self.previous();
            let right = self.and();
            expr = Expr::Logical(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    pub fn and(&mut self) -> Expr {
        let mut expr = self.equality();

        if self.is_match(vec![TokenType::And]) {
            let op = self.previous();
            let right = self.equality();
            expr = Expr::Logical(Box::new(expr), op, Box::new(right));
        }

        expr
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

        while self.is_match(vec![
            TokenType::GreaterEqual,
            TokenType::Greater,
            TokenType::LessEqual,
            TokenType::Less,
        ]) {
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
            Expr::Literal(Literal::Bool(false))
        } else if self.is_match(vec![TokenType::True]) {
            Expr::Literal(Literal::Bool(true))
        } else if self.is_match(vec![TokenType::Nil]) {
            Expr::Literal(Literal::Nil)
        } else if self.is_match(vec![TokenType::Number, TokenType::Str]) {
            Expr::Literal(self.previous().lit.unwrap())
        } else if self.is_match(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.".to_string(),
            );
            Expr::Grouping(Box::new(expr))
        } else if self.is_match(vec![TokenType::Identifier]) {
            Expr::Variable(self.previous())
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

    fn is_match(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types.iter() {
            if self.check(*token_type) {
                self.advance();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::Expr;
    use crate::scanner::Scanner;

    #[test]
    fn test_expression() {
        let s = "1 + 2";
        let mut scanner = Scanner {
            source: s.chars().collect(),
            ..Default::default()
        };
        let tokens = scanner.scan_tokens();
        let mut parser = Parser {
            tokens: tokens,
            current: 0,
        };
        assert_eq!(
            parser.expression(),
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Number(1f64))),
                Token {
                    token_type: TokenType::Plus,
                    lexeme: "+".to_string(),
                    lit: None,
                    line: 1
                },
                Box::new(Expr::Literal(Literal::Number(2f64)))
            )
        );
    }
}
