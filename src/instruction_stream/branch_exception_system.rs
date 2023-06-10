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

use crate::instruction_stream::branch_exception_system::unconditional_branch_immediate::{UnconditionalBranchImmediate, UnconditionalBranchImmediateWithAddress};
use crate::instruction_stream::branch_exception_system::barriers::Barriers;
use crate::instruction_stream::branch_exception_system::conditional_branch_imm::{ConditionalBranchImmediate, ConditionalBranchImmediateWithAddress};

pub mod conditional_branch_imm;
pub mod exception_generation;
pub mod system_instr_w_register_arg;
pub mod barriers;
pub mod pstate;
pub mod system_instructions;
pub mod system_register_move;
pub mod unconditional_branch_register;
pub mod unconditional_branch_immediate;

pub trait BranchExceptionSystem<T>: ConditionalBranchImmediate<T> + UnconditionalBranchImmediate<T> + Barriers<T> {}

pub trait BranchExceptionSystemWithAddress<T>: ConditionalBranchImmediateWithAddress<T> + UnconditionalBranchImmediateWithAddress<T> + Barriers<T> {}