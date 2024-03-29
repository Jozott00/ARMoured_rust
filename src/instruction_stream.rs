use std::{mem, slice};

use bad64::disasm;

use crate::instruction_emitter::{Emitter, InstrEmitter};
use crate::instruction_encoding::branch_exception_system::barriers::Barriers;
use crate::instruction_encoding::branch_exception_system::compare_and_branch_imm::{
    CompareAndBranchImm, CompareAndBranchImmWithAddress,
};
use crate::instruction_encoding::branch_exception_system::conditional_branch_imm::ConditionalBranchImmediate;
use crate::instruction_encoding::branch_exception_system::conditional_branch_imm::ConditionalBranchImmediateWithAddress;
use crate::instruction_encoding::branch_exception_system::exception_generation::ExceptionGeneration;
use crate::instruction_encoding::branch_exception_system::pstate::PStateInstructions;
use crate::instruction_encoding::branch_exception_system::system_instr_w_register_arg::SystemInstructionsWithRegArg;
use crate::instruction_encoding::branch_exception_system::system_instructions::SystemInstructions;
use crate::instruction_encoding::branch_exception_system::system_register_move::SystemRegisterMove;
use crate::instruction_encoding::branch_exception_system::test_and_branch_imm::{
    TestAndBranchImmediate, TestAndBranchImmediateWithAddress,
};
use crate::instruction_encoding::branch_exception_system::unconditional_branch_immediate::{
    UnconditionalBranchImmediate, UnconditionalBranchImmediateWithAddress,
};
use crate::instruction_encoding::branch_exception_system::unconditional_branch_register::UnconditionalBranchRegister;
use crate::instruction_encoding::branch_exception_system::BranchExceptionSystem;
use crate::instruction_encoding::branch_exception_system::BranchExceptionSystemWithAddress;
use crate::instruction_encoding::common_aliases::CommonAliases;
use crate::instruction_encoding::data_proc_imm::add_substract_imm::AddSubtractImmediate;
use crate::instruction_encoding::data_proc_imm::bitfield::BitfieldInstructions;
use crate::instruction_encoding::data_proc_imm::extract::ExtractInstructions;
use crate::instruction_encoding::data_proc_imm::logical_imm::LogicalImmediate;
use crate::instruction_encoding::data_proc_imm::mov_wide_imm::MovWideImmediate;
use crate::instruction_encoding::data_proc_imm::pc_rel_addr::{
    PcRelAddressing, PcRelAddressingWithAddress,
};
use crate::instruction_encoding::data_proc_imm::{
    DataProcessingImmediate, DataProcessingImmediateWithAddress,
};
use crate::instruction_encoding::data_proc_reg::add_sub_carry::AddSubtractWithCarry;
use crate::instruction_encoding::data_proc_reg::add_sub_ext_reg::AddSubtractExtendedRegister;
use crate::instruction_encoding::data_proc_reg::add_sub_shift_reg::AddSubtractShiftedRegister;
use crate::instruction_encoding::data_proc_reg::cond_compare_imm::ConditionalCompareImmediate;
use crate::instruction_encoding::data_proc_reg::cond_compare_reg::ConditionalCompareRegister;
use crate::instruction_encoding::data_proc_reg::conditional_select::ConditionalSelect;
use crate::instruction_encoding::data_proc_reg::data_proc_one_src::DataProcessingOneSource;
use crate::instruction_encoding::data_proc_reg::data_proc_three_src::DataProcessingThreeSource;
use crate::instruction_encoding::data_proc_reg::data_proc_two_src::DataProcessingTwoSource;
use crate::instruction_encoding::data_proc_reg::evaluate_into_flags::EvaluateIntoFlags;
use crate::instruction_encoding::data_proc_reg::logical_shift_reg::LogicalShiftRegister;
use crate::instruction_encoding::data_proc_reg::rotate_right_into_flags::RotateRightIntoFlags;
use crate::instruction_encoding::data_proc_reg::DataProcessingRegister;
use crate::instruction_encoding::loads_and_stores::advanced_simd_ldr_str_multi_structures::AdvancedSIMDLoadStoreMultipleStructures;
use crate::instruction_encoding::loads_and_stores::advanced_simd_ldr_str_single_structures::AdvancedSIMDLoadStoreSingleStructures;
use crate::instruction_encoding::loads_and_stores::atomic_memory_operations::AtomicMemoryOperatinos;
use crate::instruction_encoding::loads_and_stores::compare_and_swap::CompareAndSwap;
use crate::instruction_encoding::loads_and_stores::compare_and_swap_pair::CompareAndSwapPair;
use crate::instruction_encoding::loads_and_stores::ldapr_stlr_unscale_imm::LdaprStlrUnscaleImmediate;
use crate::instruction_encoding::loads_and_stores::load_register_literal::{
    LoadRegisterLiteral, LoadRegisterLiteralWithAddress,
};
use crate::instruction_encoding::loads_and_stores::load_store_exclusive_pair::LoadStoreExclusivePair;
use crate::instruction_encoding::loads_and_stores::load_store_exclusive_register::LoadStoreExclusiveRegister;
use crate::instruction_encoding::loads_and_stores::load_store_memory_tags::LoadStoreMemoryTags;
use crate::instruction_encoding::loads_and_stores::load_store_no_allocate_pair_offset::LoadStoreNoAllocatePairOffset;
use crate::instruction_encoding::loads_and_stores::load_store_ordered::LoadStoreOrdered;
use crate::instruction_encoding::loads_and_stores::load_store_reg_pair_offset::LoadStoreRegisterPairOffset;
use crate::instruction_encoding::loads_and_stores::load_store_reg_pair_post_indexed::LoadStoreRegisterPairPostIndexed;
use crate::instruction_encoding::loads_and_stores::load_store_reg_pair_pre_indexed::LoadStoreRegisterPairPreIndexed;
use crate::instruction_encoding::loads_and_stores::load_store_reg_pre_post_indexed::LoadStoreRegisterPrePostIndexed;
use crate::instruction_encoding::loads_and_stores::load_store_reg_unprivileged::LoadStoreRegisterUnprivileged;
use crate::instruction_encoding::loads_and_stores::load_store_reg_unscaled_imm::LoadStoreRegisterUnscaledImmediate;
use crate::instruction_encoding::loads_and_stores::load_store_register_pac::LoadStoreRegisterPac;
use crate::instruction_encoding::loads_and_stores::load_store_register_regoffset::LoadStoreRegisterRegisterOffset;
use crate::instruction_encoding::loads_and_stores::load_store_register_unsigned_imm::LoadStoreRegisterUnsignedImmediate;
use crate::instruction_encoding::loads_and_stores::memory_copy_and_memory_set::MemoryCopyAndMemorySet;
use crate::instruction_encoding::loads_and_stores::LoadsAndStores;
use crate::instruction_encoding::loads_and_stores::LoadsAndStoresWithAddress;
use crate::instruction_encoding::AddressableInstructionProcessor;
use crate::instruction_encoding::InstructionSetWithAddress;
use crate::instruction_encoding::{InstructionProcessor, InstructionSet};
use crate::mc_memory::{McMemory, Memory};
use crate::types::instruction::Instr;
use crate::types::Offset32;
use crate::types::{Instruction, InstructionPointer};

pub type PatchFn<M, E> = fn(&mut InstrStream<M, E>) -> ();

pub struct InstrStream<'mem, M: Memory, E: Emitter> {
    mem: &'mem mut M,
    emitter: E,
}

impl<'mem> InstrStream<'mem, McMemory, InstrEmitter> {
    pub fn new(mem: &'mem mut McMemory) -> Self {
        let emitter = InstrEmitter::from_mem(mem);
        Self { mem, emitter }
    }
}

impl<'mem, M: Memory, E: Emitter> InstructionProcessor<Instr> for InstrStream<'mem, M, E> {
    fn process(&mut self, instr: Instruction) -> Instr {
        debug_assert!(
            !self.mem.is_executable(),
            "Cannot emit instruction while memory is in execution mode"
        );

        let iptr = self.emitter.instr_ptr();
        self.emitter.emit(instr);

        Instr::new(instr, iptr)
    }
}

impl<'mem, M: Memory, E: Emitter> UnconditionalBranchImmediate<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> AddressableInstructionProcessor<Instr>
    for InstrStream<'mem, M, E>
{
    fn intr_ptr_offset_to(&self, addr: usize) -> Offset32 {
        let pc = self.emitter.instr_ptr() as usize;
        let offset_abs = pc
            .checked_sub(addr)
            .unwrap_or_else(|| addr.checked_sub(pc).unwrap());

        debug_assert!(
            offset_abs <= i32::MAX as usize,
            "Offset to address is to large (exceeds maximum of {:x})",
            i32::MAX
        );

        if addr >= pc {
            offset_abs as i32
        } else {
            -(offset_abs as i32)
        }
    }
}

/// Implementation for `InstrStream` struct.
/// A stream of instructions that uses memory layer 'M' and emitter 'E'.
///
/// 'mem: The memory size descriptor.
/// M: Represents memory.
/// E: Represents emitter.
impl<'mem, M: Memory, E: Emitter> InstrStream<'mem, M, E> {
    /// Modify the instruction located at the specified instruction pointer by applying a patch.
    /// This function saves the current instruction pointer, sets a new instruction pointer,
    /// applies the patch, and then restores the original instruction pointer.
    ///
    /// # Arguments
    ///
    /// `intr_ptr` - Provides the reference to an instruction pointer where the patch has to be applied.
    /// `patch` - The patch operation to<'mem, M: Memory, E: Emitter> InstrStream<'mem, M, E> {
    pub fn patch_at(&mut self, intr_ptr: InstructionPointer, patch: PatchFn<M, E>) {
        // save instruction pointer
        let iptr = self.emitter.instr_ptr();
        self.emitter.set_instr_ptr(intr_ptr);
        patch(self);
        // restore instruction pointer
        self.emitter.set_instr_ptr(iptr);
    }

    #[inline(always)]
    pub fn nullary_fn_ptr(&mut self) -> unsafe extern "C" fn() -> u64 {
        unsafe { mem::transmute(self.base_ptr() as usize) }
    }

    #[inline(always)]
    pub fn base_ptr(&self) -> InstructionPointer {
        self.emitter.base_ptr()
    }

    pub fn print_disasm(&self) {
        let decoded_iter = disasm(self.written_memory(), self.base_ptr() as u64);
        for instr in decoded_iter {
            if let Ok(instr) = instr {
                let encoding = instr.opcode().to_le_bytes();
                let enc_str = encoding.map(|e| format!("{e:02x}")).join(" ");
                println!("{:#x}: {enc_str}      {instr}", instr.address());
            } else {
                println!("! Incorrect instruction");
            };
        }
    }

    pub fn written_memory(&self) -> &[u8] {
        let len = (self.emitter.instr_ptr() as usize) - (self.mem.addr() as usize);
        let ptr = self.mem.addr() as *const u8;

        assert_eq!(len % 4, 0, "Len is not a multiple of 4");
        assert_eq!(
            ptr as usize % mem::align_of::<u32>(),
            0,
            "Memory not u32 aligned"
        );
        assert!(
            self.mem.len() >= len,
            "Requested length exceeds memory map!"
        );

        unsafe { slice::from_raw_parts(ptr, len) }
    }

    pub fn emit(&mut self, instr: Instruction) -> Instr {
        debug_assert!(
            !self.mem.is_executable(),
            "Cannot emit instruction while memory is in execution mode"
        );

        let iptr = self.emitter.instr_ptr();
        self.emitter.emit(instr);

        Instr::new(instr, iptr)
    }
}

impl<'mem, M: Memory, E: Emitter> UnconditionalBranchImmediateWithAddress<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> ConditionalBranchImmediate<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> ConditionalBranchImmediateWithAddress<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> SystemInstructionsWithRegArg<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> PStateInstructions<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> SystemInstructions<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> SystemRegisterMove<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> UnconditionalBranchRegister<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> CompareAndBranchImm<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> BranchExceptionSystem<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> ExceptionGeneration<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> CompareAndBranchImmWithAddress<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> TestAndBranchImmediate<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> TestAndBranchImmediateWithAddress<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> BranchExceptionSystemWithAddress<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> Barriers<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LogicalImmediate<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> AddSubtractImmediate<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> BitfieldInstructions<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> ExtractInstructions<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> DataProcessingImmediate<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> MovWideImmediate<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> PcRelAddressing<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> DataProcessingOneSource<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LogicalShiftRegister<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> AddSubtractShiftedRegister<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> AddSubtractExtendedRegister<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> AddSubtractWithCarry<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> RotateRightIntoFlags<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> EvaluateIntoFlags<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> ConditionalCompareRegister<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> ConditionalCompareImmediate<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> ConditionalSelect<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> DataProcessingThreeSource<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> DataProcessingRegister<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> DataProcessingTwoSource<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> CommonAliases<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> InstructionSet<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LoadStoreRegisterPrePostIndexed<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> LoadRegisterLiteral<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LoadStoreMemoryTags<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LoadStoreExclusivePair<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LoadStoreExclusiveRegister<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LoadStoreOrdered<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> CompareAndSwap<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LdaprStlrUnscaleImmediate<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> MemoryCopyAndMemorySet<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LoadStoreNoAllocatePairOffset<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LoadStoreRegisterPairPostIndexed<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> LoadStoreRegisterPairOffset<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LoadStoreRegisterPairPreIndexed<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> LoadStoreRegisterUnscaledImmediate<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> LoadStoreRegisterUnsignedImmediate<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> LoadStoreRegisterUnprivileged<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LoadStoreRegisterRegisterOffset<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> LoadStoreRegisterPac<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> AdvancedSIMDLoadStoreMultipleStructures<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> AdvancedSIMDLoadStoreSingleStructures<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> AtomicMemoryOperatinos<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LoadsAndStores<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> CompareAndSwapPair<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LoadsAndStoresWithAddress<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> LoadRegisterLiteralWithAddress<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> DataProcessingImmediateWithAddress<Instr>
    for InstrStream<'mem, M, E>
{
}

impl<'mem, M: Memory, E: Emitter> PcRelAddressingWithAddress<Instr> for InstrStream<'mem, M, E> {}

impl<'mem, M: Memory, E: Emitter> InstructionSetWithAddress<Instr> for InstrStream<'mem, M, E> {}

#[cfg(test)]
mod mocking_util {
    use crate::instruction_emitter::MockEmitter;
    use crate::instruction_stream::InstrStream;
    use crate::mc_memory::MockMemory;

    impl<'mem> InstrStream<'mem, MockMemory, MockEmitter> {
        pub fn new_mocked(mem: &'mem mut MockMemory, emitter: MockEmitter) -> Self {
            InstrStream { mem, emitter }
        }
    }
}
