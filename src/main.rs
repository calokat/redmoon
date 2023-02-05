use std::io;

#[derive(PartialEq)]
#[derive(Clone, Copy)]
enum Token<'a> {
    LiteralNumber(f32),
    LiteralString(&'a str),
    True,
    False,
    Nil,
    Plus,
    Minus,
    Star,
    ForwardSlash,
    Equals,
    NotEquals,
    Not,
    LeftParens,
    RightParens
}


enum Expr<'a> {
    Binary(Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Unary(Box<Expr<'a>>, Token<'a>),
    Literal(Token<'a>),
    Grouping(Box<Expr<'a>>),
}


fn lex_number(string: &str) -> (usize, Token) {
    for (i, c) in String::from(string).chars().enumerate() {
        if c.is_numeric() || c == '.' {
             continue;
        }
        return (i - 1, Token::LiteralNumber(string[0..i].parse().expect("Above code should ensure a valid parse")));
        };
        return (string.len() - 1, Token::LiteralNumber(string[..].parse().expect("Above code should ensure a valid parse")));
    }


fn is_operator(c: char) -> bool {
    match c {
        '+' => true,
        '-' => true,
        '/' => true,
        '*' => true,
        _ => false
    }
}

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        return Self { tokens, current: 0 }
    }

    pub fn primary(&mut self) -> Expr<'a> {
        if self.check_token_type(Token::True) ||
        self.check_token_type(Token::False) ||
        self.check_token_type(Token::Nil) {
            return Expr::Literal(self.previous_token());
        }
        if let Token::LiteralNumber(n) = self.current_token() {
            let res = self.current_token();
            self.advance();
            return Expr::Literal(res);
        } else if let Token::LiteralString(_) = self.current_token() {
            let res = self.current_token();
            self.advance();
            return Expr::Literal(res);
        } else if self.current_token() == Token::LeftParens {
            self.advance();
            let expr = Expr::Grouping(Box::new(self.expression()));
            if !self.check_token_type(Token::RightParens) {
                panic!("Missing right parens");
            }

            return expr;
        }
        unreachable!("All primary cases should have been handled");
    }

    pub fn unary(&mut self) -> Expr<'a> {
        if self.check_token_type(Token::Minus) {
            let operator = self.previous_token();
            let right = self.unary();
            return Expr::Unary(Box::new(right), operator);
        }
        return self.primary();
    }

    pub fn factor(&mut self) -> Expr<'a> {
        let mut expr = self.unary();

        while self.check_token_type(Token::Star) ||
        self.check_token_type(Token::ForwardSlash) {
            let operator = self.previous_token();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return expr;
    }

    pub fn term(&mut self) -> Expr<'a> {
        let mut expr = self.factor();

        while self.check_token_type(Token::Plus) ||
        self.check_token_type(Token::Minus) {
            let operator = self.previous_token();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return expr;
    }

    pub fn equality(&mut self) -> Expr<'a> {
        let mut expr = self.term();

        while self.check_token_type(Token::Equals) ||
        self.check_token_type(Token::NotEquals) {
            let operator = self.previous_token();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return expr;
}

    pub fn expression(&mut self) -> Expr<'a> {
        return self.equality();
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

fn add_exprs<'a>(t1: Token, t2: Token) -> Token<'a> {
    if let Token::LiteralNumber(f1) = t1 {
        if let Token::LiteralNumber(f2) = t2 {
            println!("{}", f1 + f2);
            return Token::LiteralNumber(f1 + f2);
        } else {
            panic!("Addition only applies to numbers");
        }
    } else {
        panic!("Addition only applies to numbers");
    }
}

fn subtract_exprs<'a>(t1: Token, t2: Token) -> Token<'a> {
    if let Token::LiteralNumber(f1) = t1 {
        match t2 {
            Token::LiteralNumber(f2) => {
                println!("{}", f1 - f2);
                return Token::LiteralNumber(f1 - f2);
            },
            _ => panic!("Subtraction only applies to numbers")
        }
    } else {
        panic!("Subtraction only applies to numbers");
    }
}

fn multiply_exprs<'a>(t1: Token, t2: Token) -> Token<'a> {
    if let Token::LiteralNumber(f1) = t1 {
        match t2 {
            Token::LiteralNumber(f2) => {
                println!("{}", f1 * f2);
                return Token::LiteralNumber(f1 * f2);
            },
            _ => panic!("Multiplication only applies to numbers")
        }
    } else {
        panic!("Multiplication only applies to numbers");
    }
}

fn divide_exprs<'a>(t1: Token, t2: Token) -> Token<'a> {
    if let Token::LiteralNumber(f1) = t1 {
        match t2 {
            Token::LiteralNumber(f2) => {
                println!("{}", f1 / f2);
                return Token::LiteralNumber(f1 / f2);
            },
            _ => panic!("Division only applies to numbers")
        }
    } else {
        panic!("Division only applies to numbers");
    }
}



fn eval(expr: Expr) -> Token {
    match expr {
        Expr::Binary(o1, op, o2) => {
            match op {
                Token::Plus => {
                    let res_op1 = eval(*o1);
                    let res_op2 = eval(*o2);
                    assert!(res_op2 != Token::Plus);
                    return add_exprs(res_op1, res_op2);
                },
                Token::Minus => {
                    let res_op1 = eval(*o1);
                    let res_op2 = eval(*o2);
                    return subtract_exprs(res_op1, res_op2);
                },
                Token::Star => {
                    let res_op1 = eval(*o1);
                    let res_op2 = eval(*o2);
                    return multiply_exprs(res_op1, res_op2);
                },
                Token::ForwardSlash => {
                    let res_op1 = eval(*o1);
                    let res_op2 = eval(*o2);
                    return divide_exprs(res_op1, res_op2);
                },
                _ => panic!("Operator not supported yet")
            }
        },
        Expr::Literal(t) => {return t;},
        Expr::Unary(e, op) => {
            if op == Token::Minus {
                if let Expr::Literal(t) = *e {
                    if let Token::LiteralNumber(n) = t {
                        return Token::LiteralNumber(-n);
                    } else {
                        panic!("Unsupported negation");
                    }
                } else if let Expr::Grouping(expr) = *e {
                    let eval_res = eval(*expr);
                    if let Token::LiteralNumber(i) = eval_res {
                        return Token::LiteralNumber(-i);
                    } else {
                        panic!("Grouping token should return number literals");
                    }
                } else {
                    panic!("Cannot negate this");
                }
            } else {
                panic!("Unsupported unary operation");
            }
        },
        Expr::Grouping(e) => {
            return eval(*e);
        }
    }
}

fn main() {
    loop {
        println!("Enter an expression: ");
        let mut expr: String = String::new();
        let mut tokens: Vec<Token> = vec![];
        match io::stdin().read_line(&mut expr) {
            Ok(_) => {
                if expr.trim().to_lowercase() == "quit" {
                    break;
                }
                let mut it = 0;
                loop {
                    if let Some(c) = expr.chars().nth(it) {
                        if c.is_whitespace() {
                            // do nothing
                        } else if c == '(' {
                            tokens.push(Token::LeftParens);
                        } else if c == ')' {
                            tokens.push(Token::RightParens);
                        } else if c.is_numeric() {
                            let (advance, tkn) = lex_number(&expr[it..]);
                            it += advance;
                            tokens.push(tkn);
                        } else if is_operator(c) {
                            tokens.push(match c {
                                    '+' => Token::Plus,
                                    '-' => Token::Minus,
                                    '/' => Token::ForwardSlash,
                                    '*' => Token::Star,
                                    _ => panic!("Unknown symbol")
                                }
                            );
                        }
                        it += 1;
                    } else {
                        break;
                    }
                }
                let mut parser = Parser::new(tokens);
                let root = parser.expression();
                let result = eval(root);
                if let Token::LiteralNumber(result) = result {
                    println!("Final evaluated number: {}", result);
                }
            },
            Err(_) => {
                println!("Error while reading input");
                break;
            }
        };
    }
}
