use redmoon::{interpreter::Interpreter, lexer::Lexer, parser::Parser, exec_repl, exec_script};
fn main() {
    let mut interp = Interpreter::new();
    let args: Vec<String> = std::env::args().collect();
    if let Some(a) = args.get(1) {
        if let Ok(f) = std::fs::read(a) {
            let buffer: String =  String::from_utf8_lossy(&f).to_string();
            exec_script(buffer);
        } else {
            println!("File {a} does not exist");
        }
        return;
    }
    loop {
        println!("Enter an expression: ");
        let mut expr: String = String::new();
        match std::io::stdin().read_line(&mut expr) {
            Ok(_) => {
                if expr.trim().to_lowercase() == "quit" {
                    break;
                }
                exec_repl(expr, &mut interp);
            },
            Err(_) => {
                println!("Error while reading input");
                break;
            }
        };
    }
}
