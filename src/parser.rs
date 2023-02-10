use crate::{Token, Expr};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self { tokens, current: 0 }
    }

    fn primary(&mut self) -> Result<Expr, String> {
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
        } else if let Token::Identifier(s) = self.current_token() {
            return Ok(Expr::Var(s));
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

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.check_token_type(Token::LessThan) ||
            self.check_token_type(Token::LessThanOrEqual) ||
            self.check_token_type(Token::Equals) ||
            self.check_token_type(Token::GreaterThanOrEqual) ||
            self.check_token_type(Token::GreaterThan) {
                let operator = self.previous_token();
                let right = self.term()?;
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

    pub fn expression(&mut self) -> Result<Expr, String> {
        if self.tokens.len() > 0 {
            return self.equality();
        }
        return Err("No valid tokens".into());
    }

    fn current_token(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous_token(&mut self) -> Token {
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