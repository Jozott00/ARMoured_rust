use std::fmt::{Display, Formatter};
use bit_seq::bseq_8;
use crate::types::type_creation_macro::make_enum;
use crate::types::{UImm1, UImm2};

pub trait ArrSpec: Copy + Into<u8>
{
    fn q(&self) -> UImm1 {
        let v = (*self).into();
        bseq_8!(v:1)
    }

    fn size(&self) -> UImm2 {
        (*self).into() >> 1
    }
}

// Arrangement specifier
make_enum!(ArrSpecX, [
    (T8B, 0b000),
    (T16B, 0b001),
    (T4H, 0b010),
    (T8H, 0b011),
    (T2S, 0b100),
    (T4S, 0b101),
    (T2D, 0b111)
]);

// Arrangement specifier for st1
make_enum!(ArrSpec1, [
    (T8B, 0b000),
    (T16B, 0b001),
    (T4H, 0b010),
    (T8H, 0b011),
    (T2S, 0b100),
    (T4S, 0b101),
    (T1D, 0b110),
    (T2D, 0b111)
]);

impl ArrSpec for ArrSpecX {}

impl ArrSpec for ArrSpec1 {}