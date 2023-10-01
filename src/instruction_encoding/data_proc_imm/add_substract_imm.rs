//! # Add/subtract (immediate)
//!
//! Implements the following instructions:
//! - [ADD (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--immediate---Add--immediate--?lang=en)
//! - [ADDS (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--immediate---Add--immediate---setting-flags-?lang=en)
//! - [SUB (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--immediate---Subtract--immediate--?lang=en)
//! - [SUBS (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--immediate---Subtract--immediate---setting-flags-?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Imm12, Register, UImm10, UImm4};
use crate::types::shifts::Shift1;

/// The `add_sub_imm` function is a helper function to generate ADD/SUB instructions
/// (with immediate value) according to the ARMv8 encoding rules. This is not intended to be used
/// directly but rather used internally by other public-facing functions.
/// The parameters are bits used in the encoding: `sf` for setting the operation size, `op` for
/// determining add/sub operation, `s` for setting the flags, `shift` for optional left shift
/// of the immediate, `imm12` is the immediate value, `rn` and `rd` are registers.
#[inline(always)]
fn emit_add_sub_imm_x<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: u8,
    op: u8,
    s: u8,
    shift: u8,
    imm12: u16,
    rn: Register,
    rd: Register,
) -> T {
    let r = bseq_32!(sf:1 op:1 s:1 10001 0 shift:1 imm12:12 rn:5 rd:5);
    proc.process(r)
}

#[inline(always)]
fn emit_add_sub_imm_w_tags_x<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: u8,
    op: u8,
    S: u8,
    o2: u8,
    uimm6: u8,
    op3: u8,
    uimm4: u8,
    rn: Register,
    rd: Register,
) -> T {
    let r = bseq_32!(sf:1 op:1 S:1 100011 o2:1 uimm6:6 op3:2 uimm4:4 rn:5 rd:5);
    proc.process(r)
}

/// # Add/subtract (immediate)
///
/// Implements the following instructions:
/// - [ADD (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--immediate---Add--immediate--?lang=en)
/// - [ADDS (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--immediate---Add--immediate---setting-flags-?lang=en)
/// - [SUB (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--immediate---Subtract--immediate--?lang=en)
/// - [SUBS (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--immediate---Subtract--immediate---setting-flags-?lang=en)
pub trait AddSubtractImmediate<T>: InstructionProcessor<T> {
    /// [ADD (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--immediate---Add--immediate--?lang=en)\
    /// Generates a 32-bit ADD instruction with immediate value. The immediate value is provided
    /// as `imm12`, and `rn` and `rd` are the source and destination registers respectively.
    ///
    /// ```asm
    /// ADD <Wd|WSP>, <Wn|WSP>, #<imm>
    /// ```
    #[inline(always)]
    fn add_32_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 0, 0, 0, 0, imm12, rn, rd)
    }

    /// [ADD (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--immediate---Add--immediate--?lang=en)\
    /// Similar to `add_32_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    ///
    /// ```asm
    /// ADD <Wd|WSP>, <Wn|WSP>, #<imm>, <shift>
    /// ```
    #[inline(always)]
    fn add_32_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 0, 0, 0, lsl.into(), imm12, rn, rd)
    }

    /// [ADD (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--immediate---Add--immediate--?lang=en)\
    /// Generates a 64-bit ADD instruction with immediate value.
    /// The parameters are same as `add_32_imm` but this operates on 64-bit registers.
    ///
    /// ```asm
    /// ADD <Wd|WSP>, <Wn|WSP>, #<imm>
    /// ```
    #[inline(always)]
    fn add_64_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 1, 0, 0, 0, imm12, rn, rd)
    }

    /// [ADD (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--immediate---Add--immediate--?lang=en)\
    /// Similar to `add_64_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    ///
    /// ```asm
    /// ADD <Wd|WSP>, <Wn|WSP>, #<imm>, <shift>
    /// ```
    #[inline(always)]
    fn add_64_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 1, 0, 0, lsl.into(), imm12, rn, rd)
    }

    /// [SUB (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--immediate---Subtract--immediate--?lang=en)\
    /// Generates a 32-bit SUB instruction with immediate value.
    /// The immediate value is provided as `imm12`, and `rn` and `rd` are the source and destination registers respectively.
    ///
    /// ```asm
    /// SUB <Wd|WSP>, <Wn|WSP>, #<imm>
    /// ```
    #[inline(always)]
    fn sub_32_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 0, 1, 0, 0, imm12, rn, rd)
    }

    /// [SUB (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--immediate---Subtract--immediate--?lang=en)\
    /// Similar to `sub_32_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    ///
    /// ```asm
    /// SUB <Wd|WSP>, <Wn|WSP>, #<imm>, <shift>
    /// ```
    #[inline(always)]
    fn sub_32_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 0, 1, 0, lsl.into(), imm12, rn, rd)
    }

    /// [SUB (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--immediate---Subtract--immediate--?lang=en)\
    /// Generates a 64-bit SUB instruction with immediate value.
    /// The parameters are the same as `sub_32_imm` but this operates on 64-bit registers.
    ///
    /// ```asm
    /// SUB <Wd|WSP>, <Wn|WSP>, #<imm>
    /// ```
    #[inline(always)]
    fn sub_64_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 1, 1, 0, 0, imm12, rn, rd)
    }

    /// [SUB (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--immediate---Subtract--immediate--?lang=en)\
    /// Similar to `sub_64_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    ///
    /// ```asm
    /// SUB <Wd|WSP>, <Wn|WSP>, #<imm>, <shift>
    /// ```
    #[inline(always)]
    fn sub_64_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 1, 1, 0, lsl.into(), imm12, rn, rd)
    }

    // The following functions are analogous to the above, but instead generating the
    // corresponding ADDS/SUBS instructions. ADDS and SUBS are same as ADD and SUB
    // but they also update the condition flags.

    /// [ADDS (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--immediate---Add--immediate---setting-flags-?lang=en)\
    /// Generates a 32-bit ADDS instruction with immediate value.
    /// The ADDS instruction is the same as the ADD instruction but also updates the condition flags.
    /// The immediate value is provided as `imm12`, and `rn` and `rd` are the source and destination registers respectively.
    ///
    /// ```asm
    /// ADDS <Wd|WSP>, <Wn|WSP>, #<imm>
    /// ```
    #[inline(always)]
    fn adds_32_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 0, 0, 1, 0, imm12, rn, rd)
    }

    /// [ADDS (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--immediate---Add--immediate---setting-flags-?lang=en)\
    /// Similar to `adds_32_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    ///
    /// ```asm
    /// ADDS <Wd|WSP>, <Wn|WSP>, #<imm>, <shift>
    /// ```
    #[inline(always)]
    fn adds_32_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 0, 0, 1, lsl.into(), imm12, rn, rd)
    }

    /// [ADDS (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--immediate---Add--immediate---setting-flags-?lang=en)\
    /// Generates a 64-bit ADDS instruction with immediate value.
    /// The parameters are the same as `adds_32_imm` but this operates on 64-bit registers.
    ///
    /// ```asm
    /// ADDS <Wd|WSP>, <Wn|WSP>, #<imm>
    /// ```
    #[inline(always)]
    fn adds_64_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 1, 0, 1, 0, imm12, rn, rd)
    }

    /// [ADDS (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--immediate---Add--immediate---setting-flags-?lang=en)\
    /// Similar to `adds_64_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    ///
    /// ```asm
    /// ADDS <Wd|WSP>, <Wn|WSP>, #<imm>, <shift>
    /// ```
    #[inline(always)]
    fn adds_64_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 1, 0, 1, lsl.into(), imm12, rn, rd)
    }

    /// [SUBS (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--immediate---Subtract--immediate---setting-flags-?lang=en)\
    /// Generates a 32-bit SUBS instruction with immediate value.
    /// The SUBS instruction is same as the SUB instruction but also updates the condition flags.
    /// The immediate value is provided as `imm12`, and `rn` and `rd` are the source and destination registers respectively.
    ///
    /// ```asm
    /// SUBS <Wd|WSP>, <Wn|WSP>, #<imm>
    /// ```
    #[inline(always)]
    fn subs_32_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 0, 1, 1, 0, imm12, rn, rd)
    }

    /// [SUBS (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--immediate---Subtract--immediate---setting-flags-?lang=en)\
    /// Similar to `subs_32_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    ///
    /// ```asm
    /// SUBS <Wd|WSP>, <Wn|WSP>, #<imm>, <shift>
    /// ```
    #[inline(always)]
    fn subs_32_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 0, 1, 1, lsl.into(), imm12, rn, rd)
    }

    /// [SUBS (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--immediate---Subtract--immediate---setting-flags-?lang=en)\
    /// Generates a 64-bit SUBS instruction with immediate value.
    /// The parameters are same as `subs_32_imm` but this operates on 64-bit registers.
    ///
    /// ```asm
    /// SUBS <Wd|WSP>, <Wn|WSP>, #<imm>
    /// ```
    #[inline(always)]
    fn subs_64_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 1, 1, 1, 0, imm12, rn, rd)
    }

    /// [SUBS (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--immediate---Subtract--immediate---setting-flags-?lang=en)\
    /// Similar to `subs_64_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    ///
    /// ```asm
    /// SUBS <Wd|WSP>, <Wn|WSP>, #<imm>, <shift>
    /// ```
    #[inline(always)]
    fn subs_64_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 1, 1, 1, lsl.into(), imm12, rn, rd)
    }

    // Add/substract (immediate, with tags)

    /// [ADDG](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDG--Add-with-Tag-?lang=en)
    ///
    /// ```asm
    /// ADDG <Xd|SP>, <Xn|SP>, #<uimm6>, #<uimm4>
    /// ```
    #[inline(always)]
    fn addg(&mut self, xd_sp: Register, xn_sp: Register, uimm6: UImm10, uimm4: UImm4) -> T {
        debug_assert!(
            uimm6 % 16 == 0,
            "uimm6 must be a multiply of 16, was {}",
            uimm6
        );
        debug_assert!(
            uimm6 <= 1008,
            "uimm6 must be in range 0 to 1008, was {}",
            uimm6
        );
        let uimm6 = uimm6 >> 4;
        emit_add_sub_imm_w_tags_x(self, 1, 0, 0, 0, uimm6 as u8, 0, uimm4, xn_sp, xd_sp)
    }

    /// [SUBG](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDG--Add-with-Tag-?lang=en)
    ///
    /// ```asm
    /// SUBG <Xd|SP>, <Xn|SP>, #<uimm6>, #<uimm4>
    /// ```
    #[inline(always)]
    fn subg(&mut self, xd_sp: Register, xn_sp: Register, uimm6: UImm10, uimm4: UImm4) -> T {
        debug_assert!(
            uimm6 % 16 == 0,
            "uimm6 must be a multiply of 16, was {}",
            uimm6
        );
        debug_assert!(
            uimm6 <= 1008,
            "uimm6 must be in range 0 to 1008, was {}",
            uimm6
        );
        let uimm6 = uimm6 >> 4;
        emit_add_sub_imm_w_tags_x(self, 1, 1, 0, 0, uimm6 as u8, 0, uimm4, xn_sp, xd_sp)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_add_x() {
        let mut prod = TestProducer::new();

        let instr = prod.add_32_imm(0, 1, 0x18);
        assert_eq!(instr, "add w0, w1, #0x18");

        let instr = prod.add_32_imm_lsl(0, 1, 0x18, Shift1::LSL12);
        assert_eq!(instr, "add w0, w1, #0x18, lsl #0xc");

        let instr = prod.add_64_imm(0, 1, 0x18);
        assert_eq!(instr, "add x0, x1, #0x18");

        let instr = prod.add_64_imm_lsl(0, 1, 0x18, Shift1::LSL12);
        assert_eq!(instr, "add x0, x1, #0x18, lsl #0xc");
    }

    #[test]
    fn test_sub_x() {
        let mut prod = TestProducer::new();

        let instr = prod.sub_32_imm(0, 1, 0x18);
        assert_eq!(instr, "sub w0, w1, #0x18");

        let instr = prod.sub_32_imm_lsl(0, 1, 0x18, Shift1::LSL12);
        assert_eq!(instr, "sub w0, w1, #0x18, lsl #0xc");

        let instr = prod.sub_64_imm(0, 1, 0x18);
        assert_eq!(instr, "sub x0, x1, #0x18");

        let instr = prod.sub_64_imm_lsl(0, 1, 0x18, Shift1::LSL12);
        assert_eq!(instr, "sub x0, x1, #0x18, lsl #0xc");
    }

    #[test]
    fn test_adds_x() {
        let mut prod = TestProducer::new();

        let instr = prod.adds_32_imm(0, 1, 0x18);
        assert_eq!(instr, "adds w0, w1, #0x18");

        let instr = prod.adds_32_imm_lsl(0, 1, 0x18, Shift1::LSL12);
        assert_eq!(instr, "adds w0, w1, #0x18, lsl #0xc");

        let instr = prod.adds_64_imm(0, 1, 0x18);
        assert_eq!(instr, "adds x0, x1, #0x18");

        let instr = prod.adds_64_imm_lsl(0, 1, 0x18, Shift1::LSL12);
        assert_eq!(instr, "adds x0, x1, #0x18, lsl #0xc");
    }

    #[test]
    fn test_subs_x() {
        let mut prod = TestProducer::new();

        let instr = prod.subs_32_imm(0, 1, 0x18);
        assert_eq!(instr, "subs w0, w1, #0x18");

        let instr = prod.subs_32_imm_lsl(0, 1, 0x18, Shift1::LSL12);
        assert_eq!(instr, "subs w0, w1, #0x18, lsl #0xc");

        let instr = prod.subs_64_imm(0, 1, 0x18);
        assert_eq!(instr, "subs x0, x1, #0x18");

        let instr = prod.subs_64_imm_lsl(0, 1, 0x18, Shift1::LSL12);
        assert_eq!(instr, "subs x0, x1, #0x18, lsl #0xc");
    }

    #[test]
    fn test_addg_subg() {
        let mut prod = TestProducer::new();

        let instr = prod.addg(0, 1, 0x10, 0xf);
        assert_eq!(instr, "addg x0, x1, #0x10, #0xf");

        let instr = prod.subg(0, 1, 0x10, 0xf);
        assert_eq!(instr, "subg x0, x1, #0x10, #0xf");

        assert_panic!("should panic: not multiply"; prod.addg(0, 1, 0x9, 0xf));
        assert_panic!("should panic: not multiply"; prod.subg(0, 1, 0x9, 0xf));
    }
}
