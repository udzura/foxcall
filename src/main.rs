use std::env;
use std::io::{IsTerminal, Read};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: foxcall <execute|translate> <source>");
        return Err("not enough arguments".into());
    }
    match args[1].as_str() {
        "execute" => {
            let mut stdin_buf = Vec::new();
            if !std::io::stdin().is_terminal() {
                std::io::stdin()
                    .read_to_end(&mut stdin_buf)
                    .map_err(|e| e.to_string())?;
            }
            let output = foxcall::execute(&args[2], &stdin_buf)?;
            print!("{}", output);
        }
        "translate" => {
            print!("{}", foxcall::translate_bf_into_foxcall(&args[2]));
        }
        cmd => {
            return Err(format!("Unknown command: {}", cmd));
        }
    }
    Ok(())
}
