use crate::types::encodable::Encodable;

/// Operation type for [DC instruction](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DC--Data-Cache-operation--an-alias-of-SYS-?lang=en).
pub enum IcOp {
    IALLUIS,
    IALLU,
    IVAU,
}

impl Encodable<(u8, u8, u8)> for IcOp {
    fn encode(&self) -> (u8, u8, u8) {
        match self {
            IcOp::IALLUIS => (0b000, 0b0001, 0b000),
            IcOp::IALLU => (0b000, 0b0101, 0b000),
            IcOp::IVAU => (0b011, 0b0101, 0b001),
        }
    }
}
