//! # Conditional branch (immediate)
//!
//! Implements the following instructions:
//! - [B.cond](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/B-cond--Branch-conditionally-?lang=en)
//! - [BC.cond](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BC-cond--Branch-Consistent-conditionally-?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::{AddressableInstructionProcessor, InstructionProcessor};
use crate::types::{Imm19, Offset32, UImm1, UImm4};
use crate::types::condition::Condition;
use crate::types::encodable::Encodable;

#[inline(always)]
fn emit_cond_branch_imm<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    o1: UImm1,
    imm19: Imm19,
    o0: UImm1,
    cond: UImm4,
) -> T {
    let i = bseq_32!(0101010 o1:1 imm19:19 o0:1 cond:4 );
    proc.process(i)
}

#[inline(always)]
fn emit_cond_branch_x_offset<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    o1: UImm1,
    offset: Offset32,
    o0: UImm1,
    cond: UImm4,
) -> T {
    debug_assert!(
        -(1 << 20) <= offset && offset < (1 << 20),
        "Offset must be within ±1MB"
    );
    debug_assert!(offset % 4 == 0, "Offset must be a multiply of 4!");
    let imm19 = offset / 4;
    emit_cond_branch_imm(proc, o1, imm19, o0, cond)
}

#[inline(always)]
fn emit_cond_branch_x_addr<P: AddressableInstructionProcessor<T>, T>(
    proc: &mut P,
    o1: UImm1,
    addr: usize,
    o0: UImm1,
    cond: UImm4,
) -> T {
    debug_assert!(addr % 4 == 0, "Addr must be 4 byte aligned!");
    let offset = proc.intr_ptr_offset_to(addr);
    emit_cond_branch_x_offset(proc, o1, offset, o0, cond)
}

// TODO: Implement with label as soon as labels are supported
pub trait ConditionalBranchImmediate<T>: InstructionProcessor<T> {
    // B.cond instruction

    /// [B.cond](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/B-cond--Branch-conditionally-?lang=en) instruction
    ///
    /// Uses specified `offset` instead of label.
    #[inline(always)]
    fn b_cond_from_byte_offset(&mut self, cond: Condition, offset: Offset32) -> T {
        emit_cond_branch_x_offset(self, 0, offset, 0, cond.encode())
    }

    // BC.cond instruction

    /// [BC.cond](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BC-cond--Branch-Consistent-conditionally-?lang=en) instruction
    ///
    /// ```asm
    /// BC.cond <imm>
    /// ```
    ///
    /// Uses specified `offset` instead of label.
    ///
    /// **Note**: FEAT_HBC required
    #[inline(always)]
    fn bc_cond_from_byte_offset(&mut self, cond: Condition, offset: Offset32) -> T {
        emit_cond_branch_x_offset(self, 0, offset, 1, cond.encode())
    }
}

pub trait ConditionalBranchImmediateWithAddress<T>:
    AddressableInstructionProcessor<T> + ConditionalBranchImmediate<T>
{
    /// [B.cond](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/B-cond--Branch-conditionally-?lang=en) instruction
    ///
    /// Uses specified `addr` instead of label.
    #[inline(always)]
    fn b_cond_to_addr(&mut self, cond: Condition, addr: usize) -> T {
        emit_cond_branch_x_addr(self, 0, addr, 0, cond.encode())
    }

    /// [BC.cond](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BC-cond--Branch-Consistent-conditionally-?lang=en) instruction
    ///
    /// Uses specified `addr` instead of label.
    ///
    /// **Note**: FEAT_HBC required
    #[inline(always)]
    fn bc_cond_to_addr(&mut self, cond: Condition, addr: usize) -> T {
        emit_cond_branch_x_addr(self, 0, addr, 1, cond.encode())
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_panic, stream_mock};
    use crate::instruction_emitter::MockEmitter;
    use crate::instruction_stream::InstrStream;
    use crate::mc_memory::MockMemory;
    use crate::types::InstructionPointer;

    use super::*;

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
