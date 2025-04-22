use std::collections::VecDeque;

use crate::{expr::Expr, stmt::Stmt, tokens::Token, values::Value};

macro_rules! binary_op {
    ($self:ident, $op:tt) => {
        assert!($self.stack.len() >= 2, "Insufficient number of arguments");
        let a = $self.stack.pop_back().unwrap();
        let b = $self.stack.pop_back().unwrap();
        if let Ok(c) = b $op a {
            println!("{}", c);
            $self.stack.push_back(c);
        }
    };
}

enum ByteCode {
    Add,
    Subtract,
    Multiply,
    Divide,
    LoadConstant(u32),
}

pub struct VmEnv {
    bc: ByteCodeBuffer,
    constants: ConstantBuffer,
    stack: VecDeque<Value>,
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
    pub fn new(s: Stmt) -> VmEnv {
        let mut vm: VmEnv = VmEnv {
            stack: VecDeque::new(),
            bc: Vec::new(),
            constants: ConstantBuffer::new(),
        };
        vm.build_bytecode_from_stmt(s);
        return vm;
    }

    fn build_bytecode_from_stmt(&mut self, stmt: Stmt) {
        if let Stmt::Chunk(cv) = stmt {
            for c in cv {
                match c {
                    Stmt::ExprStmt(e) => self.build_bytecode_from_expr(&e),
                    _ => todo!(),
                }
            }
        }
    }

    fn build_bytecode_from_expr(&mut self, e: &Expr) {
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
            _ => {
                todo!()
            }
        }
    }

    pub fn exec(&mut self) {
        for byte in self.bc.iter() {
            match byte {
                ByteCode::Add => {
                    binary_op!(self, +);
                }
                ByteCode::Subtract => {
                    binary_op!(self, -);
                }
                ByteCode::Multiply => {
                    binary_op!(self, *);
                }
                ByteCode::Divide => {
                    binary_op!(self, /);
                }
                ByteCode::LoadConstant(u) => {
                    let constant = self
                        .constants
                        .load_constant(*u as usize)
                        .expect(&format!(
                            "Constant buffer should have value at index {}",
                            *u
                        ))
                        .clone();
                    self.stack.push_back(constant);
                }
            }
        }
    }
}
