//! # Load Register (literal)
//!
//! Implements the following instructions:

use bit_seq::{bseq_32};
use crate::instruction_emitter::Emitter;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::instruction::Instr;
use crate::types::{Imm6, Imm9, Offset32, Register, UImm12, UImm5};
use crate::types::prefetch_memory::PrfOp;

// TODO: implement with label as soon as labels supported
impl<'mem, M: Memory, E: Emitter> InstrStream<'mem, M, E> {

    // Helper function -> Actual emits

    #[inline(always)]
    fn emit_ldr_x(&mut self, opc: u8, V: u8, imm19: u32, rt: Register) -> Instr {
        let r = bseq_32!(opc:2 011 V:1 00 imm19:19 rt:5);
        self.emit(r)
    }

    #[inline(always)]
    fn emit_ldr_x_offset(&mut self, opc: u8, V: u8, offset: Offset32, rt: Register) -> Instr {
        debug_assert!(-(1 << 20) <= offset && offset < (1 << 20), "Offset must be within ±1MB");
        debug_assert!(offset % 4 == 0, "Offset must be a multiply of 4!");
        let imm19 = offset / 4;
        self.emit_ldr_x(opc, V, imm19 as u32, rt)
    }

    #[inline(always)]
    fn emit_ldr_x_addr(&mut self, opc: u8, V: u8, addr: usize, rt: Register) -> Instr {
        debug_assert!(addr % 4 == 0, "Addr must be 4 byte aligned!");

        let pc = self.emitter.instr_ptr() as usize;
        let offset_abs = pc.checked_sub(addr)
            .unwrap_or_else(|| addr.checked_sub(pc).unwrap());
        debug_assert!(offset_abs < (1 << 20), "Offset must be within ±1MB");
        let offset = if addr >= pc { offset_abs as i32 } else { -(offset_abs as i32) };

        self.emit_ldr_x_offset(opc, V, offset, rt)
    }

    // LDRSW (literal)

    pub fn ldrsw_pc_rel_from_byte_offset(&mut self, xt: Register, offset: Offset32) -> Instr {
        self.emit_ldr_x_offset(0b10, 0, offset, xt)
    }

    pub fn ldrsw_pc_rel_from_addr(&mut self, xt: Register, addr: usize) -> Instr {
        self.emit_ldr_x_addr(0b10, 0, addr, xt)
    }

    // PRFM (literal)

    pub fn prfm_pc_rel_prfop_from_byte_offset(&mut self, prfop: PrfOp, offset: Offset32) -> Instr {
        self.emit_ldr_x_offset(0b11, 0, offset, prfop.encode())
    }

    pub fn prfm_pc_rel_prfop_from_addr(&mut self, prfop: PrfOp, addr: usize) -> Instr {
        self.emit_ldr_x_addr(0b11, 0, addr, prfop.encode())
    }

    pub fn prfm_pc_rel_custom_from_byte_offset(&mut self, imm5: UImm5, offset: Offset32) -> Instr {
        debug_assert!(imm5 <= 31, "imm5 must be in range 0 to 31, was {}", imm5);
        self.emit_ldr_x_offset(0b11, 0, offset, imm5)
    }

    pub fn prfm_pc_rel_custom_from_addr(&mut self, imm5: UImm5, addr: usize) -> Instr {
        debug_assert!(imm5 <= 31, "imm5 must be in range 0 to 31, was {}", imm5);
        self.emit_ldr_x_addr(0b11, 0, addr, imm5)
    }

    // LDR (literal) instructions

    #[inline(always)]
    pub fn ldr_32_pc_rel_from_byte_offset(&mut self, wt: Register, offset: Offset32) -> Instr {
        // LDR (literal)
        self.emit_ldr_x_offset(0b00, 0, offset, wt)
    }

    #[inline(always)]
    pub fn ldr_64_pc_rel_from_byte_offset(&mut self, xt: Register, offset: Offset32) -> Instr {
        // LDR (literal)
        self.emit_ldr_x_offset(0b01, 0, offset, xt)
    }

    #[inline(always)]
    pub fn ldr_32_pc_rel_from_addr(&mut self, wt: Register, addr: usize) -> Instr {
        // LDR (literal)
        self.emit_ldr_x_addr(0b00, 0, addr, wt)
    }

    #[inline(always)]
    pub fn ldr_64_pc_rel_from_addr(&mut self, xt: Register, addr: usize) -> Instr {
        // LDR (literal)
        self.emit_ldr_x_addr(0b01, 0, addr, xt)
    }

    // LDR (literal, SIMD&FP) instructions

    #[inline(always)]
    pub fn ldr_32_simd_pc_rel_from_byte_offset(&mut self, st: Register, offset: Offset32) -> Instr {
        self.emit_ldr_x_offset(0b00, 1, offset, st)
    }

    #[inline(always)]
    pub fn ldr_64_simd_pc_rel_from_byte_offset(&mut self, dt: Register, offset: Offset32) -> Instr {
        self.emit_ldr_x_offset(0b01, 1, offset, dt)
    }

    #[inline(always)]
    pub fn ldr_128_simd_pc_rel_from_byte_offset(&mut self, qt: Register, offset: Offset32) -> Instr {
        self.emit_ldr_x_offset(0b10, 1, offset, qt)
    }

    #[inline(always)]
    pub fn ldr_32_simd_pc_rel_from_addr(&mut self, st: Register, addr: usize) -> Instr {
        // LDR (literal)
        self.emit_ldr_x_addr(0b00, 1, addr, st)
    }

    #[inline(always)]
    pub fn ldr_64_simd_pc_rel_from_addr(&mut self, dt: Register, addr: usize) -> Instr {
        // LDR (literal)
        self.emit_ldr_x_addr(0b01, 1, addr, dt)
    }

    #[inline(always)]
    pub fn ldr_128_simd_pc_rel_from_addr(&mut self, qt: Register, addr: usize) -> Instr {
        // LDR (literal)
        self.emit_ldr_x_addr(0b10, 1, addr, qt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mc_memory::MockMemory;
    use crate::instruction_emitter::MockEmitter;
    use crate::{stream_mock, assert_panic};
    use crate::types::InstructionPointer;
    use crate::types::prefetch_memory::PrfPolicy::{KEEP, STRM};
    use crate::types::prefetch_memory::PrfTarget::{L1, L2};
    use crate::types::prefetch_memory::PrfType::{PLD, PLI};

    #[test]
    fn test_ldrsw_pc_rel_from_x() {
        stream_mock!(stream, {
            let instr = stream.ldrsw_pc_rel_from_byte_offset(0, -(1 << 20));
            assert_eq!(instr.to_string(), "ldrsw x0, 0xfffffffffff00000");

            let instr = stream.ldrsw_pc_rel_from_addr(0, (1 << 20)-4);
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
            let instr = stream.prfm_pc_rel_prfop_from_addr(prfop, (1 << 20)-4);
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

            let instr = stream.ldr_64_pc_rel_from_addr(0, (1 << 20)-4);
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

            let instr = stream.ldr_64_simd_pc_rel_from_addr(0, (1 << 20)-4);
            assert_eq!(instr.to_string(), "ldr d0, 0xffffc");

            let instr = stream.ldr_128_simd_pc_rel_from_addr(0, (1 << 20)-4);
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