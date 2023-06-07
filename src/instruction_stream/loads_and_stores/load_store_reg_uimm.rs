//! # Compare and swap pair
//!
//! Implements the following instructions:

use bit_seq::{bseq_16, bseq_32};
use crate::instruction_emitter::Emitter;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::instruction::Instr;
use crate::types::{Imm6, Imm9, Register, UImm12};

impl<'mem, M: Memory, E: Emitter> InstrStream<'mem, M, E> {

    // Helper functions -> Actual emits

    #[inline(always)]
    fn emit_load_store_offset(&mut self, size: u8, V: u8, opc: u8, pimm: u16, rn: Register, rt: Register) -> Instr {
        debug_assert!(0 <= pimm && pimm <= 4095, "pimm must be in range 0 to 4095");
        let r = bseq_32!(size:2 111 V:1 01 opc:2 pimm:12 rn:5 rt:5);
        self.emit(r)
    }

    /// # Arguments
    /// - mode: if pre (`0b11`) or post (`0b01`) index. Between `imm9` and `Rn` in encoding.
    #[inline(always)]
    fn emit_load_store_pre_post(&mut self, size: u8, V: u8, opc: u8, simm: Imm9, mode: u8, wt: Register, xn_sp: Register) -> Instr {
        debug_assert!(-256 <= simm && simm <= 255, "simm must be in range -256 to 255");
        let r = bseq_32!(size:2 111 V:1 00 opc:2 0 simm:9 mode:2 xn_sp:5 wt:5);
        self.emit(r)
    }

    // API methods

    // STRB instructions

    #[inline(always)]
    pub fn strb_post_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0, 0, 0, simm, 0b01, wt, xn_sp)
    }

    #[inline(always)]
    pub fn strb_pre_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0, 0, 0, simm, 0b11, wt, xn_sp)
    }

    #[inline(always)]
    pub fn strb_unsigned_offset(&mut self, wt: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        self.emit_load_store_offset(0, 0, 0, pimm, xn_sp, wt)
    }

    // LDRB instructions

    #[inline(always)]
    pub fn ldrb_post_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0, 0, 0b01, simm, 0b01, wt, xn_sp)
    }

    #[inline(always)]
    pub fn ldrb_pre_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0, 0, 0b01, simm, 0b11, wt, xn_sp)
    }

    #[inline(always)]
    pub fn ldrb_unsigned_offset(&mut self, wt: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        self.emit_load_store_offset(0, 0, 0b01, pimm, xn_sp, wt)
    }


    // LDRSB instructions

    #[inline(always)]
    pub fn ldrsb_32_imm_post_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0, 0, 0b11, simm, 0b01, wt, xn_sp)
    }

    #[inline(always)]
    pub fn ldrsb_32_imm_pre_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0, 0, 0b11, simm, 0b11, wt, xn_sp)
    }

    #[inline(always)]
    pub fn ldrsb_32_imm_unsigned_offset(&mut self, wt: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        self.emit_load_store_offset(0, 0, 0b11, pimm, xn_sp, wt)
    }

    #[inline(always)]
    pub fn ldrsb_64_imm_post_index(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0, 0, 0b10, simm, 0b01, xt, xn_sp)
    }

    #[inline(always)]
    pub fn ldrsb_64_imm_pre_index(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0, 0, 0b10, simm, 0b11, xt, xn_sp)
    }

    #[inline(always)]
    pub fn ldrsb_64_imm_unsigned_offset(&mut self, xt: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        self.emit_load_store_offset(0, 0, 0b10, pimm, xn_sp, xt)
    }


    // STR instructions

    #[inline(always)]
    pub fn str_8_imm_simd_post_index(&mut self, bt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b00, 1, 0b00, simm, 0b01, bt, xn_sp)
    }

    #[inline(always)]
    pub fn str_8_imm_simd_pre_index(&mut self, bt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b00, 1, 0b00, simm, 0b11, bt, xn_sp)
    }

    #[inline(always)]
    pub fn str_8_imm_simd_unsigned_offset(&mut self, bt: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        self.emit_load_store_offset(0b00, 1, 0b00, pimm, xn_sp, bt)
    }

    #[inline(always)]
    pub fn str_16_imm_simd_post_index(&mut self, ht: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b01, 1, 0b00, simm, 0b01, ht, xn_sp)
    }

    #[inline(always)]
    pub fn str_16_imm_simd_pre_index(&mut self, ht: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b01, 1, 0b00, simm, 0b11, ht, xn_sp)
    }

    #[inline(always)]
    pub fn str_16_imm_simd_unsigned_offset(&mut self, ht: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        debug_assert!(pimm <= 8190, "pimm must be in range 0 to 8190, was {}", pimm);
        debug_assert!(pimm % 2 == 0, "pimm must be multiply of 2, was {}", pimm);
        let pimm = pimm / 2;
        self.emit_load_store_offset(0b01, 1, 0b00, pimm, xn_sp, ht)
    }

    #[inline(always)]
    pub fn str_32_imm_simd_post_index(&mut self, st: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b10, 1, 0b00, simm, 0b01, st, xn_sp)
    }

    #[inline(always)]
    pub fn str_32_imm_simd_pre_index(&mut self, st: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b10, 1, 0b00, simm, 0b11, st, xn_sp)
    }

    #[inline(always)]
    pub fn str_32_imm_simd_unsigned_offset(&mut self, st: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        debug_assert!(pimm <= 16380, "pimm must be in range 0 to 16380, was {}", pimm);
        debug_assert!(pimm % 4 == 0, "pimm must be multiply of 4, was {}", pimm);
        let pimm = pimm / 4;
        self.emit_load_store_offset(0b10, 1, 0b00, pimm, xn_sp, st)
    }

    #[inline(always)]
    pub fn str_64_imm_simd_post_index(&mut self, dt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b11, 1, 0b00, simm, 0b01, dt, xn_sp)
    }

    #[inline(always)]
    pub fn str_64_imm_simd_pre_index(&mut self, dt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b11, 1, 0b00, simm, 0b11, dt, xn_sp)
    }

    #[inline(always)]
    pub fn str_64_imm_simd_unsigned_offset(&mut self, dt: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        debug_assert!(pimm <= 32760, "pimm must be in range 0 to 32760, was {}", pimm);
        debug_assert!(pimm % 8 == 0, "pimm must be multiply of 8, was {}", pimm);
        let pimm = pimm / 8;
        self.emit_load_store_offset(0b11, 1, 0b00, pimm, xn_sp, dt)
    }

    #[inline(always)]
    pub fn str_128_imm_simd_post_index(&mut self, qt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b00, 1, 0b10, simm, 0b01, qt, xn_sp)
    }

    #[inline(always)]
    pub fn str_128_imm_simd_pre_index(&mut self, qt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b00, 1, 0b10, simm, 0b11, qt, xn_sp)
    }

    #[inline(always)]
    pub fn str_128_imm_simd_unsigned_offset(&mut self, qt: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        debug_assert!(pimm <= 65520, "pimm must be in range 0 to 65520, was {}", pimm);
        debug_assert!(pimm % 16 == 0, "pimm must be multiply of 16, was {}", pimm);
        let pimm = pimm / 16;
        self.emit_load_store_offset(0b00, 1, 0b10, pimm, xn_sp, qt)
    }


    // LDR imm simd instructions

    #[inline(always)]
    pub fn ldr_8_imm_simd_post_index(&mut self, bt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b00, 1, 0b01, simm, 0b01, bt, xn_sp)
    }

    #[inline(always)]
    pub fn ldr_8_imm_simd_pre_index(&mut self, bt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b00, 1, 0b01, simm, 0b11, bt, xn_sp)
    }

    #[inline(always)]
    pub fn ldr_8_imm_simd_unsigned_offset(&mut self, bt: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        self.emit_load_store_offset(0b00, 1, 0b01, pimm, xn_sp, bt)
    }

    #[inline(always)]
    pub fn ldr_16_imm_simd_post_index(&mut self, ht: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b01, 1, 0b01, simm, 0b01, ht, xn_sp)
    }

    #[inline(always)]
    pub fn ldr_16_imm_simd_pre_index(&mut self, ht: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b01, 1, 0b01, simm, 0b11, ht, xn_sp)
    }

    #[inline(always)]
    pub fn ldr_16_imm_simd_unsigned_offset(&mut self, ht: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        debug_assert!(pimm <= 8190, "pimm must be in range 0 to 8190, was {}", pimm);
        debug_assert!(pimm % 2 == 0, "pimm must be multiply of 2, was {}", pimm);
        let pimm = pimm / 2;
        self.emit_load_store_offset(0b01, 1, 0b01, pimm, xn_sp, ht)
    }

    #[inline(always)]
    pub fn ldr_32_imm_simd_post_index(&mut self, st: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b10, 1, 0b01, simm, 0b01, st, xn_sp)
    }

    #[inline(always)]
    pub fn ldr_32_imm_simd_pre_index(&mut self, st: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b10, 1, 0b01, simm, 0b11, st, xn_sp)
    }

    #[inline(always)]
    pub fn ldr_32_imm_simd_unsigned_offset(&mut self, st: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        debug_assert!(pimm <= 16380, "pimm must be in range 0 to 16380, was {}", pimm);
        debug_assert!(pimm % 4 == 0, "pimm must be multiply of 4, was {}", pimm);
        let pimm = pimm / 4;
        self.emit_load_store_offset(0b10, 1, 0b01, pimm, xn_sp, st)
    }

    #[inline(always)]
    pub fn ldr_64_imm_simd_post_index(&mut self, dt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b11, 1, 0b01, simm, 0b01, dt, xn_sp)
    }

    #[inline(always)]
    pub fn ldr_64_imm_simd_pre_index(&mut self, dt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b11, 1, 0b01, simm, 0b11, dt, xn_sp)
    }

    #[inline(always)]
    pub fn ldr_64_imm_simd_unsigned_offset(&mut self, dt: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        debug_assert!(pimm <= 32760, "pimm must be in range 0 to 32760, was {}", pimm);
        debug_assert!(pimm % 8 == 0, "pimm must be multiply of 8, was {}", pimm);
        let pimm = pimm / 8;
        self.emit_load_store_offset(0b11, 1, 0b01, pimm, xn_sp, dt)
    }

    #[inline(always)]
    pub fn ldr_128_imm_simd_post_index(&mut self, qt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b00, 1, 0b11, simm, 0b01, qt, xn_sp)
    }

    #[inline(always)]
    pub fn ldr_128_imm_simd_pre_index(&mut self, qt: Register, xn_sp: Register, simm: Imm9) -> Instr {
        self.emit_load_store_pre_post(0b00, 1, 0b11, simm, 0b11, qt, xn_sp)
    }

    #[inline(always)]
    pub fn ldr_128_imm_simd_unsigned_offset(&mut self, qt: Register, xn_sp: Register, pimm: UImm12) -> Instr {
        debug_assert!(pimm <= 65520, "pimm must be in range 0 to 65520, was {}", pimm);
        debug_assert!(pimm % 16 == 0, "pimm must be multiply of 16, was {}", pimm);
        let pimm = pimm / 16;
        self.emit_load_store_offset(0b00, 1, 0b11, pimm, xn_sp, qt)
    }
}

#[cfg(test)]
mod tests {
    use bad64::decode;
    use super::*;
    use crate::mc_memory::MockMemory;
    use crate::instruction_emitter::MockEmitter;
    use crate::{stream_mock, assert_panic};
    use crate::types::InstructionPointer;

    // STRB Test Cases

    #[test]
    fn test_strb_pre_index() {
        stream_mock!(stream, {
            let instr = stream.strb_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "strb w0, [x3, #0xffffffffffffff00]!");

            let instr = stream.strb_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "strb w0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.strb_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.strb_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_strb_post_index() {
        stream_mock!(stream, {
            let instr = stream.strb_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "strb w0, [x3], #0xffffffffffffff00");

            let instr = stream.strb_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "strb w0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.strb_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.strb_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_strb_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.strb_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "strb w0, [x3]");

            let instr = stream.strb_unsigned_offset(0, 0b11111, 4095);
            assert_eq!(instr.to_string(), "strb w0, [sp, #0xfff]");

            assert_panic!("Should panic: imm out of range"; stream.strb_unsigned_offset(0, 3, 4096));
        })
    }

    // LDRB Test Cases

    #[test]
    fn test_ldrb_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldrb_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrb w0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldrb_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrb w0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldrb_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrb_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrb_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldrb_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrb w0, [x3], #0xffffffffffffff00");

            let instr = stream.ldrb_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrb w0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldrb_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrb_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrb_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldrb_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldrb w0, [x3]");

            let instr = stream.ldrb_unsigned_offset(0, 0b11111, 4095);
            assert_eq!(instr.to_string(), "ldrb w0, [sp, #0xfff]");

            assert_panic!("Should panic: imm out of range"; stream.ldrb_unsigned_offset(0, 3, 4096));
        })
    }

    // LDRSB Test Cases

    #[test]
    fn test_ldrsb_32_imm_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsb_32_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsb w0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldrsb_32_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsb w0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldrsb_32_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsb_32_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrsb_32_imm_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsb_32_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsb w0, [x3], #0xffffffffffffff00");

            let instr = stream.ldrsb_32_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsb w0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldrsb_32_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsb_32_imm_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrsb_32_imm_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldrsb_32_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldrsb w0, [x3]");

            let instr = stream.ldrsb_32_imm_unsigned_offset(0, 0b11111, 4095);
            assert_eq!(instr.to_string(), "ldrsb w0, [sp, #0xfff]");

            assert_panic!("Should panic: imm out of range"; stream.ldrsb_32_imm_unsigned_offset(0, 3, 4096));
        })
    }

    #[test]
    fn test_ldrsb_64_imm_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsb_64_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsb x0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldrsb_64_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsb x0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldrsb_64_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsb_64_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrsb_64_imm_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsb_64_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsb x0, [x3], #0xffffffffffffff00");

            let instr = stream.ldrsb_64_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsb x0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldrsb_64_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsb_64_imm_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrsb_64_imm_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldrsb_64_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldrsb x0, [x3]");

            let instr = stream.ldrsb_64_imm_unsigned_offset(0, 0b11111, 4095);
            assert_eq!(instr.to_string(), "ldrsb x0, [sp, #0xfff]");

            assert_panic!("Should panic: imm out of range"; stream.ldrsb_64_imm_unsigned_offset(0, 3, 4096));
        })
    }

    // STR Test Cases

    #[test]
    fn test_str_8_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.str_8_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str b0, [x3, #0xffffffffffffff00]!");

            let instr = stream.str_8_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str b0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.str_8_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_8_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_8_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.str_8_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str b0, [x3], #0xffffffffffffff00");

            let instr = stream.str_8_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str b0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.str_8_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_8_imm_simd_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_8_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.str_8_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "str b0, [x3]");

            let instr = stream.str_8_imm_simd_unsigned_offset(0, 0b11111, 4095);
            assert_eq!(instr.to_string(), "str b0, [sp, #0xfff]");

            assert_panic!("Should panic: imm out of range"; stream.str_8_imm_simd_unsigned_offset(0, 3, 4096));
        })
    }

    // STR 16 bit imm simd&fp

    #[test]
    fn test_str_16_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.str_16_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str h0, [x3, #0xffffffffffffff00]!");

            let instr = stream.str_16_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str h0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.str_16_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_16_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_16_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.str_16_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str h0, [x3], #0xffffffffffffff00");

            let instr = stream.str_16_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str h0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.str_16_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_16_imm_simd_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_16_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.str_16_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "str h0, [x3]");

            let instr = stream.str_16_imm_simd_unsigned_offset(0, 3, 2);
            assert_eq!(instr.to_string(), "str h0, [x3, #0x2]");

            let instr = stream.str_16_imm_simd_unsigned_offset(0, 0b11111, 8190);
            assert_eq!(instr.to_string(), "str h0, [sp, #0x1ffe]");

            assert_panic!("Should panic: imm out of range"; stream.str_16_imm_simd_unsigned_offset(0, 3, 8191));
            assert_panic!("Should panic: no multiply of 2"; stream.str_16_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    // STR 32 bit imm simd&fp

    #[test]
    fn test_str_32_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.str_32_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str s0, [x3, #0xffffffffffffff00]!");

            let instr = stream.str_32_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str s0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_32_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.str_32_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str s0, [x3], #0xffffffffffffff00");

            let instr = stream.str_32_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str s0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_simd_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_32_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.str_32_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "str s0, [x3]");

            let instr = stream.str_32_imm_simd_unsigned_offset(0, 3, 4);
            assert_eq!(instr.to_string(), "str s0, [x3, #0x4]");

            let instr = stream.str_32_imm_simd_unsigned_offset(0, 0b11111, 16380);
            assert_eq!(instr.to_string(), "str s0, [sp, #0x3ffc]");

            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_simd_unsigned_offset(0, 3, 16381));
            assert_panic!("Should panic: no multiply of 4"; stream.str_32_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    // STR 64 bit imm simd&fp

    #[test]
    fn test_str_64_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.str_64_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str d0, [x3, #0xffffffffffffff00]!");

            let instr = stream.str_64_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str d0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_64_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.str_64_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str d0, [x3], #0xffffffffffffff00");

            let instr = stream.str_64_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str d0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_simd_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_64_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.str_64_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "str d0, [x3]");

            let instr = stream.str_64_imm_simd_unsigned_offset(0, 3, 8);
            assert_eq!(instr.to_string(), "str d0, [x3, #0x8]");

            let instr = stream.str_64_imm_simd_unsigned_offset(0, 0b11111, 32760);
            assert_eq!(instr.to_string(), "str d0, [sp, #0x7ff8]");

            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_simd_unsigned_offset(0, 3, 32761));
            assert_panic!("Should panic: no multiply of 8"; stream.str_64_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    // STR 128 bit imm simd&fp

    #[test]
    fn test_str_128_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.str_128_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str q0, [x3, #0xffffffffffffff00]!");

            let instr = stream.str_128_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str q0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.str_128_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_128_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_128_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.str_128_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str q0, [x3], #0xffffffffffffff00");

            let instr = stream.str_128_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str q0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.str_128_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_128_imm_simd_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_128_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.str_128_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "str q0, [x3]");

            let instr = stream.str_128_imm_simd_unsigned_offset(0, 3, 16);
            assert_eq!(instr.to_string(), "str q0, [x3, #0x10]");

            let instr = stream.str_128_imm_simd_unsigned_offset(0, 0b11111, 65520);
            assert_eq!(instr.to_string(), "str q0, [sp, #0xfff0]");

            assert_panic!("Should panic: imm out of range"; stream.str_128_imm_simd_unsigned_offset(0, 3, 65521));
            assert_panic!("Should panic: no multiply of 8"; stream.str_128_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    // LDR Test Cases

    #[test]
    fn test_ldr_8_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_8_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr b0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldr_8_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr b0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldr_8_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_8_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_8_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_8_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr b0, [x3], #0xffffffffffffff00");

            let instr = stream.ldr_8_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr b0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldr_8_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_8_imm_simd_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_8_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_8_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldr b0, [x3]");

            let instr = stream.ldr_8_imm_simd_unsigned_offset(0, 0b11111, 4095);
            assert_eq!(instr.to_string(), "ldr b0, [sp, #0xfff]");

            assert_panic!("Should panic: imm out of range"; stream.ldr_8_imm_simd_unsigned_offset(0, 3, 4096));
        })
    }

    // LDR 16 bit imm simd&fp

    #[test]
    fn test_ldr_16_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_16_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr h0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldr_16_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr h0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldr_16_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_16_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_16_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_16_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr h0, [x3], #0xffffffffffffff00");

            let instr = stream.ldr_16_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr h0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldr_16_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_16_imm_simd_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_16_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_16_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldr h0, [x3]");

            let instr = stream.ldr_16_imm_simd_unsigned_offset(0, 3, 2);
            assert_eq!(instr.to_string(), "ldr h0, [x3, #0x2]");

            let instr = stream.ldr_16_imm_simd_unsigned_offset(0, 0b11111, 8190);
            assert_eq!(instr.to_string(), "ldr h0, [sp, #0x1ffe]");

            assert_panic!("Should panic: imm out of range"; stream.ldr_16_imm_simd_unsigned_offset(0, 3, 8191));
            assert_panic!("Should panic: no multiply of 2"; stream.ldr_16_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    // LDR 32 bit imm simd&fp

    #[test]
    fn test_ldr_32_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_32_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr s0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldr_32_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr s0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_32_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_32_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr s0, [x3], #0xffffffffffffff00");

            let instr = stream.ldr_32_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr s0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_simd_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_32_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_32_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldr s0, [x3]");

            let instr = stream.ldr_32_imm_simd_unsigned_offset(0, 3, 4);
            assert_eq!(instr.to_string(), "ldr s0, [x3, #0x4]");

            let instr = stream.ldr_32_imm_simd_unsigned_offset(0, 0b11111, 16380);
            assert_eq!(instr.to_string(), "ldr s0, [sp, #0x3ffc]");

            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_simd_unsigned_offset(0, 3, 16381));
            assert_panic!("Should panic: no multiply of 4"; stream.ldr_32_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    // LDR 64 bit imm simd&fp

    #[test]
    fn test_ldr_64_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_64_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr d0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldr_64_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr d0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_64_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_64_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr d0, [x3], #0xffffffffffffff00");

            let instr = stream.ldr_64_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr d0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_simd_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_64_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_64_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldr d0, [x3]");

            let instr = stream.ldr_64_imm_simd_unsigned_offset(0, 3, 8);
            assert_eq!(instr.to_string(), "ldr d0, [x3, #0x8]");

            let instr = stream.ldr_64_imm_simd_unsigned_offset(0, 0b11111, 32760);
            assert_eq!(instr.to_string(), "ldr d0, [sp, #0x7ff8]");

            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_simd_unsigned_offset(0, 3, 32761));
            assert_panic!("Should panic: no multiply of 8"; stream.ldr_64_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    // LDR 128 bit imm simd&fp

    #[test]
    fn test_ldr_128_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_128_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr q0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldr_128_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr q0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldr_128_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_128_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_128_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_128_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr q0, [x3], #0xffffffffffffff00");

            let instr = stream.ldr_128_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr q0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldr_128_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_128_imm_simd_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_128_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_128_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldr q0, [x3]");

            let instr = stream.ldr_128_imm_simd_unsigned_offset(0, 3, 16);
            assert_eq!(instr.to_string(), "ldr q0, [x3, #0x10]");

            let instr = stream.ldr_128_imm_simd_unsigned_offset(0, 0b11111, 65520);
            assert_eq!(instr.to_string(), "ldr q0, [sp, #0xfff0]");

            assert_panic!("Should panic: imm out of range"; stream.ldr_128_imm_simd_unsigned_offset(0, 3, 65521));
            assert_panic!("Should panic: no multiply of 8"; stream.ldr_128_imm_simd_unsigned_offset(0, 3, 1));
        })
    }
}