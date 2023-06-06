use crate::types::HW::{LSL0, LSL16, LSL32, LSL48};

pub mod shifts;
pub mod bitmask_immediate;

pub type Instruction = u32;
pub type InstructionPointer = *mut Instruction;

pub type Register = u8;
pub type Imm64 = u64;
pub type Imm32 = u32;
pub type Imm16 = u16;
pub type Imm13 = u16;
pub type Imm12 = u16;
pub type Imm6 = u8;
pub type Imm5 = u8;

// halfworld of instruction like movz, movn, ...
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
