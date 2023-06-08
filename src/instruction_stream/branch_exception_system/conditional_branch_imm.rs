//! # Conditional branch (immediate)
//!
//! Implements the following instructions:
//! - [B.cond](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/B-cond--Branch-conditionally-?lang=en)
//! - [BC.cond](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BC-cond--Branch-Consistent-conditionally-?lang=en)

use bit_seq::{bseq_16, bseq_32};
use crate::instruction_emitter::Emitter;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::instruction::Instr;
use crate::types::{Imm19, Imm6, Imm9, Offset32, Register, UImm1, UImm12, UImm4, UImm5};
use crate::types::condition::Condition;
use crate::types::encodable::Encodable;
use crate::types::prefetch_memory::PrfOp;

// TODO: Implement with label as soon as labels are supported
impl<'mem, M: Memory, E: Emitter> InstrStream<'mem, M, E> {
    #[inline(always)]
    fn emit_cond_branch_imm(&mut self, o1: UImm1, imm19: Imm19, o0: UImm1, cond: UImm4) -> Instr {
        let i = bseq_32!(0101010 o1:1 imm19:19 o0:1 cond:4 );
        self.emit(i)
    }

    #[inline(always)]
    fn emit_cond_branch_x_offset(&mut self, o1: UImm1, offset: Offset32, o0: UImm1, cond: UImm4) -> Instr {
        debug_assert!(-(1 << 20) <= offset && offset < (1 << 20), "Offset must be within ±1MB");
        debug_assert!(offset % 4 == 0, "Offset must be a multiply of 4!");
        let imm19 = offset / 4;
        self.emit_cond_branch_imm(o1, imm19, o0, cond)
    }

    #[inline(always)]
    fn emit_cond_branch_x_addr(&mut self, o1: UImm1, addr: usize, o0: UImm1, cond: UImm4) -> Instr {
        debug_assert!(addr % 4 == 0, "Addr must be 4 byte aligned!");
        let offset = self.intr_ptr_offset_to(addr);
        self.emit_cond_branch_x_offset(o1, offset, o0, cond)
    }

    // B.cond instruction

    /// [B.cond](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/B-cond--Branch-conditionally-?lang=en) instruction
    ///
    /// Uses specified `offset` instead of label.
    #[inline(always)]
    pub fn b_cond_from_byte_offset(&mut self, cond: Condition, offset: Offset32) -> Instr {
        self.emit_cond_branch_x_offset(0, offset, 0, cond.encode())
    }

    /// [B.cond](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/B-cond--Branch-conditionally-?lang=en) instruction
    ///
    /// Uses specified `addr` instead of label.
    #[inline(always)]
    pub fn b_cond_to_addr(&mut self, cond: Condition, addr: usize) -> Instr {
        self.emit_cond_branch_x_addr(0, addr, 0, cond.encode())
    }

    // BC.cond instruction

    /// [BC.cond](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BC-cond--Branch-Consistent-conditionally-?lang=en) instruction
    ///
    /// Uses specified `offset` instead of label.
    ///
    /// **Note**: FEAT_HBC required
    #[inline(always)]
    pub fn bc_cond_from_byte_offset(&mut self, cond: Condition, offset: Offset32) -> Instr {
        self.emit_cond_branch_x_offset(0, offset, 1, cond.encode())
    }

    /// [BC.cond](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BC-cond--Branch-Consistent-conditionally-?lang=en) instruction
    ///
    /// Uses specified `addr` instead of label.
    ///
    /// **Note**: FEAT_HBC required
    #[inline(always)]
    pub fn bc_cond_to_addr(&mut self, cond: Condition, addr: usize) -> Instr {
        self.emit_cond_branch_x_addr(0, addr, 1, cond.encode())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mc_memory::MockMemory;
    use crate::instruction_emitter::MockEmitter;
    use crate::{stream_mock, assert_panic};
    use crate::types::InstructionPointer;

    #[test]
    fn test_b_cond_x() {
        stream_mock!(stream, {
            let instr = stream.b_cond_from_byte_offset(Condition::EQ, -(1 << 20));
            assert_eq!(instr.to_string(), "b.eq 0xfffffffffff00000");

            let instr = stream.b_cond_to_addr(Condition::CC, 0x20);
            assert_eq!(instr.to_string(), "b.lo 0x20");

            assert_panic!("Should panic: offset out of bounds"; {
                stream.b_cond_from_byte_offset(Condition::EQ, 1 << 20);
            });

            assert_panic!("Should panic: offset out of bounds"; {
                stream.b_cond_from_byte_offset(Condition::EQ, -(1 << 20) - 1);
            });

            assert_panic!("Should panic: offset not multiply"; {
                stream.b_cond_from_byte_offset(Condition::EQ, 1);
            });

              assert_panic!("Should panic: addr not aligned"; {
                stream.b_cond_to_addr(Condition::CC, 1);
            });

            assert_panic!("Should panic: offset not withing 1MB"; {
                stream.b_cond_to_addr(Condition::CC, 1 << 20);
            });

        })
    }

    #[test]
    fn test_bc_cond_x() {
        stream_mock!(stream, {
            // cannot test positive as disassembler does not support FEAT_HBC feature

            assert_panic!("Should panic: offset out of bounds"; {
                stream.bc_cond_from_byte_offset(Condition::EQ, 1 << 20);
            });

            assert_panic!("Should panic: offset out of bounds"; {
                stream.bc_cond_from_byte_offset(Condition::EQ, -(1 << 20) - 1);
            });

            assert_panic!("Should panic: offset not multiply"; {
                stream.bc_cond_from_byte_offset(Condition::EQ, 1);
            });

              assert_panic!("Should panic: addr not aligned"; {
                stream.bc_cond_to_addr(Condition::CC, 1);
            });

            assert_panic!("Should panic: offset not withing 1MB"; {
                stream.bc_cond_to_addr(Condition::CC, 1 << 20);
            });

        })
    }
}