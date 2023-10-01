//! # Extract
//!
//! Implements the following instructions:
//! - [EXTR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EXTR--Extract-register-?lang=en)

use bit_seq::{bseq_32, bseq_8};

use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Register, UImm5, UImm6};

/// Generates the base instruction for a bit extraction operation.
/// `sf`, `op21`, `n`, `o0`, `rm`, `imms`, `rn`, and `rd` parameters are used to construct the instruction.
/// The specifics of the instruction encoding should be verified with the ARM documentation.
#[inline(always)]
fn emit_extr_x<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: u8,
    op21: u8,
    n: u8,
    o0: u8,
    rm: Register,
    imms: u8,
    rn: Register,
    rd: Register,
) -> T {
    let r = bseq_32!(sf:1 op21:2 100111 n:1 o0:1 rm:5 imms:6 rn:5 rd:5);
    proc.process(r)
}

/// # Extract
///
/// Implements the following instructions:
/// - [EXTR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EXTR--Extract-register-?lang=en)
pub trait ExtractInstructions<T>: InstructionProcessor<T> {
    /// [EXTR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EXTR--Extract-register-?lang=en) \
    /// Encodes and emits a 32-bit EXTR (extract) operation.
    ///
    /// ```asm
    /// EXTR <Wd>, <Wn>, <Wm>, #<lsb>
    /// ```
    ///
    /// # Panics
    ///
    /// This function asserts that the `lsb` argument (which represents the least significant
    /// bit number to start the extraction from) is in the range of 0 to 31 inclusive.
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register.
    /// * `rn` - The first source register.
    /// * `rm` - The second source register.
    /// * `lsb` - The least significant bit number where the extraction starts.
    #[inline(always)]
    fn extr_32(&mut self, rd: Register, rn: Register, rm: Register, lsb: UImm5) -> T {
        debug_assert!(lsb <= 31, "lsb must be in range 0 to 63");
        emit_extr_x(self, 0, 0b00, 0, 0, rm, bseq_8!(0 lsb:5), rn, rd)
    }

    /// [EXTR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EXTR--Extract-register-?lang=en) \
    /// Encodes and emits a 64-bit EXTR (extract) operation.
    ///
    /// ```asm
    /// EXTR <Xd>, <Xn>, <Xm>, #<lsb>
    /// ```
    ///
    /// # Panics
    ///
    /// This function asserts that the `lsb` argument (which represents the least significant
    /// bit number to start the extraction from) is in the range of 0 to 63 inclusive.
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register.
    /// * `rn` - The first source register.
    /// * `rm` - The second source register.
    /// * `lsb` - The least significant bit number where the extraction starts.
    #[inline(always)]
    fn extr_64(&mut self, rd: Register, rn: Register, rm: Register, lsb: UImm6) -> T {
        debug_assert!(lsb <= 63, "lsb must be in range 0 to 63");
        emit_extr_x(self, 1, 0b00, 1, 0, rm, lsb, rn, rd)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_extr_x() {
        let mut prod = TestProducer::new();

        let instr = prod.extr_32(1, 2, 6, 31);
        assert_eq!(instr, "extr w1, w2, w6, #0x1f");

        assert_panic!("Should panic: out of bounds lsb"; prod.extr_32(1, 2, 6, 32));

        let instr = prod.extr_64(1, 2, 6, 63);
        assert_eq!(instr, "extr x1, x2, x6, #0x3f");

        assert_panic!("Should panic: out of bounds lsb"; prod.extr_64(1, 2, 6, 64));
    }
}
