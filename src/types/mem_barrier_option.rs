use MemBarrierOpt::*;

use crate::types::encodable::Encodable;

/// # Memory Barrier Options
///
/// More infos in [the arm64 docs](https://developer.arm.com/documentation/den0024/a/Memory-Ordering/Barriers) under `Data Memory Barrier` (DMB)
/// and the [DSB instruction docs](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DSB--Data-Synchronization-Barrier-?lang=en#iclass_dsb_memory)
#[derive(Debug)]
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
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use crate::instruction_emitter::MockEmitter;
    use crate::instruction_encoding::branch_exception_system::barriers::Barriers;
    use crate::instruction_stream::InstrStream;
    use crate::mc_memory::MockMemory;
    use crate::stream_mock;
    use crate::types::InstructionPointer;

    use super::*;

    #[test]
    fn test_mem_barrier_opts() {
        stream_mock!(stream, {
            let instr = stream.dsb_mem_barrier_option(SY);
            assert_eq!(instr.to_string(), "dsb sy");

            let instr = stream.dsb_mem_barrier_option(ST);
            assert_eq!(instr.to_string(), "dsb st");

            let instr = stream.dsb_mem_barrier_option(LD);
            assert_eq!(instr.to_string(), "dsb ld");

            let instr = stream.dsb_mem_barrier_option(ISH);
            assert_eq!(instr.to_string(), "dsb ish");

            let instr = stream.dsb_mem_barrier_option(ISHST);
            assert_eq!(instr.to_string(), "dsb ishst");

            let instr = stream.dsb_mem_barrier_option(ISHLD);
            assert_eq!(instr.to_string(), "dsb ishld");

            let instr = stream.dsb_mem_barrier_option(NSH);
            assert_eq!(instr.to_string(), "dsb nsh");

            let instr = stream.dsb_mem_barrier_option(NSHST);
            assert_eq!(instr.to_string(), "dsb nshst");

            let instr = stream.dsb_mem_barrier_option(NSHLD);
            assert_eq!(instr.to_string(), "dsb nshld");

            let instr = stream.dsb_mem_barrier_option(OSH);
            assert_eq!(instr.to_string(), "dsb osh");

            let instr = stream.dsb_mem_barrier_option(OSHST);
            assert_eq!(instr.to_string(), "dsb oshst");

            let instr = stream.dsb_mem_barrier_option(OSHLD);
            assert_eq!(instr.to_string(), "dsb oshld");
        })
    }

    #[test]
    fn test_mem_nxs_barrier_opts() {
        stream_mock!(stream, {
            let instr = stream.dsb_mem_nxs_barrier_option(MemNXSBarrierOpt::SY);
            assert_eq!(instr.to_string(), "dsb synXS");

            let instr = stream.dsb_mem_nxs_barrier_option(MemNXSBarrierOpt::ISH);
            assert_eq!(instr.to_string(), "dsb ishnXS");

            let instr = stream.dsb_mem_nxs_barrier_option(MemNXSBarrierOpt::NSH);
            assert_eq!(instr.to_string(), "dsb nshnXS");

            let instr = stream.dsb_mem_nxs_barrier_option(MemNXSBarrierOpt::OSH);
            assert_eq!(instr.to_string(), "dsb oshnXS");
        })
    }
}
