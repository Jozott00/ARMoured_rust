//! # Unconditional branch (immediate)
//!
//! Implements the following instructions:
//! - [B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/B--Branch-?lang=en)
//! - [BL](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BL--Branch-with-Link-?lang=en)

use bit_seq::bseq_32;
use crate::instruction_encoding::{AddressableInstructionProcessor, InstructionProcessor};
use crate::types::Offset32;
use crate::types::Offset64;

#[inline(always)]
fn emit_uncond_br_imm<P: InstructionProcessor<T>, T>(proc: &mut P, op: u8, imm26: u32) -> T {
    let i = bseq_32!(op:1 00101 imm26:26);
    proc.process(i)
}

#[inline(always)]
fn emit_uncond_br_imm_offset<P: InstructionProcessor<T>, T>(proc: &mut P, op: u8, offset: Offset32) -> T {
    debug_assert!(-(128 << 20) <= offset && offset < (128 << 20), "Offset must be within ±1MB");
    debug_assert!(offset % 4 == 0, "Offset must be a multiply of 4!");
    let offset = offset / 4;
    emit_uncond_br_imm(proc, op, offset as u32)
}


/// Implements the unconditional branch (immediate) instruction.
///
/// Does only support branching by specifying the offset.
/// The [`UnconditionalBranchImmediateWithAddress`] trait also allows
/// branching by specifying the address.
pub trait UnconditionalBranchImmediate<T>: InstructionProcessor<T> {
    /// [B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/B--Branch-?lang=en)
    ///
    /// ```asm
    /// B <imm>
    /// ```
    #[inline(always)]
    fn b_from_byte_offset(&mut self, offset: Offset32) -> T {
        emit_uncond_br_imm_offset(self, 0, offset)
    }

    /// [BL](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BL--Branch-with-Link-?lang=en)
    ///
    /// ```asm
    /// BL <imm>
    /// ```
    #[inline(always)]
    fn bl_from_byte_offset(&mut self, offset: Offset32) -> T {
        emit_uncond_br_imm_offset(self, 1, offset)
    }
}

/// Implements the unconditional branch (immediate) instruction.
///
/// As [`UnconditionalBranchImmediate`] but also allows to
/// specify the addr you want to branch to, instead of calculating the offset
/// by yourself.
pub trait UnconditionalBranchImmediateWithAddress<T>: UnconditionalBranchImmediate<T> + AddressableInstructionProcessor<T> {
    /// [B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/B--Branch-?lang=en)
    ///
    /// Not real assembly:
    /// ```asm
    /// B <addr>
    /// ```
    #[inline(always)]
    fn b_to_addr(&mut self, addr: usize) -> T {
        debug_assert!(addr % 4 == 0, "addr must be 4 byte aligned!");
        let offset = self.intr_ptr_offset_to(addr);
        emit_uncond_br_imm_offset(self, 0, offset)
    }

    /// [BL](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BL--Branch-with-Link-?lang=en)
    ///
    /// Not real assembly:
    /// ```asm
    /// BL <addr>
    /// ```
    #[inline(always)]
    fn bl_to_addr(&mut self, addr: usize) -> T {
        debug_assert!(addr % 4 == 0, "addr must be 4 byte aligned!");
        let offset = self.intr_ptr_offset_to(addr);
        emit_uncond_br_imm_offset(self, 1, offset)
    }
}


#[cfg(test)]
mod tests {
    use crate::instruction_producer::InstrProducer;
    use super::*;
    use crate::mc_memory::MockMemory;
    use crate::instruction_emitter::MockEmitter;
    use crate::{stream_mock, assert_panic};
    use crate::types::InstructionPointer;
    use crate::instruction_stream::InstrStream;

    #[test]
    fn test_b_from_byte_offset() {
        let mut prod = InstrProducer::new();

        let instr = prod.b_from_byte_offset(-(128 << 20));
        assert_eq!(instr.to_string(), "b 0xfffffffff8000000");

        let instr = prod.b_from_byte_offset((128 << 20) - 4);
        assert_eq!(instr.to_string(), "b 0x7fffffc");

        assert_panic!("Should panic: offset out of bounds"; {
                prod.b_from_byte_offset(-(1 << 20) - 1);
            });

        assert_panic!("Should panic: offset not multiply"; {
                prod.b_from_byte_offset(1);
            });
    }

    #[test]
    fn test_l_from_byte_offset() {
        let mut prod = InstrProducer::new();

        let instr = prod.bl_from_byte_offset(-(128 << 20));
        assert_eq!(instr.to_string(), "bl 0xfffffffff8000000");

        let instr = prod.bl_from_byte_offset((128 << 20) - 4);
        assert_eq!(instr.to_string(), "bl 0x7fffffc");

        assert_panic!("Should panic: offset out of bounds"; {
                prod.bl_from_byte_offset(-(1 << 20) - 1);
            });

        assert_panic!("Should panic: offset not multiply"; {
                prod.bl_from_byte_offset(1);
            });
    }

    #[test]
    fn test_b_to_addr() {
        stream_mock!(stream, {
            let instr = stream.b_to_addr(0x20);
            assert_eq!(instr.to_string(), "b 0x20");

            assert_panic!("Should panic: addr not aligned"; {
                stream.b_to_addr(1);
            });

            assert_panic!("Should panic: offset not withing 1MB"; {
                stream.b_to_addr(128 << 20);
            });

        })
    }

    #[test]
    fn test_bl_to_addr() {
        stream_mock!(stream, {
            let instr = stream.bl_to_addr(0x20);
            assert_eq!(instr.to_string(), "bl 0x20");

            assert_panic!("Should panic: addr not aligned"; {
                stream.bl_to_addr(1);
            });

            assert_panic!("Should panic: offset not withing 1MB"; {
                stream.bl_to_addr(128 << 20);
            });

        })
    }
}