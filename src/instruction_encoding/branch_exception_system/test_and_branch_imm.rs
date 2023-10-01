//! # [Test and branch (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Branches--Exception-Generating-and-System-instructions?lang=en#testbranch)
//!
//! Implements the following instructions:
//!  - [TBZ - Test bit and Branch if Zero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/TBZ--Test-bit-and-Branch-if-Zero-?lang=en)
//!  - [TBNZ - Test bit and Branch if Nonzero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/TBNZ--Test-bit-and-Branch-if-Nonzero-?lang=en)

use bit_seq::{bseq_32, bseq_8};

use crate::instruction_encoding::{AddressableInstructionProcessor, InstructionProcessor};
use crate::types::{Imm14, Offset16, Offset32, Register, UImm1, UImm5, UImm6};
use crate::types::register::RegConstr;

#[inline(always)]
fn emit_test_branch_imm<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    b5: UImm1,
    op: UImm1,
    b40: UImm5,
    imm14: Imm14,
    rt: Register,
) -> T {
    let i = bseq_32!(b5:1 011011 op:1 b40:5 imm14:14 rt:5);
    proc.process(i)
}

#[inline(always)]
fn emit_test_branch_x_offset<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    rc: RegConstr,
    op: UImm1,
    offset: Offset32,
    imm: UImm6,
) -> T {
    debug_assert!(
        imm < 32 || matches!(rc, RegConstr::X(_)),
        "W register is only permitted for imm values less than 32"
    );
    debug_assert!(
        -(1 << 15) <= offset && offset < (1 << 15),
        "Offset must be within ±32KB"
    );
    debug_assert!(offset % 4 == 0, "Offset must be a multiply of 4!");
    let imm14 = offset / 4;
    let b5 = imm >> 5;
    let b40 = bseq_8!(imm:5);
    let rt = match rc {
        RegConstr::W(reg) => reg,
        RegConstr::X(reg) => reg,
    };
    emit_test_branch_imm(proc, b5, op, b40, imm14 as Imm14, rt)
}

#[inline(always)]
fn emit_test_branch_x_addr<P: AddressableInstructionProcessor<T>, T>(
    proc: &mut P,
    rc: RegConstr,
    op: UImm1,
    addr: usize,
    imm: UImm6,
) -> T {
    debug_assert!(addr % 4 == 0, "Addr must be 4 byte aligned!");
    let offset = proc.intr_ptr_offset_to(addr);
    emit_test_branch_x_offset(proc, rc, op, offset, imm)
}

/// # [Test and branch (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Branches--Exception-Generating-and-System-instructions?lang=en#testbranch)
///
/// Implements the following instructions:
///  - [TBZ - Test bit and Branch if Zero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/TBZ--Test-bit-and-Branch-if-Zero-?lang=en)
///  - [TBNZ - Test bit and Branch if Nonzero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/TBNZ--Test-bit-and-Branch-if-Nonzero-?lang=en)
pub trait TestAndBranchImmediate<T>: InstructionProcessor<T> {
    /// [TBZ - Test bit and Branch if Zero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/TBZ--Test-bit-and-Branch-if-Zero-?lang=en)
    ///
    /// Test bit and Branch if Zero compares the value of a test bit with zero, and conditionally branches to a label at a PC-relative offset if the comparison is equal. It provides a hint that this is not a subroutine call or return. This instruction does not affect condition flags.
    ///
    /// ```asm
    /// TBZ <R><t>, #<imm>, <offset>
    /// ```
    #[inline(always)]
    fn tbz_from_byte_offset(&mut self, rt: RegConstr, imm: UImm5, offset: Offset16) -> T {
        emit_test_branch_x_offset(self, rt, 0, offset as Offset32, imm)
    }

    /// [TBNZ - Test bit and Branch if Nonzero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/TBNZ--Test-bit-and-Branch-if-Nonzero-?lang=en)
    ///
    /// Test bit and Branch if Nonzero compares the value of a bit in a general-purpose register with zero, and conditionally branches to a label at a PC-relative offset if the comparison is not equal. It provides a hint that this is not a subroutine call or return. This instruction does not affect condition flags.
    ///
    /// ```asm
    /// TBNZ <R><t>, #<imm>, <offset>
    /// ```
    #[inline(always)]
    fn tbnz_from_byte_offset(&mut self, rt: RegConstr, imm: UImm5, offset: Offset16) -> T {
        emit_test_branch_x_offset(self, rt, 1, offset as Offset32, imm)
    }
}

/// # [Test and branch (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Branches--Exception-Generating-and-System-instructions?lang=en#testbranch)
///
/// Implements the following instructions:
///  - [TBZ - Test bit and Branch if Zero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/TBZ--Test-bit-and-Branch-if-Zero-?lang=en)
///  - [TBNZ - Test bit and Branch if Nonzero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/TBNZ--Test-bit-and-Branch-if-Nonzero-?lang=en)
pub trait TestAndBranchImmediateWithAddress<T>: AddressableInstructionProcessor<T> {
    /// [TBZ - Test bit and Branch if Zero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/TBZ--Test-bit-and-Branch-if-Zero-?lang=en)
    ///
    /// Test bit and Branch if Zero compares the value of a test bit with zero, and conditionally branches to a label at a PC-relative offset if the comparison is equal. It provides a hint that this is not a subroutine call or return. This instruction does not affect condition flags.
    ///
    /// ```asm
    /// TBZ <R><t>, #<imm>, <addr>
    /// ```
    #[inline(always)]
    fn tbz_to_addr(&mut self, rt: RegConstr, imm: UImm5, addr: usize) -> T {
        emit_test_branch_x_addr(self, rt, 0, addr, imm)
    }

    /// [TBNZ - Test bit and Branch if Nonzero](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/TBNZ--Test-bit-and-Branch-if-Nonzero-?lang=en)
    ///
    /// Test bit and Branch if Nonzero compares the value of a bit in a general-purpose register with zero, and conditionally branches to a label at a PC-relative offset if the comparison is not equal. It provides a hint that this is not a subroutine call or return. This instruction does not affect condition flags.
    ///
    /// ```asm
    /// TBNZ <R><t>, #<imm>, <label>
    /// ```
    #[inline(always)]
    fn tbnz_to_addr(&mut self, rt: RegConstr, imm: UImm5, addr: usize) -> T {
        emit_test_branch_x_addr(self, rt, 1, addr, imm)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_panic, stream_mock};
    use crate::instruction_emitter::MockEmitter;
    use crate::instruction_stream::InstrStream;
    use crate::mc_memory::MockMemory;
    use crate::types::InstructionPointer;
    use crate::types::register::RegConstr;

    use super::*;

    #[test]
    fn test_tbz_x() {
        stream_mock!(stream, {
            let instr = stream.tbz_from_byte_offset(RegConstr::X(0), 63, i16::MIN);
            assert_eq!(instr.to_string(), "tbz x0, #0x3f, 0xffffffffffff8000");

            let instr = stream.tbz_from_byte_offset(RegConstr::X(0), 31, i16::MIN);
            assert_eq!(instr.to_string(), "tbz w0, #0x1f, 0xffffffffffff8000");

            let instr = stream.tbz_to_addr(RegConstr::X(0), 63, 0x20);
            assert_eq!(instr.to_string(), "tbz x0, #0x3f, 0x20");

            let instr = stream.tbz_to_addr(RegConstr::X(0), 31, 0x20);
            assert_eq!(instr.to_string(), "tbz w0, #0x1f, 0x20");

            assert_panic!("Should panic: offset not multiply"; {
                stream.tbz_from_byte_offset(RegConstr::X(0), 63, 1);
            });

            assert_panic!("Should panic: addr not aligned"; {
                let instr = stream.tbz_to_addr(RegConstr::X(0), 63, 1);
            });

            assert_panic!("Should panic: imm greater than 32"; {
                stream.tbz_from_byte_offset(RegConstr::W(0), 32, 1);
            });

            assert_panic!("Should panic: imm greater than 32"; {
                let instr = stream.tbz_to_addr(RegConstr::W(0), 32, 1);
            });
        })
    }

    #[test]
    fn test_tbnz_x() {
        stream_mock!(stream, {
            let instr = stream.tbnz_from_byte_offset(RegConstr::X(0), 63, i16::MIN);
            assert_eq!(instr.to_string(), "tbnz x0, #0x3f, 0xffffffffffff8000");

            let instr = stream.tbnz_from_byte_offset(RegConstr::X(0), 31, i16::MIN);
            assert_eq!(instr.to_string(), "tbnz w0, #0x1f, 0xffffffffffff8000");

            let instr = stream.tbnz_to_addr(RegConstr::X(0), 63, 0x20);
            assert_eq!(instr.to_string(), "tbnz x0, #0x3f, 0x20");

            let instr = stream.tbnz_to_addr(RegConstr::X(0), 31, 0x20);
            assert_eq!(instr.to_string(), "tbnz w0, #0x1f, 0x20");

            assert_panic!("Should panic: offset not multiply"; {
                stream.tbnz_from_byte_offset(RegConstr::X(0), 63, 1);
            });

            assert_panic!("Should panic: addr not aligned"; {
                let instr = stream.tbnz_to_addr(RegConstr::X(0), 63, 1);
            });

            assert_panic!("Should panic: imm greater than 32"; {
                stream.tbnz_from_byte_offset(RegConstr::W(0), 32, 1);
            });

            assert_panic!("Should panic: imm greater than 32"; {
                let instr = stream.tbnz_to_addr(RegConstr::W(0), 32, 1);
            });
        })
    }
}
