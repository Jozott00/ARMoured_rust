use num::{FromPrimitive, Unsigned};
use crate::types::HW::{LSL0, LSL16, LSL32, LSL48};

pub type Instruction = u32;
pub type InstructionPointer = *mut Instruction;

pub type Register = u8;
pub type Imm16 = u16;

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
            16 => LSL16,
            32 => LSL32,
            48 => LSL48,
            _ => panic!("Invalid HW LSL {value}!") // TODO: Better error handling
        }
    }
}

impl From<HW> for u8 {
    fn from(value: HW) -> Self {
        match value {
            LSL0 => 0,
            LSL16 => 16,
            LSL32 => 32,
            LSL48 => 48
        }
    }
}
