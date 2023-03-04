use crate::{Token, Expr, Stmt, Value, table::{Table}, native_function::NativeFunction, function::Function};
use std::{collections::{VecDeque}, borrow::{BorrowMut}};


pub struct Interpreter {
    _G: Table,
    stack: VecDeque<Table>,
}

impl Interpreter {
    pub fn new() -> Self {
        let print = Value::NativeFunctionDef(NativeFunction::new(Box::new(|interp, args| {
            if let Some(v) = args.get(0) {
                println!("Native print: {v}");
            }
            None
        })));
        let mut _G = Table::new();
        _G.insert(Value::String("print".into()), print);
        Self { _G, stack: VecDeque::new() }
    }

    fn push_env(&mut self) {
        self.stack.push_back(Table::new());
    }

    fn push_custom_env(&mut self, env: Table) {
        println!("Pushing custom env");
        for (name, value) in &env {
            println!("{}: {value}", name);
        };
        self.stack.push_back(env);
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
        for t in self.stack.iter().rev() {
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

    fn value_length(v: &Value) -> Option<Value> {
        match v {
            Value::String(s) => {
                Some(Value::Number(ordered_float::OrderedFloat(s.len() as f32)))
            },
            // TODO: Add support for table lengths
            _ => None
        }
    }
    
    fn equals(t1: Value, t2: Value) -> Value {
        if let Value::Interrupt = t2 {
            panic!("Impossible value");
        }
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
            Value::NativeFunctionDef(nf1) => {
                match t2 {
                    Value::NativeFunctionDef(nf2) => Value::Boolean(nf1 == nf2),
                    _ => Value::Boolean(false)
                }
            }
            Value::Table(ut1) => {
                match t2 {
                    Value::Table(ut2) => Value::Boolean(ut1 == ut2),
                    _ => Value::Boolean(false)
                }
            },
            Value::ValList(_list) => {
                panic!("Cannot compare value lists to each other");
            }, 
            Value::Interrupt => {
                panic!("Impossible value");
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

    fn is_truthy(&self, v: &Value) -> bool {
        match v {
            Value::String(s) => !s.is_empty(),
            Value::Nil => false,
            Value::Boolean(b) => b.clone(),
            _ => true
        }
    }

    fn eval_block(&mut self, stmts: &Vec<Stmt>) -> Result<Option<Expr>, String> {
        for s in stmts {
            let res = self.eval_stmt(s);
           if let Err(err) = res {
            return Err(err);
           } else if let Ok(None) = res {
                continue;
           } else {
            return res;
           }
        }
        Ok(None)
    }

    // fn complete_closure(&mut self, func: &mut Function) {
    //     for (name, value) in func.get_closure().table.as_ref().borrow_mut().iter_mut() {
    //         if let Value::String(r) = name {
    //             *value = self.find_var(&r).unwrap_or(&Value::Nil).clone();
    //         } else {
    //             println!("Ohno");
    //         }
    //     }
    // } 

    pub fn eval_stmt(&mut self, s: &Stmt) -> Result<Option<Expr>, String> {
        match s {
            Stmt::Empty => {
                return Ok(None);
            },
            Stmt::ExprStmt(e) => {
                self.eval_expr(&e);
                Ok(None)
            },
            Stmt::Assignment(var, val) => {
                let mut val_vec = vec![];
                if let Expr::Exprlist(el) = val {
                    for e in el.into_iter() {
                        let e_res = self.eval_expr(&e);
                        if let Value::ValList(vl) = e_res {
                            for v in vl.into_iter() {
                                // if let Value::FunctionDef(mut fd) = v {
                                //     self.complete_closure(&mut fd);
                                //     val_vec.push(Value::FunctionDef(fd));
                                // } else {
                                //     val_vec.push(v);
                                // }
                                val_vec.push(v);
                            }
                        } else {
                            // if let Value::FunctionDef(mut fd) = e_res {
                            //     self.complete_closure(&mut fd);
                            //     val_vec.push(Value::FunctionDef(fd));
                            // } else {
                            //     val_vec.push(e_res);
                            // }
                            val_vec.push(e_res);
                        }
                    }
                }
                if let Expr::Exprlist(var_list) = var {
                    let mut val_counter = 0;
                    for var in var_list.iter() {
                        if let Expr::Var(var_name) = var {
                            let t = self.stack.iter_mut().find(|entry| {entry.get(&Value::String(var_name.to_string())) != None}).unwrap_or_else(|| &mut self._G);
                            if let Some(val) = val_vec.get(val_counter) {
                                t.insert(Value::String(var_name.clone()), val.clone());
                            } else {
                                t.insert(Value::String(var_name.clone()), Value::Nil);
                            }
                        } else if let Expr::Accessor(accessors, field) = var {
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
                return Ok(None);
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
                return Ok(None);

            }
            Stmt::Block(stmts) => {
                return self.eval_block(stmts);
            },
            Stmt::DoBlock(stmts) => {
                self.push_env();
                let eval_res = self.eval_block(stmts);
                if let Ok(Some(expr)) = eval_res {
                    let literal = self.eval_expr(&expr);
                    self.pop_env();
                    return Ok(Some(Expr::Literal(literal)));
                } else if let Err(_) = eval_res {
                    self.pop_env();
                    return eval_res;
                } else {
                    self.pop_env();
                    Ok(None)
                }
            }
            Stmt::IfStmt(cond, body, _else) => {
                let cond_res = self.eval_expr(&cond);
                let mut eval_res = Ok(None);
                if self.is_truthy(&cond_res) {
                    self.push_env();
                    eval_res = self.eval_stmt(&*body);
                    self.pop_env();
                    if let Ok(None) = eval_res {
                        // do nothing
                    } else if let Ok(Some(ret)) = eval_res {
                        return Ok(Some(ret));
                    }
                } else {
                    self.push_env();
                    eval_res = self.eval_stmt(_else);
                    self.pop_env();
                    if let Ok(None) = eval_res {
                        // do nothing
                    } else if let Ok(Some(ret)) = eval_res {
                        return Ok(Some(ret));
                    }
                }
                eval_res
            },
            Stmt::WhileLoop(cond, body) => {
                loop {
                    let cond_res = self.eval_expr(&cond);
                    if self.is_truthy(&cond_res) {
                        self.push_env();
                        let res = self.eval_stmt(&*body);
                        if let Err(s) = res {
                            self.pop_env();
                            return Err(s);
                        } else if let Ok(None) = res {
                            // Do nothing
                        } else if let Ok(Some(ret)) = res {
                            if let Expr::Literal(Value::Interrupt) = ret {
                                break;
                            }
                            self.pop_env();
                            return Ok(Some(ret));
                        }
                        self.pop_env();
                    } else {
                        break;
                    }
                }
                Ok(None)
            },
            Stmt::RepeatUntilLoop(body, cond) => {
                loop {
                    self.push_env();
                    let stmt_res = self.eval_stmt(&*body);
                    if let Err(s) = stmt_res {
                        return Err(s);
                    } else if let Ok(None) = stmt_res {
                        // Do nothing
                    } else if let Ok(Some(Expr::Literal(Value::Interrupt))) = stmt_res {
                        break;
                    } else if let Ok(Some(ret)) = stmt_res {
                        return Ok(Some(ret));
                    }
                    let cond_res = self.eval_expr(cond);
                    if self.is_truthy(&cond_res) {
                        self.pop_env();
                        break;
                    }
                    self.pop_env();
                }
                Ok(None)
            },
            Stmt::Return(ret) => {
                Ok(Some(ret.clone()))
            },
            Stmt::Break => Ok(Some(Expr::Literal(Value::Interrupt))),
            Stmt::Chunk(stmts) => {
                for s in stmts.iter() {
                    let res = self.eval_stmt(s);
                    if !res.is_ok() {
                        return res;
                    } else if let Ok(Some(e)) = res {
                        if let Expr::Literal(Value::Interrupt) = e {
                            return Err("Break outside loop".into());
                        }
                    }
                }
                Ok(None)
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
                    },
                    Token::And => {
                        let v1 = self.eval_expr(&*o1);
                        if !self.is_truthy(&v1) {
                            return v1;
                        }
                        return self.eval_expr(&*o2);
                    },
                    Token::Or => {
                        let v1 = self.eval_expr(&*o1);
                        if self.is_truthy(&v1) {
                            return v1;
                        }
                        return self.eval_expr(&*o2);
                    }
                    _ => panic!("Operator not supported yet")
                }
            },
            Expr::Literal(t) => {
                t.clone()
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
                } else if op == &Token::Not {
                    let to_not = &self.eval_expr(e);
                    return Value::Boolean(!self.is_truthy(to_not)); 
                } else if op == &Token::Pound {
                    return Self::value_length(&self.eval_expr(e)).unwrap_or_else(|| Value::Nil);
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
                if el.len() == 1 {
                    if let Some(e) = el.get(0) {
                        return self.eval_expr(e);
                    }
                }
                let mut values: Vec<Value> = vec![];
                for e in el.iter() {
                    values.push(self.eval_expr(e));
                }
                return Value::ValList(values);
            },
            Expr::FunctionCall(func_id, vars) => {
                let func_val = self.eval_expr(&**func_id);
                    match func_val {
                        Value::FunctionDef(fd) => {
                            let mut args_decls: Vec<Stmt> = vec![];
                            let mut arg_counter = 0;
                            for param in fd.get_params() {
                                if let Some(arg) = vars.get(arg_counter) {
                                    args_decls.push(Stmt::LocalAssignment(Expr::Exprlist(vec![param.clone()]), Expr::Exprlist(vec![arg.clone()])));
                                } else {
                                    args_decls.push(Stmt::LocalAssignment(Expr::Exprlist(vec![param.clone()]), Expr::Exprlist(vec![Expr::Literal(Value::Nil)])));
                                }
                                arg_counter += 1;
                            }
                            let func_body = fd.get_body();
                            self.push_env();
                            // self.push_custom_env(fd.get_closure().table.as_ref().borrow().clone());
                            for decl in args_decls.into_iter() {
                                if let Err(e) = self.eval_stmt(&decl) {
                                    panic!("Error declaring args: {e}");
                                }
                            }
                            let func_eval = self.eval_stmt(func_body);
                            if let Err(func_body_err) = func_eval {
                                println!("{func_body_err}");
                            } else if let Ok(None) = func_eval {
                                // Do nothing
                            } else if let Ok(Some(func_ret)) = func_eval {
                                if let Expr::Literal(Value::Interrupt) = func_ret {
                                    panic!("Break outside loop");
                                }
                                let ret_val = self.eval_expr(&func_ret);
                                self.pop_env();
                                return ret_val;
                            }
                            self.pop_env();
                        },
                        Value::NativeFunctionDef(nf) => {
                            let mut args: Vec<Value> = vec![];
                            for p in vars.iter() {
                                args.push(self.eval_expr(p));
                            }
                            self.push_env();
                            let func_eval = nf.call(self, &mut args);
                            self.pop_env();
                            if let Some(ret_val) = func_eval {
                                return ret_val;
                            }
                        },
                        Value::Nil => {
                            println!("Cannot call nil");
                        },
                        _ => {
                            println!("Cannot call value");
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
            },
            Expr::FieldList(fl) => {
                let user_table = crate::table::UserTable::new();
                for (key, value) in fl.into_iter() {
                    user_table.table.as_ref().borrow_mut().insert(self.eval_expr(&*key), self.eval_expr(&*value));
                }
                return Value::Table(user_table);
            },
        }
    }
}
