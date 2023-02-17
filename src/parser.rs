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
            println!("Parser: Variable {s}");
            self.advance();
            return Ok(Expr::Var(s.clone()));
        } else if Token::Assign == self.current_token() {
            println!("It's Assign");
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
        return self.equality();
    }

    fn expr_list(&mut self) -> Result<Expr, String> {
        let mut expr_vec = vec![self.expression()?];
        while self.check_token_type(Token::Comma) {
            expr_vec.push(self.expression()?);
        }
        return Ok(Expr::Exprlist(expr_vec));
    }

    fn assignment(&mut self) -> Result<Stmt, String> {
        let expr = self.expr_list()?;
        if self.check_token_type(Token::Assign) {
            let right = self.expr_list()?;
            return Ok(Stmt::Assignment(expr, right));
        }
        return Ok(Stmt::ExprStmt(expr));
    }

    fn do_block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut res = vec![];
        while !self.check_token_type(Token::End) && self.current < self.tokens.len() - 1 {
            res.push(self.statement()?);
        }
        assert!(self.previous_token() == Token::End, "Missing \"End\" keyword");
        return Ok(res);
    }

    fn local_assignment(&mut self) -> Result<Stmt, String> {
        let assign_stmt = self.assignment()?;
        match assign_stmt {
            Stmt::Assignment(vars, vals) => {
                return Ok(Stmt::LocalAssignment(vars, vals));
            },
            Stmt::ExprStmt(vars) => {
                return Ok(Stmt::LocalAssignment(vars, Expr::Exprlist(vec![])));
            },
            _ => Err("Invalid local assignment".into())
        }
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.check_token_type(Token::Semicolon) {
            return Ok(Stmt::Empty);
        } else if self.check_token_type(Token::Do) {
            let res = self.do_block()?;
            return Ok(Stmt::Block(res));
        } else if self.check_token_type(Token::Local) {
            return self.local_assignment();
        } else if self.check_token_type(Token::If) {
            let cond = self.expression()?;
            assert!(self.check_token_type(Token::Then), "If statement missing \"then\" keyword");
            let body = self.do_block()?;
            return Ok(Stmt::IfStmt(cond, Box::new(Stmt::Block(body))));
        } else if self.check_token_type(Token::While) {
            let cond = self.expression()?;
            assert!(self.check_token_type(Token::Do), "while loop missing \"do\" keyword");
            let body = self.do_block()?;
            return Ok(Stmt::WhileLoop(cond, Box::new(Stmt::Block(body))));
        }
        return self.assignment();
    }


    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut res = vec![];
        while self.current < self.tokens.len() - 1 {
            res.push(self.statement()?);
        }
        Ok(res)
    }

    pub fn chunk(&mut self) -> Result<Vec<Stmt>, String> {
        if self.tokens.len() == 0 {
            return Err("No valid tokens".into());
        }

        return Ok(self.block()?);
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