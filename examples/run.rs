use std::env;

fn main() {
    let src = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: run <foxcall_source>");
        std::process::exit(1);
    });
    if let Err(e) = foxcall::execute(&src) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
