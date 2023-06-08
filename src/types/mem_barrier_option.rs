use crate::types::encodable::Encodable;
use MemBarrierOpt::*;


/// # Memory Barrier Options
///
/// More infos in [the arm64 docs](https://developer.arm.com/documentation/den0024/a/Memory-Ordering/Barriers) under `Data Memory Barrier` (DMB)
/// and the [DSB instruction docs](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DSB--Data-Synchronization-Barrier-?lang=en#iclass_dsb_memory)
pub enum MemBarrierOpt {
    SY,
    ST,
    LD,
    ISH,
    ISHST,
    ISHLD,
    NSH,
    NSHST,
    NSHLD,
    OSH,
    OSHST,
    OSHLD,
}

/// # Memory nXS Barrier Options
///
/// More infos in [DSB nXS instruction docs](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DSB--Data-Synchronization-Barrier-?lang=en)
pub enum MemNXSBarrierOpt {
    SY,
    ISH,
    NSH,
    OSH,
}


impl Encodable<u8> for MemBarrierOpt {
    fn encode(&self) -> u8 {
        match self {
            SY => 0b1111,
            ST => 0b1110,
            LD => 0b1101,
            ISH => 0b1011,
            ISHST => 0b1010,
            ISHLD => 0b1001,
            NSH => 0b0111,
            NSHST => 0b0110,
            NSHLD => 0b0101,
            OSH => 0b0011,
            OSHST => 0b0010,
            OSHLD => 0b0001,
        }
    }
}


impl Encodable<u8> for MemNXSBarrierOpt {
    fn encode(&self) -> u8 {
        match self {
            MemNXSBarrierOpt::SY => 0b11,
            MemNXSBarrierOpt::ISH => 0b10,
            MemNXSBarrierOpt::NSH => 0b01,
            MemNXSBarrierOpt::OSH => 0b00,
        }
    }
}