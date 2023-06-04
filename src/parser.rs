use std::collections::VecDeque;

use ordered_float::OrderedFloat;

use crate::{Token, Expr, Stmt, function::Function, values::Value, table::UserTable};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self { tokens, current: 0 }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if let Some(Token::Literal(v)) = self.current_token() {
            self.advance();
            return Ok(Expr::Literal(v.clone()));
        } else if self.check_token_type(Token::LeftParens) {
            let expr_res = self.expr_list();
            if let Ok(expr) = expr_res {
                let expr = Expr::Grouping(Box::new(expr));
                if !self.check_token_type(Token::RightParens) {
                    return Err("Missing right parens".into());
                }
                return Ok(expr);
            }
            return expr_res;
        } else if let Some(Token::Identifier(s)) = self.current_token() {
            self.advance();
            return Ok(Expr::Var(s));
        } else if Some(Token::Function) == self.current_token() {
            return self.function_def();
        } else if Some(Token::LeftCurlyBrace) == self.current_token() {
            return self.table();
        }
        return Err("Unknown token".into());
    }

    fn extract_vars_from_expr(expr: &Expr, params: &Vec<Expr>) -> Vec<Value> {
        let mut res = vec![];
        match expr {
            Expr::Exprlist(list) => {
                for var in list {
                    res.append(&mut Self::extract_vars_from_expr(&var, params));
                }
            },
            Expr::Binary(left, _, right) => {
                res.append(&mut Self::extract_vars_from_expr(&*left, params));
                res.append(&mut Self::extract_vars_from_expr(&*right, params));
            },
            Expr::FieldList(list) => {
                for (name, value) in list {
                    res.append(&mut Self::extract_vars_from_expr(&*name, params));
                    res.append(&mut Self::extract_vars_from_expr(&*value, params));
                }
            },
            Expr::FunctionCall(func, args) => {
                res.append(&mut Self::extract_vars_from_expr(&*func, params));
                for a in args {
                    res.append(&mut Self::extract_vars_from_expr(&a, params));
                }
            },
            Expr::Grouping(group) => {
                res.append(&mut Self::extract_vars_from_expr(&*group, params));
            },
            Expr::Literal(_) => {/* Do nothing */},
            Expr::Var(s) => {
                if !params.contains(expr) {
                    res.push(Value::String(s.clone()))
                }
            },
            Expr::Unary(expr, _) => {
                res.append(&mut Self::extract_vars_from_expr(&*expr, params));
            },
            Expr::Accessor(accessee, accessor) => {
                res.append(&mut Self::extract_vars_from_expr(&*accessee, params));
                res.append(&mut Self::extract_vars_from_expr(&*accessor, params));

            }
        };
        res
    }

    fn extract_vars_from_stmt(stmt: &Stmt, params: &Vec<Expr>) -> Vec<Value> {
        let mut res = vec![];
        match stmt {
            Stmt::Assignment(vars, vals) => {
                res.append(&mut Self::extract_vars_from_expr(vars, params));
                res.append(&mut Self::extract_vars_from_expr(vals, params));
            },
            Stmt::ExprStmt(expr)=> {
                res.append(&mut Self::extract_vars_from_expr(expr, params));
            },
            Stmt::LocalAssignment(_, vals) => {
                res.append(&mut Self::extract_vars_from_expr(vals, params));
            },
            Stmt::Return(expr) => {
                res.append(&mut Self::extract_vars_from_expr(expr, params));
            },
            Stmt::DoBlock(stmts) => {
                for s in stmts {
                    res.append(&mut Self::extract_vars_from_stmt(s, params));
                }
            },
            Stmt::Block(stmts) => {
                for s in stmts {
                    res.append(&mut Self::extract_vars_from_stmt(s, params));
                }
            },
            Stmt::IfStmt(cond, body, else_clause) => {
                res.append(&mut Self::extract_vars_from_expr(cond, params));
                res.append(&mut Self::extract_vars_from_stmt(&**body, params));
                res.append(&mut Self::extract_vars_from_stmt(&**else_clause, params));
            },
            Stmt::RepeatUntilLoop(body, cond) => {
                res.append(&mut Self::extract_vars_from_stmt(&**body, params));
                res.append(&mut Self::extract_vars_from_expr(cond, params));
            },
            Stmt::WhileLoop(cond, body) => {
                res.append(&mut Self::extract_vars_from_expr(cond, params));
                res.append(&mut Self::extract_vars_from_stmt(&**body, params));
            },
            Stmt::Empty => {/* Do nothing */},
            Stmt::Break => {/* Do nothing */},
            Stmt::Chunk(_) => {panic!("Chunk should only be at the global level")},
        };
        res
    }

    fn function_def(&mut self) -> Result<Expr, String> {
        self.advance();
        let mut f_name = None;
        if let Some(Token::Identifier(s)) = self.current_token() {
            f_name = Some(s);
            self.advance();
        }
        assert!(self.check_token_type(Token::LeftParens), "Function definition needs an opening parentheses");
        let params ;
        
        if self.current_token() != Some(Token::RightParens) {
            params = self.expr_list()?
        } else {
            params = Expr::Exprlist(vec![]);
        }

        assert!(self.check_token_type(Token::RightParens), "Function definition needs a closing parentheses");
        if let Expr::Exprlist(params) = params {
            let body = Box::new(Stmt::Block(self.do_block()?));
            let closure = UserTable::new();
            let found_vars = Self::extract_vars_from_stmt(&*body, &params);
            for fv in found_vars.into_iter() {
                closure.table.as_ref().borrow_mut().insert(fv, Value::Nil);
            }
            return Ok(Expr::Literal(Value::FunctionDef(Function::new(body, params, f_name, VecDeque::new()))));
        } else {
            return Err("Invalid parameter in function definition".into());
        }
    }

    fn accessor(&mut self) -> Result<Expr, String> {
        let mut left = self.primary()?;
        loop {
            if self.check_token_type(Token::Period) {
                let field = self.primary()?;
                if let Expr::Var(name) = field {
                    left = Expr::Accessor(Box::new(left), Box::new(Expr::Literal(Value::String(name))));
                }
            } else if self.check_token_type(Token::LeftSquareBracket) {
                let right = self.expression()?;
                left = Expr::Accessor(Box::new(left), Box::new(right));
                if !self.check_token_type(Token::RightSquareBracket) {
                    return Err("Missing right square bracket".into());
                }
            } else if self.check_token_type(Token::LeftParens) {
                if !self.check_token_type(Token::RightParens) {
                    let args = self.expr_list()?;
                    if !self.check_token_type(Token::RightParens) {
                        return Err("Function call is missing right parens".into());
                    }
                    if let Expr::Exprlist(args) = args {
                        left = Expr::FunctionCall(Box::new(left), args);
                    }
                } else {
                    left = Expr::FunctionCall(Box::new(left), vec![]);
                }
            } else if let Some(Token::Literal(v)) = self.current_token() {
                self.advance();
                if let Value::String(_) = v {
                    left = Expr::FunctionCall(Box::new(left), vec![Expr::Literal(v)]);
                }
            } else if self.check_token_type(Token::LeftCurlyBrace) {
                left = Expr::FunctionCall(Box::new(left), vec![self.field_list()?]);
            } else {
                break;
            }
        }
        return Ok(left);
    }


    fn unary(&mut self) -> Result<Expr, String> {
        if self.check_token_type(Token::Minus) || self.check_token_type(Token::Not) || self.check_token_type(Token::Pound) {
            let operator = self.previous_token();
            if let Ok(right) = self.unary() {
                return Ok(Expr::Unary(Box::new(right), operator));
            } else {
                return Err("Unsupported unary operation".into())
            }
        }
        return self.accessor();
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

    fn is_field_seperator(&mut self) -> bool {
        self.check_token_type(Token::Comma) || self.check_token_type(Token::Semicolon)
    }

    fn field_list(&mut self) -> Result<Expr, String> {
        let mut fields: Vec<(Box<Expr>, Box<Expr>)> = vec![];
        let mut field_counter = 1;
        while !self.check_token_type(Token::RightCurlyBrace) {
            if self.check_token_type(Token::LeftSquareBracket) {
                let key = self.expression()?;
                assert!(self.check_token_type(Token::RightSquareBracket), "expect closing bracket when declaring fields in a table");
                assert!(self.check_token_type(Token::Assign), "field needs to be assigned to");
                let value = self.expression()?;
                fields.push((Box::new(key), Box::new(value)));
                assert!(self.is_field_seperator() || self.current_token() == Some(Token::RightCurlyBrace), "Fields need to be properly separated");
            } else {
                let expr = self.expression()?;
                if self.is_field_seperator() || self.current_token() == Some(Token::RightCurlyBrace) {
                    fields.push((Box::new(Expr::Literal(Value::Number(OrderedFloat(field_counter as f32)))), Box::new(expr)));
                } else if self.check_token_type(Token::Assign) {
                    if let Expr::Var(s) = expr {
                        let value = self.expression()?;
                        fields.push((Box::new(Expr::Literal(Value::String(s))), Box::new(value)));

                    }
                }
            }
            field_counter += 1;
        }
        Ok(Expr::FieldList(fields))
    }

    fn table(&mut self) -> Result<Expr, String> {
        if self.check_token_type(Token::LeftCurlyBrace) {
            return self.field_list();
        }
        return self.equality();
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;
        while self.check_token_type(Token::And) {
            expr = Expr::Binary(Box::new(expr), Token::And, Box::new(self.or()?));
        }
        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;
        while self.check_token_type(Token::Or) {
            expr = Expr::Binary(Box::new(expr), Token::Or, Box::new(self.table()?));
        }
        return Ok(expr);
    }


    fn expression(&mut self) -> Result<Expr, String> {
        return self.or();
    }

    fn expr_list(&mut self) -> Result<Expr, String> {
        let mut expr_vec = vec![self.expression()?];
        while self.check_token_type(Token::Comma) {
            expr_vec.push(self.expression()?);
        }
        return Ok(Expr::Exprlist(expr_vec));
    }

    fn assignment(&mut self) -> Result<Stmt, String> {
        if self.current_token() == Some(Token::Function) {
            let func = self.function_def()?;
            if let Expr::Literal(Value::FunctionDef(fd)) = func {
                if let Some(id_str) = fd.get_name() {
                    return Ok(Stmt::Assignment(Expr::Exprlist(vec![Expr::Var(id_str)]), Expr::Exprlist(vec![Expr::Literal(Value::FunctionDef(fd))])));
                } else {
                    return Err("Cannot assign to function without name".into());
                }
            } else {
                return Err("Invalid function definition".into());
            }
        }
        let expr = self.expr_list()?;
        if self.check_token_type(Token::Assign) {
            let right = self.expr_list()?;
            return Ok(Stmt::Assignment(expr, right));
        }
        return Ok(Stmt::ExprStmt(expr));
    }

    fn do_block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut res = vec![];
        while !self.check_token_type(Token::End) && self.current < self.tokens.len() {
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

    fn repeat_body(&mut self) -> Result<Stmt, String> {
        let mut body = vec![];
        while !self.check_token_type(Token::Until) {
            body.push(self.statement()?);
        }
        Ok(Stmt::Block(body))
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        let cond = self.expression()?;
        assert!(self.check_token_type(Token::Then), "If statement missing \"then\" keyword");
        let mut stmts: Vec<Stmt> = vec![];
        while !self.check_token_type(Token::Else) && !self.check_token_type(Token::Elseif) && !self.check_token_type(Token::End) && self.current < self.tokens.len() {
            stmts.push(self.statement()?);
        }
        if self.previous_token() == Token::Else {
            return Ok(Stmt::IfStmt(cond, Box::new(Stmt::Block(stmts)), Box::new(Stmt::Block(self.do_block()?))));
        } else if self.previous_token() == Token::Elseif {
            return Ok(Stmt::IfStmt(cond, Box::new(Stmt::Block(stmts)), Box::new(self.if_statement()?)));
        } else {
            return Ok(Stmt::IfStmt(cond, Box::new(Stmt::Block(stmts)), Box::new(Stmt::Empty)));
        }

    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.check_token_type(Token::Semicolon) {
            return Ok(Stmt::Empty);
        } else if self.check_token_type(Token::Do) {
            let res = self.do_block()?;
            return Ok(Stmt::DoBlock(res));
        } else if self.check_token_type(Token::Local) {
            return self.local_assignment();
        } else if self.check_token_type(Token::If) {
            return self.if_statement();
        } else if self.check_token_type(Token::While) {
            let cond = self.expression()?;
            assert!(self.check_token_type(Token::Do), "while loop missing \"do\" keyword");
            let body = self.do_block()?;
            return Ok(Stmt::WhileLoop(cond, Box::new(Stmt::Block(body))));
        } else if self.check_token_type(Token::Repeat) {
            let body = self.repeat_body()?;
            let cond = self.expression()?;
            return Ok(Stmt::RepeatUntilLoop(Box::new(body), cond));
        } else if self.check_token_type(Token::Return) {
            return Ok(Stmt::Return(self.expr_list()?));
        } else if self.check_token_type(Token::Break) {
            return Ok(Stmt::Break);
        }
        return self.assignment();
    }


    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut res = vec![];
        while self.current < self.tokens.len() {
            res.push(self.statement()?);
        }
        Ok(res)
    }

    pub fn chunk(&mut self) -> Result<Stmt, String> {
        if self.tokens.len() == 0 {
            return Err("No valid tokens".into());
        }

        return Ok(Stmt::Chunk(self.block()?));
    }

    fn current_token(&self) -> Option<Token> {
        return self.tokens.get(self.current).cloned();
    }

    fn previous_token(&self) -> Token {
        if self.current == 0 {
            panic!("Went back before the beginning");
        }

        return self.tokens[self.current - 1].clone();
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    fn check_token_type(&mut self, _type: Token) -> bool {
        let res = self.current_token() == Some(_type);
        if res {
            self.advance();
        }

        res
    }
}