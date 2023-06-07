use bad64::decode;
use crate::types::{Instruction, InstructionPointer};

#[derive(Debug)]
pub struct Instr {
    enc: Instruction,
    ptr: InstructionPointer,
}

impl Instr {
    pub fn new(enc: Instruction, ptr: InstructionPointer) -> Self {
        Instr { enc, ptr }
    }

    pub fn encoding(&self) -> Instruction {
        self.enc
    }

    pub fn to_string(&self) -> String {
        let Ok(decoded) = decode(self.enc, self.ptr as u64) else {
            return format!("<unknown instruction {:#x} >", self.enc.to_le());
        };

        format!("{decoded}")
    }
}