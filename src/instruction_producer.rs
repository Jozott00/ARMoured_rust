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
use crate::instruction_encoding::{InstructionProcessor, InstructionSet};
use crate::instruction_encoding::branch_exception_system::unconditional_branch_immediate::UnconditionalBranchImmediate;
use crate::types::{Instruction, InstructionPointer};
use crate::types::instruction::Instr;

pub struct InstrProducer {}

impl InstrProducer {
    pub fn new() -> Self {
        InstrProducer {}
    }
}


impl InstructionProcessor<Instr> for InstrProducer {
    fn process(&mut self, instr: Instruction) -> Instr {
        Instr::new(instr, 0 as InstructionPointer)
    }
}


impl DataProcessingImmediate<Instr> for InstrProducer {}

impl PcRelAddressing<Instr> for InstrProducer {}

impl AddSubtractImmediate<Instr> for InstrProducer {}

impl LogicalImmediate<Instr> for InstrProducer {}

impl MovWideImmediate<Instr> for InstrProducer {}

impl BitfieldInstructions<Instr> for InstrProducer {}

impl ExtractInstructions<Instr> for InstrProducer {}

impl BranchExceptionSystem<Instr> for InstrProducer {}

impl ConditionalBranchImmediate<Instr> for InstrProducer {}

impl ExceptionGeneration<Instr> for InstrProducer {}

impl SystemInstructionsWithRegArg<Instr> for InstrProducer {}

impl Barriers<Instr> for InstrProducer {}

impl PStateInstructions<Instr> for InstrProducer {}

impl SystemInstructions<Instr> for InstrProducer {}

impl SystemRegisterMove<Instr> for InstrProducer {}

impl UnconditionalBranchRegister<Instr> for InstrProducer {}

impl UnconditionalBranchImmediate<Instr> for InstrProducer {}

impl LoadsAndStores<Instr> for InstrProducer {}

impl CompareAndSwapPair<Instr> for InstrProducer {}

impl LoadStoreRegUImm<Instr> for InstrProducer {}

impl LoadRegisterLiteral<Instr> for InstrProducer {}

impl DataProcessingOneSource<Instr> for InstrProducer {}

impl DataProcessingRegister<Instr> for InstrProducer {}

impl DataProcessingTwoSource<Instr> for InstrProducer {}

impl InstructionSet<Instr> for InstrProducer {}



