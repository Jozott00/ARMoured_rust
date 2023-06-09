//! # [Branches, Exception Generating and System instructions](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Branches--Exception-Generating-and-System-instructions?lang=en#condbranch)
//!
//! Contains the following instruction types:
//! - [Conditional branch (immediate)](conditional_branch_imm)
//! - [Exception generation](exception_generation)
//! - [System instructions with register argument](system_instr_w_register_arg)
//! - [Barriers](barriers)

pub mod conditional_branch_imm;
pub mod exception_generation;
pub mod system_instr_w_register_arg;
pub mod barriers;
pub mod pstate;
pub mod system_instructions;
pub mod system_register_move;