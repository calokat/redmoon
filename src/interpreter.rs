use crate::{Token, Expr, Stmt};
use std::collections::HashMap;
pub struct Interpreter {
    _G: HashMap<String, Token>
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
    
    pub fn eval_stmt(&mut self, s: Stmt) -> Result<(), String> {
        match s {
            Stmt::Empty => {
                println!("AN EMPTY AND FUTILE GESTURE");
                return Ok(());
            },
            Stmt::ExprStmt(e) => {
                let t = self.eval_expr(e);
                match t {
                    Token::LiteralNumber(n) => {
                        println!("{}", n);
                        return Ok(());
                    },
                    Token::LiteralString(s) => {
                        println!("{}", s);
                        return Ok(());
                    },
                    Token::True => {
                        println!("True");
                        return Ok(());
                    },
                    Token::False => {
                        println!("False");
                        return Ok(());
                    },
                    Token::Nil => {
                        println!("Expr Nil");
                        return Ok(());
                    },
                    Token::Identifier(s) => {
                        println!("Global Count: {}", self._G.len());
                        if let Some(t) = self._G.get(&s) {
                            match t {
                                Token::LiteralNumber(n) => {
                                    println!("{}", n);
                                    return Ok(());
                                },
                                Token::LiteralString(s) => {
                                    println!("{}", s);
                                    return Ok(());
                                },
                                Token::True => {
                                    println!("True");
                                    return Ok(());
                                },
                                Token::False => {
                                    println!("False");
                                    return Ok(());
                                },
                                Token::Nil => {
                                    println!("Identifier Nil");
                                    return Ok(());
                                },
                                _ => {
                                    return Err("Error retrieving variable".into());
                                }
                            }
                        } else {
                            println!("Nil");
                            return Ok(());
                        }
                    }
                    _ => {
                        return Err("Whoops unknown Token".into());
                    }
                }
            },
            Stmt::Assignment(var, val) => {
                println!("Evaluating assignment");
                if let Expr::Var(id) = var {
                    let val = self.eval_expr(val);
                    self._G.insert(id, val);
                    return Ok(());
                } else {
                    return Err("Cannot assign".into());
                }
            }
        }
    }
    
    fn eval_expr(&mut self, expr: Expr) -> Token {
        match expr {
            Expr::Binary(o1, op, o2) => {
                match op {
                    Token::Plus => {
                        let t1 = self.eval_expr(*o1);
                        let t2 = self.eval_expr(*o2);
                        return Self::add_exprs(t1, t2);
                    },
                    Token::Minus => {
                        let t1 = self.eval_expr(*o1);
                        let t2 = self.eval_expr(*o2);
                        return Self::subtract_exprs(t1, t2);
                    },
                    Token::Star => {
                        let t1 = self.eval_expr(*o1);
                        let t3 = self.eval_expr(*o2);
                        return Self::multiply_exprs(t1, t3);
                    },
                    Token::ForwardSlash => {
                        let t1 = self.eval_expr(*o1);
                        let t2 = self.eval_expr(*o2);
                        return Self::divide_exprs(t1, t2);
                    },
                    Token::LessThanOrEqual => {
                        let t1 = self.eval_expr(*o1);
                        let t2 = self.eval_expr(*o2);
                        return Self::less_than_or_equal(t1, t2);
                    },
                    Token::LessThan => {
                        let t1 = self.eval_expr(*o1);
                        let t2 = self.eval_expr(*o2);
                        return Self::less_than(t1, t2);
                    },
                    Token::Equals => {
                        let t1 = self.eval_expr(*o1);
                        let t2 = self.eval_expr(*o2);
                        return Self::equals(t1, t2);
                    },
                    Token::GreaterThanOrEqual => {
                        let t1 = self.eval_expr(*o1);
                        let t2 = self.eval_expr(*o2);
                        return Self::greater_than_or_equal(t1, t2);
                    },
                    Token::GreaterThan => {
                        let t1 = self.eval_expr(*o1);
                        let t2 = self.eval_expr(*o2);
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
                    let eval_res = self.eval_expr(*expr);
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
                return self.eval_expr(*e);
            },
            Expr::Var(s) => {
                return Token::Identifier(s);
            }
        }
    }
}