use crate::types::encodable::Encodable;
use AtOp::*;


/// Operation type for [AT instruction](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AT--Address-Translate--an-alias-of-SYS-?lang=en).
pub enum AtOp {
    S1E1R,
    S1E1W,
    S1E0R,
    S1E0W,
    S1E1RP,
    S1E1WP,
    S1E2R,
    S1E2W,
    S12E1R,
    S12E1W,
    S12E0R,
    S12E0W,
    S1E3R,
    S1E3W,
}


impl Encodable<(u8, u8, u8)> for AtOp {
    /// Encoding for AT Operation. Consult the [arm64 docs](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AT--Address-Translate--an-alias-of-SYS-?lang=en) for more.
    ///
    /// Returns `(op1, CRm<0>, op2)`
    fn encode(&self) -> (u8, u8, u8) {
        match self {
            S1E1R => (0, 0, 0),
            S1E1W => (0, 0, 1),
            S1E0R => (0, 0, 0b010),
            S1E0W => (0, 0, 0b011),
            S1E1RP => (0, 1, 0b000),
            S1E1WP => (0, 1, 0b001),
            S1E2R => (0b100, 0, 0b000),
            S1E2W => (0b100, 0, 0b001),
            S12E1R => (0b100, 0, 0b100),
            S12E1W => (0b100, 0, 0b101),
            S12E0R => (0b100, 0, 0b110),
            S12E0W => (0b100, 0, 0b111),
            S1E3R => (0b110, 0, 0b000),
            S1E3W => (0b110, 0, 0b001),
        }
    }
}