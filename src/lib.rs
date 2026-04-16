pub mod parser;
pub mod vm;

#[derive(Debug, Clone, Copy)]
pub enum Insn {
    IncrPrt,
    DecrPrt,
    IncrVal,
    DecrVal,
    Print,
    Scan,
    JumpFwd(usize),
    JumpBwd(usize),
}

pub use parser::parse;
pub use vm::Machine;

pub fn execute(src: &str) -> Result<(), String> {
    let insns = parse(src)?;
    let mut machine = Machine::new(insns);
    machine.run();
    Ok(())
}

/// helper for debugging
pub fn translate_bf_into_foxcall(src: &str) -> String {
    let mut result = String::new();
    for c in src.chars() {
        match c {
            '>' => result.push_str("ルー"),
            '<' => result.push_str("ルルー"),
            '+' => result.push_str("ルルルー"),
            '-' => result.push_str("ルルルルー"),
            '.' => result.push_str("ルルルルルー"),
            ',' => result.push_str("ビー"),
            '[' => result.push_str("ルビー"),
            ']' => result.push_str("カイギ"),
            _ => {}
        }
    }
    result
}