use std::collections::VecDeque;

use crate::{expr::Expr, stmt::Stmt, table::UserTable, tokens::Token, values::Value};

macro_rules! binary_op {
    ($self:ident, $op:tt) => {
        assert!($self.stack.len() >= 2, "Insufficient number of arguments");
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

enum ByteCode {
    Add,
    Break,
    Subtract,
    Multiply,
    Divide,
    LoadConstant(u32),
    Branch,
    JumpTo(usize),
    JumpBy(usize),
    JumpBack(usize),
    SetEnv,
    GetEnv,
    Placeholder,
}

pub struct VmEnv {
    bc: ByteCodeBuffer,
    constants: ConstantBuffer,
    stack: VecDeque<Value>,
    envs: VecDeque<UserTable>,
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
    fn get_current_env(&self) -> &UserTable {
        self.envs
            .back()
            .expect("Internal Error: At least one environment must be active")
    }

    fn get_current_env_mut(&mut self) -> &mut UserTable {
        self.envs
            .back_mut()
            .expect("Internal Error: At least one environment must be active")
    }

    fn is_truthy(&self, v: &Value) -> bool {
        match v {
            Value::String(s) => !s.is_empty(),
            Value::Nil => false,
            Value::Boolean(b) => b.clone(),
            _ => true,
        }
    }

    pub fn new(s: Stmt) -> VmEnv {
        let mut envs = VecDeque::new();
        envs.push_back(UserTable::new());
        let mut vm: VmEnv = VmEnv {
            envs,
            stack: VecDeque::new(),
            bc: Vec::new(),
            constants: ConstantBuffer::new(),
        };
        vm.build_bytecode_from_stmt(s);
        return vm;
    }

    fn build_bytecode_from_stmt(&mut self, stmt: Stmt) -> usize {
        let initial_bc_length = self.bc.len();
        match stmt {
            Stmt::Chunk(cv) => {
                for c in cv {
                    self.build_bytecode_from_stmt(c);
                }
            }
            Stmt::Block(cv) => {
                for c in cv {
                    self.build_bytecode_from_stmt(c);
                }
            }
            Stmt::ExprStmt(e) => {
                self.build_bytecode_from_expr(&e);
            }
            Stmt::IfStmt(e, b1, b2) => {
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
            }
            Stmt::WhileLoop(cond, body) => {
                let cond_length = self.build_bytecode_from_expr(&cond);
                self.bc.push(ByteCode::Branch);
                self.bc.push(ByteCode::Placeholder);
                let placeholder_index = self.bc.len() - 1;
                let body_length = self.build_bytecode_from_stmt(*body);
                self.bc
                    .push(ByteCode::JumpBack(2 + body_length + cond_length));
                self.bc[placeholder_index] = ByteCode::JumpBy(body_length + 1);
            }
            Stmt::Break => {
                self.bc.push(ByteCode::Break);
            }
            Stmt::Empty => {}
            Stmt::Assignment(l, r) => {
                self.build_bytecode_from_expr(&r);
                match &l {
                    Expr::Exprlist(vars) => {
                        for var in vars {
                            match &var {
                                &Expr::Var(var_name) => {
                                    self.constants.add_constant(
                                        Value::String(var_name.clone()),
                                        &mut self.bc,
                                    );
                                    self.bc.push(ByteCode::SetEnv);
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
                if let Expr::Literal(v) = &**a {
                    self.constants.add_constant(v.clone(), &mut self.bc);
                } else {
                    self.build_bytecode_from_expr(&*a);
                }
                if let Expr::Literal(v) = &**b {
                    self.constants.add_constant(v.clone(), &mut self.bc);
                } else {
                    self.build_bytecode_from_expr(&*b);
                }
                if op == &Token::Plus {
                    self.bc.push(ByteCode::Add);
                } else if op == &Token::Minus {
                    self.bc.push(ByteCode::Subtract);
                } else if op == &Token::Star {
                    self.bc.push(ByteCode::Multiply);
                } else if op == &Token::ForwardSlash {
                    self.bc.push(ByteCode::Divide);
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
                    if self.is_truthy(&a) {
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
                    continue;
                }
                Some(&ByteCode::JumpBack(i)) => {
                    icounter -= i;
                }
                Some(&ByteCode::JumpBy(i)) => {
                    icounter += i;
                }
                Some(&ByteCode::SetEnv) => {
                    assert!(self.stack.len() >= 2, "Insufficient number of arguments");
                    let l = self.stack.pop_back().unwrap();
                    let r = self.stack.pop_back().unwrap();
                    match &l {
                        Value::String(_) => {
                            println!("{} gets assigned to {}", r, l);
                            self.get_current_env_mut().table.borrow_mut().insert(l, r);
                        }
                        _ => panic!("Cannot assign to expression"),
                    }
                }
                Some(&ByteCode::GetEnv) => {
                    assert!(self.stack.len() >= 1, "Insufficient number of arguments");
                    let value = {
                        let key = self.stack.pop_back().unwrap();
                        let current_table = self.get_current_env().table.borrow();
                        current_table.get(&key).unwrap_or(&Value::Nil).clone()
                    };
                    self.stack.push_back(value);
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
