//! # PC-rel. addressing
//!
//! - ADR
//! - ADRP

pub use bit_seq::{bseq_32, bseq_8};
use num::Signed;
use crate::instruction_emitter::Emitter;
use crate::instruction_encoding::{AddressableInstructionProcessor, InstructionProcessor};

use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::{HW, Imm16, InstructionPointer, Offset32, Offset64, Register};
use crate::types::instruction::Instr;

/// Helper function to emit PC-relative addressing instructions.
///
/// # Arguments
///
/// * `op` - This is an operation selector. It is `0` for `ADR` and `1` for `ADRP`.
/// * `immlo` - The lower 2 bits of the immediate value.
/// * `immhi` - The higher 19 bits of the immediate value.
/// * `rd` - The destination register.
#[inline(always)]
fn emit_pc_rel_addr<P: InstructionProcessor<T>, T>(proc: &mut P, op: u8, immlo: u8, immhi: u32, rd: Register) -> T {
    let r = bseq_32!(op:1 immlo:2 10000 immhi:19 rd:5);
    proc.emit(r)
}

// TODO: Add ADR with label as soon as labels exists
pub trait PcRelAddressing<T>: InstructionProcessor<T> {
    /// Emit an `ADR` instruction.
    ///
    /// This function generates an `ADR` instruction that forms a PC-relative address
    /// using an offset in bytes.
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register.
    /// * `offset` - The PC-relative offset in bytes. It must be within the range ±1MB and multiple of 4.
    #[inline(always)]
    fn adr_from_byte_offset(&mut self, rd: Register, offset: Offset32) -> T {
        // check if offset is in range of +-1MB and a multiply of 4
        debug_assert!(-(1 << 20) <= offset && offset < (1 << 20), "Offset must be within ±1MB");
        debug_assert!(offset % 4 == 0, "Offset must be a multiply of 4!");
        let immlo = offset & 0b11;
        let immhi = offset >> 2;
        emit_pc_rel_addr(self, 0, immlo as u8, immhi as u32, rd)
    }

    /// Emit an `ADRP` instruction.
    ///
    /// This function generates an `ADRP` instruction that forms a PC-relative address
    /// to a 4KB page, using an offset in bytes.
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register.
    /// * `offset` - The PC-relative offset in bytes. It must be a multiple of 4096 and within ±4GB.
    #[inline(always)]
    fn adrp_from_byte_offset(&mut self, rd: Register, offset: Offset64) -> T {
        debug_assert!(offset % 4096 == 0, "Offset must be a multiply of 4096!");
        debug_assert!(-((1 << 30) * 4) <= offset && offset < ((1 << 30) * 4), "Offset must be within ±4GB");

        // shift 12 bits (divide by 4096)
        let offset = offset >> 12;

        let immlo = offset & 0b11;
        let immhi = offset >> 2;

        emit_pc_rel_addr(self, 1, immlo as u8, immhi as u32, rd)
    }
}

pub trait PcRelAddressingWithAddress<T>: AddressableInstructionProcessor<T> {
    /// Emit an `ADR` instruction.
    ///
    /// This function generates an `ADR` instruction that forms a PC-relative address
    /// from a given address.
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register.
    /// * `addr` - The absolute address. It must be 4-byte aligned.
    #[inline(always)]
    fn adr_from_addr(&mut self, rd: Register, addr: usize) -> T {
        debug_assert!(addr % 4 == 0, "Address must be 4 byte aligned!");

        let offset = self.intr_ptr_offset_to(addr);
        debug_assert!(-(1 << 20) <= offset && offset < (1 << 20), "Offset must be within ±1MB");

        let immlo = offset & 0b11;
        let immhi = offset >> 2;
        emit_pc_rel_addr(self, 0, immlo as u8, immhi as u32, rd)
    }
}