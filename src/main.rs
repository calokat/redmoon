mod tokens;
mod expr;
mod parser;
mod lexer;

use std::io;
use tokens::Token;
use expr::Expr;
use parser::Parser;
use lexer::Lexer;

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

fn less_than_or_equal<'a>(t1: Token, t2: Token) -> Token<'a> {
    if let Token::LiteralNumber(f1) = t1 {
        match t2 {
            Token::LiteralNumber(f2) => {
                println!("{}", f1 <= f2);
                if f1 <= f2 {
                    return Token::True;
                }
                return Token::False;
            },
            _ => panic!("Comparison only applies to numbers")
        }
    } else {
        panic!("Comparison only applies to numbers");
    }
}

fn less_than<'a>(t1: Token, t2: Token) -> Token<'a> {
    if let Token::LiteralNumber(f1) = t1 {
        match t2 {
            Token::LiteralNumber(f2) => {
                println!("{}", f1 < f2);
                if f1 < f2 {
                    return Token::True;
                }
                return Token::False;
            },
            _ => panic!("Comparison only applies to numbers")
        }
    } else {
        panic!("Comparison only applies to numbers");
    }
}

fn equals<'a>(t1: Token, t2: Token) -> Token<'a> {
    if let Token::LiteralNumber(f1) = t1 {
        match t2 {
            Token::LiteralNumber(f2) => {
                println!("{}", f1 == f2);
                if f1 == f2 {
                    return Token::True;
                }
                return Token::False;
            },
            _ => panic!("Comparison only applies to numbers")
        }
    } else {
        panic!("Comparison only applies to numbers");
    }
}

fn greater_than_or_equal<'a>(t1: Token, t2: Token) -> Token<'a> {
    if let Token::LiteralNumber(f1) = t1 {
        match t2 {
            Token::LiteralNumber(f2) => {
                println!("{}", f1 >= f2);
                if f1 >= f2 {
                    return Token::True;
                }
                return Token::False;
            },
            _ => panic!("Comparison only applies to numbers")
        }
    } else {
        panic!("Comparison only applies to numbers");
    }
}

fn greater_than<'a>(t1: Token, t2: Token) -> Token<'a> {
    if let Token::LiteralNumber(f1) = t1 {
        match t2 {
            Token::LiteralNumber(f2) => {
                println!("{}", f1 > f2);
                if f1 > f2 {
                    return Token::True;
                }
                return Token::False;
            },
            _ => panic!("Comparison only applies to numbers")
        }
    } else {
        panic!("Comparison only applies to numbers");
    }
}



fn eval(expr: Expr) -> Token {
    match expr {
        Expr::Binary(o1, op, o2) => {
            match op {
                Token::Plus => {
                    let t1 = eval(*o1);
                    let t2 = eval(*o2);
                    return add_exprs(t1, t2);
                },
                Token::Minus => {
                    let t1 = eval(*o1);
                    let t2 = eval(*o2);
                    return subtract_exprs(t1, t2);
                },
                Token::Star => {
                    let t1 = eval(*o1);
                    let t3 = eval(*o2);
                    return multiply_exprs(t1, t3);
                },
                Token::ForwardSlash => {
                    let t1 = eval(*o1);
                    let t2 = eval(*o2);
                    return divide_exprs(t1, t2);
                },
                Token::LessThanOrEqual => {
                    let t1 = eval(*o1);
                    let t2 = eval(*o2);
                    return less_than_or_equal(t1, t2);
                },
                Token::LessThan => {
                    let t1 = eval(*o1);
                    let t2 = eval(*o2);
                    return less_than(t1, t2);
                },
                Token::Equals => {
                    let t1 = eval(*o1);
                    let t2 = eval(*o2);
                    return equals(t1, t2);
                },
                Token::GreaterThanOrEqual => {
                    let t1 = eval(*o1);
                    let t2 = eval(*o2);
                    return greater_than_or_equal(t1, t2);
                },
                Token::GreaterThan => {
                    let t1 = eval(*o1);
                    let t2 = eval(*o2);
                    return greater_than(t1, t2);
                }
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
                    } else {
                        match result {
                            Token::False => println!("False"),
                            Token::True => println!("True"),
                            _ => println!("Token should not be a result of an expression")
                        }
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
