//! # Data processing - immediate instructions
//!
//! It consists of
//! - Add/subtract (immediate)
//! - Bitfield
//! - Extract
//! - Logical (immediate)
//! - Move wide (immediate)
//! - PC rel. addressing

use crate::instruction_encoding::data_proc_imm::add_substract_imm::AddSubtractImmediate;
use crate::instruction_encoding::data_proc_imm::bitfield::BitfieldInstructions;
use crate::instruction_encoding::data_proc_imm::extract::ExtractInstructions;
use crate::instruction_encoding::data_proc_imm::logical_imm::LogicalImmediate;
use crate::instruction_encoding::data_proc_imm::pc_rel_addr::PcRelAddressingWithAddress;
use crate::instruction_encoding::data_proc_imm::mov_wide_imm::MovWideImmediate;
use crate::instruction_encoding::data_proc_imm::pc_rel_addr::PcRelAddressing;

pub mod add_substract_imm;
pub mod bitfield;
pub mod extract;
pub mod logical_imm;
pub mod mov_wide_imm;
pub mod pc_rel_addr;


pub trait DataProcessingImmediate<T>: PcRelAddressing<T>
+ AddSubtractImmediate<T>
+ LogicalImmediate<T>
+ MovWideImmediate<T>
+ BitfieldInstructions<T>
+ ExtractInstructions<T>
{}

pub trait DataProcessingImmediateWithAddress<T>: DataProcessingImmediate<T>
+ PcRelAddressingWithAddress<T>
{}