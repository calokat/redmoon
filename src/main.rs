mod tokens;
mod expr;
mod parser;

use std::io;
use tokens::Token;
use expr::Expr;
use parser::Parser;

pub struct Lexer {
    expr_str: String,
    current: usize
}

impl<'a> Lexer {
    pub fn new(s: String) -> Self {
        return Self { expr_str: s, current: 0 };
    }

    pub fn tokenize(&mut self) -> Vec<Token<'a>> {
        let mut ret: Vec<Token<'a>> = vec![];
        while self.current < self.expr_str.len() {
            if let Some(c) = self.expr_str.chars().nth(self.current) {
                if c.is_whitespace() {
                    self.current += 1;
                } else if c.is_numeric() {
                    ret.push(self.lex_number());
                } else if self.is_operator(c) {
                    ret.push(self.lex_operator(c));
                    self.current += 1;
                } else if c == '(' {
                    println!("Here it is again");
                    ret.push(Token::LeftParens);
                    self.current += 1;
                } else if c == ')' {
                    println!("Here it is");
                    ret.push(Token::RightParens);
                    self.current += 1;
            }
            }
        }
        return ret;
    }

    fn lex_number(&mut self) -> Token<'a> {
        let scan_start = self.current;
        while self.current < self.expr_str.len() {
            let c = self.expr_str.chars().nth(self.current).expect("lex_number(): Access should not be out of bounds");
            if c.is_numeric() || c == '.' {
                self.current += 1;
                continue;
            }
            break;
        };
            return Token::LiteralNumber(self.expr_str[scan_start..self.current].parse().expect("lex_number(): Above code should ensure a valid parse"));
    }

    fn lex_operator(&self, c: char) -> Token<'a> {
        match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::ForwardSlash,
            '*' => Token::Star,
            _ => panic!("Unknown symbol")
        }
    }

    fn is_operator(&self, c: char) -> bool {
        match c {
            '+' => true,
            '-' => true,
            '/' => true,
            '*' => true,
            _ => false
        }
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
        match io::stdin().read_line(&mut expr) {
            Ok(_) => {
                if expr.trim().to_lowercase() == "quit" {
                    break;
                }
                
                let mut lexer = Lexer::new(expr);
                let tokens = lexer.tokenize();
                let mut parser = Parser::new(tokens);
                let root_res = parser.expression();
                if let Ok(root) = root_res {
                    let result = eval(root);
                    if let Token::LiteralNumber(result) = result {
                        println!("Final evaluated number: {}", result);
                    }    
                } else if let Err(msg) = root_res {
                    println!("{}", msg);
                    continue;
                }
            },
            Err(_) => {
                println!("Error while reading input");
                break;
            }
        };
    }
}
