//! # PSTATE
//!
//! Implements the following instructions:
//! - [MSR (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MSR--immediate---Move-immediate-value-to-Special-Register-?lang=en)
//! - [CFINV](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CFINV--Invert-Carry-Flag-?lang=en)
//! - [XAFLAG](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/XAFLAG--Convert-floating-point-condition-flags-from-external-format-to-Arm-format-?lang=en)
//! - [AXFLAG](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AXFLAG--Convert-floating-point-condition-flags-from-Arm-to-external-format-?lang=en)

use bit_seq::{bseq_32, bseq_8};
use crate::instruction_emitter::Emitter;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::instruction::Instr;
use crate::types::{UImm16, UImm4};
use crate::types::encodable::Encodable;
use crate::types::pstate::PStateField;

impl<'mem, M: Memory, E: Emitter> InstrStream<'mem, M, E> {
    #[inline(always)]
    fn emit_pstate(&mut self, op1: u8, crm: UImm4, op2: u8, rt: u8) -> Instr {
        let i = bseq_32!(11010101 0:5 op1:3 0100 crm:4 op2:3 rt:5);
        self.emit(i)
    }

    /// [MSR (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MSR--immediate---Move-immediate-value-to-Special-Register-?lang=en)
    ///
    /// `MSR <pstatefield>, #<imm>`
    ///
    /// **Warning**: Some PStateFields not tested
    #[inline(always)]
    pub fn msr_imm(&mut self, pstatefield: PStateField, imm: UImm4) -> Instr {
        debug_assert!(pstatefield != PStateField::ALLINT || imm == 0 || imm == 1, "imm must be either 0 or 1 if pstatefield is ALLINT");
        let (op1, op2) = pstatefield.encode();
        self.emit_pstate(op1, imm, op2, 0b11111)
    }


    /// [CFINV](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CFINV--Invert-Carry-Flag-?lang=en)
    ///
    /// `CFINV`
    ///
    /// *Info*: FEAT_FlagM required
    #[inline(always)]
    pub fn cfinv(&mut self) -> Instr {
        self.emit_pstate(0, 0, 0, 0b11111)
    }

    /// [XAFLAG](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/XAFLAG--Convert-floating-point-condition-flags-from-external-format-to-Arm-format-?lang=en)
    ///
    /// `XAFLAG`
    ///
    /// *Info*: FEAT_FlagM2 required
    #[inline(always)]
    pub fn xaflag(&mut self) -> Instr {
        self.emit_pstate(0, 0, 1, 0b11111)
    }

    /// [AXFLAG](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AXFLAG--Convert-floating-point-condition-flags-from-Arm-to-external-format-?lang=en)
    ///
    /// `AXFLAG`
    ///
    /// *Info*: FEAT_FlagM2 required
    #[inline(always)]
    pub fn axflag(&mut self) -> Instr {
        self.emit_pstate(0, 0, 0b010, 0b11111)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mc_memory::MockMemory;
    use crate::instruction_emitter::MockEmitter;
    use crate::{assert_panic, stream_mock};
    use crate::types::InstructionPointer;
    use crate::types::pstate::PStateField::*;

    #[test]
    fn test_msr_imm() {
        stream_mock!(stream, {
            let instr = stream.msr_imm(UAO, 0x2);
            assert_eq!(instr.to_string(), "msr uao, #0x2");

            assert_panic!("Should panic: allint only 1 or 0 allowed"; stream.msr_imm(ALLINT, 0x2));
        })
    }

    #[test]
    fn test_cfinv() {
        stream_mock!(stream, {
            let instr = stream.cfinv();
            assert_eq!(instr.to_string(), "cfinv");
        })
    }

    #[test]
    fn test_xaflag() {
        stream_mock!(stream, {
            let instr = stream.xaflag();
            assert_eq!(instr.to_string(), "xaflag");
        })
    }

    #[test]
    fn test_axflag() {
        stream_mock!(stream, {
            let instr = stream.axflag();
            assert_eq!(instr.to_string(), "axflag");
        })
    }
}