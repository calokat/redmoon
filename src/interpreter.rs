use crate::{Token, Expr, Stmt, Value, table::{UserTable, Table}, native_function::NativeFunction, function::Function, gc::gc_store::GcStore, gc::{gc_values::GcValue, gc_key::GcKey}};
use std::{collections::{VecDeque}, borrow::{BorrowMut}};
use ordered_float::OrderedFloat;
#[cfg(target_family = "wasm")]
use wasm_bindgen::{JsValue, prelude::*};

#[cfg(target_family = "wasm")]
#[wasm_bindgen(module = "output-helper.js")]
extern "C" {
    fn append_to_output(str: JsValue);
}

pub struct Interpreter {
    _G: UserTable,
    stack: VecDeque<UserTable>,
    gc: GcStore
}

impl Interpreter {
    pub fn new() -> Self {
        let print = Value::NativeFunctionDef(NativeFunction::new(Box::new(|interp, args| {
            
            if let Some(v) = args.get(0) {
                #[cfg(target_family = "wasm")]
                {
                    let v_str: JsValue = format!("{}\n", v).into();
                    append_to_output(v_str);
                }
                println!("Native print: {v}");
            }
            None
        })));
        let setmetatable = Value::NativeFunctionDef(NativeFunction::new(Box::new(|interp, args| {
            if args.len() < 2 {
                println!("Error in setmetatable(): insufficient number of arguments");
                ()
            }
            let table = args[0].clone();
            let meta = args[1].clone();

            match table {
                Value::Table(mut t) => {
                    let gc_table_value = interp.gc.modify_value(&t).unwrap();
                    match meta {
                        Value::Table(m) => {
                            if let GcValue::Table(gc_table) = gc_table_value {
                                gc_table.insert(Value::MetaKey, Value::Table(m));
                            }
                        },
                        _ => {
                            println!("Error in setmetatable(): both parameters must be tables");
                            ()
                        }
                    }
                },
                _ => {
                    println!("Error in setmetatable(): both parameters must be tables");
                    ()
                }
            }
            Some(args[0].clone())
        })));

        let getmetatable = Value::NativeFunctionDef(NativeFunction::new(Box::new(|interp, args| {
            if let Some(Value::Table(t)) = args.get(0) {
                if let Some(GcValue::Table(gc_table)) = interp.gc.get_value(t) {
                    return gc_table.get(&Value::MetaKey).cloned();
                }
            }
            return None;
        })));

        let collectgarbage = Value::NativeFunctionDef(NativeFunction::new(Box::new(|interp, args| {
            let mut stack = interp.get_stack().clone();
            stack.push_front(interp._G.clone());
            interp.gc.collect_garbage(&stack);
            return Some(Value::Nil);
        })));

        let assert = Value::NativeFunctionDef(NativeFunction::new(Box::new(|interp, args| {
            if let Some(v) = args.get(0) {
                if interp.is_truthy(v) {
                    return Some(Value::ValList(args.to_vec()));
                } else {
                    let error_msg = if let Some(em) = args.get(1) {
                        em.clone()
                    } else {
                        Value::String("Assertion failed!".into())
                    };

                    panic!("{}", &error_msg);
                }
            }
            panic!("assert(): Requires at least 1 argument");
        })));
        let mut _G = UserTable::new();
        _G.table.as_ref().borrow_mut().insert(Value::String("print".into()), print);
        _G.table.as_ref().borrow_mut().insert(Value::String("setmetatable".into()), setmetatable);
        _G.table.as_ref().borrow_mut().insert(Value::String("getmetatable".into()), getmetatable);
        _G.table.as_ref().borrow_mut().insert(Value::String("collectgarbage".into()), collectgarbage);
        _G.table.as_ref().borrow_mut().insert(Value::String("assert".into()), assert);
        Self { _G, stack: VecDeque::new(), gc: GcStore::new() }
    }

    fn push_env(&mut self) {
        self.stack.push_back(UserTable::new());
    }

    fn push_custom_env(&mut self, env: UserTable) {
        self.stack.push_back(env);
    }

    fn pop_env(&mut self) {
        self.stack.pop_back();
    }

    fn get_current_stack_env(&mut self) -> &mut UserTable {
        if let Some(env) = self.stack.back_mut() {
            return env;
        } else {
            return self._G.borrow_mut();
        }

    }

    fn find_var(&self, name: &String) -> Option<Value> {
        let val_key = Value::String(name.clone());
        for t in self.stack.iter().rev() {
            if let Some(ret) = t.table.as_ref().borrow().get(&val_key) {
                return Some(ret.clone());
            }
        }
        return self._G.table.as_ref().borrow_mut().get(&val_key).cloned();
    }

    fn stringify(&self, v: Value) -> Result<Value, String> {
        match v {
            Value::String(s) => return Ok(Value::String(s)),
            Value::Number(n) => return Ok(Value::String(format!("{}", n))),
            _ => return Err("Cannot stringify value".into())
        }
    }

    fn are_both_values_numbers(v1: &Value, v2: &Value) -> Option<(ordered_float::OrderedFloat<f32>, ordered_float::OrderedFloat<f32>)> {
        if let Value::Number(n1) = v1 {
            if let Value::Number(n2) = v2 {
                return Some((n1.clone(), n2.clone()));
            }
        }
        return None;
    }

    fn which_value_is_table<'a>(&self, v1: &'a Value, v2: &'a Value) -> Option<&'a Value> {
        if let Value::Table(table1) = v1 {
            if let Some(GcValue::Table(_)) = self.gc.get_value(table1) {
                return Some(v1);
            }
            return None;
        } else if let Value::Table(table2) = v2 {
            if let Some(GcValue::Table(_)) = self.gc.get_value(table2) {
                return Some(v2);
            }
            return None;
        }
        None
    }

    fn get_table(&self, key: &GcKey) -> Option<&Table> {
        if let Some(GcValue::Table(t)) = self.gc.get_value(&key) {
            return Some(t);
        }
        None
    }

    fn get_metatable<'a>(t: &'a Table) -> Option<GcKey> {
        if let Some(Value::Table(ref ut)) = t.get(&Value::MetaKey) {
            return Some(ut.clone());
        }
        return None;
    }

    fn add_vals<'a>(&mut self, t1: &'a Value, t2: &'a Value) -> Value {
        if let Some((n1, n2)) = Self::are_both_values_numbers(&t1, &t2) {
            return Value::Number(n1 + n2);
        } else if let Some(Value::Table(table)) = self.which_value_is_table(&t1, &t2) {
            if let Some(table) = self.get_table(table) {
                if let Some(key) = Self::get_metatable(table) {
                    if let Some(meta_table) = self.get_table(&key) {
                        if let Some(Value::FunctionDef(fd)) = meta_table.get(&Value::String("__add".into())) {
                            self.call_fn(&fd.clone(), &vec![Expr::Literal(t1.clone()), Expr::Literal(t2.clone())]);
                        }
                    }    
                }
            }
        }
        return Value::Nil;
    }
    
    fn subtract_vals(&mut self, t1: Value, t2: Value) -> Value {
        if let Some((n1, n2)) = Self::are_both_values_numbers(&t1, &t2) {
            return Value::Number(n1 - n2);
        } else if let Some(Value::Table(table)) = self.which_value_is_table(&t1, &t2) {
            if let Some(table) = self.get_table(table) {
                if let Some(key) = Self::get_metatable(table) {
                    if let Some(meta_table) = self.get_table(&key) {
                        if let Some(Value::FunctionDef(fd)) = meta_table.get(&Value::String("__sub".into())) {
                            self.call_fn(&fd.clone(), &vec![Expr::Literal(t1.clone()), Expr::Literal(t2.clone())]);
                        }
                    }    
                }
            }
        }
        return Value::Nil;
    }
    
    fn multiply_vals(&mut self, t1: Value, t2: Value) -> Value {
        if let Some((n1, n2)) = Self::are_both_values_numbers(&t1, &t2) {
            return Value::Number(n1 * n2);
        } else if let Some(Value::Table(table)) = self.which_value_is_table(&t1, &t2) {
            if let Some(table) = self.get_table(table) {
                if let Some(key) = Self::get_metatable(table) {
                    if let Some(meta_table) = self.get_table(&key) {
                        if let Some(Value::FunctionDef(fd)) = meta_table.get(&Value::String("__mul".into())) {
                            self.call_fn(&fd.clone(), &vec![Expr::Literal(t1.clone()), Expr::Literal(t2.clone())]);
                        }
                    }    
                }
            }
        }
        return Value::Nil;
    }
    
    fn divide_vals(&mut self, t1: Value, t2: Value) -> Value {
        if let Some((n1, n2)) = Self::are_both_values_numbers(&t1, &t2) {
            return Value::Number(n1 / n2);
        } else if let Some(Value::Table(table)) = self.which_value_is_table(&t1, &t2) {
            if let Some(table) = self.get_table(table) {
                if let Some(key) = Self::get_metatable(table) {
                    if let Some(meta_table) = self.get_table(&key) {
                        if let Some(Value::FunctionDef(fd)) = meta_table.get(&Value::String("__div".into())) {
                            self.call_fn(&fd.clone(), &vec![Expr::Literal(t1.clone()), Expr::Literal(t2.clone())]);
                        }
                    }    
                }
            }
        }
        return Value::Nil;
    }
    
    fn less_than_or_equal(&mut self, t1: Value, t2: Value) -> Value {
        if let Some((n1, n2)) = Self::are_both_values_numbers(&t1, &t2) {
            return Value::Boolean(n1 <= n2);
        } else if let Some(Value::Table(table)) = self.which_value_is_table(&t1, &t2) {
            if let Some(table) = self.get_table(table) {
                if let Some(key) = Self::get_metatable(table) {
                    if let Some(meta_table) = self.get_table(&key) {
                        if let Some(Value::FunctionDef(fd)) = meta_table.get(&Value::String("__le".into())) {
                            self.call_fn(&fd.clone(), &vec![Expr::Literal(t1.clone()), Expr::Literal(t2.clone())]);
                        }
                    }    
                }
            }
        }
        return Value::Nil;
    }
    
    fn less_than(&mut self, t1: Value, t2: Value) -> Value {
        if let Some((n1, n2)) = Self::are_both_values_numbers(&t1, &t2) {
            return Value::Boolean(n1 < n2);
        } else if let Some(Value::Table(table)) = self.which_value_is_table(&t1, &t2) {
            if let Some(table) = self.get_table(table) {
                if let Some(key) = Self::get_metatable(table) {
                    if let Some(meta_table) = self.get_table(&key) {
                        if let Some(Value::FunctionDef(fd)) = meta_table.get(&Value::String("__lt".into())) {
                            self.call_fn(&fd.clone(), &vec![Expr::Literal(t1.clone()), Expr::Literal(t2.clone())]);
                        }
                    }    
                }
            }
        }
        return Value::Nil;
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
    
    fn equals(&mut self, t1: Value, t2: Value) -> Value {
        if let Value::Interrupt = t2 {
            panic!("Impossible value");
        }
        if let Value::MetaKey = t2 {
            panic!("Impossible value");
        }
        // if let Some(table) = self.which_value_is_table(&t1, &t2) {
        //     let maybe_eq_metamethod: Option<Function> = Self::get_metamethod(self.get_table(&Self::get_metatable(table).unwrap()), "__eq".into());
        //     if let Some(maybe_eq_metamethod) = maybe_eq_metamethod {
        //         return self.call_fn(&maybe_eq_metamethod, &vec![Expr::Literal(t1.clone()), Expr::Literal(t2.clone())]);
        //     }
        // }
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
            },
            Value::MetaKey => {
                panic!("Impossible value");
            }

        }
    }
    
    fn greater_than_or_equal(&mut self, t1: Value, t2: Value) -> Value {
        // if let Some((n1, n2)) = Self::are_both_values_numbers(&t1, &t2) {
        //     return Value::Boolean(n1 >= n2);
        // } else if let Some(table) = Self::which_value_is_table(&t1, &t2) {
        //     let maybe_metamethod: Option<Function> = Self::get_metamethod(Self::get_metatable(table), "__ge".into());
        //     if let Some(maybe_metamethod) = maybe_metamethod {
        //         return self.call_fn(&maybe_metamethod, &vec![Expr::Literal(t1.clone()), Expr::Literal(t2.clone())]);
        //     }
        // }
        return Value::Nil;
    }
    
    fn greater_than(&mut self, t1: Value, t2: Value) -> Value {
        // if let Some((n1, n2)) = Self::are_both_values_numbers(&t1, &t2) {
        //     return Value::Boolean(n1 > n2);
        // } else if let Some(table) = Self::which_value_is_table(&t1, &t2) {
        //     let maybe_metamethod: Option<Function> = Self::get_metamethod(Self::get_metatable(table), "__gt".into());
        //     if let Some(maybe_metamethod) = maybe_metamethod {
        //         return self.call_fn(&maybe_metamethod, &vec![Expr::Literal(t1.clone()), Expr::Literal(t2.clone())]);
        //     }
        // }
        return Value::Nil;
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

    fn complete_closure(&mut self, func: &mut Function) {
        // for (name, value) in func.get_closure().table.as_ref().borrow_mut().iter_mut() {
        //     if let Value::String(s) = name {
        //         *value = self.find_var(&s).unwrap_or_else(|| {println!("Found var {s}, but it's nil"); Value::Nil}).clone();
        //     } else {
        //         panic!("Capturing variable that does not exist");
        //     }
        // }
        func.set_closure(self.stack.clone())
    }

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
                                val_vec.push(v);
                            }
                        } else {
                            val_vec.push(e_res);
                        }
                    }
                }
                if let Expr::Exprlist(var_list) = var {
                    let mut val_counter = 0;
                    for var in var_list.iter() {
                        if let Expr::Var(var_name) = var {
                            let t = self.stack.iter_mut().find(|entry| {entry.table.as_ref().borrow().get(&Value::String(var_name.to_string())) != None}).unwrap_or_else(|| &mut self._G);
                            if let Some(val) = val_vec.get(val_counter) {
                                t.table.as_ref().borrow_mut().insert(Value::String(var_name.clone()), val.clone());
                            } else {
                                t.table.as_ref().borrow_mut().insert(Value::String(var_name.clone()), Value::Nil);
                            }
                        } else if let Expr::Accessor(accessors, field) = var {
                            let key = self.eval_expr(field.as_ref());
                            let resolved_accessors = self.eval_expr(accessors.as_ref());
                            if let Value::Table(accessed_table) = resolved_accessors {
                                if let Some(GcValue::Table(accessed_table)) = self.gc.modify_value(&accessed_table) {
                                    accessed_table.insert(key, val_vec[val_counter].clone());                                    
                                }
                            }
                        }
                        if let Some(Value::FunctionDef(fd)) = val_vec.get_mut(val_counter) {
                            self.complete_closure(fd);
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
                                self.get_current_stack_env().table.as_ref().borrow_mut().insert(Value::String(var_name.clone()), val.clone());
                            } else {
                                self.get_current_stack_env().table.as_ref().borrow_mut().insert(Value::String(var_name.clone()), Value::Nil);
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
            Stmt::NumericForLoop(control_var, control_value, limit, step, body) => {
                let step = self.eval_expr(step);
                if let Value::Number(step_float) = step {
                    if step_float == OrderedFloat(0.0) {
                        panic!("In a numeric for loop, \"step\" cannot be 0");
                    }
                    let control_value = self.eval_expr(control_value);
                    if let Value::Number(mut control_float) = control_value {
                        let limit = self.eval_expr(limit);
                        if let Value::Number(limit_float) = limit {
                            while (step_float > OrderedFloat(0.0) && control_float <= limit_float) ||
                            (step_float < OrderedFloat(0.0) && control_float >= limit_float) {
                                self.push_env();
                                let control_stmt = Stmt::LocalAssignment(control_var.clone(), Expr::Exprlist(vec![Expr::Literal(Value::Number(control_float))]));
                                self.eval_stmt(&control_stmt)?;
                                for s in body {
                                    self.eval_stmt(s)?;
                                }
                                self.pop_env();
                                control_float += step_float;
                            }
                            return Ok(None);
                        } else {
                            panic!("\"limit\" is required to be a number");
                        }
                    } else {
                        panic!("\"control\" is required to be a number")
                    }
                } else {
                    panic!("\"step\" is required to be a number")
                }
            }
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
                        return self.add_vals(&t1, &t2);
                    },
                    Token::Minus => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
                        return self.subtract_vals(t1, t2);
                    },
                    Token::Star => {
                        let t1 = self.eval_expr(&*o1);
                        let t3 = self.eval_expr(&*o2);
                        return self.multiply_vals(t1, t3);
                    },
                    Token::ForwardSlash => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
                        return self.divide_vals(t1, t2);
                    },
                    Token::LessThanOrEqual => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
                        return self.less_than_or_equal(t1, t2);
                    },
                    Token::LessThan => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
                        return self.less_than(t1, t2);
                    },
                    Token::Equals => {
                        let t1 = self.eval_expr(*&o1);
                        let t2 = self.eval_expr(&*o2);
                        return self.equals(t1, t2);
                    },
                    Token::GreaterThanOrEqual => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
                        return self.greater_than_or_equal(t1, t2);
                    },
                    Token::GreaterThan => {
                        let t1 = self.eval_expr(&*o1);
                        let t2 = self.eval_expr(&*o2);
                        return self.greater_than(t1, t2);
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
                if let Value::FunctionDef(fd) = t.clone().borrow_mut() {
                    self.complete_closure(fd)
                }
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
                            return self.call_fn(&fd, vars);
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
                    let table = self.get_table(&ut).unwrap();
                    if let Some(accessed_value) = table.get(&accessor) {
                        return accessed_value.clone();
                    }
                } else if let Expr::Accessor(_, _) = bt.as_ref() {
                    return self.eval_expr(bt.as_ref());
                }
                Value::Nil
            },
            Expr::FieldList(fl) => {
                let mut user_table = crate::table::Table::new();
                for (key, value) in fl.into_iter() {
                    user_table.insert(self.eval_expr(&*key), self.eval_expr(&*value));
                }
                let gc_key = GcKey::new();
                self.gc.store(gc_key.clone(), GcValue::Table(user_table));
                return Value::Table(gc_key);
            },
        }
    }

    fn call_fn(&mut self, fd: &Function, vars: &Vec<Expr>) -> Value {
        let mut arg_values: Vec<Value> = vec![];
        for v in vars {
            let arg_value = self.eval_expr(v);
            if let Value::ValList(mut vl) = arg_value {
                arg_values.append(&mut vl);
            } else {
                arg_values.push(arg_value);
            }
        }
        let mut args_decls: Vec<Stmt> = vec![];
        let mut arg_counter = 0;
        for param in fd.get_params() {
            args_decls.push(Stmt::LocalAssignment(Expr::Exprlist(vec![param.clone()]), Expr::Exprlist(vec![Expr::Literal(arg_values.get(arg_counter).unwrap_or_else(|| &Value::Nil).clone())])));
            arg_counter += 1;
        }
        let func_body = fd.get_body();
        for c in fd.get_closure().clone() {
            self.push_custom_env(c.clone());
        }
        for decl in args_decls.into_iter() {
            if let Err(e) = self.eval_stmt(&decl) {
                panic!("Error declaring args: {e}");
            }
        }
        let func_eval = self.eval_stmt(&func_body);
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
        return Value::Nil;
    }

    pub fn get_stack(&self) -> &VecDeque<UserTable> {
        &self.stack
    }
}
