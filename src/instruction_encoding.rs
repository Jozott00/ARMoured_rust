pub mod branch_exception_system;
pub mod data_proc_imm;
pub mod loads_and_stores;


use crate::instruction_encoding::branch_exception_system::{BranchExceptionSystem, BranchExceptionSystemWithAddress};
use crate::instruction_encoding::data_proc_imm::{DataProcessingImmediate, DataProcessingImmediateWithAddress};
use crate::instruction_encoding::loads_and_stores::{LoadsAndStores, LoadsAndStoresWithAddress};
use crate::types::{Instruction, Offset32};

pub trait InstructionProcessor<T>: Sized {
    /// Processes the encoded instruction
    ///
    /// The implementation and meaning of this function can vary
    /// depending on the purpose of the.
    fn emit(&mut self, instr: Instruction) -> T;
}

pub trait AddressableInstructionProcessor<T>: InstructionProcessor<T> {
    /// Calculates the offset between the current instruction pointer
    /// and the provided `addr`.
    ///
    /// Used by pc relative operations such as `b_to_addr(<addr>)`.
    fn intr_ptr_offset_to(&self, addr: usize) -> Offset32;
}

/// Bundles all instructions of the Arm64 instruction set
/// but does not contain pc relative instruction.
pub trait InstructionSet<T>: DataProcessingImmediate<T>
+ BranchExceptionSystem<T>
+ LoadsAndStores<T>
{}

/// Bundles all instruction of Arm64 instruction set
/// and does provide pc relative functionality for some instructions.
pub trait InstructionSetWithAddress<T>: InstructionSet<T>
+ DataProcessingImmediateWithAddress<T>
+ BranchExceptionSystemWithAddress<T>
+ LoadsAndStoresWithAddress<T>
{}
