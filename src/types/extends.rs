use std::fmt::{Display, Formatter};
use crate::types::type_creation_macro::make_enum;
use crate::types::UImm3;

#[derive(PartialEq)]
pub enum RegExtend {
    UXTB,
    UXTH,
    UXTW,
    UXTX,
    SXTB,
    SXTH,
    SXTW,
    SXTX,
}

impl From<UImm3> for RegExtend {
    fn from(value: UImm3) -> Self {
        match value {
            0b000 => RegExtend::UXTB,
            0b001 => RegExtend::UXTH,
            0b010 => RegExtend::UXTW,
            0b011 => RegExtend::UXTX,
            0b100 => RegExtend::SXTB,
            0b101 => RegExtend::SXTH,
            0b110 => RegExtend::SXTW,
            0b111 => RegExtend::SXTX,
            _ => panic!("wrong encoding for extend, was {value:#b}")
        }
    }
}

impl From<RegExtend> for u8 {
    fn from(value: RegExtend) -> Self {
        match value {
            RegExtend::UXTB => 0b000,
            RegExtend::UXTH => 0b001,
            RegExtend::UXTW => 0b010,
            RegExtend::UXTX => 0b011,
            RegExtend::SXTB => 0b100,
            RegExtend::SXTH => 0b101,
            RegExtend::SXTW => 0b110,
            RegExtend::SXTX => 0b111,
        }
    }
}


impl Display for RegExtend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RegExtend::UXTB => f.write_str("UXTB"),
            RegExtend::UXTH => f.write_str("UXTH"),
            RegExtend::UXTW => f.write_str("UXTW"),
            RegExtend::UXTX => f.write_str("UXTX"),
            RegExtend::SXTB => f.write_str("SXTB"),
            RegExtend::SXTH => f.write_str("SXTH"),
            RegExtend::SXTW => f.write_str("SXTW"),
            RegExtend::SXTX => f.write_str("SXTX"),
        }
    }
}

make_enum!(RegExtendLSL, [(UXTW, 0b010), (LSL, 0b011), (SXTW, 0b110), (SXTX, 0b111)]);
