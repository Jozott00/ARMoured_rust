use crate::instruction_stream::branch_exception_system::unconditional_branch_immediate::UnconditionalBranchImmediateGenerator;
use crate::types::{Instruction, Offset32};

pub trait InstructionProcessor<T>: Sized {
    fn emit(&mut self, instr: Instruction) -> T;
}

pub trait AddressableInstructionProcessor<T>: InstructionProcessor<T> {
    fn intr_ptr_offset_to(&self, addr: usize) -> Offset32;
}

pub trait Instructions<T>: UnconditionalBranchImmediateGenerator<T> {}
