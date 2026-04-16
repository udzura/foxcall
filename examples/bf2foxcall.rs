use std::env;

fn main() {
    let bf_src = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: bf2foxcall <brainfuck_source>");
        std::process::exit(1);
    });
    println!("{}", foxcall::translate_bf_into_foxcall(&bf_src));
}
