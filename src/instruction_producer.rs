use crate::instruction_encoding::InstructionProcessor;
use crate::instruction_stream::branch_exception_system::unconditional_branch_immediate::UnconditionalBranchImmediateGenerator;
use crate::types::{Instruction, InstructionPointer};
use crate::types::instruction::Instr;

pub struct InstrProducer {}

impl InstrProducer {
    pub fn new() -> Self {
        InstrProducer {}
    }
}


impl InstructionProcessor<Instr> for InstrProducer {
    fn emit(&mut self, instr: Instruction) -> Instr {
        Instr::new(instr, 0 as InstructionPointer)
    }
}

impl UnconditionalBranchImmediateGenerator<Instr> for InstrProducer {}
