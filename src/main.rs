mod tokens;
mod expr;
mod parser;
mod lexer;
mod interpreter;
mod stmt;

use std::io;
use tokens::Token;
use expr::Expr;
use parser::Parser;
use lexer::Lexer;
use stmt::Stmt;

fn main() {
    let mut interp = interpreter::Interpreter::new();
    loop {
        println!("Enter an expression: ");
        let mut expr: String = String::new();
        match io::stdin().read_line(&mut expr) {
            Ok(_) => {
                if expr.trim().to_lowercase() == "quit" {
                    break;
                }
                
                let mut lexer = Lexer::new(expr.as_str());
                let tokens = lexer.tokenize();
                let mut parser = Parser::new(tokens);
                let smt = parser.statement();
                match smt {
                    Ok(smt) => {
                        if let Err(s) = interp.eval_stmt(smt) {
                            println!("{}", s);
                        }
                    },
                    Err(s) => {
                        println!("{}", s);
                    }
                }
            },
            Err(_) => {
                println!("Error while reading input");
                break;
            }
        };
    }
}
