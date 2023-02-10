use crate::{Token, Expr};
use std::collections::HashMap;
pub struct Interpreter {
    _G: HashMap<String, Expr>
}

impl Interpreter {
    pub fn new() -> Self {
        Self { _G: HashMap::new() }
    }

    fn add_exprs(t1: Token, t2: Token) -> Token {
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
    
    fn subtract_exprs(t1: Token, t2: Token) -> Token {
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
    
    fn multiply_exprs(t1: Token, t2: Token) -> Token {
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
    
    fn divide_exprs(t1: Token, t2: Token) -> Token {
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
    
    fn less_than_or_equal(t1: Token, t2: Token) -> Token {
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
    
    fn less_than(t1: Token, t2: Token) -> Token {
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
    
    fn equals(t1: Token, t2: Token) -> Token {
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
    
    fn greater_than_or_equal<'a>(t1: Token, t2: Token) -> Token {
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
    
    fn greater_than<'a>(t1: Token, t2: Token) -> Token {
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
    
                                
    
    pub fn eval(&mut self, expr: Expr) -> Token {
        match expr {
            Expr::Binary(o1, op, o2) => {
                match op {
                    Token::Plus => {
                        let t1 = self.eval(*o1);
                        let t2 = self.eval(*o2);
                        return Self::add_exprs(t1, t2);
                    },
                    Token::Minus => {
                        let t1 = self.eval(*o1);
                        let t2 = self.eval(*o2);
                        return Self::subtract_exprs(t1, t2);
                    },
                    Token::Star => {
                        let t1 = self.eval(*o1);
                        let t3 = self.eval(*o2);
                        return Self::multiply_exprs(t1, t3);
                    },
                    Token::ForwardSlash => {
                        let t1 = self.eval(*o1);
                        let t2 = self.eval(*o2);
                        return Self::divide_exprs(t1, t2);
                    },
                    Token::LessThanOrEqual => {
                        let t1 = self.eval(*o1);
                        let t2 = self.eval(*o2);
                        return Self::less_than_or_equal(t1, t2);
                    },
                    Token::LessThan => {
                        let t1 = self.eval(*o1);
                        let t2 = self.eval(*o2);
                        return Self::less_than(t1, t2);
                    },
                    Token::Equals => {
                        let t1 = self.eval(*o1);
                        let t2 = self.eval(*o2);
                        return Self::equals(t1, t2);
                    },
                    Token::GreaterThanOrEqual => {
                        let t1 = self.eval(*o1);
                        let t2 = self.eval(*o2);
                        return Self::greater_than_or_equal(t1, t2);
                    },
                    Token::GreaterThan => {
                        let t1 = self.eval(*o1);
                        let t2 = self.eval(*o2);
                        return Self::greater_than(t1, t2);
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
                    let eval_res = self.eval(*expr);
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
                return self.eval(*e);
            },
            Expr::Var(s) => {
                return Token::Nil;
            }
        }
    }
}