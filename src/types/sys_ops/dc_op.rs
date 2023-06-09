use crate::types::encodable::Encodable;

/// Operation type for [DC instruction](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DC--Data-Cache-operation--an-alias-of-SYS-?lang=en).
pub enum DcOp {
    IVAC,
    ISW,
    IGVAC,
    IGSW,
    IGDVAC,
    IGDSW,
    CSW,
    CGSW,
    CGDSW,
    CISW,
    CIGSW,
    CIGDSW,
    ZVA,
    GVA,
    GZVA,
    CVAC,
    CGVAC,
    CGDVAC,
    CVAU,
    CVAP,
    CGVAP,
    CGDVAP,
    CVADP,
    CGVADP,
    CGDVADP,
    CIVAC,
    CIGVAC,
    CIGDVAC,
}


impl Encodable<(u8, u8, u8)> for DcOp {
    fn encode(&self) -> (u8, u8, u8) {
        match self {
            DcOp::IVAC => (0b000, 0b0110, 0b001),
            DcOp::ISW => (0b000, 0b0110, 0b010),
            DcOp::IGVAC => (0b000, 0b0110, 0b011),
            DcOp::IGSW => (0b000, 0b0110, 0b100),
            DcOp::IGDVAC => (0b000, 0b0110, 0b101),
            DcOp::IGDSW => (0b000, 0b0110, 0b110),
            DcOp::CSW => (0b000, 0b1010, 0b010),
            DcOp::CGSW => (0b000, 0b1010, 0b100),
            DcOp::CGDSW => (0b000, 0b1010, 0b110),
            DcOp::CISW => (0b000, 0b1110, 0b010),
            DcOp::CIGSW => (0b000, 0b1110, 0b100),
            DcOp::CIGDSW => (0b000, 0b1110, 0b110),
            DcOp::ZVA => (0b011, 0b0100, 0b001),
            DcOp::GVA => (0b011, 0b0100, 0b011),
            DcOp::GZVA => (0b011, 0b0100, 0b100),
            DcOp::CVAC => (0b011, 0b1010, 0b001),
            DcOp::CGVAC => (0b011, 0b1010, 0b011),
            DcOp::CGDVAC => (0b011, 0b1010, 0b101),
            DcOp::CVAU => (0b011, 0b1011, 0b001),
            DcOp::CVAP => (0b011, 0b1100, 0b001),
            DcOp::CGVAP => (0b011, 0b1100, 0b011),
            DcOp::CGDVAP => (0b011, 0b1100, 0b101),
            DcOp::CVADP => (0b011, 0b1101, 0b001),
            DcOp::CGVADP => (0b011, 0b1101, 0b011),
            DcOp::CGDVADP => (0b011, 0b1101, 0b101),
            DcOp::CIVAC => (0b011, 0b1110, 0b001),
            DcOp::CIGVAC => (0b011, 0b1110, 0b011),
            DcOp::CIGDVAC => (0b011, 0b1110, 0b101),
        }
    }
}