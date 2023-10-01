use bit_seq::bseq_8;

#[derive(Copy, Clone)]
pub struct PrfOp(pub PrfType, pub PrfTarget, pub PrfPolicy);

#[derive(Copy, Clone)]
pub enum PrfType {
    PLD,
    PLI,
    PST,
}

#[derive(Copy, Clone)]
pub enum PrfTarget {
    L1,
    L2,
    L3,
}

#[derive(Copy, Clone)]
pub enum PrfPolicy {
    KEEP,
    STRM,
}

impl PrfOp {
    pub fn encode(&self) -> u8 {
        let ty = self.0.encode();
        let target = self.1.encode();
        let policy = self.2.encode();
        bseq_8!(ty:2 target:2 policy:1)
    }
}

impl PrfType {
    pub fn encode(&self) -> u8 {
        match self {
            PrfType::PLD => 0b00,
            PrfType::PLI => 0b01,
            PrfType::PST => 0b10,
        }
    }
}

impl PrfTarget {
    pub fn encode(&self) -> u8 {
        match self {
            PrfTarget::L1 => 0b00,
            PrfTarget::L2 => 0b01,
            PrfTarget::L3 => 0b10,
        }
    }
}

impl PrfPolicy {
    pub fn encode(&self) -> u8 {
        match self {
            PrfPolicy::KEEP => 0b0,
            PrfPolicy::STRM => 0b1,
        }
    }
}
