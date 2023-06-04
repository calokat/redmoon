pub mod tokens;
pub mod expr;
pub mod parser;
pub mod lexer;
pub mod interpreter;
pub mod stmt;
pub mod values;
pub mod function;
pub mod table;
pub mod native_function;
pub mod gc;

use interpreter::Interpreter;
use tokens::Token;
use expr::Expr;
use parser::Parser;
use lexer::Lexer;
use stmt::Stmt;
use values::Value;

pub fn exec_script(script: String) {
    let mut interp = Interpreter::new();
    exec_repl(script, &mut interp);
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

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn execute(blob: &str) {
    exec_script(blob.into())
}