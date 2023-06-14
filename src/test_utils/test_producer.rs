use bad64::decode;
use crate::instruction_encoding::data_proc_reg::add_sub_carry::AddSubtractWithCarry;
use crate::instruction_encoding::data_proc_reg::add_sub_ext_reg::AddSubtractExtendedRegister;
use crate::instruction_encoding::data_proc_reg::add_sub_shift_reg::AddSubtractShiftedRegister;
use crate::instruction_encoding::data_proc_reg::logical_shift_reg::LogicalShiftRegister;
use crate::instruction_encoding::data_proc_reg::data_proc_one_src::DataProcessingOneSource;
use crate::instruction_encoding::data_proc_reg::data_proc_two_src::DataProcessingTwoSource;
use crate::instruction_encoding::data_proc_reg::DataProcessingRegister;
use crate::instruction_encoding::branch_exception_system::barriers::Barriers;
use crate::instruction_encoding::branch_exception_system::BranchExceptionSystem;
use crate::instruction_encoding::branch_exception_system::conditional_branch_imm::ConditionalBranchImmediate;
use crate::instruction_encoding::branch_exception_system::exception_generation::ExceptionGeneration;
use crate::instruction_encoding::branch_exception_system::pstate::PStateInstructions;
use crate::instruction_encoding::branch_exception_system::system_instr_w_register_arg::SystemInstructionsWithRegArg;
use crate::instruction_encoding::branch_exception_system::system_instructions::SystemInstructions;
use crate::instruction_encoding::branch_exception_system::system_register_move::SystemRegisterMove;
use crate::instruction_encoding::branch_exception_system::unconditional_branch_immediate::UnconditionalBranchImmediate;
use crate::instruction_encoding::branch_exception_system::unconditional_branch_register::UnconditionalBranchRegister;
use crate::instruction_encoding::data_proc_imm::add_substract_imm::AddSubtractImmediate;
use crate::instruction_encoding::data_proc_imm::bitfield::BitfieldInstructions;
use crate::instruction_encoding::data_proc_imm::DataProcessingImmediate;
use crate::instruction_encoding::data_proc_imm::extract::ExtractInstructions;
use crate::instruction_encoding::data_proc_imm::logical_imm::LogicalImmediate;
use crate::instruction_encoding::data_proc_imm::mov_wide_imm::MovWideImmediate;
use crate::instruction_encoding::data_proc_imm::pc_rel_addr::PcRelAddressing;
use crate::instruction_encoding::loads_and_stores::compare_and_swap_pair::CompareAndSwapPair;
use crate::instruction_encoding::loads_and_stores::load_register_literal::LoadRegisterLiteral;
use crate::instruction_encoding::loads_and_stores::load_store_reg_uimm::LoadStoreRegUImm;
use crate::instruction_encoding::loads_and_stores::LoadsAndStores;
use crate::types::Instruction;
use crate::instruction_encoding::{InstructionProcessor, InstructionSet};

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

impl LoadsAndStores<InstrRes> for TestProducer {}

impl CompareAndSwapPair<InstrRes> for TestProducer {}

impl LoadStoreRegUImm<InstrRes> for TestProducer {}

impl LoadRegisterLiteral<InstrRes> for TestProducer {}

impl DataProcessingOneSource<InstrRes> for TestProducer {}

impl LogicalShiftRegister<InstrRes> for TestProducer {}

impl AddSubtractShiftedRegister<InstrRes> for TestProducer {}

impl AddSubtractExtendedRegister<InstrRes> for TestProducer {}

impl AddSubtractWithCarry<InstrRes> for TestProducer {}

impl DataProcessingRegister<InstrRes> for TestProducer {}

impl DataProcessingTwoSource<InstrRes> for TestProducer {}

impl InstructionSet<InstrRes> for TestProducer {}