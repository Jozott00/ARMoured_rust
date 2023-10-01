//! # Bitfield
//!
//! Implements the following instructions:
//! - [SBFM](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SBFM--Signed-Bitfield-Move-?lang=en)
//! - [BFM](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BFM--Bitfield-Move-?lang=en)
//! - [UBFM](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UBFM--Unsigned-Bitfield-Move-?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Register, UImm6};

/// Generates the base instruction for a bitfield operation.
/// `sf`, `opc`, `n`, `immr`, `imms`, `rn`, and `rd` parameters are used to construct the instruction.
/// Note that the details of the instruction encoding should be checked with the ARM documentation.
#[inline(always)]
fn emit_bitfield<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: u8,
    opc: u8,
    n: u8,
    immr: UImm6,
    imms: UImm6,
    rn: Register,
    rd: Register,
) -> T {
    if sf == 1 {
        debug_assert!(immr <= 63, "Immr can only be in range of 0 to 63");
        debug_assert!(imms <= 63, "Immr can only be in range of 0 to 63");
    } else {
        debug_assert!(immr <= 31, "Immr can only be in range of 0 to 31");
        debug_assert!(imms <= 31, "Immr can only be in range of 0 to 31");
    }

    let r = bseq_32!(sf:1 opc:2 100110 n:1 immr:6 imms:6 rn:5 rd:5);
    proc.process(r)
}

/// # Bitfield
///
/// Implements the following instructions:
/// - [SBFM](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SBFM--Signed-Bitfield-Move-?lang=en)
/// - [BFM](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BFM--Bitfield-Move-?lang=en)
/// - [UBFM](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UBFM--Unsigned-Bitfield-Move-?lang=en)
pub trait BitfieldInstructions<T>: InstructionProcessor<T> {
    /// [SBFM](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SBFM--Signed-Bitfield-Move-?lang=en)
    ///
    /// Generates a signed bitfield move (SBFM) instruction for 32-bit registers.
    /// SBFM extracts a bitfield from the source register, sign extends it, and writes it to the destination register.
    /// The `rd`, `rn`, `immr` and `imms` parameters represent the destination register, source register, rotate amount and the width of the bitfield respectively.
    ///
    /// ```asm
    /// SBFM <Wd>, <Wn>, #<immr>, #<imms>
    /// ```
    #[inline(always)]
    fn sbfm_32(&mut self, wd: Register, wn: Register, immr: UImm6, imms: UImm6) -> T {
        emit_bitfield(self, 0, 0b00, 0, immr, imms, wn, wd)
    }

    /// [SBFM](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SBFM--Signed-Bitfield-Move-?lang=en)
    ///
    /// Generates a signed bitfield move (SBFM) instruction for 64-bit registers.
    /// The parameters are the same as [`sbfm_32`](#method.sbfm_32) but this operates on 64-bit registers.
    ///
    /// ```asm
    /// SBFM <Xd>, <Xn>, #<immr>, #<imms>
    /// ```
    #[inline(always)]
    fn sbfm_64(&mut self, xd: Register, xn: Register, immr: UImm6, imms: UImm6) -> T {
        emit_bitfield(self, 1, 0b00, 1, immr, imms, xn, xd)
    }

    /// [BFM](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BFM--Bitfield-Move-?lang=en)
    ///
    /// Generates a bitfield move (BFM) instruction for 32-bit registers.
    /// BFM extracts a bitfield from the source register, and writes it to the destination register without sign extension.
    /// The `rd`, `rn`, `immr` and `imms` parameters represent the destination register, source register, rotate amount and the width of the bitfield respectively.
    ///
    /// ```asm
    /// BFM <Wd>, <Wn>, #<immr>, #<imms>
    /// ```
    #[inline(always)]
    fn bfm_32(&mut self, wd: Register, wn: Register, immr: UImm6, imms: UImm6) -> T {
        emit_bitfield(self, 0, 0b01, 0, immr, imms, wn, wd)
    }

    /// [BFM](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BFM--Bitfield-Move-?lang=en)
    ///
    /// Generates a bitfield move (BFM) instruction for 64-bit registers.
    /// The parameters are the same as [`bfm_32`](#method.bfm_32) but this operates on 64-bit registers.
    ///
    /// ```asm
    /// BFM <Xd>, <Xn>, #<immr>, #<imms>
    /// ```
    #[inline(always)]
    fn bfm_64(&mut self, xd: Register, xn: Register, immr: UImm6, imms: UImm6) -> T {
        emit_bitfield(self, 1, 0b01, 1, immr, imms, xn, xd)
    }

    /// [UBFM](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UBFM--Unsigned-Bitfield-Move-?lang=en)
    ///
    /// Generates an unsigned bitfield move (UBFM) instruction for 32-bit registers.
    /// UBFM extracts a bitfield from the source register, zero extends it, and writes it to the destination register.
    /// The `rd`, `rn`, `immr` and `imms` parameters represent the destination register, source register, rotate amount and the width of the bitfield respectively.
    ///
    /// ```asm
    /// UBFM <Wd>, <Wn>, #<immr>, #<imms>
    /// ```
    #[inline(always)]
    fn ubfm_32(&mut self, wd: Register, wn: Register, immr: UImm6, imms: UImm6) -> T {
        emit_bitfield(self, 0, 0b10, 0, immr, imms, wn, wd)
    }

    /// [UBFM](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UBFM--Unsigned-Bitfield-Move-?lang=en)
    ///
    /// Generates an unsigned bitfield move (UBFM) instruction for 64-bit registers.
    /// The parameters are the same as [`ubfm_32`](#method.ubfm_32) but this operates on 64-bit registers.
    ///
    /// ```asm
    /// UBFM <Xd>, <Xn>, #<immr>, #<imms>
    /// ```
    #[inline(always)]
    fn ubfm_64(&mut self, rd: Register, rn: Register, immr: UImm6, imms: UImm6) -> T {
        emit_bitfield(self, 1, 0b10, 1, immr, imms, rn, rd)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_sbfm_x() {
        let mut prod = TestProducer::new();

        // tested with other disasm
        let instr = prod.sbfm_32(1, 2, 0x8, 0x2);
        assert_eq!(instr, "sbfiz w1, w2, #0x18, #0x3");

        assert_panic!("Should panic: out of bounds immr"; prod.sbfm_32(1, 2, 32, 0x2));
        assert_panic!("Should panic: out of bounds imms"; prod.sbfm_32(1, 2, 0x8, 32));

        // tested with other disasm
        let instr = prod.sbfm_64(1, 2, 0x8, 0x2);
        assert_eq!(instr, "sbfiz x1, x2, #0x38, #0x3");

        assert_panic!("Should panic: out of bounds immr"; prod.sbfm_64(1, 2, 64, 0x2));
        assert_panic!("Should panic: out of bounds imms"; prod.sbfm_64(1, 2, 0x8, 64));
    }

    #[test]
    fn test_bfm_x() {
        let mut prod = TestProducer::new();

        // tested with other disasm
        let instr = prod.bfm_32(1, 2, 0x8, 0x2);
        assert_eq!(instr, "bfi w1, w2, #0x18, #0x3");

        assert_panic!("Should panic: out of bounds immr"; prod.bfm_32(1, 2, 32, 0x2));
        assert_panic!("Should panic: out of bounds imms"; prod.bfm_32(1, 2, 0x8, 32));

        // tested with other disasm
        let instr = prod.bfm_64(1, 2, 0x8, 0x2);
        assert_eq!(instr, "bfi x1, x2, #0x38, #0x3");

        assert_panic!("Should panic: out of bounds immr"; prod.bfm_64(1, 2, 64, 0x2));
        assert_panic!("Should panic: out of bounds imms"; prod.bfm_64(1, 2, 0x8, 64));
    }

    #[test]
    fn test_ubfm_x() {
        let mut prod = TestProducer::new();

        // tested with other disasm
        let instr = prod.ubfm_32(1, 2, 0x8, 0x2);
        assert_eq!(instr, "ubfiz w1, w2, #0x18, #0x3");

        assert_panic!("Should panic: out of bounds immr"; prod.ubfm_32(1, 2, 32, 0x2));
        assert_panic!("Should panic: out of bounds imms"; prod.ubfm_32(1, 2, 0x8, 32));

        // tested with other disasm
        let instr = prod.ubfm_64(1, 2, 0x8, 0x2);
        assert_eq!(instr, "ubfiz x1, x2, #0x38, #0x3");

        assert_panic!("Should panic: out of bounds immr"; prod.ubfm_64(1, 2, 64, 0x2));
        assert_panic!("Should panic: out of bounds imms"; prod.ubfm_64(1, 2, 0x8, 64));
    }
}
