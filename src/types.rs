use crate::types::HW::{LSL0, LSL16, LSL32, LSL48};

pub mod shifts;
pub mod bitmask_immediate;
pub mod instruction;
pub mod prefetch_memory;
pub mod encodable;
pub mod condition;
pub mod mem_barrier_option;
pub mod pstate;
pub mod sys_ops;
pub mod extends;

pub type Instruction = u32;
pub type InstructionPointer = *mut Instruction;

pub type Register = u8;

pub type Imm64 = u64;
pub type Imm32 = u32;
pub type Imm19 = i32;
pub type Imm18 = i32;
pub type Imm16 = u16;
pub type Imm13 = i16;
pub type Imm12 = u16;
pub type Imm11 = i16;
pub type Imm10 = i16;
pub type Imm9 = i16;
pub type Imm6 = u8;
pub type Imm5 = u8;

pub type UImm64 = u64;
pub type UImm32 = u32;
pub type UImm16 = u16;
pub type UImm12 = u16;
pub type UImm10 = u16;
pub type UImm6 = u8;
pub type UImm5 = u8;
pub type UImm4 = u8;
pub type UImm3 = u8;
pub type UImm2 = u8;
pub type UImm1 = u8;

pub type Offset32 = i32;
pub type Offset64 = i64;

// halfworld of instruction like movz, movn, ...
#[derive(PartialEq)]
pub enum HW {
    LSL0,
    LSL16,
    LSL32,
    LSL48,
}

impl From<u8> for HW {
    fn from(value: u8) -> HW {
        match value {
            0 => LSL0,
            1 => LSL16,
            2 => LSL32,
            3 => LSL48,
            _ => panic!("Invalid HW LSL {value}!") // TODO: Better error handling
        }
    }
}

impl From<HW> for u8 {
    fn from(value: HW) -> Self {
        match value {
            LSL0 => 0,
            LSL16 => 1,
            LSL32 => 2,
            LSL48 => 3
        }
    }
}
