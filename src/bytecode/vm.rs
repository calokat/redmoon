use crate::{expr::Expr, stmt::Stmt, tokens::Token, values::Value};

enum ByteCodeOp {
    ADD,
}
enum ByteCode {
    Op(ByteCodeOp),
    ConstRef(u32)
}

pub struct VmEnv {
    bc: ByteCodeBuffer,
    constants: Vec<Value>,
}

type ByteCodeBuffer = Vec<ByteCode>;

impl VmEnv {
    pub fn new(s: Stmt) -> VmEnv {
        let mut vm: VmEnv = VmEnv { bc: Vec::new(), constants: Vec::new() };
        if let Stmt::Chunk(cv) = s {
            for c in cv {
                match c {
                    Stmt::ExprStmt(e) => {
                        match e {
                            Expr::Exprlist(ev) => {
                                match &ev[0] {
                                    Expr::Binary(a,op ,b) => {
                                        if *op == Token::Plus {
                                            if let Expr::Literal(Value::Number(a)) = **a  {
                                                vm.constants.push(Value::Number(a));
                                            }
                                            if let Expr::Literal(Value::Number(b)) = **b  {
                                                vm.constants.push(Value::Number(b));
                                            }
                                            vm.bc.push(ByteCode::ConstRef(0));
                                            vm.bc.push(ByteCode::ConstRef(1));
                                            vm.bc.push(ByteCode::Op(ByteCodeOp::ADD));
                                        } else if let Token::Identifier(s) = op {
                                            panic!("Op is {}", s);
                                        }  else {
                                            todo!()
                                        }        
                                    },
                                    _ => todo!(),
                                }
                            }
                            _ => todo!()
                        }
                    },
                    _ => todo!()
                }
            }
        }
        return vm;
    }
    
    pub fn exec(&self) {
        let mut working_args = Vec::<&Value>::new();
        for byte in self.bc.iter() {
            match byte {
                ByteCode::ConstRef(i) => {
                    working_args.push(&self.constants[*i as usize]);
                },
                ByteCode::Op(op) => {
                    match op {
                        &ByteCodeOp::ADD => {
                            let a = match working_args[0] {
                                &Value::Number(n) => n,
                                _ => todo!()
                            };
                            let b = match working_args[1] {
                                &Value::Number(n) => n,
                                _ => todo!()
                            };
                            let result = a + b;
                            println!("Holy moly {}", result);
                        }
                    }
                }
            }
        }
    }
}

