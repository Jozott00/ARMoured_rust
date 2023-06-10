//! # Bitfield
//!
//! - SBFM 32bit
//! - BFM 32bit
//! - UBFM 32bit
//! - SBFM 64bit
//! - BFM 64bit
//! - UBFM 64bit

use bit_seq::bseq_32;
use crate::instruction_emitter::Emitter;
use crate::instruction_encoding::InstructionProcessor;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::{Imm6, Register};


/// Generates the base instruction for a bitfield operation.
/// `sf`, `opc`, `N`, `immr`, `imms`, `rn`, and `rd` parameters are used to construct the instruction.
/// Note that the details of the instruction encoding should be checked with the ARM documentation.
#[inline(always)]
fn emit_bitfield<P: InstructionProcessor<T>, T>(proc: &mut P, sf: u8, opc: u8, N: u8, immr: Imm6, imms: Imm6, rn: Register, rd: Register) -> T {
    if sf == 1 {
        debug_assert!(0 <= immr && immr <= 63, "Immr can only be in range of 0 to 63");
        debug_assert!(0 <= immr && immr <= 63, "Immr can only be in range of 0 to 63");
    } else {
        debug_assert!(0 <= immr && immr <= 31, "Immr can only be in range of 0 to 31");
        debug_assert!(0 <= immr && immr <= 31, "Immr can only be in range of 0 to 31");
    }

    let r = bseq_32!(sf:1 opc:2 100110 N:1 immr:6 imms:6 rn:5 rd:5);
    proc.emit(r)
}

pub trait BitfieldInstructions<T>: InstructionProcessor<T> {
    /// Generates a signed bitfield move (SBFM) instruction for 32-bit registers.
    /// SBFM extracts a bitfield from the source register, sign extends it, and writes it to the destination register.
    /// The `rd`, `rn`, `immr` and `imms` parameters represent the destination register, source register, rotate amount and the width of the bitfield respectively.
    #[inline(always)]
    fn sbfm_32(&mut self, rd: Register, rn: Register, immr: Imm6, imms: Imm6) -> T {
        emit_bitfield(self, 0, 0b00, 0, immr, imms, rn, rd)
    }

    /// Generates a signed bitfield move (SBFM) instruction for 64-bit registers.
    /// The parameters are the same as [`sbfm_32`](#method.sbfm_32) but this operates on 64-bit registers.
    #[inline(always)]
    fn sbfm_64(&mut self, rd: Register, rn: Register, immr: Imm6, imms: Imm6) -> T {
        emit_bitfield(self, 1, 0b00, 1, immr, imms, rn, rd)
    }

    /// Generates a bitfield move (BFM) instruction for 32-bit registers.
    /// BFM extracts a bitfield from the source register, and writes it to the destination register without sign extension.
    /// The `rd`, `rn`, `immr` and `imms` parameters represent the destination register, source register, rotate amount and the width of the bitfield respectively.
    #[inline(always)]
    fn bfm_32(&mut self, rd: Register, rn: Register, immr: Imm6, imms: Imm6) -> T {
        emit_bitfield(self, 0, 0b01, 0, immr, imms, rn, rd)
    }

    /// Generates a bitfield move (BFM) instruction for 64-bit registers.
    /// The parameters are the same as [`bfm_32`](#method.bfm_32) but this operates on 64-bit registers.
    #[inline(always)]
    fn bfm_64(&mut self, rd: Register, rn: Register, immr: Imm6, imms: Imm6) -> T {
        emit_bitfield(self, 1, 0b01, 1, immr, imms, rn, rd)
    }

    /// Generates an unsigned bitfield move (UBFM) instruction for 32-bit registers.
    /// UBFM extracts a bitfield from the source register, zero extends it, and writes it to the destination register.
    /// The `rd`, `rn`, `immr` and `imms` parameters represent the destination register, source register, rotate amount and the width of the bitfield respectively.
    #[inline(always)]
    fn ubfm_32(&mut self, rd: Register, rn: Register, immr: Imm6, imms: Imm6) -> T {
        emit_bitfield(self, 0, 0b10, 0, immr, imms, rn, rd)
    }

    /// Generates an unsigned bitfield move (UBFM) instruction for 64-bit registers.
    /// The parameters are the same as [`ubfm_32`](#method.ubfm_32) but this operates on 64-bit registers.
    #[inline(always)]
    fn ubfm_64(&mut self, rd: Register, rn: Register, immr: Imm6, imms: Imm6) -> T {
        emit_bitfield(self, 1, 0b10, 1, immr, imms, rn, rd)
    }
}

