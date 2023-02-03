use std::{io, borrow::BorrowMut};

struct Token<'a> {
    string: &'a str,
}


fn lexNumber(string: &str) -> (usize, Token) {
    for (i, c) in String::from(string).chars().enumerate() {
        if c.is_numeric() || c == '.' {
             continue;
        }
        return (i, Token {
            string: &string[0..i],
        });
    }
    return (string.len(), Token {
        string,
    });
}

fn isOperator(c: char) -> bool {
    match c {
        '+' => true,
        '-' => true,
        '/' => true,
        '*' => true,
        _ => false
    }
}

fn main() {
    loop {
        println!("Enter an expression: ");
        let mut expr: String = String::new();
        let mut tokens: Vec<Token> = vec![];
        match io::stdin().read_line(&mut expr) {
            Ok(_) => {
                if expr.trim().to_lowercase() == "quit" {
                    break;
                }
                let mut it = 0;
                let tokens: &mut Vec<Token> = tokens.borrow_mut();
                loop {
                    if let Some(c) = expr.chars().nth(it) {
                        if c.is_whitespace() {
                            // do nothing
                        } else if c == '(' {
                        } else if c == ')' {
                        } else if c.is_numeric() {
                            let (advance, tkn) = lexNumber(&expr[it..]);
                            it += advance;
                            tokens.push(tkn);
                        } else if isOperator(c) {
                            tokens.push(Token {
                                string:  &expr[it..it + 1]
                            });
                        }
                        it += 1;
                    } else {
                        for t in tokens.into_iter() {
                            println!("{}", t.string);
                        }
                        break;
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
