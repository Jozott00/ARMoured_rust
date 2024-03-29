//! # Common Aliases
//!
//! This module implements common aliases such as MOV.
//!
//! Implements the following instructions:
//!  - [MOV (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOV--register---Move--register---an-alias-of-ORR--shifted-register--?lang=en)
//!  - [ADD - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--shifted-register---Add--shifted-register--?lang=en)
//!  - [ADDS - register - setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--shifted-register---Add--shifted-register---setting-flags-?lang=en)
//!  - [SUB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--shifted-register---Subtract--shifted-register--?lang=en)
//!  - [SUBS - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--shifted-register---Subtract--shifted-register---setting-flags-?lang=en)
//!  - [MUL - Multiply](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MUL--Multiply--an-alias-of-MADD-?lang=en)
//!  - [NEG - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/NEG--shifted-register---Negate--shifted-register---an-alias-of-SUB--shifted-register--?lang=en)
//!  - [NOP](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/NOP--No-Operation-?lang=en)

use crate::instruction_encoding::data_proc_reg::add_sub_shift_reg::AddSubtractShiftedRegister;
use crate::instruction_encoding::data_proc_reg::data_proc_three_src::DataProcessingThreeSource;
use crate::instruction_encoding::data_proc_reg::logical_shift_reg::LogicalShiftRegister;
use crate::types::{Register, UImm5, UImm6};
use crate::types::register::{WZR, XZR};
use crate::types::shifts::Shift3;

/// # Common Aliases
///
/// This trait implements common aliases such as MOV.
///
/// Implements the following instructions:
///  - [MOV (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOV--register---Move--register---an-alias-of-ORR--shifted-register--?lang=en)
///  - [ADD - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--shifted-register---Add--shifted-register--?lang=en)
///  - [ADDS - register - setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--shifted-register---Add--shifted-register---setting-flags-?lang=en)
///  - [SUB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--shifted-register---Subtract--shifted-register--?lang=en)
///  - [SUBS - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--shifted-register---Subtract--shifted-register---setting-flags-?lang=en)
///  - [MUL - Multiply](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MUL--Multiply--an-alias-of-MADD-?lang=en)
///  - [NEG - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/NEG--shifted-register---Negate--shifted-register---an-alias-of-SUB--shifted-register--?lang=en)
///  - [NOP](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/NOP--No-Operation-?lang=en)
pub trait CommonAliases<T>:
    LogicalShiftRegister<T> + AddSubtractShiftedRegister<T> + DataProcessingThreeSource<T>
{
    /// [MOV (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOV--register---Move--register---an-alias-of-ORR--shifted-register--?lang=en)
    ///
    /// Move (register) copies the value in a source register to the destination register.
    ///
    /// This is an alias of ORR (shifted register). This means:
    /// - The encodings in this description are named to match the encodings of ORR (shifted register).
    /// - The description of ORR (shifted register) gives the operational pseudocode for this instruction.
    ///
    /// ```asm
    /// MOV <Wd>, <Wm>
    /// ```
    #[inline(always)]
    fn mov_32_reg(&mut self, wd: Register, wm: Register) -> T {
        self.orr_32(wd, WZR, wm, None)
    }

    /// [MOV (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOV--register---Move--register---an-alias-of-ORR--shifted-register--?lang=en)
    ///
    /// Move (register) copies the value in a source register to the destination register.
    ///
    /// This is an alias of ORR (shifted register). This means:
    /// - The encodings in this description are named to match the encodings of ORR (shifted register).
    /// - The description of ORR (shifted register) gives the operational pseudocode for this instruction.
    ///
    /// ```asm
    /// MOV <Xd>, <Xm>
    /// ```
    #[inline(always)]
    fn mov_64_reg(&mut self, xd: Register, xm: Register) -> T {
        self.orr_64(xd, XZR, xm, None)
    }

    /// [ADD - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--shifted-register---Add--shifted-register--?lang=en)
    ///
    /// Alias of: Add (shifted register) adds a register value and an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// ADD <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn add_32_reg(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        self.add_32_reg_shift(wd, wn, wm, Shift3::LSL(0))
    }

    /// [ADD - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--shifted-register---Add--shifted-register--?lang=en)
    ///
    /// Alias of: Add (shifted register) adds a register value and an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// ADD <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn add_64_reg(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        self.add_64_reg_shift(xd, xn, xm, Shift3::LSL(0))
    }

    /// [ADDS - register -  Add - register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--shifted-register---Add--shifted-register---setting-flags-?lang=en)
    ///
    /// Alias of: Add (shifted register), setting flags, adds a register value and an optionally-shifted register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the alias CMN (shifted register).
    ///
    /// ```asm
    /// ADDS <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn adds_32_reg(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        self.adds_32_reg_shift(wd, wn, wm, Shift3::LSL(0))
    }

    /// [ADDS - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--shifted-register---Add--shifted-register---setting-flags-?lang=en)
    ///
    /// Alias of: Add (shifted register), setting flags, adds a register value and an optionally-shifted register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the alias CMN (shifted register).
    ///
    /// ```asm
    /// ADDS <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn adds_64_reg(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        self.adds_64_reg_shift(xd, xn, xm, Shift3::LSL(0))
    }

    /// [SUB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--shifted-register---Subtract--shifted-register--?lang=en)
    ///
    /// Alias of: Subtract (shifted register) subtracts an optionally-shifted register value from a register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias NEG (shifted register).
    ///
    /// ```asm
    /// SUB <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn sub_32_reg(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        self.sub_32_reg_shift(wd, wn, wm, Shift3::LSL(0))
    }

    /// [SUB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--shifted-register---Subtract--shifted-register--?lang=en)
    ///
    /// Alias of: Subtract (shifted register) subtracts an optionally-shifted register value from a register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias NEG (shifted register).
    ///
    /// ```asm
    /// SUB <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn sub_64_reg(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        self.sub_64_reg_shift(xd, xn, xm, Shift3::LSL(0))
    }

    /// [SUBS - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--shifted-register---Subtract--shifted-register---setting-flags-?lang=en)
    ///
    /// Alias of: Subtract (shifted register), setting flags, subtracts an optionally-shifted register value from a register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the aliases CMP (shifted register), and NEGS.
    ///
    /// ```asm
    /// SUBS <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn subs_32_reg(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        self.subs_32_reg_shift(wd, wn, wm, Shift3::LSL(0))
    }

    /// [SUBS - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--shifted-register---Subtract--shifted-register---setting-flags-?lang=en)
    ///
    /// Alias of: Subtract (shifted register), setting flags, subtracts an optionally-shifted register value from a register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the aliases CMP (shifted register), and NEGS.
    ///
    /// ```asm
    /// SUBS <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn subs_64_reg(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        self.subs_64_reg_shift(xd, xn, xm, Shift3::LSL(0))
    }

    /// [MUL - Multiply](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MUL--Multiply--an-alias-of-MADD-?lang=en)
    ///
    /// : Rd = Rn * Rm.
    ///
    /// This is an alias of MADD. This means:
    /// - The encodings in this description are named to match the encodings of MADD.
    /// - The description of MADD gives the operational pseudocode for this instruction.
    ///
    /// ```asm
    /// MUL <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn mul_32_reg(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        self.madd_32(wd, wn, wm, WZR)
    }

    /// [MUL - Multiply](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MUL--Multiply--an-alias-of-MADD-?lang=en)
    ///
    /// : Rd = Rn * Rm.
    ///
    /// This is an alias of MADD. This means:
    /// - The encodings in this description are named to match the encodings of MADD.
    /// - The description of MADD gives the operational pseudocode for this instruction.
    ///
    /// ```asm
    /// MUL <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn mul_64_reg(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        self.madd_64(xd, xn, xm, XZR)
    }

    /// [NEG - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/NEG--shifted-register---Negate--shifted-register---an-alias-of-SUB--shifted-register--?lang=en)
    /// Negate (shifted register) negates an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// This is an alias of SUB (shifted register). This means:
    /// - The encodings in this description are named to match the encodings of SUB (shifted register).
    /// - The description of SUB (shifted register) gives the operational pseudocode for this instruction.
    ///
    /// ```asm
    /// NEG <Wd>, <Wm>{, <shift> #<amount>}
    /// ```
    #[inline(always)]
    fn neg_32_reg(&mut self, wd: Register, wm: Register, shift: Option<Shift3<UImm5>>) -> T {
        self.sub_32_reg_shift(wd, WZR, wm, shift.unwrap_or(Shift3::LSL(0)))
    }

    /// [NEG - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/NEG--shifted-register---Negate--shifted-register---an-alias-of-SUB--shifted-register--?lang=en)
    /// Negate (shifted register) negates an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// This is an alias of SUB (shifted register). This means:
    /// - The encodings in this description are named to match the encodings of SUB (shifted register).
    /// - The description of SUB (shifted register) gives the operational pseudocode for this instruction.
    ///
    /// ```asm
    /// NEG <Wd>, <Wm>{, <shift> #<amount>}
    /// ```
    #[inline(always)]
    fn neg_64_reg(&mut self, xd: Register, xm: Register, shift: Option<Shift3<UImm6>>) -> T {
        self.sub_64_reg_shift(xd, WZR, xm, shift.unwrap_or(Shift3::LSL(0)))
    }

    /// [NOP](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/NOP--No-Operation-?lang=en)
    ///
    /// No Operation does nothing, other than advance the value of the program counter by 4. This instruction can be used for instruction alignment purposes.
    ///
    /// ```asm
    /// NOP
    /// ```
    fn nop(&mut self) -> T {
        self.process(0b11010101000000110010000000011111)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_mov() {
        let mut prod = TestProducer::new();

        let instr = prod.mov_32_reg(3, 4);
        assert_eq!(instr, "mov w3, w4");

        let instr = prod.mov_64_reg(1, 28);
        assert_eq!(instr, "mov x1, x28");
    }

    #[test]
    fn test_addx() {
        let mut prod = TestProducer::new();

        let instr = prod.add_32_reg(3, 4, 20);
        assert_eq!(instr, "add w3, w4, w20");

        let instr = prod.add_64_reg(1, 28, 20);
        assert_eq!(instr, "add x1, x28, x20");

        let instr = prod.adds_32_reg(3, 4, 20);
        assert_eq!(instr, "adds w3, w4, w20");

        let instr = prod.adds_64_reg(1, 28, 20);
        assert_eq!(instr, "adds x1, x28, x20");
    }

    #[test]
    fn test_subx() {
        let mut prod = TestProducer::new();

        let instr = prod.sub_32_reg(3, 4, 20);
        assert_eq!(instr, "sub w3, w4, w20");

        let instr = prod.sub_64_reg(1, 28, 20);
        assert_eq!(instr, "sub x1, x28, x20");

        let instr = prod.subs_32_reg(3, 4, 20);
        assert_eq!(instr, "subs w3, w4, w20");

        let instr = prod.subs_64_reg(1, 28, 20);
        assert_eq!(instr, "subs x1, x28, x20");
    }

    #[test]
    fn test_mul() {
        let mut prod = TestProducer::new();

        let instr = prod.mul_32_reg(3, 4, 20);
        assert_eq!(instr, "mul w3, w4, w20");

        let instr = prod.mul_64_reg(1, 28, 20);
        assert_eq!(instr, "mul x1, x28, x20");
    }

    #[test]
    fn test_neg() {
        let mut prod = TestProducer::new();

        let instr = prod.neg_32_reg(3, 20, None);
        assert_eq!(instr, "neg w3, w20");

        let instr = prod.neg_32_reg(3, 20, Shift3::ASR(31).into());
        assert_eq!(instr, "neg w3, w20, asr #0x1f");

        let instr = prod.neg_64_reg(1, 20, None);
        assert_eq!(instr, "neg x1, x20");

        let instr = prod.neg_64_reg(1, 20, Shift3::LSR(63).into());
        assert_eq!(instr, "neg x1, x20, lsr #0x3f");
    }

    #[test]
    fn test_nop() {
        let mut prod = TestProducer::new();

        let instr = prod.nop();
        assert_eq!(instr, "nop");
    }
}
