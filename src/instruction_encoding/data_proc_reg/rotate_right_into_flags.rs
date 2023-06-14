//! # [Rotate right into flags](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#rmif)
//!
//! Implements the following instructions:
//!  - [RMIF - Rotate - Mask Insert Flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RMIF--Rotate--Mask-Insert-Flags-?lang=en)



use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Register, UImm4, UImm6};

#[inline(always)]
fn emit_rotate_right_in_flags<P: InstructionProcessor<T>, T>(proc: &mut P, sf: u8, op: u8, s: u8, imm6: u8, rn: Register, o2: u8, mask: u8) -> T {
    let i = bseq_32!(sf:1 op:1 s:1 11010000 imm6:6 00001 rn:5 o2:1 mask:4);
    proc.process(i)
}


/// # [Rotate right into flags](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#rmif)
///
/// Implements the following instructions:
///  - [RMIF - Rotate - Mask Insert Flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RMIF--Rotate--Mask-Insert-Flags-?lang=en)
pub trait RotateRightIntoFlags<T>: InstructionProcessor<T> {
    /// [RMIF - Rotate - Mask Insert Flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RMIF--Rotate--Mask-Insert-Flags-?lang=en)
    ///
    /// Performs a rotation right of a value held in a general purpose register by an immediate value, and then inserts a selection of the bottom four bits of the result of the rotation into the PSTATE flags, under the control of a second immediate mask.
    ///
    /// ```asm
    /// RMIF <Xn>, #<shift>, #<mask>
    /// ```
    #[inline(always)]
    fn rmif(&mut self, xn: Register, shift: UImm6, mask: UImm4) -> T {
        emit_rotate_right_in_flags(self, 1, 0, 1, shift, xn, 0, mask)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_rmif() {
        let mut prod = TestProducer::new();

        let instr = prod.rmif(3, 4, 5);
        assert_eq!(instr, "rmif x3, #0x4, #0x5");
    }
}