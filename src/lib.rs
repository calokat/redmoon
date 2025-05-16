pub mod bytecode;
pub mod expr;
pub mod function;
pub mod gc;
pub mod lexer;
pub mod native_function;
pub mod number;
pub mod parser;
pub mod stmt;
pub mod table;
pub mod tokens;
pub mod values;
pub mod vm;

use expr::Expr;
use lexer::Lexer;
use parser::Parser;
use stmt::Stmt;
use tokens::Token;
use values::Value;

pub fn exec_script(script: String) {
    exec_bytecode(script);
}

pub fn exec_bytecode(script: String) {
    let mut lexer = Lexer::new(&script.as_str());
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let chunk = parser.chunk();
    if let Ok(stmt) = chunk {
        let vm = crate::vm::VmEnv::new(stmt);
        vm.exec();
    } else if let Err(e) = chunk {
        println!("{}", e);
    }
}

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn execute(blob: &str) {
    exec_script(blob.into())
}
