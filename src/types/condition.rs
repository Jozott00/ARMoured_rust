use crate::types::encodable::Encodable;
use crate::types::UImm4;
use Condition::*;

pub enum Condition {
    EQ,
    NE,
    CS,
    HS,
    CC,
    LO,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
    NV,
}
impl Encodable<UImm4> for Condition {
    fn encode(&self) -> UImm4 {
        match self {
            EQ => 0b0000,
            NE => 0b0001,
            CS | HS => 0b0010,
            CC | LO => 0b0011,
            MI => 0b0100,
            PL => 0b0101,
            VS => 0b0110,
            VC => 0b0111,
            HI => 0b1000,
            LS => 0b1001,
            GE => 0b1010,
            LT => 0b1011,
            GT => 0b1100,
            LE => 0b1101,
            AL => 0b1110,
            NV => 0b1111,
        }
    }
}
