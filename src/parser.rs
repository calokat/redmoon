use crate::{Token, Expr};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        return Self { tokens, current: 0 }
    }

    pub fn primary(&mut self) -> Result<Expr<'a>, String> {
        if self.check_token_type(Token::True) ||
        self.check_token_type(Token::False) ||
        self.check_token_type(Token::Nil) {
            return Ok(Expr::Literal(self.previous_token()));
        }
        if let Token::LiteralNumber(_) = self.current_token() {
            let res = self.current_token();
            self.advance();
            return Ok(Expr::Literal(res));
        } else if let Token::LiteralString(_) = self.current_token() {
            let res = self.current_token();
            self.advance();
            return Ok(Expr::Literal(res));
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
        }
        return Err("Unknown token".into());
    }

    pub fn unary(&mut self) -> Result<Expr<'a>, String> {
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

    pub fn factor(&mut self) -> Result<Expr<'a>, String> {
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

    pub fn term(&mut self) -> Result<Expr<'a>, String> {
        let mut expr = self.factor()?;

        while self.check_token_type(Token::Plus) ||
        self.check_token_type(Token::Minus) {
            let operator = self.previous_token();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    pub fn equality(&mut self) -> Result<Expr<'a>, String> {
        let mut expr = self.term()?;

        while self.check_token_type(Token::Equals) ||
        self.check_token_type(Token::NotEquals) {
            let operator = self.previous_token();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
}

    pub fn expression(&mut self) -> Result<Expr<'a>, String> {
        if self.tokens.len() > 0 {
            return self.equality();
        }
        return Err("No valid tokens".into());
    }

    fn current_token(&self) -> Token<'a> {
        return self.tokens[self.current];
    }

    fn previous_token(&mut self) -> Token<'a> {
        if self.current == 0 {
            panic!("Went back before the beginning");
        }

        return self.tokens[self.current - 1];
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