use crate::{Token, Expr, Stmt, Value};
use std::collections::{HashMap, VecDeque};

type Table = HashMap<Value, Value>;

pub struct Interpreter {
    _G: Table,
    stack: VecDeque<Table>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { _G: Table::new(), stack: VecDeque::new() }
    }

    fn find_var(&self, name: String) -> Option<&Value> {
        let val_key = Value::String(name);
        for t in self.stack.iter() {
            if let Some(ret) = t.get(&val_key) {
                return Some(ret);
            }
        }
        return self._G.get(&val_key);
    }

    fn stringify(&self, v: Value) -> Result<Value, String> {
        match v {
            Value::String(s) => return Ok(Value::String(s)),
            Value::Number(n) => return Ok(Value::String(format!("{}", n))),
            _ => return Err("Cannot stringify value".into())
        }
    }

    fn add_vals(t1: Value, t2: Value) -> Value {
        if let Value::Number(f1) = t1 {
            if let Value::Number(f2) = t2 {
                println!("{}", f1 + f2);
                return Value::Number(f1 + f2);
            } else {
                panic!("Addition only applies to numbers");
            }
        } else {
            panic!("Addition only applies to numbers");
        }
    }
    
    fn subtract_vals(t1: Value, t2: Value) -> Value {
        if let Value::Number(f1) = t1 {
            match t2 {
                Value::Number(f2) => {
                    println!("{}", f1 - f2);
                    return Value::Number(f1 - f2);
                },
                _ => panic!("Subtraction only applies to numbers")
            }
        } else {
            panic!("Subtraction only applies to numbers");
        }
    }
    
    fn multiply_vals(t1: Value, t2: Value) -> Value {
        if let Value::Number(f1) = t1 {
            match t2 {
                Value::Number(f2) => {
                    println!("{}", f1 * f2);
                    return Value::Number(f1 * f2);
                },
                _ => panic!("Multiplication only applies to numbers")
            }
        } else {
            panic!("Multiplication only applies to numbers");
        }
    }
    
    fn divide_vals(t1: Value, t2: Value) -> Value {
        if let Value::Number(f1) = t1 {
            match t2 {
                Value::Number(f2) => {
                    println!("{}", f1 / f2);
                    return Value::Number(f1 / f2);
                },
                _ => panic!("Division only applies to numbers")
            }
        } else {
            panic!("Division only applies to numbers");
        }
    }
    
    fn less_than_or_equal(t1: Value, t2: Value) -> Value {
        if let Value::Number(f1) = t1 {
            match t2 {
                Value::Number(f2) => {
                    println!("{}", f1 <= f2);
                    Value::Boolean(f1 <= f2)
                },
                _ => panic!("Comparison only applies to numbers")
            }
        } else {
            panic!("Comparison only applies to numbers");
        }
    }
    
    fn less_than(t1: Value, t2: Value) -> Value {
        if let Value::Number(f1) = t1 {
            match t2 {
                Value::Number(f2) => {
                    println!("{}", f1 < f2);
                    return Value::Boolean(f1 < f2);
                },
                _ => panic!("Comparison only applies to numbers")
            }
        } else {
            panic!("Comparison only applies to numbers");
        }
    }
    
    fn equals(t1: Value, t2: Value) -> Value {
        if let Value::Number(f1) = t1 {
            match t2 {
                Value::Number(f2) => {
                    println!("{}", f1 == f2);
                    Value::Boolean(f1 == f2)
                },
                _ => panic!("Comparison only applies to numbers")
            }
        } else {
            panic!("Comparison only applies to numbers");
        }
    }
    
    fn greater_than_or_equal(t1: Value, t2: Value) -> Value {
        if let Value::Number(f1) = t1 {
            match t2 {
                Value::Number(f2) => {
                    println!("{}", f1 >= f2);
                    Value::Boolean(f1 >= f2)
                },
                _ => panic!("Comparison only applies to numbers")
            }
        } else {
            panic!("Comparison only applies to numbers");
        }
    }
    
    fn greater_than(t1: Value, t2: Value) -> Value {
        if let Value::Number(f1) = t1 {
            match t2 {
                Value::Number(f2) => {
                    println!("{}", f1 > f2);
                    Value::Boolean(f1 > f2)
                },
                _ => panic!("Comparison only applies to numbers")
            }
        } else {
            panic!("Comparison only applies to numbers");
        }
    }
    
    pub fn eval_stmt(&mut self, s: Stmt) -> Result<(), String> {
        println!("Evaluating statement");
        match s {
            Stmt::Empty => {
                return Ok(());
            },
            Stmt::ExprStmt(e) => {
                let v = self.eval_expr(e);
                match v {
                    Value::Boolean(b) => {
                        println!("{}", b);
                        return Ok(());
                    },
                    Value::Nil => {
                        println!("Nil");
                        return Ok(());
                    },
                    Value::Number(n) => {
                        println!("{}", n);
                        return Ok(());
                    },
                    Value::String(s) => {
                        println!("{}", s);
                        return Ok(());
                    },
                }
            },
            Stmt::Assignment(var, val) => {
                println!("Evaluating assignment");
                let mut val_vec = vec![];
                if let Expr::Exprlist(el) = val {
                    for e in el.into_iter() {
                        val_vec.push(self.eval_expr(e));
                    }
                }
                if let Expr::Exprlist(var_list) = var {
                    let mut var_dq = VecDeque::from(var_list);
                    let mut val_dq = VecDeque::from(val_vec);
                    while let Some(Expr::Var(var)) = var_dq.pop_front() {
                        if let Some(value) = val_dq.pop_front() {
                            self._G.insert(Value::String(var), value);
                        } else {
                            self._G.insert(Value::String(var), Value::Nil);
                        }
                    }
                } else {
                    return Err("Cannot assign to this".into());
                }
                return Ok(());
            }
        }
    }
    
    
    fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Binary(o1, op, o2) => {
                match op {
                    Token::Plus => {
                        let t1 = self.eval_expr(*o1);
                        let t2 = self.eval_expr(*o2);
                        return Self::add_vals(t1, t2);
                    },
                    Token::Minus => {
                        let t1 = self.eval_expr(*o1);
                        let t2 = self.eval_expr(*o2);
                        return Self::subtract_vals(t1, t2);
                    },
                    Token::Star => {
                        let t1 = self.eval_expr(*o1);
                        let t3 = self.eval_expr(*o2);
                        return Self::multiply_vals(t1, t3);
                    },
                    Token::ForwardSlash => {
                        let t1 = self.eval_expr(*o1);
                        let t2 = self.eval_expr(*o2);
                        return Self::divide_vals(t1, t2);
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
                    },
                    Token::Concatenation => {
                        let t1 = self.eval_expr(*o1);
                        let t2 = self.eval_expr(*o2);
                        let s1 = self.stringify(t1);
                        let s2 = self.stringify(t2);
                        if let Ok(Value::String(s1)) = s1 {
                            if let Ok(Value::String(s2)) = s2 {
                                return Value::String(s1 + &s2);
                            }
                        }
                        panic!("Cannot concatenate");
                    }
                    _ => panic!("Operator not supported yet")
                }
            },
            Expr::Literal(t) => {
                t
            },
            Expr::Unary(e, op) => {
                if op == Token::Minus {
                    if let Expr::Literal(t) = *e {
                        if let Value::Number(n) = t {
                            return Value::Number(-n);
                        } else {
                            panic!("Unsupported negation");
                        }
                    } else if let Expr::Grouping(expr) = *e {
                    let eval_res = self.eval_expr(*expr);
                        if let Value::Number(i) = eval_res {
                            return Value::Number(-i);
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
                if let Some(v) = self.find_var(s) {
                    return v.clone();
                }
                Value::Nil
            },
            Expr::Exprlist(el) => {
                let mut res = String::new();
                for e in el {
                    let e_res = self.eval_expr(e);
                    match e_res {
                        Value::Boolean(b) => {
                            if b  {
                                res += "True\t";
                            } else {
                                res += "False\t";
                            }
                        },
                        Value::Nil => {
                            res += "Nil\t"
                        },
                        Value::Number(n) => {
                            let n = n.0;
                            res += format!("{n}\t").as_str();
                        },
                        Value::String(s) => {
                            res += &(s + "\t");
                        }
                    }
                }
                return Value::String(res);
            },
        }
    }
}