use crate::{Token, Expr, Stmt};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self { tokens, current: 0 }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if let Token::Literal(v) = self.current_token() {
            self.advance();
            return Ok(Expr::Literal(v.clone()));
        } else if self.current_token() == Token::LeftParens {
            self.advance();
            let expr_res = self.expression();
            if let Ok(expr) = expr_res {
                let expr = Expr::Grouping(Box::new(expr));
                if !self.check_token_type(Token::RightParens) {
                    return Err("Missing right parens".into());
                }
                return Ok(expr);    
            }
            return expr_res;
        } else if let Token::Identifier(s) = self.current_token() {
            self.advance();
            return Ok(Expr::Var(s.clone()));
        } else if Token::Equals == self.current_token() {
            println!("Aha advance strikes again");
        }
        return Err("Unknown token".into());
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.check_token_type(Token::Minus) {
            let operator = self.previous_token();
            if let Ok(right) = self.unary() {
                return Ok(Expr::Unary(Box::new(right), operator));
            } else {
                return Err("Unsupported unary operation".into())
            }
        }
        return self.primary();
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let unary = self.unary();
        if let Ok(mut expr) = unary {
            while self.check_token_type(Token::Star) ||
            self.check_token_type(Token::ForwardSlash) {
                let operator = self.previous_token();
                let right = self.unary();
                if let Ok(right) = right {
                    expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
                } else {
                    return right;
                }
            }
    
            return Ok(expr);
        } else {
            return unary;
        }
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.check_token_type(Token::Plus) ||
        self.check_token_type(Token::Minus) {
            let operator = self.previous_token();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    fn concat(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.check_token_type(Token::Concatenation) {
            println!("Concat checking");
            expr = Expr::Binary(Box::new(expr), Token::Concatenation, Box::new(self.term()?));
        }
        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.concat()?;
        while self.check_token_type(Token::LessThan) ||
            self.check_token_type(Token::LessThanOrEqual) ||
            self.check_token_type(Token::Equals) ||
            self.check_token_type(Token::GreaterThanOrEqual) ||
            self.check_token_type(Token::GreaterThan) {
                let operator = self.previous_token();
                let right = self.concat()?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

            return Ok(expr);
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.check_token_type(Token::Equals) ||
        self.check_token_type(Token::NotEquals) {
            let operator = self.previous_token();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    fn expression(&mut self) -> Result<Expr, String> {
        if self.tokens.len() > 0 {
            return self.equality();
        }
        return Err("No valid tokens".into());
    }

    pub fn statement(&mut self) -> Result<Stmt, String> {
        if self.tokens.len() == 0 {
            return Err("No valid tokens".into());
        }
        if self.check_token_type(Token::Semicolon) {
            return Ok(Stmt::Empty);
        }
        if let Token::Identifier(s) = self.current_token() {
            let s = s.clone();
            let peek = self.peek_next_token();
            if let Some(pt) = peek {
                if pt == Token::Assign {
                    self.advance(); self.advance();
                    return Ok(Stmt::Assignment(Expr::Var(s), self.expression()?));
                }
            }
        }
        return Ok(Stmt::ExprStmt(self.expression()?));
    }

    fn current_token(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn peek_next_token(&self) -> Option<Token> {
        if self.current < self.tokens.len() - 1 {
            return Some(self.tokens[self.current + 1].clone());
        }
        None
    }

    fn previous_token(&self) -> Token {
        if self.current == 0 {
            panic!("Went back before the beginning");
        }

        return self.tokens[self.current - 1].clone();
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() - 1 {
            self.current += 1;
        }
    }

    fn check_token_type(&mut self, _type: Token) -> bool {
        let res = self.current_token() == _type;
        if res {
            self.advance();
        }

        res
    }
}