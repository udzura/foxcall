use std::{cell::Cell, io::Read};

use crate::Insn;

pub struct Machine {
    tape: [u8; 30000],
    data_ptr: usize,
    insns: Vec<Cell<Insn>>,
    insn_ptr: usize,
}

impl Machine {
    pub fn new(insns: Vec<Cell<Insn>>) -> Self {
        Self {
            tape: [0; 30000],
            data_ptr: 0,
            insns,
            insn_ptr: 0,
        }
    }

    pub fn step(&mut self) -> bool {
        if self.insn_ptr >= self.insns.len() {
            return false;
        }
        let insn = self.insns[self.insn_ptr].get();
        match insn {
            Insn::IncrPrt => self.data_ptr += 1,
            Insn::DecrPrt => self.data_ptr -= 1,
            Insn::IncrVal => self.tape[self.data_ptr] += 1,
            Insn::DecrVal => self.tape[self.data_ptr] -= 1,
            Insn::Print => print!("{}", self.tape[self.data_ptr] as char),
            Insn::Scan => {
                let mut buf = [0];
                std::io::stdin().read_exact(&mut buf).unwrap();
                self.tape[self.data_ptr] = buf[0];
            }
            Insn::JumpFwd(jump_to) => {
                if self.tape[self.data_ptr] == 0 {
                    self.insn_ptr = jump_to;
                    return true;
                }
            }
            Insn::JumpBwd(jump_to) => {
                if self.tape[self.data_ptr] != 0 {
                    self.insn_ptr = jump_to;
                    return true;
                }
            }
        }
        self.insn_ptr += 1;
        true
    }

    pub fn run(&mut self) {
        while self.step() {}
    }
}