//! # Add/subtract (immediate)
//!
//! - ADD 32bit
//! - ADDS 32bit
//! - SUB 32bit
//! - SUBS 32bit
//! - ADD 64bit
//! - ADDS 64bit
//! - SUB 64bit
//! - SUBS 64bit

use bit_seq::bseq_32;
use crate::instruction_emitter::Emitter;
use crate::instruction_encoding::InstructionProcessor;

use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::{Imm12, Register};
use crate::types::shifts::Shift1;

/// The `add_sub_imm` function is a helper function to generate ADD/SUB instructions
/// (with immediate value) according to the ARMv8 encoding rules. This is not intended to be used
/// directly but rather used internally by other public-facing functions.
/// The parameters are bits used in the encoding: `sf` for setting the operation size, `op` for
/// determining add/sub operation, `S` for setting the flags, `shift` for optional left shift
/// of the immediate, `imm12` is the immediate value, `rn` and `rd` are registers.
#[inline(always)]
fn emit_add_sub_imm_x<P: InstructionProcessor<T>, T>(proc: &mut P, sf: u8, op: u8, S: u8, shift: u8, imm12: u16, rn: Register, rd: Register) -> T {
    let r = bseq_32!(sf:1 op:1 S:1 10001 0 shift:1 imm12:12 rn:5 rd:5);
    proc.emit(r)
}

pub trait AddSubtractImmediate<T>: InstructionProcessor<T> {
    /// Generates a 32-bit ADD instruction with immediate value. The immediate value is provided
    /// as `imm12`, and `rn` and `rd` are the source and destination registers respectively.
    #[inline(always)]
    fn add_32_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 0, 0, 0, 0, imm12, rn, rd)
    }

    /// Similar to `add_32_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    #[inline(always)]
    fn add_32_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 0, 0, 0, lsl.into(), imm12, rn, rd)
    }

    /// Generates a 64-bit ADD instruction with immediate value.
    /// The parameters are same as `add_32_imm` but this operates on 64-bit registers.
    #[inline(always)]
    fn add_64_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 1, 0, 0, 0, imm12, rn, rd)
    }

    /// Similar to `add_64_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    #[inline(always)]
    fn add_64_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 1, 0, 0, lsl.into(), imm12, rn, rd)
    }

    /// Generates a 32-bit SUB instruction with immediate value.
    /// The immediate value is provided as `imm12`, and `rn` and `rd` are the source and destination registers respectively.
    #[inline(always)]
    fn sub_32_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 0, 1, 0, 0, imm12, rn, rd)
    }

    /// Similar to `sub_32_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    #[inline(always)]
    fn sub_32_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 0, 1, 0, lsl.into(), imm12, rn, rd)
    }

    /// Generates a 64-bit SUB instruction with immediate value.
    /// The parameters are the same as `sub_32_imm` but this operates on 64-bit registers.
    #[inline(always)]
    fn sub_64_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 1, 1, 0, 0, imm12, rn, rd)
    }

    /// Similar to `sub_64_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    #[inline(always)]
    fn sub_64_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 1, 1, 0, lsl.into(), imm12, rn, rd)
    }

    // The following functions are analogous to the above, but instead generating the
    // corresponding ADDS/SUBS instructions. ADDS and SUBS are same as ADD and SUB
    // but they also update the condition flags.

    /// Generates a 32-bit ADDS instruction with immediate value.
    /// The ADDS instruction is the same as the ADD instruction but also updates the condition flags.
    /// The immediate value is provided as `imm12`, and `rn` and `rd` are the source and destination registers respectively.
    #[inline(always)]
    fn adds_32_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 0, 0, 1, 0, imm12, rn, rd)
    }

    /// Similar to `adds_32_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    #[inline(always)]
    fn adds_32_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 0, 0, 1, lsl.into(), imm12, rn, rd)
    }

    /// Generates a 64-bit ADDS instruction with immediate value.
    /// The parameters are the same as `adds_32_imm` but this operates on 64-bit registers.
    #[inline(always)]
    fn adds_64_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 1, 0, 1, 0, imm12, rn, rd)
    }

    /// Similar to `adds_64_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    #[inline(always)]
    fn adds_64_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 1, 0, 1, lsl.into(), imm12, rn, rd)
    }


    /// Generates a 32-bit SUBS instruction with immediate value.
    /// The SUBS instruction is same as the SUB instruction but also updates the condition flags.
    /// The immediate value is provided as `imm12`, and `rn` and `rd` are the source and destination registers respectively.
    #[inline(always)]
    fn subs_32_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 0, 1, 1, 0, imm12, rn, rd)
    }

    /// Similar to `subs_32_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    #[inline(always)]
    fn subs_32_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 0, 1, 1, lsl.into(), imm12, rn, rd)
    }

    /// Generates a 64-bit SUBS instruction with immediate value.
    /// The parameters are same as `subs_32_imm` but this operates on 64-bit registers.
    #[inline(always)]
    fn subs_64_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) -> T {
        emit_add_sub_imm_x(self, 1, 1, 1, 0, imm12, rn, rd)
    }

    /// Similar to `subs_64_imm`, but with the ability to left shift the immediate value
    /// by an amount specified in the `lsl` parameter.
    #[inline(always)]
    fn subs_64_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) -> T {
        emit_add_sub_imm_x(self, 1, 1, 1, lsl.into(), imm12, rn, rd)
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction_producer::InstrProducer;
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
}