//! # Exception Generation
//!
//! Implements the following instructions:
//! - [CLREX](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLREX--Clear-Exclusive-?lang=en)
//! - [DSB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DSB--Data-Synchronization-Barrier-?lang=en)
//! - [DMB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DMB--Data-Memory-Barrier-?lang=en)
//! - [ISB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ISB--Instruction-Synchronization-Barrier-?lang=en)
//! - [SB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SB--Speculation-Barrier-?lang=en)
//! - [DSB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DSB--Data-Synchronization-Barrier-?lang=en)
//! - [PSSBB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PSSBB--Physical-Speculative-Store-Bypass-Barrier--an-alias-of-DSB-?lang=en)
//! - [SSBB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SSBB--Speculative-Store-Bypass-Barrier--an-alias-of-DSB-?lang=en)

use bit_seq::{bseq_32, bseq_8};
use crate::instruction_emitter::Emitter;
use crate::instruction_encoding::InstructionProcessor;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::instruction::Instr;
use crate::types::{Register, UImm4};
use crate::types::encodable::Encodable;
use crate::types::mem_barrier_option::{MemBarrierOpt, MemNXSBarrierOpt};

#[inline(always)]
fn emit_barrier_x<P: InstructionProcessor<T>, T>(proc: &mut P, crm: u8, op2: u8, rt: Register) -> T {
    let i = bseq_32!(11010101 0:6 110011 crm:4 op2:3 rt:5);
    proc.emit(i)
}

/// # Exception Generation
///
/// Implements the following instructions:
/// - [CLREX](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLREX--Clear-Exclusive-?lang=en)
/// - [DSB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DSB--Data-Synchronization-Barrier-?lang=en)
/// - [DMB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DMB--Data-Memory-Barrier-?lang=en)
/// - [ISB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ISB--Instruction-Synchronization-Barrier-?lang=en)
/// - [SB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SB--Speculation-Barrier-?lang=en)
/// - [DSB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DSB--Data-Synchronization-Barrier-?lang=en)
/// - [PSSBB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PSSBB--Physical-Speculative-Store-Bypass-Barrier--an-alias-of-DSB-?lang=en)
/// - [SSBB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SSBB--Speculative-Store-Bypass-Barrier--an-alias-of-DSB-?lang=en)
pub trait Barriers<T>: InstructionProcessor<T> {
    /// [CLREX](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLREX--Clear-Exclusive-?lang=en)
    ///
    /// `CLREX #<imm>`
    fn clrex(&mut self, imm: UImm4) -> T {
        emit_barrier_x(self, imm, 0b010, 0b11111)
    }

    /// [DSB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DSB--Data-Synchronization-Barrier-?lang=en)
    ///
    /// `DSB <option>`
    fn dsb_mem_barrier_option(&mut self, option: MemBarrierOpt) -> T {
        emit_barrier_x(self, option.encode(), 0b100, 0b11111)
    }

    /// [DSB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DSB--Data-Synchronization-Barrier-?lang=en)
    ///
    /// `DSB #<imm>`
    fn dsb_mem_barrier_imm(&mut self, imm: UImm4) -> T {
        emit_barrier_x(self, imm, 0b100, 0b11111)
    }

    /// [DMB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DMB--Data-Memory-Barrier-?lang=en)
    ///
    /// `DMB <option>`
    fn dmb_option(&mut self, option: MemBarrierOpt) -> T {
        emit_barrier_x(self, option.encode(), 0b101, 0b11111)
    }

    /// [DMB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DMB--Data-Memory-Barrier-?lang=en)
    ///
    /// `DMB #<imm>`
    fn dmb_imm(&mut self, imm: UImm4) -> T {
        emit_barrier_x(self, imm, 0b101, 0b11111)
    }

    /// [ISB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ISB--Instruction-Synchronization-Barrier-?lang=en)
    ///
    /// `ISB sy`
    fn isb_sy(&mut self) -> T {
        emit_barrier_x(self, MemBarrierOpt::SY.encode(), 0b110, 0b11111)
    }

    /// [ISB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ISB--Instruction-Synchronization-Barrier-?lang=en)
    ///
    /// `ISB #<imm>`
    fn isb_imm(&mut self, imm: UImm4) -> T {
        emit_barrier_x(self, imm, 0b110, 0b11111)
    }

    /// [SB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SB--Speculation-Barrier-?lang=en)
    ///
    /// `SB`
    fn sb(&mut self) -> T {
        emit_barrier_x(self, 0, 0b111, 0b11111)
    }

    /// [DSB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DSB--Data-Synchronization-Barrier-?lang=en)
    ///
    /// `DSB <option>nXS`
    fn dsb_mem_nxs_barrier_option(&mut self, option: MemNXSBarrierOpt) -> T {
        let option = option.encode();
        emit_barrier_x(self, bseq_8!(option:2 10), 0b001, 0b11111)
    }


    /// [DSB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DSB--Data-Synchronization-Barrier-?lang=en)
    ///
    /// `DSB nXS<#imm>`
    fn dsb_mem_nxs_barrier_imm(&mut self, imm: UImm4) -> T {
        debug_assert!([16, 20, 24, 28].contains(&imm), "imm must be one of 16, 20, 24, 28, was {}", imm);
        let imm = imm >> 2;
        emit_barrier_x(self, bseq_8!(imm:2 10), 0b001, 0b11111)
    }

    // aliases

    /// [PSSBB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PSSBB--Physical-Speculative-Store-Bypass-Barrier--an-alias-of-DSB-?lang=en)
    ///
    /// `PSSBB`
    fn pssbb(&mut self) -> T {
        self.dsb_mem_barrier_imm(0b0100)
    }

    /// [SSBB](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SSBB--Speculative-Store-Bypass-Barrier--an-alias-of-DSB-?lang=en)
    ///
    /// `SSBB`
    fn ssbb(&mut self) -> T {
        self.dsb_mem_barrier_imm(0b0000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mc_memory::MockMemory;
    use crate::instruction_emitter::MockEmitter;
    use crate::{assert_panic, stream_mock};
    use crate::types::InstructionPointer;

    #[test]
    fn test_clrex() {
        stream_mock!(stream, {
            let instr = stream.clrex(0x1);
            assert_eq!(instr.to_string(), "clrex #0x1");
        })
    }

    #[test]
    fn test_dsb_mem_barrier_x() {
        stream_mock!(stream, {
            let instr = stream.dsb_mem_barrier_option(MemBarrierOpt::LD);
            assert_eq!(instr.to_string(), "dsb ld");

            let instr = stream.dsb_mem_barrier_imm(0b1011);
            assert_eq!(instr.to_string(), "dsb ish");
        })
    }

    #[test]
    fn test_dmb_x() {
        stream_mock!(stream, {
            let instr = stream.dmb_option(MemBarrierOpt::LD);
            assert_eq!(instr.to_string(), "dmb ld");

            let instr = stream.dmb_imm(0b1011);
            assert_eq!(instr.to_string(), "dmb ish");
        })
    }

    #[test]
    fn test_isb_x() {
        stream_mock!(stream, {
            let instr = stream.isb_sy();
            assert_eq!(instr.to_string(), "isb");

            let instr = stream.isb_imm(0x4);
            assert_eq!(instr.to_string(), "isb #0x4");
        })
    }

    #[test]
    fn test_sb() {
        stream_mock!(stream, {
            let instr = stream.sb();
            assert_eq!(instr.to_string(), "sb");
        })
    }

    #[test]
    fn test_dsb_mem_nxs_barrier_x() {
        stream_mock!(stream, {
            let instr = stream.dsb_mem_nxs_barrier_option(MemNXSBarrierOpt::SY);
            assert_eq!(instr.to_string(), "dsb synXS");

            let instr = stream.dsb_mem_nxs_barrier_imm(20);
            assert_eq!(instr.to_string(), "dsb nshnXS");

            assert_panic!("Should panic: non allowed value"; stream.dsb_mem_nxs_barrier_imm(5));
        })
    }

    #[test]
    fn test_pssbb() {
        stream_mock!(stream, {
            let instr = stream.pssbb();
            assert_eq!(instr.to_string(), "pssbb");
        })
    }

    #[test]
    fn test_ssbb() {
        stream_mock!(stream, {
            let instr = stream.ssbb();
            assert_eq!(instr.to_string(), "ssbb");
        })
    }
}