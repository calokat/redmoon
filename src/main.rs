mod tokens;
mod expr;
mod parser;
mod lexer;
mod interpreter;

use std::io;
use tokens::Token;
use expr::Expr;
use parser::Parser;
use lexer::Lexer;

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
                let root_res = parser.expression();
                if let Ok(root) = root_res {
                    let result = interp.eval(root);
                    if let Token::LiteralNumber(result) = result {
                        println!("Final evaluated number: {}", result);
                    } else {
                        match result {
                            Token::False => println!("False"),
                            Token::True => println!("True"),
                            _ => println!("Token should not be a result of an expression")
                        }
                    }    
                } else if let Err(msg) = root_res {
                    println!("{}", msg);
                    continue;
                }
            },
            Err(_) => {
                println!("Error while reading input");
                break;
            }
        };
    }
}
