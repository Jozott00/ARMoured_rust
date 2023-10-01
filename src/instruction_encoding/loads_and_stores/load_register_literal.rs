//! # Load Register (literal)
//!
//! Implements the following instructions:
//! - LDRSW \<Xt>, \<imm32> - offset instead of label
//! - LDRSW \<Xt>, \<imm32> - addr instead of label

use bit_seq::bseq_32;

use crate::instruction_encoding::{AddressableInstructionProcessor, InstructionProcessor};
use crate::types::prefetch_memory::PrfOp;
use crate::types::{Offset32, Register, UImm5};

// Helper function -> Actual emits

/// Encodes and emits an instruction for Load Register(LDR) using its parameters.
#[inline(always)]
fn emit_ldr_x<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    opc: u8,
    v: u8,
    imm19: u32,
    rt: Register,
) -> T {
    let r = bseq_32!(opc:2 011 v:1 00 imm19:19 rt:5);
    proc.process(r)
}

/// Uses the provided offset to calculate the immediate for the LDR instruction and then calls emit_ldr_x to encode and emit the instruction.
#[inline(always)]
fn emit_ldr_x_offset<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    opc: u8,
    v: u8,
    offset: Offset32,
    rt: Register,
) -> T {
    debug_assert!(
        -(1 << 20) <= offset && offset < (1 << 20),
        "Offset must be within ±1MB"
    );
    debug_assert!(offset % 4 == 0, "Offset must be a multiply of 4!");
    let imm19 = offset / 4;
    emit_ldr_x(proc, opc, v, imm19 as u32, rt)
}

/// Calculates the offset from the program counter to the provided address, and then calls emit_ldr_x_offset to encode and emit the instruction.
#[inline(always)]
fn emit_ldr_x_addr<P: AddressableInstructionProcessor<T>, T>(
    proc: &mut P,
    opc: u8,
    v: u8,
    addr: usize,
    rt: Register,
) -> T {
    debug_assert!(addr % 4 == 0, "Addr must be 4 byte aligned!");

    let offset = proc.intr_ptr_offset_to(addr);
    emit_ldr_x_offset(proc, opc, v, offset, rt)
}

// TODO: implement with label as soon as labels supported
pub trait LoadRegisterLiteral<T>: InstructionProcessor<T> {
    // LDRSW (literal)

    /// Emits an LDRSW (Load Register Signed Word) instruction with a literal from the program counter with a provided offset.
    #[inline(always)]
    fn ldrsw_pc_rel_from_byte_offset(&mut self, xt: Register, offset: Offset32) -> T {
        emit_ldr_x_offset(self, 0b10, 0, offset, xt)
    }

    // PRFM (literal)

    /// Emits a PRFM (Prefetch Memory) instruction using a provided prefetch operation and an offset from the program counter.
    #[inline(always)]
    fn prfm_pc_rel_prfop_from_byte_offset(&mut self, prfop: PrfOp, offset: Offset32) -> T {
        emit_ldr_x_offset(self, 0b11, 0, offset, prfop.encode())
    }

    /// Emits a PRFM (Prefetch Memory) instruction using a custom immediate and an offset from the program counter.
    #[inline(always)]
    fn prfm_pc_rel_custom_from_byte_offset(&mut self, imm5: UImm5, offset: Offset32) -> T {
        debug_assert!(imm5 <= 31, "imm5 must be in range 0 to 31, was {}", imm5);
        emit_ldr_x_offset(self, 0b11, 0, offset, imm5)
    }

    // LDR (literal) instructions

    /// Emits an LDR (Load Register) 32-bit instruction from a provided offset via pc relative addressing.
    #[inline(always)]
    fn ldr_32_pc_rel_from_byte_offset(&mut self, wt: Register, offset: Offset32) -> T {
        // LDR (literal)
        emit_ldr_x_offset(self, 0b00, 0, offset, wt)
    }

    /// Emits an LDR (Load Register) 64-bit instruction from a provided offset via pc relative addressing.
    #[inline(always)]
    fn ldr_64_pc_rel_from_byte_offset(&mut self, xt: Register, offset: Offset32) -> T {
        // LDR (literal)
        emit_ldr_x_offset(self, 0b01, 0, offset, xt)
    }

    // LDR (literal, SIMD&FP) instructions

    #[inline(always)]
    fn ldr_32_simd_pc_rel_from_byte_offset(&mut self, st: Register, offset: Offset32) -> T {
        emit_ldr_x_offset(self, 0b00, 1, offset, st)
    }

    #[inline(always)]
    fn ldr_64_simd_pc_rel_from_byte_offset(&mut self, dt: Register, offset: Offset32) -> T {
        emit_ldr_x_offset(self, 0b01, 1, offset, dt)
    }

    #[inline(always)]
    fn ldr_128_simd_pc_rel_from_byte_offset(&mut self, qt: Register, offset: Offset32) -> T {
        emit_ldr_x_offset(self, 0b10, 1, offset, qt)
    }
}

pub trait LoadRegisterLiteralWithAddress<T>:
    LoadRegisterLiteral<T> + AddressableInstructionProcessor<T>
{
    // LDRSW (literal)

    /// Emits an LDRSW (Load Register Signed Word) instruction with a literal from the program counter to a provided address.
    #[inline(always)]
    fn ldrsw_pc_rel_from_addr(&mut self, xt: Register, addr: usize) -> T {
        emit_ldr_x_addr(self, 0b10, 0, addr, xt)
    }

    // PRFM (literal)

    /// Emits a PRFM (Prefetch Memory) instruction using a provided prefetch operation and an address.
    #[inline(always)]
    fn prfm_pc_rel_prfop_from_addr(&mut self, prfop: PrfOp, addr: usize) -> T {
        emit_ldr_x_addr(self, 0b11, 0, addr, prfop.encode())
    }

    /// Emits a PRFM (Prefetch Memory) instruction using a custom immediate and an address.
    #[inline(always)]
    fn prfm_pc_rel_custom_from_addr(&mut self, imm5: UImm5, addr: usize) -> T {
        debug_assert!(imm5 <= 31, "imm5 must be in range 0 to 31, was {}", imm5);
        emit_ldr_x_addr(self, 0b11, 0, addr, imm5)
    }

    // LDR (literal) instructions

    /// Emits an LDR (Load Register) 32-bit instruction from a provided address via pc relative addressing.
    #[inline(always)]
    fn ldr_32_pc_rel_from_addr(&mut self, wt: Register, addr: usize) -> T {
        // LDR (literal)
        emit_ldr_x_addr(self, 0b00, 0, addr, wt)
    }

    /// Emits an LDR (Load Register) 64-bit instruction from a provided address via pc relative addressing.
    #[inline(always)]
    fn ldr_64_pc_rel_from_addr(&mut self, xt: Register, addr: usize) -> T {
        // LDR (literal)
        emit_ldr_x_addr(self, 0b01, 0, addr, xt)
    }

    // LDR (literal, SIMD&FP) instructions

    #[inline(always)]
    fn ldr_32_simd_pc_rel_from_addr(&mut self, st: Register, addr: usize) -> T {
        // LDR (literal)
        emit_ldr_x_addr(self, 0b00, 1, addr, st)
    }

    #[inline(always)]
    fn ldr_64_simd_pc_rel_from_addr(&mut self, dt: Register, addr: usize) -> T {
        // LDR (literal)
        emit_ldr_x_addr(self, 0b01, 1, addr, dt)
    }

    #[inline(always)]
    fn ldr_128_simd_pc_rel_from_addr(&mut self, qt: Register, addr: usize) -> T {
        // LDR (literal)
        emit_ldr_x_addr(self, 0b10, 1, addr, qt)
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction_emitter::MockEmitter;
    use crate::instruction_stream::InstrStream;
    use crate::mc_memory::MockMemory;
    use crate::types::prefetch_memory::PrfPolicy::{KEEP, STRM};
    use crate::types::prefetch_memory::PrfTarget::{L1, L2};
    use crate::types::prefetch_memory::PrfType::{PLD, PLI};
    use crate::types::InstructionPointer;
    use crate::{assert_panic, stream_mock};

    use super::*;

    #[test]
    fn test_ldrsw_pc_rel_from_x() {
        stream_mock!(stream, {
            let instr = stream.ldrsw_pc_rel_from_byte_offset(0, -(1 << 20));
            assert_eq!(instr.to_string(), "ldrsw x0, 0xfffffffffff00000");

            let instr = stream.ldrsw_pc_rel_from_addr(0, (1 << 20) - 4);
            assert_eq!(instr.to_string(), "ldrsw x0, 0xffffc");
        })
    }

    #[test]
    fn test_prfop_pc_rel_from_x() {
        stream_mock!(stream, {
            let prfop = PrfOp(PLD, L1, KEEP);
            let instr = stream.prfm_pc_rel_prfop_from_byte_offset(prfop, -(1 << 20));
            assert_eq!(instr.to_string(), "prfm pldl1keep, 0xfffffffffff00000");

            let prfop = PrfOp(PLI, L2, STRM);
            let instr = stream.prfm_pc_rel_prfop_from_addr(prfop, (1 << 20) - 4);
            assert_eq!(instr.to_string(), "prfm plil2strm, 0xffffc");
        })
    }

    #[test]
    fn test_ldr_32_pc_rel_from_byte_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_32_pc_rel_from_byte_offset(0, (1 << 20) - 4);
            assert_eq!(instr.to_string(), "ldr w0, 0xffffc");

            let instr = stream.ldr_32_pc_rel_from_byte_offset(0, -(1 << 20));
            assert_eq!(instr.to_string(), "ldr w0, 0xfffffffffff00000");

            assert_panic!("Should panic: offset out of bounds"; {
                stream.ldr_32_pc_rel_from_byte_offset(0, 1 << 20);
            });

            assert_panic!("Should panic: offset out of bounds"; {
                stream.ldr_32_pc_rel_from_byte_offset(0, -(1 << 20) - 1);
            });

            assert_panic!("Should panic: offset not multiply"; {
                stream.ldr_32_pc_rel_from_byte_offset(0, 1);
            });
        })
    }

    #[test]
    fn test_ldr_64_pc_rel_from_byte_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_64_pc_rel_from_byte_offset(0, (1 << 20) - 4);
            assert_eq!(instr.to_string(), "ldr x0, 0xffffc");

            let instr = stream.ldr_64_pc_rel_from_byte_offset(0, -(1 << 20));
            assert_eq!(instr.to_string(), "ldr x0, 0xfffffffffff00000");

            assert_panic!("Should panic: offset out of bounds"; {
                stream.ldr_64_pc_rel_from_byte_offset(0, 1 << 20);
            });

            assert_panic!("Should panic: offset out of bounds"; {
                stream.ldr_64_pc_rel_from_byte_offset(0, -(1 << 20) - 1);
            });

            assert_panic!("Should panic: offset not multiply"; {
                stream.ldr_64_pc_rel_from_byte_offset(0, 1);
            });
        })
    }

    #[test]
    fn test_ldr_x_pc_rel_from_addr() {
        stream_mock!(stream, {
            let instr = stream.ldr_32_pc_rel_from_addr(0, 0x20);
            assert_eq!(instr.to_string(), "ldr w0, 0x20");

            let instr = stream.ldr_64_pc_rel_from_addr(0, (1 << 20) - 4);
            assert_eq!(instr.to_string(), "ldr x0, 0xffffc");

            assert_panic!("Should panic: addr not aligned"; {
                stream.ldr_32_pc_rel_from_addr(0, 1);
            });

            assert_panic!("Should panic: offset not withing 1MB"; {
                stream.ldr_32_pc_rel_from_addr(0, 1 << 20);
            });
        })
    }

    #[test]
    fn test_ldr_x_simd_pc_rel_from_byte_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_32_simd_pc_rel_from_byte_offset(0, (1 << 20) - 4);
            assert_eq!(instr.to_string(), "ldr s0, 0xffffc");

            let instr = stream.ldr_64_simd_pc_rel_from_byte_offset(0, -(1 << 20));
            assert_eq!(instr.to_string(), "ldr d0, 0xfffffffffff00000");

            let instr = stream.ldr_128_simd_pc_rel_from_byte_offset(0, -(1 << 20));
            assert_eq!(instr.to_string(), "ldr q0, 0xfffffffffff00000");
        })
    }

    #[test]
    fn test_ldr_x_simd_pc_rel_from_addr() {
        stream_mock!(stream, {
            let instr = stream.ldr_32_simd_pc_rel_from_addr(0, 0x20);
            assert_eq!(instr.to_string(), "ldr s0, 0x20");

            let instr = stream.ldr_64_simd_pc_rel_from_addr(0, (1 << 20) - 4);
            assert_eq!(instr.to_string(), "ldr d0, 0xffffc");

            let instr = stream.ldr_128_simd_pc_rel_from_addr(0, (1 << 20) - 4);
            assert_eq!(instr.to_string(), "ldr q0, 0xffffc");

            assert_panic!("Should panic: addr not aligned"; {
                stream.ldr_32_simd_pc_rel_from_addr(0, 1);
            });

            assert_panic!("Should panic: offset not withing 1MB"; {
                stream.ldr_32_simd_pc_rel_from_addr(0, 1 << 20);
            });
        })
    }
}
