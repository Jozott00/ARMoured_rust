//! # Move wide (immediate)
//!
//! - MOVN 32bit
//! - MOVZ 32bit
//! - MOVK 32bit
//! - MOVN 64bit
//! - MOVZ 64bit
//! - MOVK 64bit

use bit_seq::bseq_32;
use crate::instruction_emitter::Emitter;
use crate::instruction_encoding::InstructionProcessor;

use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::{HW, Imm16, Register};

/// Internal function used to encode `MOV` instructions with immediate values.
/// Parameters:
/// * `sf`: defines whether the operation is 32-bit or 64-bit.
/// * `opc`: defines the specific `MOV` operation.
/// * `hw`: provides the shift amount.
/// * `d`: specifies the destination register.
/// * `imm`: specifies the immediate value.
#[inline(always)]
fn emit_mov_imm_x<P: InstructionProcessor<T>, T>(proc: &mut P, sf: u8, opc: u8, hw: u8, d: u8, imm: u16) -> T {
    let r = bseq_32!(sf:1 opc:2 100101 hw:2 imm:16 d:5);
    proc.emit(r)
}

pub trait MovWideImmediate<T>: InstructionProcessor<T> {
    // TODO: move somewhere else
    #[inline(always)]
    fn mov_reg(&mut self, sf: bool, dest: Register, src: Register) -> T {
        let sf_i = if sf { 1 } else { 0 };
        let r = bseq_32!(sf_i:1 0101010 0:3 src:5 0:6 !0:5 dest:5);
        self.emit(r)
    }

    // MOVN 32 and 64

    /// Generates a `MOVN` (Move Not) instruction. The `sf` parameter indicates whether it's a 32-bit (`false`) or 64-bit (`true`) operation.
    /// * `d`: destination register.
    /// * `imm`: immediate value.
    /// * `lsl`: shift amount as defined by the `HW` enum.
    #[inline(always)]
    fn movn_imm(&mut self, sf: bool, d: Register, imm: Imm16, lsl: HW) -> T {
        emit_mov_imm_x(self, sf.into(), 0b00, lsl.into(), d, imm)
    }

    /// Generates a 32-bit `MOVN` (Move Not) instruction.
    #[inline(always)]
    fn movn_32_imm(&mut self, d: Register, imm: Imm16) -> T {
        self.movn_imm(false, d, imm, 0.into())
    }

    /// Generates a 32-bit `MOVN` (Move Not) instruction.
    #[inline(always)]
    fn movn_32_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) -> T {
        self.movn_imm(false, d, imm, lsl)
    }

    /// Generates a 64-bit `MOVN` (Move Not) instruction.
    #[inline(always)]
    fn movn_64_imm(&mut self, d: Register, imm: Imm16) -> T {
        self.movn_imm(true, d, imm, 0.into())
    }

    /// Generates a 64-bit `MOVN` (Move Not) instruction.
    #[inline(always)]
    fn movn_64_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) -> T {
        self.movn_imm(true, d, imm, lsl)
    }

    // MOVZ 32 and 64

    /// Generates a `MOVZ` (Move Zero) instruction. The `sf` parameter indicates whether it's a 32-bit (`false`) or 64-bit (`true`) operation.
    #[inline(always)]
    fn movz_imm(&mut self, sf: bool, d: Register, imm: Imm16, lsl: HW) -> T {
        emit_mov_imm_x(self, sf.into(), 0b10, lsl.into(), d, imm)
    }

    /// Generates a 32-bit `MOVZ` (Move Zero) instruction.
    #[inline(always)]
    fn movz_32_imm(&mut self, d: Register, imm: Imm16) -> T {
        self.movz_imm(false, d, imm, 0.into())
    }

    /// Generates a 32-bit `MOVZ` (Move Zero) instruction.
    #[inline(always)]
    fn movz_32_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) -> T {
        self.movz_imm(false, d, imm, lsl)
    }

    /// Generates a 64-bit `MOVZ` (Move Zero) instruction.
    #[inline(always)]
    fn movz_64_imm(&mut self, d: Register, imm: Imm16) -> T {
        self.movz_imm(true, d, imm, 0.into())
    }

    /// Generates a 64-bit `MOVZ` (Move Zero) instruction.
    #[inline(always)]
    fn movz_64_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) -> T {
        self.movz_imm(true, d, imm, lsl)
    }

    // MOVK 32 and 64

    /// Generates a `MOVK` (Move Keep) instruction. The `sf` parameter indicates whether it's a 32-bit (`false`) or 64-bit (`true`) operation.
    #[inline(always)]
    fn movk_imm(&mut self, sf: bool, d: Register, imm: Imm16, lsl: HW) -> T {
        emit_mov_imm_x(self, sf.into(), 0b11, lsl.into(), d, imm)
    }

    /// Generates a 32-bit `MOVK` (Move Keep) instruction.
    #[inline(always)]
    fn movk_32_imm(&mut self, d: Register, imm: Imm16) -> T {
        self.movk_imm(false, d, imm, 0.into())
    }

    /// Generates a 32-bit `MOVK` (Move Keep) instruction.
    #[inline(always)]
    fn movk_32_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) -> T {
        self.movk_imm(false, d, imm, lsl)
    }

    /// Generates a 64-bit `MOVK` (Move Keep) instruction.
    #[inline(always)]
    fn movk_64_imm(&mut self, d: Register, imm: Imm16) -> T {
        self.movk_imm(true, d, imm, 0.into())
    }

    /// Generates a 64-bit `MOVK` (Move Keep) instruction.
    #[inline(always)]
    fn movk_64_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) -> T {
        self.movk_imm(true, d, imm, lsl)
    }
    
    /// An alias of the `MOVZ` 32 bit instruction
    #[inline(always)]
    fn mov_32_imm(&mut self, d: Register, imm: Imm16) -> T {
        self.movz_imm(false, d, imm, 0.into())
    }

    /// An alias of the `MOVZ` 32 bit instruction
    #[inline(always)]
    fn mov_32_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) -> T {
        self.movz_imm(false, d, imm, lsl)
    }


    /// An alias of the `MOVZ` 64 bit instruction
    #[inline(always)]
    fn mov_64_imm(&mut self, d: Register, imm: Imm16) -> T {
        self.movz_imm(true, d, imm, 0.into())
    }

    /// An alias of the `MOVZ` 64 bit instruction
    #[inline(always)]
    fn mov_64_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) -> T {
        self.movz_imm(true, d, imm, lsl)
    }
}

