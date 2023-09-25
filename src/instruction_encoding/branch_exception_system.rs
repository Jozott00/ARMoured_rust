//! # [Branches, Exception Generating and System instructions](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Branches--Exception-Generating-and-System-instructions?lang=en#condbranch)
//!
//! Contains the following instruction types:
//! - [Conditional branch (immediate)](conditional_branch_imm)
//! - [Exception generation](exception_generation)
//! - [System instructions with register argument](system_instr_w_register_arg)
//! - [Barriers](barriers)
//! - [PSTATE](pstate)
//! - [System Instructions](system_instructions)
//! - [System register move](system_register_move)
//! - [Unconditional branch (register)](unconditional_branch_register)

use crate::instruction_encoding::branch_exception_system::barriers::Barriers;
use crate::instruction_encoding::branch_exception_system::compare_and_branch_imm::{
    CompareAndBranchImm, CompareAndBranchImmWithAddress,
};
use crate::instruction_encoding::branch_exception_system::conditional_branch_imm::{
    ConditionalBranchImmediate, ConditionalBranchImmediateWithAddress,
};
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

pub mod barriers;
pub mod compare_and_branch_imm;
pub mod conditional_branch_imm;
pub mod exception_generation;
pub mod pstate;
pub mod system_instr_w_register_arg;
pub mod system_instructions;
pub mod system_register_move;
pub mod test_and_branch_imm;
pub mod unconditional_branch_immediate;
pub mod unconditional_branch_register;

pub trait BranchExceptionSystem<T>:
    ConditionalBranchImmediate<T>
    + ExceptionGeneration<T>
    + SystemInstructionsWithRegArg<T>
    + Barriers<T>
    + PStateInstructions<T>
    + SystemInstructions<T>
    + SystemRegisterMove<T>
    + UnconditionalBranchRegister<T>
    + UnconditionalBranchImmediate<T>
    + CompareAndBranchImm<T>
    + TestAndBranchImmediate<T>
{
}

pub trait BranchExceptionSystemWithAddress<T>:
    BranchExceptionSystem<T>
    + ConditionalBranchImmediateWithAddress<T>
    + UnconditionalBranchImmediateWithAddress<T>
    + CompareAndBranchImmWithAddress<T>
    + TestAndBranchImmediateWithAddress<T>
{
}
