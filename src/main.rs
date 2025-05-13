use redmoon::exec_script;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(a) = args.get(1) {
        if let Ok(f) = std::fs::read(a) {
            let buffer: String = String::from_utf8_lossy(&f).to_string();
            exec_script(buffer);
        } else {
            println!("File {a} does not exist");
        }
        return;
    }
}
