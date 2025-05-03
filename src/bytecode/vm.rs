use std::collections::VecDeque;

use crate::{
    expr::Expr,
    gc::{gc_key::GcKey, gc_store::GcStore, gc_values::GcValue},
    stmt::Stmt,
    table::{Table, UserTable},
    tokens::Token,
    values::Value,
};

macro_rules! binary_op {
    ($self:ident, $op:tt) => {
        assert!($self.stack.len() >= 2, "Insufficient number of arguments {}", $self.stack.len());
        let a = $self.stack.pop_back().unwrap();
        let b = $self.stack.pop_back().unwrap();
        if let Ok(c) = b $op a {
            println!("{}", c);
            $self.stack.push_back(c);
        } else {
            println!("What do you think you are doing");
        }
    };
}

macro_rules! compare_nums {
    ($v1:ident, $v2:ident, $op:tt) => {
        match $v1 {
            Value::Number(n1) => match $v2 {
                Value::Number(n2) => n1 $op n2,
                _ => false,
            },
            _ => false,
        }
    };
}

macro_rules! logical_op {
    ($v1:ident, $v2:ident, $op:tt) => {
        VmEnv::is_truthy($v1) $op VmEnv::is_truthy($v2)
    };
}

enum ByteCode {
    Add,
    And,
    LessThan,
    GreaterThanOrEqual,
    GreaterThan,
    LessThanOrEqual,
    Break,
    Subtract,
    Multiply,
    Divide,
    LoadConstant(u32),
    Equals,
    Branch,
    JumpTo(usize),
    JumpBy(usize),
    JumpBack(usize),
    Or,
    Not,
    SetGlobalEnv,
    SetLocalEnv,
    SetTableField,
    GetTableField,
    GetEnv,
    PushEnv,
    PopEnv,
    Placeholder,
}

pub struct VmEnv {
    bc: ByteCodeBuffer,
    constants: ConstantBuffer,
    stack: VecDeque<Value>,
    global_env: UserTable,
    local_envs: VecDeque<UserTable>,
    gc: GcStore,
}

struct ConstantBuffer {
    v: Vec<Value>,
    latest: u32,
}

impl ConstantBuffer {
    pub fn new() -> Self {
        Self {
            v: Vec::new(),
            latest: 0,
        }
    }
    pub fn add_constant(&mut self, val: Value, bc: &mut ByteCodeBuffer) {
        self.v.push(val);
        bc.push(ByteCode::LoadConstant(self.latest));
        self.latest += 1;
    }
    pub fn load_constant(&self, index: usize) -> Option<&Value> {
        self.v.get(index)
    }
}

type ByteCodeBuffer = Vec<ByteCode>;

impl VmEnv {
    fn get_current_env_mut(&mut self) -> &mut UserTable {
        self.local_envs.back_mut().unwrap_or(&mut self.global_env)
    }

    fn find_var(&self, key: &Value) -> Value {
        return self
            .local_envs
            .iter()
            .find_map(|le| {
                if let Some(v) = le.table.borrow().get(key) {
                    return Some(v).cloned();
                } else {
                    return None;
                }
            })
            .or(self.global_env.table.borrow().get(key).cloned())
            .unwrap_or(Value::Nil);
    }

    fn equals(t1: Value, t2: Value) -> Value {
        match t1 {
            Value::Number(n1) => match t2 {
                Value::Number(n2) => Value::Boolean(n1 == n2),
                _ => Value::Boolean(false),
            },
            Value::Nil => match t2 {
                Value::Nil => Value::Boolean(true),
                _ => Value::Boolean(false),
            },
            Value::Boolean(b1) => match t2 {
                Value::Boolean(b2) => Value::Boolean(b1 == b2),
                _ => Value::Boolean(false),
            },
            Value::String(s1) => match t2 {
                Value::String(s2) => Value::Boolean(s1 == s2),
                _ => Value::Boolean(false),
            },
            Value::FunctionDef(f1) => match t2 {
                Value::FunctionDef(f2) => Value::Boolean(f1 == f2),
                _ => Value::Boolean(false),
            },
            Value::NativeFunctionDef(nf1) => match t2 {
                Value::NativeFunctionDef(nf2) => Value::Boolean(nf1 == nf2),
                _ => Value::Boolean(false),
            },
            Value::Table(ut1) => match t2 {
                Value::Table(ut2) => Value::Boolean(ut1 == ut2),
                _ => Value::Boolean(false),
            },
            Value::ValList(_list) => {
                panic!("Cannot compare value lists to each other");
            }
            Value::Interrupt => {
                panic!("Impossible value");
            }
            Value::MetaKey => {
                panic!("Impossible value");
            }
            Value::Varargs(_) => {
                return if let Value::Varargs(_) = t2 {
                    Value::Boolean(true)
                } else {
                    Value::Boolean(false)
                }
            }
            Value::VarargsIdentifier => Value::Boolean(t2 == Value::VarargsIdentifier),
        }
    }

    fn is_truthy(v: &Value) -> bool {
        match v {
            Value::String(s) => !s.is_empty(),
            Value::Nil => false,
            Value::Boolean(b) => b.clone(),
            _ => true,
        }
    }

    pub fn new(s: Stmt) -> VmEnv {
        let local_envs = VecDeque::new();
        let global_env = UserTable::new();
        let mut vm: VmEnv = VmEnv {
            local_envs,
            stack: VecDeque::new(),
            bc: Vec::new(),
            constants: ConstantBuffer::new(),
            global_env,
            gc: GcStore::new(),
        };
        vm.build_bytecode_from_stmt(s);
        return vm;
    }

    fn build_bytecode_from_stmt(&mut self, stmt: Stmt) -> usize {
        let initial_bc_length = self.bc.len();
        match stmt {
            Stmt::Chunk(cv) | Stmt::Block(cv) | Stmt::DoBlock(cv) => {
                for c in cv {
                    self.build_bytecode_from_stmt(c);
                }
            }
            Stmt::ExprStmt(e) => {
                self.build_bytecode_from_expr(&e);
            }
            Stmt::IfStmt(e, b1, b2) => {
                self.bc.push(ByteCode::PushEnv);
                self.build_bytecode_from_expr(&e);
                self.bc.push(ByteCode::Branch);
                self.bc.push(ByteCode::Placeholder);
                let bc_diff_1 = self.build_bytecode_from_stmt(*b1);
                let placeholder_index = self.bc.len() - 1 - bc_diff_1;
                self.bc[placeholder_index] = ByteCode::JumpBy(bc_diff_1 + 1);

                self.bc.push(ByteCode::Placeholder);
                let bc_diff_2 = self.build_bytecode_from_stmt(*b2);
                let placeholder_index = self.bc.len() - 1 - bc_diff_2;
                self.bc[placeholder_index] = ByteCode::JumpBy(bc_diff_2);
                self.bc.push(ByteCode::PopEnv);
            }
            Stmt::WhileLoop(cond, body) => {
                self.bc.push(ByteCode::PushEnv);
                let cond_length = self.build_bytecode_from_expr(&cond);
                self.bc.push(ByteCode::Branch);
                self.bc.push(ByteCode::Placeholder);
                let placeholder_index = self.bc.len() - 1;
                let body_length = self.build_bytecode_from_stmt(*body);
                self.bc
                    .push(ByteCode::JumpBack(3 + body_length + cond_length));
                self.bc[placeholder_index] = ByteCode::JumpBy(body_length + 1);
                self.bc.push(ByteCode::PopEnv);
            }
            Stmt::Break => {
                self.bc.push(ByteCode::Break);
            }
            Stmt::Empty => {}
            Stmt::Assignment(l, r) => {
                self.build_bytecode_from_expr(&r);
                match &l {
                    Expr::Exprlist(vars) => {
                        for var in vars.iter().rev() {
                            match &var {
                                &Expr::Var(var_name) => {
                                    self.constants.add_constant(
                                        Value::String(var_name.clone()),
                                        &mut self.bc,
                                    );
                                    self.bc.push(ByteCode::SetGlobalEnv);
                                }
                                _ => panic!("Cannot assign to expression"),
                            }
                        }
                    }
                    _ => panic!("Cannot assign to expression"),
                }
            }
            Stmt::RepeatUntilLoop(body, cond) => {
                self.bc.push(ByteCode::PushEnv);
                let body_len = self.build_bytecode_from_stmt(*body);
                let cond_len = self.build_bytecode_from_expr(&cond);
                self.bc.push(ByteCode::Branch);
                self.bc.push(ByteCode::JumpBack(body_len + cond_len + 2));
                self.bc.push(ByteCode::PopEnv);
            }
            Stmt::LocalAssignment(l, r) => {
                self.build_bytecode_from_expr(&r);
                match &l {
                    Expr::Exprlist(vars) => {
                        for var in vars.iter().rev() {
                            match &var {
                                &Expr::Var(var_name) => {
                                    self.constants.add_constant(
                                        Value::String(var_name.clone()),
                                        &mut self.bc,
                                    );
                                    self.bc.push(ByteCode::SetLocalEnv);
                                }
                                _ => panic!("Cannot assign to expression"),
                            }
                        }
                    }
                    _ => panic!("Cannot assign to expression"),
                }
            }
            _ => todo!(),
        }
        return self.bc.len() - initial_bc_length;
    }

    fn build_bytecode_from_expr(&mut self, e: &Expr) -> usize {
        let initial_bc_length = self.bc.len();
        match e {
            Expr::Exprlist(expr_list) => {
                for el in expr_list.iter() {
                    self.build_bytecode_from_expr(el);
                }
            }
            Expr::Binary(a, op, b) => {
                self.build_bytecode_from_expr(&**a);
                self.build_bytecode_from_expr(&**b);
                match op {
                    &Token::Plus => self.bc.push(ByteCode::Add),
                    &Token::Minus => self.bc.push(ByteCode::Subtract),
                    &Token::Star => self.bc.push(ByteCode::Multiply),
                    &Token::ForwardSlash => self.bc.push(ByteCode::Divide),
                    &Token::LessThan => self.bc.push(ByteCode::LessThan),
                    &Token::LessThanOrEqual => self.bc.push(ByteCode::LessThanOrEqual),
                    &Token::GreaterThanOrEqual => self.bc.push(ByteCode::GreaterThanOrEqual),
                    &Token::GreaterThan => self.bc.push(ByteCode::GreaterThan),
                    &Token::Equals => self.bc.push(ByteCode::Equals),
                    &Token::And => self.bc.push(ByteCode::And),
                    &Token::Or => self.bc.push(ByteCode::Or),
                    _ => panic!("Unsupported binary operation"),
                }
            }
            Expr::Grouping(e) => {
                self.build_bytecode_from_expr(e);
            }
            Expr::Literal(l) => {
                self.constants.add_constant(l.clone(), &mut self.bc);
            }
            Expr::Var(name) => {
                self.constants
                    .add_constant(Value::String(name.clone()), &mut self.bc);
                self.bc.push(ByteCode::GetEnv);
            }
            Expr::Unary(e, t) => match t {
                Token::Not => {
                    self.build_bytecode_from_expr(&**e);
                    self.bc.push(ByteCode::Not);
                }
                _ => panic!("Invalid unary operator"),
            },
            Expr::FieldList(fl) => {
                let gc_key = GcKey::new();
                self.constants
                    .add_constant(Value::Table(gc_key.clone()), &mut self.bc);
                let table_constant_index = self.constants.latest - 1;
                println!("index is {table_constant_index}");
                for (key, value) in fl.into_iter() {
                    self.build_bytecode_from_expr(key);
                    self.build_bytecode_from_expr(value);
                    self.bc.push(ByteCode::LoadConstant(table_constant_index));
                    self.bc.push(ByteCode::SetTableField);
                }
                self.gc.store(gc_key, GcValue::Table(Table::new()));
            }
            Expr::Accessor(t, a) => {
                self.build_bytecode_from_expr(a);
                self.build_bytecode_from_expr(t);
                self.bc.push(ByteCode::GetTableField);
            }
            _ => {
                todo!()
            }
        }
        return self.bc.len() - initial_bc_length;
    }

    pub fn exec(&mut self) {
        let mut icounter = 0usize;
        loop {
            match self.bc.get(icounter) {
                Some(&ByteCode::Add) => {
                    binary_op!(self, +);
                }
                Some(&ByteCode::Subtract) => {
                    binary_op!(self, -);
                }
                Some(&ByteCode::Multiply) => {
                    binary_op!(self, *);
                }
                Some(&ByteCode::Divide) => {
                    binary_op!(self, /);
                }
                Some(&ByteCode::LessThan) => {
                    assert!(
                        self.stack.len() >= 2,
                        "Insufficient number of arguments {}",
                        self.stack.len()
                    );
                    let r = self.stack.pop_back().unwrap();
                    let l = self.stack.pop_back().unwrap();
                    self.stack.push_back(Value::Boolean(compare_nums!(l, r, <)));
                }
                Some(&ByteCode::GreaterThanOrEqual) => {
                    assert!(
                        self.stack.len() >= 2,
                        "Insufficient number of arguments {}",
                        self.stack.len()
                    );
                    let r = self.stack.pop_back().unwrap();
                    let l = self.stack.pop_back().unwrap();
                    self.stack
                        .push_back(Value::Boolean(compare_nums!(l, r, >=)));
                }
                Some(&ByteCode::GreaterThan) => {
                    assert!(
                        self.stack.len() >= 2,
                        "Insufficient number of arguments {}",
                        self.stack.len()
                    );
                    let r = self.stack.pop_back().unwrap();
                    let l = self.stack.pop_back().unwrap();
                    self.stack.push_back(Value::Boolean(compare_nums!(l, r, >)));
                }
                Some(&ByteCode::LessThanOrEqual) => {
                    assert!(
                        self.stack.len() >= 2,
                        "Insufficient number of arguments {}",
                        self.stack.len()
                    );
                    let r = self.stack.pop_back().unwrap();
                    let l = self.stack.pop_back().unwrap();
                    self.stack
                        .push_back(Value::Boolean(compare_nums!(l, r, <=)));
                }
                Some(&ByteCode::LoadConstant(u)) => {
                    let constant = self
                        .constants
                        .load_constant(u as usize)
                        .expect(&format!("Constant buffer should have value at index {}", u))
                        .clone();
                    self.stack.push_back(constant);
                }
                Some(&ByteCode::Branch) => {
                    assert!(self.stack.len() >= 1, "Insufficient number of arguments");
                    let a = self.stack.pop_back().unwrap();
                    if Self::is_truthy(&a) {
                        icounter += 1;
                    }
                }
                Some(&ByteCode::Break) => {
                    while let Some(bc) = self.bc.get(icounter) {
                        if let &ByteCode::JumpBack(_) = bc {
                            break;
                        }
                        icounter += 1;
                    }
                    if let None = self.bc.get(icounter) {
                        panic!("Used 'break' outside of a loop");
                    }
                }
                Some(&ByteCode::JumpTo(i)) => {
                    icounter = i;
                }
                Some(&ByteCode::And) => {
                    let l = &self.stack.pop_back().unwrap();
                    let r = &self.stack.pop_back().unwrap();

                    self.stack.push_back(Value::Boolean(logical_op!(l, r, &&)));
                }
                Some(&ByteCode::Or) => {
                    let l = &self.stack.pop_back().unwrap();
                    let r = &self.stack.pop_back().unwrap();

                    self.stack.push_back(Value::Boolean(logical_op!(l, r, ||)));
                }
                Some(&ByteCode::Not) => {
                    if let Value::Boolean(b) = self
                        .stack
                        .pop_back()
                        .expect("Stack should have at least 1 argument")
                    {
                        self.stack.push_back(Value::Boolean(!b));
                    }
                }
                Some(&ByteCode::Equals) => {
                    let v1 = self.stack.pop_back().unwrap();
                    let v2 = self.stack.pop_back().unwrap();

                    self.stack.push_back(Self::equals(v1, v2));
                }
                Some(&ByteCode::JumpBack(i)) => {
                    icounter -= i;
                }
                Some(&ByteCode::JumpBy(i)) => {
                    icounter += i;
                }
                Some(&ByteCode::SetGlobalEnv) => {
                    assert!(self.stack.len() >= 2, "Insufficient number of arguments");
                    let l = self.stack.pop_back().unwrap();
                    let r = self.stack.pop_back().unwrap();
                    match &l {
                        Value::String(_) => {
                            println!("{} gets assigned to {}", r, l);
                            self.global_env.table.borrow_mut().insert(l, r);
                        }
                        _ => panic!("Cannot assign to expression"),
                    }
                }
                Some(&ByteCode::SetLocalEnv) => {
                    assert!(self.stack.len() >= 2, "Insufficient number of arguments");
                    let l = self.stack.pop_back().unwrap();
                    let r = self.stack.pop_back().unwrap();
                    match &l {
                        Value::String(_) => {
                            println!("{} gets assigned to {} locally", r, l);
                            self.get_current_env_mut().table.borrow_mut().insert(l, r);
                        }
                        _ => panic!("Cannot assign to expression"),
                    }
                }
                Some(&ByteCode::GetEnv) => {
                    assert!(self.stack.len() >= 1, "Insufficient number of arguments");
                    let value = {
                        let key = self.stack.pop_back().unwrap();
                        self.find_var(&key)
                    };
                    self.stack.push_back(value);
                }
                Some(&ByteCode::PushEnv) => {
                    self.local_envs.push_back(UserTable::new());
                }
                Some(&ByteCode::PopEnv) => {
                    self.local_envs.pop_back();
                }
                Some(&ByteCode::GetTableField) => {
                    let table = self.stack.pop_back().expect("Expected table to access");
                    let accessor = self.stack.pop_back().expect("Expected accessor for table");
                    if let Value::Table(gc_key) = table {
                        if let Some(GcValue::Table(tbl)) = self.gc.get_value(&gc_key.clone()) {
                            self.stack
                                .push_back(tbl.get(&accessor).unwrap_or(&Value::Nil).clone());
                        }
                    }
                }
                Some(&ByteCode::SetTableField) => {
                    let table = self.stack.pop_back().expect("Expected table to set");
                    let value = self.stack.pop_back().expect("Expected value");
                    let key = self.stack.pop_back().expect("Expected accessor");
                    match table {
                        Value::Table(gc_key) => match self.gc.modify_value(&gc_key) {
                            Some(table) => match table {
                                GcValue::Table(table) => table.insert(key, value),
                            },
                            None => panic!("Missing table in GC store"),
                        },
                        _ => panic!("Cannot set value of non-table"),
                    };
                }
                Some(&ByteCode::Placeholder) => {
                    panic!("Internal error during bytecode generation")
                }
                None => {
                    break;
                }
            }
            icounter += 1;
        }
    }
}
