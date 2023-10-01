//! # Move wide (immediate)
//!
//! Implements the following instructions:
//! - [MOVN](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVN--Move-wide-with-NOT-?lang=en)
//! - [MOVZ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVZ--Move-wide-with-zero-?lang=en)
//! - [MOVK](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVK--Move-wide-with-keep-?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Register, UImm16, HW};

/// Internal function used to encode `MOV` instructions with immediate values.
/// Parameters:
/// * `sf`: defines whether the operation is 32-bit or 64-bit.
/// * `opc`: defines the specific `MOV` operation.
/// * `hw`: provides the shift amount.
/// * `d`: specifies the destination register.
/// * `imm`: specifies the immediate value.
#[inline(always)]
fn emit_mov_imm_x<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: u8,
    opc: u8,
    hw: u8,
    d: u8,
    imm: u16,
) -> T {
    let r = bseq_32!(sf:1 opc:2 100101 hw:2 imm:16 d:5);
    proc.process(r)
}

/// Generates a `MOVN` (Move Not) instruction. The `sf` parameter indicates whether it's a 32-bit (`false`) or 64-bit (`true`) operation.
/// * `d`: destination register.
/// * `imm`: immediate value.
/// * `lsl`: shift amount as defined by the `HW` enum.
#[inline(always)]
fn emit_movn_imm_x<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: bool,
    d: Register,
    imm: UImm16,
    lsl: HW,
) -> T {
    emit_mov_imm_x(proc, sf.into(), 0b00, lsl.into(), d, imm)
}

/// Generates a `MOVZ` (Move Zero) instruction. The `sf` parameter indicates whether it's a 32-bit (`false`) or 64-bit (`true`) operation.
#[inline(always)]
fn emit_movz_imm_x<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: bool,
    d: Register,
    imm: UImm16,
    lsl: HW,
) -> T {
    emit_mov_imm_x(proc, sf.into(), 0b10, lsl.into(), d, imm)
}

/// Generates a `MOVK` (Move Keep) instruction. The `sf` parameter indicates whether it's a 32-bit (`false`) or 64-bit (`true`) operation.
#[inline(always)]
fn emit_movk_imm_x<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: bool,
    d: Register,
    imm: UImm16,
    lsl: HW,
) -> T {
    emit_mov_imm_x(proc, sf.into(), 0b11, lsl.into(), d, imm)
}

/// # Move wide (immediate)
///
/// Implements the following instructions:
/// - [MOVN](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVN--Move-wide-with-NOT-?lang=en)
/// - [MOVZ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVZ--Move-wide-with-zero-?lang=en)
/// - [MOVK](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVK--Move-wide-with-keep-?lang=en)
pub trait MovWideImmediate<T>: InstructionProcessor<T> {
    // MOVN 32 and 64

    /// [MOVN](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVN--Move-wide-with-NOT-?lang=en)\
    /// Generates a 32-bit `MOVN` (Move Not) instruction.
    ///
    /// ```asm
    /// MOVN <Wd>, #<imm>
    /// ```
    #[inline(always)]
    fn movn_32_imm(&mut self, wd: Register, imm: UImm16) -> T {
        emit_movn_imm_x(self, false, wd, imm, 0.into())
    }

    /// [MOVN](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVN--Move-wide-with-NOT-?lang=en)\
    /// Generates a 32-bit `MOVN` (Move Not) instruction.
    ///
    /// ```asm
    /// MOVN <Wd>, #<imm>, LSL #<shift>
    /// ```
    #[inline(always)]
    fn movn_32_imm_lsl(&mut self, wd: Register, imm: UImm16, lsl: HW) -> T {
        debug_assert!(
            lsl == HW::LSL0 || lsl == HW::LSL16,
            "lsl shift must be either 0 or 16 for 32 bit mov"
        );
        emit_movn_imm_x(self, false, wd, imm, lsl)
    }

    /// [MOVN](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVN--Move-wide-with-NOT-?lang=en)\
    /// Generates a 64-bit `MOVN` (Move Not) instruction.
    ///
    /// ```asm
    /// MOVN <Xd>, #<imm>
    /// ```
    #[inline(always)]
    fn movn_64_imm(&mut self, xd: Register, imm: UImm16) -> T {
        emit_movn_imm_x(self, true, xd, imm, 0.into())
    }

    /// [MOVN](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVN--Move-wide-with-NOT-?lang=en)\
    /// Generates a 64-bit `MOVN` (Move Not) instruction.
    ///
    /// ```asm
    /// MOVN <Xd>, #<imm>, LSL #<shift>
    /// ```
    #[inline(always)]
    fn movn_64_imm_lsl(&mut self, xd: Register, imm: UImm16, lsl: HW) -> T {
        emit_movn_imm_x(self, true, xd, imm, lsl)
    }

    // MOVZ 32 and 64

    /// [MOVZ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVZ--Move-wide-with-zero-?lang=en) \
    /// Generates a 32-bit `MOVZ` (Move Zero) instruction.
    ///
    /// ```asm
    /// MOVZ <Wd>, #<imm>
    /// ```
    #[inline(always)]
    fn movz_32_imm(&mut self, wd: Register, imm: UImm16) -> T {
        emit_movz_imm_x(self, false, wd, imm, 0.into())
    }

    /// [MOVZ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVZ--Move-wide-with-zero-?lang=en) \
    /// Generates a 32-bit `MOVZ` (Move Zero) instruction.
    ///
    /// ```asm
    /// MOVZ <Wd>, #<imm>, LSL #<shift>
    /// ```
    #[inline(always)]
    fn movz_32_imm_lsl(&mut self, wd: Register, imm: UImm16, lsl: HW) -> T {
        debug_assert!(
            lsl == HW::LSL0 || lsl == HW::LSL16,
            "lsl shift must be either 0 or 16 for 32 bit mov"
        );
        emit_movz_imm_x(self, false, wd, imm, lsl)
    }

    /// [MOVZ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVZ--Move-wide-with-zero-?lang=en) \
    /// Generates a 64-bit `MOVZ` (Move Zero) instruction.
    ///
    /// ```asm
    /// MOVZ <Xd>, #<imm>
    /// ```
    #[inline(always)]
    fn movz_64_imm(&mut self, xd: Register, imm: UImm16) -> T {
        emit_movz_imm_x(self, true, xd, imm, 0.into())
    }

    /// [MOVZ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVZ--Move-wide-with-zero-?lang=en) \
    /// Generates a 64-bit `MOVZ` (Move Zero) instruction.
    ///
    /// ```asm
    /// MOVZ <Xd>, #<imm>, LSL #<shift>
    /// ```
    #[inline(always)]
    fn movz_64_imm_lsl(&mut self, xd: Register, imm: UImm16, lsl: HW) -> T {
        emit_movz_imm_x(self, true, xd, imm, lsl)
    }

    // MOVK 32 and 64

    /// [MOVK](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVK--Move-wide-with-keep-?lang=en) \
    /// Generates a 32-bit `MOVK` (Move Keep) instruction.
    ///
    /// ```asm
    /// MOVK <Wd>, #<imm>
    /// ```
    #[inline(always)]
    fn movk_32_imm(&mut self, wd: Register, imm: UImm16) -> T {
        emit_movk_imm_x(self, false, wd, imm, 0.into())
    }

    /// [MOVK](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVK--Move-wide-with-keep-?lang=en) \
    /// Generates a 32-bit `MOVK` (Move Keep) instruction.
    ///
    /// ```asm
    /// MOVK <Wd>, #<imm>, LSL #<shift>
    /// ```
    #[inline(always)]
    fn movk_32_imm_lsl(&mut self, wd: Register, imm: UImm16, lsl: HW) -> T {
        debug_assert!(
            lsl == HW::LSL0 || lsl == HW::LSL16,
            "lsl shift must be either 0 or 16 for 32 bit mov"
        );
        emit_movk_imm_x(self, false, wd, imm, lsl)
    }

    /// [MOVK](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVK--Move-wide-with-keep-?lang=en) \
    /// Generates a 64-bit `MOVK` (Move Keep) instruction.
    ///
    /// ```asm
    /// MOVK <Xd>, #<imm>
    /// ```
    #[inline(always)]
    fn movk_64_imm(&mut self, xd: Register, imm: UImm16) -> T {
        emit_movk_imm_x(self, true, xd, imm, 0.into())
    }

    /// [MOVK](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOVK--Move-wide-with-keep-?lang=en) \
    /// Generates a 64-bit `MOVK` (Move Keep) instruction.
    ///
    /// ```asm
    /// MOVK <Xd>, #<imm>, LSL #<shift>
    /// ```
    #[inline(always)]
    fn movk_64_imm_lsl(&mut self, xd: Register, imm: UImm16, lsl: HW) -> T {
        emit_movk_imm_x(self, true, xd, imm, lsl)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_movn_x() {
        let mut prod = TestProducer::new();

        let instr = prod.movn_32_imm(2, 0xffff);
        assert_eq!(instr, "movn w2, #0xffff");

        let instr = prod.movn_32_imm_lsl(2, 0xffff, HW::LSL16);
        assert_eq!(instr, "movn w2, #0xffff, lsl #0x10");

        assert_panic!("Should panic: wrong shift"; prod.movn_32_imm_lsl(2, 0xffff, HW::LSL32));

        let instr = prod.movn_64_imm(2, 0xffff);
        assert_eq!(instr, "mov x2, #0xffffffffffff0000");

        let instr = prod.movn_64_imm_lsl(2, 0xffff, HW::LSL16);
        assert_eq!(instr, "mov x2, #0xffffffff0000ffff");
    }

    #[test]
    fn test_movk_x() {
        let mut prod = TestProducer::new();

        let instr = prod.movk_32_imm(2, 0xffff);
        assert_eq!(instr, "movk w2, #0xffff");

        let instr = prod.movk_32_imm_lsl(2, 0xffff, HW::LSL16);
        assert_eq!(instr, "movk w2, #0xffff, lsl #0x10");

        assert_panic!("Should panic: wrong shift"; prod.movk_32_imm_lsl(2, 0xffff, HW::LSL32));

        let instr = prod.movk_64_imm(2, 0xffff);
        assert_eq!(instr, "movk x2, #0xffff");

        let instr = prod.movk_64_imm_lsl(2, 0xffff, HW::LSL32);
        assert_eq!(instr, "movk x2, #0xffff, lsl #0x20");
    }

    #[test]
    fn test_movz_x() {
        let mut prod = TestProducer::new();

        let instr = prod.movz_32_imm(2, 0xffff);
        assert_eq!(instr, "mov w2, #0xffff");

        let instr = prod.movz_32_imm_lsl(2, 0xffff, HW::LSL16);
        assert_eq!(instr, "mov w2, #0xffffffffffff0000");

        assert_panic!("Should panic: wrong shift"; prod.movz_32_imm_lsl(2, 0xffff, HW::LSL32));

        let instr = prod.movz_64_imm(2, 0xffff);
        assert_eq!(instr, "mov x2, #0xffff");

        let instr = prod.movz_64_imm_lsl(2, 0xffff, HW::LSL32);
        assert_eq!(instr, "mov x2, #0xffff00000000");
    }
}
