//! # System instructions
//!
//! Implements the following instructions:
//! - [SYS - System instruction](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SYS--System-instruction-?lang=en)
//! - [SYSL - System instruction with result](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SYSL--System-instruction-with-result-?lang=en)
//!
//! Provided aliases:
//! - [AT](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AT--Address-Translate--an-alias-of-SYS-?lang=en)
//! - [CFP](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CFP--Control-Flow-Prediction-Restriction-by-Context--an-alias-of-SYS-?lang=en)
//! - [CPP](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPP--Cache-Prefetch-Prediction-Restriction-by-Context--an-alias-of-SYS-?lang=en)
//! - [DC](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DC--Data-Cache-operation--an-alias-of-SYS-?lang=en)
//! - [DVP](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DVP--Data-Value-Prediction-Restriction-by-Context--an-alias-of-SYS-?lang=en)
//! - [IC](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/IC--Instruction-Cache-operation--an-alias-of-SYS-?lang=en)
//! - [TLBI](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/TLBI--TLB-Invalidate-operation--an-alias-of-SYS-?lang=en)

use bit_seq::{bseq_32, bseq_8};
use crate::instruction_emitter::Emitter;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::instruction::Instr;
use crate::types::{Register, UImm16, UImm3, UImm4};
use crate::types::encodable::Encodable;
use crate::types::sys_ops::at_op::AtOp;
use crate::types::sys_ops::dc_op::DcOp;
use crate::types::sys_ops::ic_op::IcOp;

impl<'mem, M: Memory, E: Emitter> InstrStream<'mem, M, E> {
    #[inline(always)]
    fn emit_system_instruction(&mut self, l: u8, op1: u8, crn: u8, crm: u8, op2: u8, rt: Register) -> Instr {
        let i = bseq_32!(1101010100 l:1 01 op1:3 crn:4 crm:4 op2:3 rt:5);
        self.emit(i)
    }

    // instructions

    /// [SYS - System instruction](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SYS--System-instruction-?lang=en)
    ///
    /// `SYS #<op1>,<Cn>,<Cm>,#<op2>{,<Xt>}`
    #[inline(always)]
    pub fn sys(&mut self, op1: UImm3, cn: UImm4, cm: UImm4, op2: UImm3, xt: Option<Register>) -> Instr {
        self.emit_system_instruction(0, op1, cn, cm, op2, xt.unwrap_or(0b11111))
    }

    /// [SYSL - System instruction with result](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SYSL--System-instruction-with-result-?lang=en)
    ///
    /// `SYSL <Xt>,#<op1>,<Cn>,<Cm>,#<op2>`
    #[inline(always)]
    pub fn sysl(&mut self, xt: Register, op1: UImm3, cn: UImm4, cm: UImm4, op2: UImm3) -> Instr {
        self.emit_system_instruction(1, op1, cn, cm, op2, xt)
    }

    // aliases


    /// [AT](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AT--Address-Translate--an-alias-of-SYS-?lang=en)
    ///
    ///
    /// `AT <at_op>,<Xt>`
    #[inline(always)]
    pub fn at(&mut self, at_op: AtOp, xt: Register) -> Instr {
        let (op1, crm0, op2) = at_op.encode();
        let crm = bseq_8!(100 crm0:1);
        self.sys(op1, 0b0111, crm, op2, Some(xt))
    }

    /// [CFP](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CFP--Control-Flow-Prediction-Restriction-by-Context--an-alias-of-SYS-?lang=en)
    ///
    /// `CFP RCTX,<Xt>`
    #[inline(always)]
    pub fn cfp_rctx(&mut self, xt: Register) -> Instr {
        self.sys(3, 7, 3, 4, Some(xt))
    }

    /// [CPP](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPP--Cache-Prefetch-Prediction-Restriction-by-Context--an-alias-of-SYS-?lang=en)
    ///
    /// `CPP RCTX,<Xt>`
    #[inline(always)]
    pub fn cpp_rctx(&mut self, xt: Register) -> Instr {
        self.sys(3, 7, 3, 7, Some(xt))
    }

    /// [DC](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DC--Data-Cache-operation--an-alias-of-SYS-?lang=en)
    ///
    /// `DC <dc_op>,<Xt>`
    #[inline(always)]
    pub fn dc(&mut self, dc_op: DcOp, xt: Register) -> Instr {
        let (op1, crm, op2) = dc_op.encode();
        self.sys(op1, 0b0111, crm, op2, Some(xt))
    }

    /// [DVP](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DVP--Data-Value-Prediction-Restriction-by-Context--an-alias-of-SYS-?lang=en)
    ///
    ///
    /// `DVP RCTX,<Xt>`
    #[inline(always)]
    pub fn dvp_rctx(&mut self, xt: Register) -> Instr {
        self.sys(3, 7, 3, 5, Some(xt))
    }

    /// [IC](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/IC--Instruction-Cache-operation--an-alias-of-SYS-?lang=en)
    ///
    /// `IC <ic_op>,<Xt>`
    #[inline(always)]
    pub fn ic(&mut self, ic_op: IcOp, xt: Option<Register>) -> Instr {
        let (op1, crm, op2) = ic_op.encode();
        self.sys(op1, 7, crm, op2, xt)
    }

    //TODO: [TLBI](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/TLBI--TLB-Invalidate-operation--an-alias-of-SYS-?lang=en)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mc_memory::MockMemory;
    use crate::instruction_emitter::MockEmitter;
    use crate::{stream_mock};
    use crate::types::InstructionPointer;

    #[test]
    fn test_sys() {
        stream_mock!(stream, {
            let instr = stream.sys(1, 2, 3, 4, Some(1));
            assert_eq!(instr.to_string(), "sys #0x1, c2, c3, #0x4, x1");

            let instr = stream.sys(1, 2, 3, 4, None);
            assert_eq!(instr.to_string(), "sys #0x1, c2, c3, #0x4");
        })
    }

    #[test]
    fn test_sysl() {
        stream_mock!(stream, {
            let instr = stream.sysl(1, 1, 2, 3, 4);
            assert_eq!(instr.to_string(), "sysl x1, #0x1, c2, c3, #0x4");
        })
    }

    #[test]
    fn test_sys_aliases() {
        stream_mock!(stream, {
            let instr = stream.at(AtOp::S1E0R, 1);
            assert_eq!(instr.to_string(), "at S1E0R, x1");

            let instr = stream.cfp_rctx(1);
            assert_eq!(instr.to_string(), "cfp rctx, x1");

            let instr = stream.cpp_rctx(1);
            assert_eq!(instr.to_string(), "cpp rctx, x1");

            let instr = stream.dc(DcOp::IVAC, 1);
            assert_eq!(instr.to_string(), "dc ivac, x1");

            let instr = stream.dvp_rctx(1);
            assert_eq!(instr.to_string(), "dvp rctx, x1");

            let instr = stream.ic(IcOp::IALLUIS, None);
            assert_eq!(instr.to_string(), "ic ialluis");
        })
    }
}