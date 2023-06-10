//! # Extract
//!
//! - EXTR 32bit
//! - EXTR 64bit

use bit_seq::{bseq_32, bseq_8};
use crate::instruction_emitter::Emitter;
use crate::instruction_encoding::InstructionProcessor;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::{Imm5, Imm6, Register};

/// Generates the base instruction for a bit extraction operation.
/// `sf`, `op21`, `N`, `o0`, `rm`, `imms`, `rn`, and `rd` parameters are used to construct the instruction.
/// The specifics of the instruction encoding should be verified with the ARM documentation.
#[inline(always)]
fn emit_extr_x<P: InstructionProcessor<T>, T>(proc: &mut P, sf: u8, op21: u8, N: u8, o0: u8, rm: Register, imms: u8, rn: Register, rd: Register) -> T {
    let r = bseq_32!(sf:1 op21:2 100111 N:1 o0:1 rm:5 imms:6 rn:5 rd:5);
    proc.emit(r)
}

pub trait ExtractInstructions<T>: InstructionProcessor<T> {
    /// Encodes and emits a 32-bit EXTR (extract) operation.
    ///
    /// # Panics
    ///
    /// This function asserts that the `lsb` argument (which represents the least significant
    /// bit number to start the extraction from) is in the range of 0 to 31 inclusive.
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register.
    /// * `rn` - The first source register.
    /// * `rm` - The second source register.
    /// * `lsb` - The least significant bit number where the extraction starts.
    #[inline(always)]
    fn extr_32(&mut self, rd: Register, rn: Register, rm: Register, lsb: Imm5) -> T {
        debug_assert!(0 <= lsb && lsb <= 31, "lsb must be in range 0 to 63");
        emit_extr_x(self, 0, 0b00, 0, 0, rm, bseq_8!(0 lsb:5), rn, rd)
    }

    /// Encodes and emits a 64-bit EXTR (extract) operation.
    ///
    /// # Panics
    ///
    /// This function asserts that the `lsb` argument (which represents the least significant
    /// bit number to start the extraction from) is in the range of 0 to 63 inclusive.
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register.
    /// * `rn` - The first source register.
    /// * `rm` - The second source register.
    /// * `lsb` - The least significant bit number where the extraction starts.
    #[inline(always)]
    fn extr_64(&mut self, rd: Register, rn: Register, rm: Register, lsb: Imm6) -> T {
        debug_assert!(0 <= lsb && lsb <= 63, "lsb must be in range 0 to 63");
        emit_extr_x(self, 1, 0b00, 1, 0, rm, lsb, rn, rd)
    }
}

