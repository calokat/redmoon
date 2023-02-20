use crate::{Token, Expr, Stmt, Value, table::{Table, UserTable}};
use std::{collections::{VecDeque}, borrow::{BorrowMut, Borrow}};


pub struct Interpreter {
    _G: Table,
    stack: VecDeque<Table>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { _G: Table::new(), stack: VecDeque::new() }
    }

    fn push_env(&mut self) {
        self.stack.push_back(Table::new());
    }

    fn pop_env(&mut self) {
        self.stack.pop_back();
    }

    fn get_current_stack_env(&mut self) -> &mut Table {
        if let Some(env) = self.stack.back_mut() {
            return env;
        } else {
            return self._G.borrow_mut();
        }

    }

    fn find_var(&self, name: &String) -> Option<&Value> {
        let val_key = Value::String(name.clone());
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
        match t1 {
            Value::Number(n1) => {
                match t2 {
                    Value::Number(n2) => Value::Boolean(n1 == n2),
                    _ => Value::Boolean(false)
                }
            },
            Value::Nil => {
                match t2 {
                    Value::Nil => Value::Boolean(true),
                    _ => Value::Boolean(false)
                }
            },
            Value::Boolean(b1) => {
                match t2 {
                    Value::Boolean(b2) => Value::Boolean(b1 == b2),
                    _ => Value::Boolean(false)
                }
            },
            Value::String(s1) => {
                match t2 {
                    Value::String(s2) => Value::Boolean(s1 == s2),
                    _ => Value::Boolean(false)
                }
            },
            Value::FunctionDef(f1) => {
                match t2 {
                    Value::FunctionDef(f2) => Value::Boolean(f1 == f2),
                    _ => Value::Boolean(false)
                }
            },
            Value::Table(ut1) => {
                match t2 {
                    Value::Table(ut2) => Value::Boolean(ut1 == ut2),
                    _ => Value::Boolean(false)
                }
            }
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

    fn is_truthy(&self, v: Value) -> bool {
        match v {
            Value::String(s) => !s.is_empty(),
            Value::Nil => false,
            Value::Boolean(b) => b,
            _ => true
        }
    }

    pub fn eval_stmt(&mut self, s: &Stmt) -> Result<(), String> {
        match s {
            Stmt::Empty => {
                return Ok(());
            },
            Stmt::ExprStmt(e) => {
                let v = self.eval_expr(&e);
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
                    Value::FunctionDef(_fd) => {
                        println!("Function definition");
                        return Ok(());
                    },
                    Value::Table(_ut) => {
                        println!("Table!");
                        return Ok(())
                    }
                }
            },
            Stmt::Assignment(var, val) => {
                let mut val_vec = vec![];
                if let Expr::Exprlist(el) = val {
                    for e in el.into_iter() {
                        val_vec.push(self.eval_expr(&e));
                    }
                }
                if let Expr::Exprlist(var_list) = var {
                    let mut val_counter = 0;
                    for var in var_list.iter() {
                        if let Expr::Var(var_name) = var {
                            if let Some(val) = val_vec.get(val_counter) {
                                self._G.insert(Value::String(var_name.clone()), val.clone());
                            } else {
                                self._G.insert(Value::String(var_name.clone()), Value::Nil);
                            }
                        } else if let Expr::Accessor(accessors, field) = var {
                            println!("Hello darkness");
                            let key = self.eval_expr(field.as_ref());
                            let resolved_accessors = self.eval_expr(accessors.as_ref());
                            if let Value::Table(accessed_table) = resolved_accessors {
                                accessed_table.table.as_ref().borrow_mut().insert(key, val_vec[val_counter].clone());
                            }
                        }
                        val_counter += 1;
                    }
                } else {
                    return Err("Cannot assign to this".into());
                }
                return Ok(());
            },
            Stmt::LocalAssignment(var, val) => {
                let mut val_vec = vec![];
                if let Expr::Exprlist(el) = val {
                    for e in el.into_iter() {
                        val_vec.push(self.eval_expr(&e));
                    }
                }
                if let Expr::Exprlist(var_list) = var {
                    let mut val_counter = 0;
                    for var in var_list.iter() {
                        if let Expr::Var(var_name) = var {
                            if let Some(val) = val_vec.get(val_counter) {
                                self.get_current_stack_env().insert(Value::String(var_name.clone()), val.clone());
                            } else {
                                self.get_current_stack_env().insert(Value::String(var_name.clone()), Value::Nil);
                            }
                            val_counter += 1;
                        } else {
                            return Err("Cannot assign to this".into());
                        }
                    }
                } else {
                    return Err("Cannot assign to this".into());
                }
                return Ok(());

            }
            Stmt::Block(stmts) => {
                self.push_env();
                for s in stmts {
                   if let Err(err) = self.eval_stmt(s) {
                    return Err(err);
                   }
                }
                self.pop_env();
                Ok(())
            },
            Stmt::IfStmt(cond, body) => {
                let cond_res = self.eval_expr(&cond);
                let mut eval_res = Ok(());
                if self.is_truthy(cond_res) {
                    self.push_env();
                    eval_res = self.eval_stmt(&*body);
                    self.pop_env();
                }
                eval_res
            },
            Stmt::WhileLoop(cond, body) => {
                loop {
                    let cond_res = self.eval_expr(&cond);
                    if self.is_truthy(cond_res) {
                        self.push_env();
                        if let Err(s) = self.eval_stmt(&*body) {
                            return Err(s);
                        }
                        self.pop_env();
                    } else {
                        break;
                    }
                }
                Ok(())
            },
            Stmt::RepeatUntilLoop(body, cond) => {
                loop {
                    self.push_env();
                    if let Err(s) = self.eval_stmt(&*body) {
                        return Err(s);
                    }
                    let cond_res = self.eval_expr(cond);
                    if self.is_truthy(cond_res) {
                        self.pop_env();
                        break;
                    }
                    self.pop_env();
                }
                Ok(())
            }
        }
    }
    
    
    fn eval_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Binary(o1, op, o2) => {
                match op {
                    Token::Plus => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
                        return Self::add_vals(t1, t2);
                    },
                    Token::Minus => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
                        return Self::subtract_vals(t1, t2);
                    },
                    Token::Star => {
                        let t1 = self.eval_expr(&*o1);
                        let t3 = self.eval_expr(&*o2);
                        return Self::multiply_vals(t1, t3);
                    },
                    Token::ForwardSlash => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
                        return Self::divide_vals(t1, t2);
                    },
                    Token::LessThanOrEqual => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
                        return Self::less_than_or_equal(t1, t2);
                    },
                    Token::LessThan => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
                        return Self::less_than(t1, t2);
                    },
                    Token::Equals => {
                        let t1 = self.eval_expr(*&o1);
                        let t2 = self.eval_expr(&*o2);
                        return Self::equals(t1, t2);
                    },
                    Token::GreaterThanOrEqual => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
                        return Self::greater_than_or_equal(t1, t2);
                    },
                    Token::GreaterThan => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
                        return Self::greater_than(t1, t2);
                    },
                    Token::Concatenation => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
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
                match t {
                    Value::Boolean(b) => Value::Boolean(*b),
                    Value::Nil => Value::Nil,
                    Value::String(s) => Value::String(s.clone()),
                    Value::Number(n) => Value::Number(*n),
                    Value::FunctionDef(fd) => Value::FunctionDef(fd.clone()),
                    Value::Table(ut) => Value::Table(ut.clone()),
                }
            },
            Expr::Unary(e, op) => {
                if op == &Token::Minus {
                    if let Expr::Literal(t) = &**e {
                        if let Value::Number(n) = t {
                            return Value::Number(-n);
                        } else {
                            panic!("Unsupported negation");
                        }
                    } else if let Expr::Grouping(expr) = &**e {
                    let eval_res = self.eval_expr(&*expr);
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
                if let Expr::Exprlist(el) = &**e {
                    if el.len() == 1 {
                        return self.eval_expr(&el[0]);
                    }
                }
                return self.eval_expr(&*e);
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
                        },
                        Value::FunctionDef(_) => {
                            res += "<function> \t".into()
                        },
                        Value::Table(_) => {
                            res += "<table> \t".into()
                        }
                    }
                }
                return Value::String(res);
            },
            Expr::FunctionCall(func_id, _vars) => {
                println!("Return statements coming soon");
                if let Expr::Var(func_id) = &**func_id {
                    let func_val = self.find_var(func_id);
                    match func_val {
                        Some(v) => {
                            match v {
                                Value::FunctionDef(fd) => {
                                    if let Err(s) = self.eval_stmt(&*fd.get_impl().body.clone()) {
                                        println!("{s}");
                                        return Value::Nil;
                                    }
                                }
                                _ => {

                                }
                            }
                        },
                        _ => {println!("Cannot call value");}
                    }
                }
                return Value::Nil;
            },
            Expr::Accessor(bt, ba) => {
                if let Value::Table(ut) = self.eval_expr(bt.as_ref()) {
                    let accessor = self.eval_expr(ba.as_ref());
                    if let Some(accessed_value) = ut.table.as_ref().borrow().get(&accessor) {
                        return accessed_value.clone();
                    }
                } else if let Expr::Accessor(_, _) = bt.as_ref() {
                    return self.eval_expr(bt.as_ref());
                }
                Value::Nil
            }
        }
    }
}