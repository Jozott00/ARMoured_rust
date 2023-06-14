//! # Add/subtract (shifted register)
//!
//! Implements the following instructions:
//!  - [ADD - shifted register ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--shifted-register---Add--shifted-register--?lang=en)
//!  - [ADDS - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--shifted-register---Add--shifted-register---setting-flags-?lang=en)
//!  - [SUB - shifted register ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--shifted-register---Subtract--shifted-register--?lang=en)
//!  - [SUBS - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--shifted-register---Subtract--shifted-register---setting-flags-?lang=en)



use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Register, UImm5, UImm6};
use crate::types::shifts::{Shift3, Shift4};

#[inline(always)]
fn emit_add_sub<P: InstructionProcessor<T>, T>(proc: &mut P, sf: u8, op: u8, s: u8, shift: u8, rm: Register, imm6: u8, rn: Register, rd: Register) -> T {
    let i = bseq_32!(sf:1 op:1 s:1 01011 shift:2 0 rm:5 imm6:6 rn:5 rd:5);
    proc.process(i)
}

#[inline(always)]
fn emit_add_sub_shift<P: InstructionProcessor<T>, T>(proc: &mut P, sf: u8, op: u8, s: u8, shift: Shift3<u8>, rm: Register, rn: Register, rd: Register) -> T {
    let (shift, imm6): (u8, u8) = shift.into();
    emit_add_sub(proc, sf, op, s, shift, rm, imm6, rn, rd)
}

/// # Add/subtract (shifted register)
///
/// Implements the following instructions:
///  - [ADD - shifted register ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--shifted-register---Add--shifted-register--?lang=en)
///  - [ADDS - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--shifted-register---Add--shifted-register---setting-flags-?lang=en)
///  - [SUB - shifted register ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--shifted-register---Subtract--shifted-register--?lang=en)
///  - [SUBS - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--shifted-register---Subtract--shifted-register---setting-flags-?lang=en)
pub trait AddSubtractShiftedRegister<T>: InstructionProcessor<T> {
    /// [ADD - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--shifted-register---Add--shifted-register--?lang=en)
    ///
    /// Add (shifted register) adds a register value and an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// ADD <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
    /// ```
    #[inline(always)]
    fn add_32_reg_shift(&mut self, wd: Register, wn: Register, wm: Register, shift: Shift3<UImm5>) -> T {
        emit_add_sub_shift(self, 0, 0, 0, shift, wm, wn, wd)
    }


    /// [ADD - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--shifted-register---Add--shifted-register--?lang=en)
    ///
    /// Add (shifted register) adds a register value and an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// ADD <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
    /// ```
    #[inline(always)]
    fn add_64_reg_shift(&mut self, xd: Register, xn: Register, xm: Register, shift: Shift3<UImm6>) -> T {
        emit_add_sub_shift(self, 1, 0, 0, shift, xm, xn, xd)
    }


    /// [ADDS - shifted register -  Add - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--shifted-register---Add--shifted-register---setting-flags-?lang=en)
    ///
    /// Add (shifted register), setting flags, adds a register value and an optionally-shifted register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the alias CMN (shifted register).
    ///
    /// ```asm
    /// ADDS <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
    /// ```
    #[inline(always)]
    fn adds_32_reg_shift(&mut self, wd: Register, wn: Register, wm: Register, shift: Shift3<UImm5>) -> T {
        emit_add_sub_shift(self, 0, 0, 1, shift, wm, wn, wd)
    }


    /// [ADDS - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--shifted-register---Add--shifted-register---setting-flags-?lang=en)
    ///
    /// Add (shifted register), setting flags, adds a register value and an optionally-shifted register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the alias CMN (shifted register).
    ///
    /// ```asm
    /// ADDS <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
    /// ```
    #[inline(always)]
    fn adds_64_reg_shift(&mut self, xd: Register, xn: Register, xm: Register, shift: Shift3<UImm6>) -> T {
        emit_add_sub_shift(self, 1, 0, 1, shift, xm, xn, xd)
    }


    /// [SUB - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--shifted-register---Subtract--shifted-register--?lang=en)
    ///
    /// Subtract (shifted register) subtracts an optionally-shifted register value from a register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias NEG (shifted register).
    ///
    /// ```asm
    /// SUB <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
    /// ```
    #[inline(always)]
    fn sub_32_reg_shift(&mut self, wd: Register, wn: Register, wm: Register, shift: Shift3<UImm5>) -> T {
        emit_add_sub_shift(self, 0, 1, 0, shift, wm, wn, wd)
    }


    /// [SUB - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--shifted-register---Subtract--shifted-register--?lang=en)
    ///
    /// Subtract (shifted register) subtracts an optionally-shifted register value from a register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias NEG (shifted register).
    ///
    /// ```asm
    /// SUB <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
    /// ```
    #[inline(always)]
    fn sub_64_reg_shift(&mut self, xd: Register, xn: Register, xm: Register, shift: Shift3<UImm6>) -> T {
        emit_add_sub_shift(self, 1, 1, 0, shift, xm, xn, xd)
    }


    /// [SUBS - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--shifted-register---Subtract--shifted-register---setting-flags-?lang=en)
    ///
    /// Subtract (shifted register), setting flags, subtracts an optionally-shifted register value from a register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the aliases CMP (shifted register), and NEGS.
    ///
    /// ```asm
    /// SUBS <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
    /// ```
    #[inline(always)]
    fn subs_32_reg_shift(&mut self, wd: Register, wn: Register, wm: Register, shift: Shift3<UImm5>) -> T {
        emit_add_sub_shift(self, 0, 1, 1, shift, wm, wn, wd)
    }


    /// [SUBS - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--shifted-register---Subtract--shifted-register---setting-flags-?lang=en)
    ///
    /// Subtract (shifted register), setting flags, subtracts an optionally-shifted register value from a register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the aliases CMP (shifted register), and NEGS.
    ///
    /// ```asm
    /// SUBS <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
    /// ```
    #[inline(always)]
    fn subs_64_reg_shift(&mut self, xd: Register, xn: Register, xm: Register, shift: Shift3<UImm6>) -> T {
        emit_add_sub_shift(self, 1, 1, 1, shift, xm, xn, xd)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_add() {
        let mut prod = TestProducer::new();

        let instr = prod.add_32_reg_shift(3, 4, 5, Shift3::LSL(6));
        assert_eq!(instr, "add w3, w4, w5, lsl #0x6");

        let instr = prod.add_64_reg_shift(3, 4, 5, Shift3::LSR(6));
        assert_eq!(instr, "add x3, x4, x5, lsr #0x6");
    }

    #[test]
    fn test_adds() {
        let mut prod = TestProducer::new();

        let instr = prod.adds_32_reg_shift(3, 4, 5, Shift3::ASR(6));
        assert_eq!(instr, "adds w3, w4, w5, asr #0x6");

        let instr = prod.adds_64_reg_shift(3, 4, 5, Shift3::LSR(6));
        assert_eq!(instr, "adds x3, x4, x5, lsr #0x6");
    }

    #[test]
    fn test_sub() {
        let mut prod = TestProducer::new();

        let instr = prod.sub_32_reg_shift(3, 4, 5, Shift3::ASR(6));
        assert_eq!(instr, "sub w3, w4, w5, asr #0x6");

        let instr = prod.sub_64_reg_shift(3, 4, 5, Shift3::LSR(6));
        assert_eq!(instr, "sub x3, x4, x5, lsr #0x6");
    }

    #[test]
    fn test_subs() {
        let mut prod = TestProducer::new();

        let instr = prod.subs_32_reg_shift(3, 4, 5, Shift3::ASR(6));
        assert_eq!(instr, "subs w3, w4, w5, asr #0x6");

        let instr = prod.subs_64_reg_shift(3, 4, 5, Shift3::LSR(6));
        assert_eq!(instr, "subs x3, x4, x5, lsr #0x6");
    }
}