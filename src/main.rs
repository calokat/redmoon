mod tokens;
mod expr;
mod parser;
mod lexer;
mod interpreter;
mod stmt;
mod values;
mod function;
mod table;
mod native_function;

use std::io;
use interpreter::Interpreter;
use tokens::Token;
use expr::Expr;
use parser::Parser;
use lexer::Lexer;
use stmt::Stmt;
use values::Value;

fn main() {
    let mut interp = interpreter::Interpreter::new();
    let args: Vec<String> = std::env::args().collect();
    if let Some(a) = args.get(1) {
        if let Ok(f) = std::fs::read(a) {
            let buffer: String =  String::from_utf8_lossy(&f).to_string();
            exec(buffer, &mut interp);
        } else {
            println!("File {a} does not exist");
        }
        return;
    }
    loop {
        println!("Enter an expression: ");
        let mut expr: String = String::new();
        match io::stdin().read_line(&mut expr) {
            Ok(_) => {
                if expr.trim().to_lowercase() == "quit" {
                    break;
                }
                exec(expr, &mut interp);
            },
            Err(_) => {
                println!("Error while reading input");
                break;
            }
        };
    }
}

fn exec(expr: String, interp: &mut Interpreter) {
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