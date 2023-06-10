//! # System instructions
//!
//! Implements the following instructions:
//! - [BR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BR--Branch-to-Register-?lang=en)
//! - [BRAA, BRAAZ, BRAB, BRABZ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BRAA--BRAAZ--BRAB--BRABZ--Branch-to-Register--with-pointer-authentication-?lang=en)
//! - [BLR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BLR--Branch-with-Link-to-Register-?lang=en)
//! - [BLRAA, BLRAAZ, BLRAB, BLRABZ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BLRAA--BLRAAZ--BLRAB--BLRABZ--Branch-with-Link-to-Register--with-pointer-authentication-?lang=en)
//! - [RET](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RET--Return-from-subroutine-?lang=en)
//! - [RETAA, RETAB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RETAA--RETAB--Return-from-subroutine--with-pointer-authentication-?lang=en)
//! - [ERET](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ERET--Exception-Return-?lang=en)
//! - [ERETAA, ERETAB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ERETAA--ERETAB--Exception-Return--with-pointer-authentication-?lang=en)
//! - [DRPS](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DRPS--Debug-restore-process-state-?lang=en)


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
    fn emit_uncond_br_reg(&mut self, opc: u8, op2: u8, op3: u8, rn: Register, op4: u8) -> Instr {
        let i = bseq_32!(1101011 opc:4 op2:5 op3:6 rn:5 op4:5);
        self.emit(i)
    }

    fn emit_br_x(&mut self, z: u8, op: u8, a: u8, m: u8, rn: Register, rm: Register) -> Instr {
        let opc = bseq_8!(z:1 0 op:2);
        let op3 = bseq_8!(a:1 m:1);
        self.emit_uncond_br_reg(opc, 0b11111, op3, rn, rm)
    }


    /// [BR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BR--Branch-to-Register-?lang=en)
    ///
    /// `BR <Xn>`
    #[inline(always)]
    pub fn br(&mut self, xn: Register) -> Instr {
        self.emit_br_x(0, 0, 0, 0, xn, 0)
    }

    /// [BRAAZ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BRAA--BRAAZ--BRAB--BRABZ--Branch-to-Register--with-pointer-authentication-?lang=en)
    ///
    /// `BRAAZ <Xn>`
    ///
    /// *Note*: FEAT_PAuth required
    #[inline(always)]
    pub fn braaz(&mut self, xn: Register) -> Instr {
        self.emit_br_x(0, 0, 1, 0, xn, 0b11111)
    }

    /// [BRAA](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BRAA--BRAAZ--BRAB--BRABZ--Branch-to-Register--with-pointer-authentication-?lang=en)
    ///
    /// `BRAA <Xn>,<Xm|SP>`
    ///
    /// *Note*: FEAT_PAuth required
    #[inline(always)]
    pub fn braa(&mut self, xn: Register, xm_sp: Register) -> Instr {
        self.emit_br_x(1, 0, 1, 0, xn, xm_sp)
    }

    /// [BRABZ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BRAA--BRAAZ--BRAB--BRABZ--Branch-to-Register--with-pointer-authentication-?lang=en)
    ///
    /// `BRABZ <Xn>`
    ///
    /// *Note*: FEAT_PAuth required
    #[inline(always)]
    pub fn brabz(&mut self, xn: Register) -> Instr {
        self.emit_br_x(0, 0, 1, 1, xn, 0b11111)
    }

    /// [BRAB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BRAA--BRAAZ--BRAB--BRABZ--Branch-to-Register--with-pointer-authentication-?lang=en)
    ///
    /// ```asm
    /// BRAB <Xn>,<Xm|SP>
    /// ```
    ///
    /// *Note*: FEAT_PAuth required
    #[inline(always)]
    pub fn brab(&mut self, xn: Register, xm_sp: Register) -> Instr {
        self.emit_br_x(1, 0, 1, 1, xn, xm_sp)
    }

    /// [BLR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BLR--Branch-with-Link-to-Register-?lang=en)
    ///
    /// ```asm
    /// BLR <Xn>
    /// ```
    #[inline(always)]
    pub fn blr(&mut self, xn: Register) -> Instr {
        self.emit_br_x(0, 1, 0, 0, xn, 0)
    }


    /// [BLRAAZ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BLRAA--BLRAAZ--BLRAB--BLRABZ--Branch-with-Link-to-Register--with-pointer-authentication-?lang=en)
    ///
    /// ```asm
    /// BLRAAZ <Xn>
    /// ```
    #[inline(always)]
    pub fn blraaz(&mut self, xn: Register) -> Instr {
        self.emit_br_x(0, 1, 1, 0, xn, 0b11111)
    }


    /// [BLRAA](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BLRAA--BLRAAZ--BLRAB--BLRABZ--Branch-with-Link-to-Register--with-pointer-authentication-?lang=en)
    ///
    /// ```asm
    /// BLRAA <Xn>, <Xm|SP>
    /// ```
    #[inline(always)]
    pub fn blraa(&mut self, xn: Register, xm_sp: Register) -> Instr {
        self.emit_br_x(1, 1, 1, 0, xn, xm_sp)
    }


    /// [BLRABZ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BLRAA--BLRAAZ--BLRAB--BLRABZ--Branch-with-Link-to-Register--with-pointer-authentication-?lang=en)
    ///
    /// ```asm
    /// BLRABZ <Xn>
    /// ```
    #[inline(always)]
    pub fn blrabz(&mut self, xn: Register) -> Instr {
        self.emit_br_x(0, 1, 1, 1, xn, 0b11111)
    }


    /// [BLRAB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BLRAA--BLRAAZ--BLRAB--BLRABZ--Branch-with-Link-to-Register--with-pointer-authentication-?lang=en)
    ///
    /// ```asm
    /// BLRAB <Xn>, <Xm|SP>
    /// ```
    #[inline(always)]
    pub fn blrab(&mut self, xn: Register, xm_sp: Register) -> Instr {
        self.emit_br_x(1, 1, 1, 1, xn, xm_sp)
    }


    /// [RET](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RET--Return-from-subroutine-?lang=en)
    ///
    /// ```asm
    /// RET <Xn>
    /// ```
    #[inline(always)]
    pub fn ret_reg(&mut self, xn: Register) -> Instr {
        self.emit_br_x(0, 0b10, 0, 0, xn, 0)
    }

    /// [RET](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RET--Return-from-subroutine-?lang=en)
    ///
    /// Same as `ret_reg` with register X30.
    ///
    /// ```asm
    /// RET
    /// ```
    #[inline(always)]
    pub fn ret(&mut self) -> Instr {
        self.ret_reg(30)
    }

    /// [RETAA, RETAB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RETAA--RETAB--Return-from-subroutine--with-pointer-authentication-?lang=en)
    ///
    /// ```asm
    /// RETAA
    /// ```
    #[inline(always)]
    pub fn retaa(&mut self) -> Instr {
        self.emit_br_x(0, 0b10, 1, 0, 0b11111, 0b11111)
    }

    /// [RETAA](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RETAA--RETAB--Return-from-subroutine--with-pointer-authentication-?lang=en)
    ///
    /// ```asm
    /// RETAB
    /// ```
    #[inline(always)]
    pub fn retab(&mut self) -> Instr {
        self.emit_br_x(0, 0b10, 1, 1, 0b11111, 0b11111)
    }

    /// [ERET](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ERET--Exception-Return-?lang=en)
    ///
    /// ```asm
    /// ERET
    /// ```
    #[inline(always)]
    pub fn eret(&mut self) -> Instr {
        self.emit_uncond_br_reg(0b0100, 0b11111, 0, 0b11111, 0)
    }

    /// [ERETAA](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ERETAA--ERETAB--Exception-Return--with-pointer-authentication-?lang=en)
    ///
    /// ```asm
    /// ERETAA
    /// ```
    ///
    /// *Info*: `FEAT_PAuth` required \
    /// **Warning**: not tested
    #[inline(always)]
    pub fn eretaa(&mut self) -> Instr {
        self.emit_uncond_br_reg(0b0100, 0b11111, 0b10, 0b11111, 0)
    }

    /// [ERETAB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ERETAA--ERETAB--Exception-Return--with-pointer-authentication-?lang=en)
    ///
    /// ```asm
    /// ERETAB
    /// ```
    ///
    /// *Info*: `FEAT_PAuth` required \
    /// **Warning**: not tested
    #[inline(always)]
    pub fn eretab(&mut self) -> Instr {
        self.emit_uncond_br_reg(0b0100, 0b11111, 0b11, 0b11111, 0)
    }


    /// [DRPS](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DRPS--Debug-restore-process-state-?lang=en)
    ///
    /// ```asm
    /// DRPS
    /// ```
    #[inline(always)]
    pub fn drps(&mut self) -> Instr {
        self.emit_uncond_br_reg(0b0101, 0b11111, 0, 0b11111, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mc_memory::MockMemory;
    use crate::instruction_emitter::MockEmitter;
    use crate::{stream_mock};
    use crate::types::InstructionPointer;

    #[test]
    fn test_br_x() {
        stream_mock!(stream, {
            let instr = stream.br(1);
            assert_eq!(instr.to_string(), "br x1");

            let instr = stream.braaz(1);
            assert_eq!(instr.to_string(), "braaz x1");

            let instr = stream.braa(1, 0);
            assert_eq!(instr.to_string(), "braa x1, x0");

            let instr = stream.brabz(1);
            assert_eq!(instr.to_string(), "brabz x1");

            let instr = stream.brab(1, 0);
            assert_eq!(instr.to_string(), "brab x1, x0");
        })
    }

    #[test]
    fn test_blr_x() {
        stream_mock!(stream, {
            let instr = stream.blr(1);
            assert_eq!(instr.to_string(), "blr x1");

            let instr = stream.blraaz(1);
            assert_eq!(instr.to_string(), "blraaz x1");

            let instr = stream.blraa(1, 0);
            assert_eq!(instr.to_string(), "blraa x1, x0");

            let instr = stream.blrabz(1);
            assert_eq!(instr.to_string(), "blrabz x1");

            let instr = stream.blrab(1, 0);
            assert_eq!(instr.to_string(), "blrab x1, x0");
        })
    }

    #[test]
    fn test_ret_x() {
        stream_mock!(stream, {
            let instr = stream.ret();
            assert_eq!(instr.to_string(), "ret");

            let instr = stream.ret_reg(1);
            assert_eq!(instr.to_string(), "ret x1");

            let instr = stream.retaa();
            assert_eq!(instr.to_string(), "retaa");

            let instr = stream.retab();
            assert_eq!(instr.to_string(), "retab");
        })
    }

    #[test]
    fn test_eret_x() {
        stream_mock!(stream, {
            let instr = stream.eret();
            assert_eq!(instr.to_string(), "eret");
        })
    }

    #[test]
    fn test_drps() {
        stream_mock!(stream, {
            let instr = stream.drps();
            assert_eq!(instr.to_string(), "drps");
        })
    }
}