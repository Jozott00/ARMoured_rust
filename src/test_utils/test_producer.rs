use bad64::decode;

use crate::instruction_encoding::branch_exception_system::barriers::Barriers;
use crate::instruction_encoding::branch_exception_system::compare_and_branch_imm::CompareAndBranchImm;
use crate::instruction_encoding::branch_exception_system::conditional_branch_imm::ConditionalBranchImmediate;
use crate::instruction_encoding::branch_exception_system::exception_generation::ExceptionGeneration;
use crate::instruction_encoding::branch_exception_system::pstate::PStateInstructions;
use crate::instruction_encoding::branch_exception_system::system_instr_w_register_arg::SystemInstructionsWithRegArg;
use crate::instruction_encoding::branch_exception_system::system_instructions::SystemInstructions;
use crate::instruction_encoding::branch_exception_system::system_register_move::SystemRegisterMove;
use crate::instruction_encoding::branch_exception_system::test_and_branch_imm::TestAndBranchImmediate;
use crate::instruction_encoding::branch_exception_system::unconditional_branch_immediate::UnconditionalBranchImmediate;
use crate::instruction_encoding::branch_exception_system::unconditional_branch_register::UnconditionalBranchRegister;
use crate::instruction_encoding::branch_exception_system::BranchExceptionSystem;
use crate::instruction_encoding::common_aliases::CommonAliases;
use crate::instruction_encoding::data_proc_imm::add_substract_imm::AddSubtractImmediate;
use crate::instruction_encoding::data_proc_imm::bitfield::BitfieldInstructions;
use crate::instruction_encoding::data_proc_imm::extract::ExtractInstructions;
use crate::instruction_encoding::data_proc_imm::logical_imm::LogicalImmediate;
use crate::instruction_encoding::data_proc_imm::mov_wide_imm::MovWideImmediate;
use crate::instruction_encoding::data_proc_imm::pc_rel_addr::PcRelAddressing;
use crate::instruction_encoding::data_proc_imm::DataProcessingImmediate;
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
use crate::instruction_encoding::loads_and_stores::load_register_literal::LoadRegisterLiteral;
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
use crate::instruction_encoding::{InstructionProcessor, InstructionSet};
use crate::types::Instruction;

type InstrRes = String;

pub struct TestProducer {}

impl TestProducer {
    pub fn new() -> Self {
        TestProducer {}
    }
}

impl InstructionProcessor<InstrRes> for TestProducer {
    fn process(&mut self, instr: Instruction) -> String {
        let Ok(decoded) = decode(instr, 0u64) else {
            let encoding = instr.to_le_bytes();
            let enc_str = encoding.map(|e| format!("{e:02x}")).join(" ");
            return format!("<unknown instruction: {enc_str}>");
        };

        format!("{decoded}")
    }
}

impl DataProcessingImmediate<InstrRes> for TestProducer {}

impl PcRelAddressing<InstrRes> for TestProducer {}

impl AddSubtractImmediate<InstrRes> for TestProducer {}

impl LogicalImmediate<InstrRes> for TestProducer {}

impl MovWideImmediate<InstrRes> for TestProducer {}

impl BitfieldInstructions<InstrRes> for TestProducer {}

impl ExtractInstructions<InstrRes> for TestProducer {}

impl CompareAndBranchImm<InstrRes> for TestProducer {}

impl TestAndBranchImmediate<InstrRes> for TestProducer {}

impl BranchExceptionSystem<InstrRes> for TestProducer {}

impl ConditionalBranchImmediate<InstrRes> for TestProducer {}

impl ExceptionGeneration<InstrRes> for TestProducer {}

impl SystemInstructionsWithRegArg<InstrRes> for TestProducer {}

impl Barriers<InstrRes> for TestProducer {}

impl PStateInstructions<InstrRes> for TestProducer {}

impl SystemInstructions<InstrRes> for TestProducer {}

impl SystemRegisterMove<InstrRes> for TestProducer {}

impl UnconditionalBranchRegister<InstrRes> for TestProducer {}

impl UnconditionalBranchImmediate<InstrRes> for TestProducer {}

impl LoadStoreMemoryTags<InstrRes> for TestProducer {}

impl LoadStoreExclusivePair<InstrRes> for TestProducer {}

impl LoadStoreExclusiveRegister<InstrRes> for TestProducer {}

impl LoadStoreOrdered<InstrRes> for TestProducer {}

impl CompareAndSwap<InstrRes> for TestProducer {}

impl LdaprStlrUnscaleImmediate<InstrRes> for TestProducer {}

impl MemoryCopyAndMemorySet<InstrRes> for TestProducer {}

impl LoadStoreNoAllocatePairOffset<InstrRes> for TestProducer {}

impl LoadStoreRegisterPairPostIndexed<InstrRes> for TestProducer {}

impl LoadStoreRegisterPairOffset<InstrRes> for TestProducer {}

impl LoadStoreRegisterPairPreIndexed<InstrRes> for TestProducer {}

impl LoadStoreRegisterUnscaledImmediate<InstrRes> for TestProducer {}

impl LoadStoreRegisterUnsignedImmediate<InstrRes> for TestProducer {}

impl LoadStoreRegisterUnprivileged<InstrRes> for TestProducer {}

impl LoadStoreRegisterRegisterOffset<InstrRes> for TestProducer {}

impl LoadStoreRegisterPac<InstrRes> for TestProducer {}

impl AdvancedSIMDLoadStoreMultipleStructures<InstrRes> for TestProducer {}

impl AdvancedSIMDLoadStoreSingleStructures<InstrRes> for TestProducer {}

impl AtomicMemoryOperatinos<InstrRes> for TestProducer {}

impl LoadsAndStores<InstrRes> for TestProducer {}

impl CompareAndSwapPair<InstrRes> for TestProducer {}

impl LoadStoreRegisterPrePostIndexed<InstrRes> for TestProducer {}

impl LoadRegisterLiteral<InstrRes> for TestProducer {}

impl DataProcessingOneSource<InstrRes> for TestProducer {}

impl LogicalShiftRegister<InstrRes> for TestProducer {}

impl AddSubtractShiftedRegister<InstrRes> for TestProducer {}

impl AddSubtractExtendedRegister<InstrRes> for TestProducer {}

impl AddSubtractWithCarry<InstrRes> for TestProducer {}

impl RotateRightIntoFlags<InstrRes> for TestProducer {}

impl EvaluateIntoFlags<InstrRes> for TestProducer {}

impl ConditionalCompareRegister<InstrRes> for TestProducer {}

impl ConditionalCompareImmediate<InstrRes> for TestProducer {}

impl ConditionalSelect<InstrRes> for TestProducer {}

impl DataProcessingThreeSource<InstrRes> for TestProducer {}

impl DataProcessingRegister<InstrRes> for TestProducer {}

impl DataProcessingTwoSource<InstrRes> for TestProducer {}

impl CommonAliases<InstrRes> for TestProducer {}

impl InstructionSet<InstrRes> for TestProducer {}
