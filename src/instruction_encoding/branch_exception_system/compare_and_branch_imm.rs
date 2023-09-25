//! # [Compare and branch (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Branches--Exception-Generating-and-System-instructions?lang=en#compbranch)
//!
//! Implements the following instructions:
//!  - [CBZ - Compare and Branch on Zero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CBZ--Compare-and-Branch-on-Zero-?lang=en)
//!  - [CBNZ - Compare and Branch on Nonzero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CBNZ--Compare-and-Branch-on-Nonzero-?lang=en)

use crate::instruction_encoding::{AddressableInstructionProcessor, InstructionProcessor};
use crate::types::{Imm19, Offset32, Register, UImm1, UImm19, UImm4};
use bit_seq::bseq_32;

#[inline(always)]
fn emit_cmp_branch_imm<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: UImm1,
    op: UImm1,
    imm19: Imm19,
    rt: Register,
) -> T {
    let i = bseq_32!(sf:1 011010 op:1 imm19:19 rt:5);
    proc.process(i)
}

#[inline(always)]
fn emit_cmp_branch_x_offset<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: UImm1,
    op: UImm1,
    offset: Offset32,
    rt: Register,
) -> T {
    debug_assert!(
        -(1 << 20) <= offset && offset < (1 << 20),
        "Offset must be within ±1MB"
    );
    debug_assert!(offset % 4 == 0, "Offset must be a multiply of 4!");
    let imm19 = offset / 4;
    emit_cmp_branch_imm(proc, sf, op, imm19, rt)
}

#[inline(always)]
fn emit_cmp_branch_x_addr<P: AddressableInstructionProcessor<T>, T>(
    proc: &mut P,
    sf: UImm1,
    op: UImm1,
    addr: usize,
    rt: Register,
) -> T {
    debug_assert!(addr % 4 == 0, "Addr must be 4 byte aligned!");
    let offset = proc.intr_ptr_offset_to(addr);
    emit_cmp_branch_x_offset(proc, sf, op, offset, rt)
}

/// # [Compare and branch (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Branches--Exception-Generating-and-System-instructions?lang=en#compbranch)
///
/// Implements the following instructions:
///  - [CBZ - Compare and Branch on Zero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CBZ--Compare-and-Branch-on-Zero-?lang=en)
///  - [CBNZ - Compare and Branch on Nonzero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CBNZ--Compare-and-Branch-on-Nonzero-?lang=en)
pub trait CompareAndBranchImm<T>: InstructionProcessor<T> {
    /// [CBZ - Compare and Branch on Zero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CBZ--Compare-and-Branch-on-Zero-?lang=en)
    ///
    /// Compare and Branch on Zero compares the value in a register with zero, and conditionally branches to a label at a PC-relative offset if the comparison is equal. It provides a hint that this is not a subroutine call or return. This instruction does not affect condition flags.
    ///
    /// ```asm
    /// CBZ <Wt>, <label>
    /// ```
    ///
    /// Uses specified `offset` instead of label.
    #[inline(always)]
    fn cbz_32_from_byte_offset(&mut self, wt: Register, offset: Offset32) -> T {
        emit_cmp_branch_x_offset(self, 0, 0, offset, wt)
    }

    /// [CBZ - Compare and Branch on Zero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CBZ--Compare-and-Branch-on-Zero-?lang=en)
    ///
    /// Compare and Branch on Zero compares the value in a register with zero, and conditionally branches to a label at a PC-relative offset if the comparison is equal. It provides a hint that this is not a subroutine call or return. This instruction does not affect condition flags.
    ///
    /// ```asm
    /// CBZ <Xt>, <label>
    /// ```
    ///
    /// Uses specified `offset` instead of label.
    #[inline(always)]
    fn cbz_64_from_byte_offset(&mut self, xt: Register, offset: Offset32) -> T {
        emit_cmp_branch_x_offset(self, 1, 0, offset, xt)
    }

    /// [CBNZ - Compare and Branch on Nonzero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CBNZ--Compare-and-Branch-on-Nonzero-?lang=en)
    ///
    /// Compare and Branch on Nonzero compares the value in a register with zero, and conditionally branches to a label at a PC-relative offset if the comparison is not equal. It provides a hint that this is not a subroutine call or return. This instruction does not affect the condition flags.
    ///
    /// ```asm
    /// CBNZ <Wt>, <label>
    /// ```
    ///
    /// Uses specified `offset` instead of label.
    #[inline(always)]
    fn cbnz_32_from_byte_offset(&mut self, wt: Register, offset: Offset32) -> T {
        emit_cmp_branch_x_offset(self, 0, 1, offset, wt)
    }

    /// [CBNZ - Compare and Branch on Nonzero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CBNZ--Compare-and-Branch-on-Nonzero-?lang=en)
    ///
    /// Compare and Branch on Nonzero compares the value in a register with zero, and conditionally branches to a label at a PC-relative offset if the comparison is not equal. It provides a hint that this is not a subroutine call or return. This instruction does not affect the condition flags.
    ///
    /// ```asm
    /// CBNZ <Xt>, <label>
    /// ```
    ///
    /// Uses specified `offset` instead of label.
    #[inline(always)]
    fn cbnz_64_from_byte_offset(&mut self, xt: Register, offset: Offset32) -> T {
        emit_cmp_branch_x_offset(self, 1, 1, offset, xt)
    }
}

// TODO: Implement with label as soon as labels are supported
pub trait CompareAndBranchImmWithAddress<T>: AddressableInstructionProcessor<T> {
    /// [CBZ - Compare and Branch on Zero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CBZ--Compare-and-Branch-on-Zero-?lang=en)
    ///
    /// Compare and Branch on Zero compares the value in a register with zero, and conditionally branches to a label at a PC-relative offset if the comparison is equal. It provides a hint that this is not a subroutine call or return. This instruction does not affect condition flags.
    ///
    /// ```asm
    /// CBZ <Wt>, <label>
    /// ```
    /// Uses specified `addr` instead of `label`.
    #[inline(always)]
    fn cbz_32_to_addr(&mut self, wt: Register, addr: usize) -> T {
        emit_cmp_branch_x_addr(self, 0, 0, addr, wt)
    }

    /// [CBZ - Compare and Branch on Zero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CBZ--Compare-and-Branch-on-Zero-?lang=en)
    ///
    /// Compare and Branch on Zero compares the value in a register with zero, and conditionally branches to a label at a PC-relative offset if the comparison is equal. It provides a hint that this is not a subroutine call or return. This instruction does not affect condition flags.
    ///
    /// ```asm
    /// CBZ <Xt>, <label>
    /// ```
    /// Uses specified `addr` instead of `label`.
    #[inline(always)]
    fn cbz_64_to_addr(&mut self, xt: Register, addr: usize) -> T {
        emit_cmp_branch_x_addr(self, 1, 0, addr, xt)
    }

    /// [CBNZ - Compare and Branch on Nonzero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CBNZ--Compare-and-Branch-on-Nonzero-?lang=en)
    ///
    /// Compare and Branch on Nonzero compares the value in a register with zero, and conditionally branches to a label at a PC-relative offset if the comparison is not equal. It provides a hint that this is not a subroutine call or return. This instruction does not affect the condition flags.
    ///
    /// ```asm
    /// CBNZ <Wt>, <label>
    /// ```
    /// Uses specified `addr` instead of `label`.
    #[inline(always)]
    fn cbnz_32_to_addr(&mut self, wt: Register, addr: usize) -> T {
        emit_cmp_branch_x_addr(self, 0, 1, addr, wt)
    }

    /// [CBNZ - Compare and Branch on Nonzero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CBNZ--Compare-and-Branch-on-Nonzero-?lang=en)
    ///
    /// Compare and Branch on Nonzero compares the value in a register with zero, and conditionally branches to a label at a PC-relative offset if the comparison is not equal. It provides a hint that this is not a subroutine call or return. This instruction does not affect the condition flags.
    ///
    /// ```asm
    /// CBNZ <Xt>, <label>
    /// ```
    /// Uses specified `addr` instead of `label`.
    #[inline(always)]
    fn cbnz_64_to_addr(&mut self, xt: Register, addr: usize) -> T {
        emit_cmp_branch_x_addr(self, 1, 1, addr, xt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction_emitter::MockEmitter;
    use crate::instruction_stream::InstrStream;
    use crate::mc_memory::MockMemory;
    use crate::types::InstructionPointer;
    use crate::{assert_panic, stream_mock};

    #[test]
    fn test_cbz_x() {
        stream_mock!(stream, {
            let instr = stream.cbz_32_from_byte_offset(0, -(1 << 20));
            assert_eq!(instr.to_string(), "cbz w0, 0xfffffffffff00000");

            let instr = stream.cbz_32_to_addr(0, 0x20);
            assert_eq!(instr.to_string(), "cbz w0, 0x20");

            assert_panic!("Should panic: offset out of bounds"; {
                stream.cbz_32_from_byte_offset(0, 1 << 20);
            });

            assert_panic!("Should panic: offset out of bounds"; {
                stream.cbz_32_from_byte_offset(0, -(1 << 20)-1);
            });

            assert_panic!("Should panic: offset not multiply"; {
                stream.cbz_32_from_byte_offset(0, 1);
            });

            assert_panic!("Should panic: addr not aligned"; {
                stream.cbz_32_to_addr(0, 1);
            });

            assert_panic!("Should panic: offset not withing 1MB"; {
                stream.cbz_32_to_addr(0, 1 << 20);
            });

            let instr = stream.cbz_64_from_byte_offset(0, -(1 << 20));
            assert_eq!(instr.to_string(), "cbz x0, 0xfffffffffff00000");

            let instr = stream.cbz_64_to_addr(0, 0x20);
            assert_eq!(instr.to_string(), "cbz x0, 0x20");

            assert_panic!("Should panic: offset out of bounds"; {
                stream.cbz_64_from_byte_offset(0, 1 << 20);
            });

            assert_panic!("Should panic: offset out of bounds"; {
                stream.cbz_64_from_byte_offset(0, -(1 << 20)-1);
            });

            assert_panic!("Should panic: offset not multiply"; {
                stream.cbz_64_from_byte_offset(0, 1);
            });

            assert_panic!("Should panic: addr not aligned"; {
                stream.cbz_64_to_addr(0, 1);
            });

            assert_panic!("Should panic: offset not withing 1MB"; {
                stream.cbz_64_to_addr(0, 1 << 20);
            });
        })
    }

    #[test]
    fn test_cbnz_x() {
        stream_mock!(stream, {
            let instr = stream.cbnz_32_from_byte_offset(0, -(1 << 20));
            assert_eq!(instr.to_string(), "cbnz w0, 0xfffffffffff00000");

            let instr = stream.cbnz_32_to_addr(0, 0x20);
            assert_eq!(instr.to_string(), "cbnz w0, 0x20");

            assert_panic!("Should panic: offset out of bounds"; {
                stream.cbnz_32_from_byte_offset(0, 1 << 20);
            });

            assert_panic!("Should panic: offset out of bounds"; {
                stream.cbnz_32_from_byte_offset(0, -(1 << 20)-1);
            });

            assert_panic!("Should panic: offset not multiply"; {
                stream.cbnz_32_from_byte_offset(0, 1);
            });

            assert_panic!("Should panic: addr not aligned"; {
                stream.cbnz_32_to_addr(0, 1);
            });

            assert_panic!("Should panic: offset not withing 1MB"; {
                stream.cbnz_32_to_addr(0, 1 << 20);
            });

            let instr = stream.cbnz_64_from_byte_offset(0, -(1 << 20));
            assert_eq!(instr.to_string(), "cbnz x0, 0xfffffffffff00000");

            let instr = stream.cbnz_64_to_addr(0, 0x20);
            assert_eq!(instr.to_string(), "cbnz x0, 0x20");

            assert_panic!("Should panic: offset out of bounds"; {
                stream.cbnz_64_from_byte_offset(0, 1 << 20);
            });

            assert_panic!("Should panic: offset out of bounds"; {
                stream.cbnz_64_from_byte_offset(0, -(1 << 20)-1);
            });

            assert_panic!("Should panic: offset not multiply"; {
                stream.cbnz_64_from_byte_offset(0, 1);
            });

            assert_panic!("Should panic: addr not aligned"; {
                stream.cbnz_64_to_addr(0, 1);
            });

            assert_panic!("Should panic: offset not withing 1MB"; {
                stream.cbnz_64_to_addr(0, 1 << 20);
            });
        })
    }
}
