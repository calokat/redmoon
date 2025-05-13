pub mod bytecode;
pub mod expr;
pub mod function;
pub mod gc;
pub mod interpreter;
pub mod lexer;
pub mod native_function;
pub mod parser;
pub mod stmt;
pub mod table;
pub mod tokens;
pub mod values;
pub mod vm;

use expr::Expr;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use stmt::Stmt;
use tokens::Token;
use values::Value;

pub fn exec_script(script: String) {
    let mut interp = Interpreter::new();
    exec_bytecode(script);
}

pub fn exec_repl(expr: String, interp: &mut Interpreter) {
    let mut lexer = Lexer::new(expr.as_str());
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let chunk = parser.chunk();
    if let Ok(chunk) = chunk {
        if let Err(err) = interp.eval_stmt(&chunk) {
            println!("{err}");
        }
    } else if let Err(s) = chunk {
        println!("Error parsing: {s}");
    }
}

pub fn exec_bytecode(script: String) {
    let mut lexer = Lexer::new(&script.as_str());
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let chunk = parser.chunk();
    if let Ok(stmt) = chunk {
        let mut vm = crate::vm::VmEnv::new(stmt);
        vm.exec();
    }
}

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn execute(blob: &str) {
    exec_script(blob.into())
}
