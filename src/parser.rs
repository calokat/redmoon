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

    fn expr_list(&mut self) -> Result<Expr, String> {
        let mut expr_vec = vec![self.expression()?];
        while self.check_token_type(Token::Comma) {
            expr_vec.push(self.expression()?);
        }
        return Ok(Expr::Exprlist(expr_vec));
    }

    fn var_list(&mut self, first_var_name: String) -> Result<Expr, String> {
        let mut var_vec = vec![Expr::Var(first_var_name)];
        self.advance();
        while self.check_token_type(Token::Comma) {
            if let Token::Identifier(s) = self.current_token() {
                var_vec.push(Expr::Var(s));
                self.advance();
            }
        }
        return Ok(Expr::Varlist(var_vec));
    }

    pub fn statement(&mut self) -> Result<Stmt, String> {
        if self.tokens.len() == 0 {
            return Err("No valid tokens".into());
        }
        if self.check_token_type(Token::Semicolon) {
            return Ok(Stmt::Empty);
        }
        if let Token::Identifier(s) = self.current_token() {
            let var_list = self.var_list(s)?;
            if self.check_token_type(Token::Assign) {
                let expr_list = self.expr_list()?;
                return Ok(Stmt::Assignment(var_list, expr_list));
            }
        }
        return Ok(Stmt::ExprStmt(self.expression()?));
    }

    fn current_token(&self) -> Token {
        return self.tokens[self.current].clone();
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