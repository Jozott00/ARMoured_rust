//! # [Evaluate into flags](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#setf)
//!
//! Implements the following instructions:
//!  - [SETF8 - SETF16 - Evaluation of 8 or 16 bit flag values](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETF8--SETF16--Evaluation-of-8-or-16-bit-flag-values-?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::Register;

#[inline(always)]
fn emit_eval_in_flags<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: u8,
    op: u8,
    s: u8,
    opcode2: u8,
    sz: u8,
    rn: Register,
    o3: u8,
    mask: u8,
) -> T {
    let i = bseq_32!(sf:1 op:1 s:1 11010000 opcode2:6 sz:1 0010 rn:5 o3:1 mask:4);
    proc.process(i)
}

/// # [Evaluate into flags](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#setf)
///
/// Implements the following instructions:
///  - [SETF8 - SETF16 - Evaluation of 8 or 16 bit flag values](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETF8--SETF16--Evaluation-of-8-or-16-bit-flag-values-?lang=en)
pub trait EvaluateIntoFlags<T>: InstructionProcessor<T> {
    /// [SETF8 - SETF16 - Evaluation of 8 or 16 bit flag values](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETF8--SETF16--Evaluation-of-8-or-16-bit-flag-values-?lang=en)
    ///
    /// Set the PSTATE.NZV flags based on the value in the specified general-purpose register. SETF8 treats the value as an 8 bit value, and SETF16 treats the value as an 16 bit value.
    ///
    /// The PSTATE.C flag is not affected by these instructions.
    ///
    /// ```asm
    /// SETF8 <Wn>
    /// ```
    #[inline(always)]
    fn setf8(&mut self, wn: Register) -> T {
        emit_eval_in_flags(self, 0, 0, 1, 0, 0, wn, 0, 0b1101)
    }

    /// [SETF8 - SETF16 - Evaluation of 8 or 16 bit flag values](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETF8--SETF16--Evaluation-of-8-or-16-bit-flag-values-?lang=en)
    ///
    /// Set the PSTATE.NZV flags based on the value in the specified general-purpose register. SETF8 treats the value as an 8 bit value, and SETF16 treats the value as an 16 bit value.
    ///
    /// The PSTATE.C flag is not affected by these instructions.
    ///
    /// ```asm
    /// SETF16 <Wn>
    /// ```
    #[inline(always)]
    fn setf16(&mut self, wn: Register) -> T {
        emit_eval_in_flags(self, 0, 0, 1, 0, 1, wn, 0, 0b1101)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_setf8() {
        let mut prod = TestProducer::new();

        let instr = prod.setf8(3);
        assert_eq!(instr, "setf8 w3");
    }

    #[test]
    fn test_setf16() {
        let mut prod = TestProducer::new();

        let instr = prod.setf16(3);
        assert_eq!(instr, "setf16 w3");
    }
}
