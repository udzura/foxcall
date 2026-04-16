use std::env;
use std::io::Read;

fn main() {
    let src = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: run <foxcall_source>");
        std::process::exit(1);
    });
    let mut stdin_buf = Vec::new();
    std::io::stdin().read_to_end(&mut stdin_buf).unwrap();
    match foxcall::execute(&src, &stdin_buf) {
        Ok(output) => print!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
