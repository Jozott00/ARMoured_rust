//! # [Data-processing (3 source)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#dp_3src)
//!
//! Implements the following instructions:
//!  - [MADD - Multiply Add](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MADD--Multiply-Add-?lang=en)
//!  - [MSUB - Multiply Subtract](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MSUB--Multiply-Subtract-?lang=en)
//!  - [SMADDL - Signed Multiply Add Long](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SMADDL--Signed-Multiply-Add-Long-?lang=en)
//!  - [SMSUBL - Signed Multiply Subtract Long](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SMSUBL--Signed-Multiply-Subtract-Long-?lang=en)
//!  - [SMULH - Signed Multiply High](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SMULH--Signed-Multiply-High-?lang=en)
//!  - [UMADDL - Unsigned Multiply Add Long](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UMADDL--Unsigned-Multiply-Add-Long-?lang=en)
//!  - [UMSUBL - Unsigned Multiply Subtract Long](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UMSUBL--Unsigned-Multiply-Subtract-Long-?lang=en)
//!  - [UMULH - Unsigned Multiply High](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UMULH--Unsigned-Multiply-High-?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::Register;

#[inline(always)]
fn emit_data_proc_three<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: u8,
    op54: u8,
    op31: u8,
    rm: Register,
    o0: u8,
    ra: Register,
    rn: Register,
    rd: Register,
) -> T {
    let i = bseq_32!(sf:1 op54:2 11011 op31:3 rm:5 o0:1 ra:5 rn:5 rd:5);
    //                      1  00     11011 000    00010 0 00110 00100 00011
    //                      1  00     11011 000    00011 0 00110 00100 00011
    proc.process(i)
}

/// # [Data-processing (3 source)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#dp_3src)
///
/// Implements the following instructions:
///  - [MADD - Multiply Add](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MADD--Multiply-Add-?lang=en)
///  - [MSUB - Multiply Subtract](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MSUB--Multiply-Subtract-?lang=en)
///  - [SMADDL - Signed Multiply Add Long](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SMADDL--Signed-Multiply-Add-Long-?lang=en)
///  - [SMSUBL - Signed Multiply Subtract Long](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SMSUBL--Signed-Multiply-Subtract-Long-?lang=en)
///  - [SMULH - Signed Multiply High](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SMULH--Signed-Multiply-High-?lang=en)
///  - [UMADDL - Unsigned Multiply Add Long](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UMADDL--Unsigned-Multiply-Add-Long-?lang=en)
///  - [UMSUBL - Unsigned Multiply Subtract Long](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UMSUBL--Unsigned-Multiply-Subtract-Long-?lang=en)
///  - [UMULH - Unsigned Multiply High](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UMULH--Unsigned-Multiply-High-?lang=en)
pub trait DataProcessingThreeSource<T>: InstructionProcessor<T> {
    /// [MADD - Multiply Add](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MADD--Multiply-Add-?lang=en)
    ///
    /// Multiply-Add multiplies two register values, adds a third register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias MUL.
    ///
    /// ```asm
    /// MADD <Wd>, <Wn>, <Wm>, <Wa>
    /// ```
    #[inline(always)]
    fn madd_32(&mut self, wd: Register, wn: Register, wm: Register, wa: Register) -> T {
        emit_data_proc_three(self, 0, 0, 0, wm, 0, wa, wn, wd)
    }

    /// [MADD - Multiply Add](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MADD--Multiply-Add-?lang=en)
    ///
    /// Multiply-Add multiplies two register values, adds a third register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias MUL.
    ///
    /// ```asm
    /// MADD <Xd>, <Xn>, <Xm>, <Xa>
    /// ```
    #[inline(always)]
    fn madd_64(&mut self, xd: Register, xn: Register, xm: Register, xa: Register) -> T {
        emit_data_proc_three(self, 1, 0, 0, xm, 0, xa, xn, xd)
    }

    /// [MSUB - Multiply Subtract](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MSUB--Multiply-Subtract-?lang=en)
    ///
    /// Multiply-Subtract multiplies two register values, subtracts the product from a third register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias MNEG.
    ///
    /// ```asm
    /// MSUB <Wd>, <Wn>, <Wm>, <Wa>
    /// ```
    #[inline(always)]
    fn msub_32(&mut self, wd: Register, wn: Register, wm: Register, wa: Register) -> T {
        emit_data_proc_three(self, 0, 0, 0, wm, 1, wa, wn, wd)
    }

    /// [MSUB - Multiply Subtract](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MSUB--Multiply-Subtract-?lang=en)
    ///
    /// Multiply-Subtract multiplies two register values, subtracts the product from a third register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias MNEG.
    ///
    /// ```asm
    /// MSUB <Xd>, <Xn>, <Xm>, <Xa>
    /// ```
    #[inline(always)]
    fn msub_64(&mut self, xd: Register, xn: Register, xm: Register, xa: Register) -> T {
        emit_data_proc_three(self, 1, 0, 0, xm, 1, xa, xn, xd)
    }

    /// [SMADDL - Signed Multiply Add Long](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SMADDL--Signed-Multiply-Add-Long-?lang=en)
    ///
    /// Signed Multiply-Add Long multiplies two 32-bit register values, adds a 64-bit register value, and writes the result to the 64-bit destination register.
    ///
    /// This instruction is used by the alias SMULL.
    ///
    /// ```asm
    /// SMADDL <Xd>, <Wn>, <Wm>, <Xa>
    /// ```
    #[inline(always)]
    fn smaddl(&mut self, xd: Register, wn: Register, wm: Register, xa: Register) -> T {
        emit_data_proc_three(self, 1, 0, 0b001, wm, 0, xa, wn, xd)
    }

    /// [SMSUBL - Signed Multiply Subtract Long](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SMSUBL--Signed-Multiply-Subtract-Long-?lang=en)
    ///
    /// Signed Multiply-Subtract Long multiplies two 32-bit register values, subtracts the product from a 64-bit register value, and writes the result to the 64-bit destination register.
    ///
    /// This instruction is used by the alias SMNEGL.
    ///
    /// ```asm
    /// SMSUBL <Xd>, <Wn>, <Wm>, <Xa>
    /// ```
    #[inline(always)]
    fn smsubl(&mut self, xd: Register, wn: Register, xm: Register, xa: Register) -> T {
        emit_data_proc_three(self, 1, 0, 0b001, xm, 1, xa, wn, xd)
    }

    /// [SMULH - Signed Multiply High](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SMULH--Signed-Multiply-High-?lang=en)
    ///
    /// Signed Multiply High multiplies two 64-bit register values, and writes bits[127:64] of the 128-bit result to the 64-bit destination register.
    ///
    /// ```asm
    /// SMULH <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn smulh(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        emit_data_proc_three(self, 1, 0, 0b010, xm, 0, 0b11111, xn, xd)
    }

    /// [UMADDL - Unsigned Multiply Add Long](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UMADDL--Unsigned-Multiply-Add-Long-?lang=en)
    ///
    /// Unsigned Multiply-Add Long multiplies two 32-bit register values, adds a 64-bit register value, and writes the result to the 64-bit destination register.
    ///
    /// This instruction is used by the alias UMULL.
    ///
    /// ```asm
    /// UMADDL <Xd>, <Wn>, <Wm>, <Xa>
    /// ```
    #[inline(always)]
    fn umaddl(&mut self, xd: Register, wn: Register, wm: Register, xa: Register) -> T {
        emit_data_proc_three(self, 1, 0, 0b101, wm, 0, xa, wn, xd)
    }

    /// [UMSUBL - Unsigned Multiply Subtract Long](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UMSUBL--Unsigned-Multiply-Subtract-Long-?lang=en)
    ///
    /// Unsigned Multiply-Subtract Long multiplies two 32-bit register values, subtracts the product from a 64-bit register value, and writes the result to the 64-bit destination register.
    ///
    /// This instruction is used by the alias UMNEGL.
    ///
    /// ```asm
    /// UMSUBL <Xd>, <Wn>, <Wm>, <Xa>
    /// ```
    #[inline(always)]
    fn umsubl(&mut self, xd: Register, wn: Register, wm: Register, xa: Register) -> T {
        emit_data_proc_three(self, 1, 0, 0b101, wm, 1, xa, wn, xd)
    }

    /// [UMULH - Unsigned Multiply High](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UMULH--Unsigned-Multiply-High-?lang=en)
    ///
    /// Unsigned Multiply High multiplies two 64-bit register values, and writes bits[127:64] of the 128-bit result to the 64-bit destination register.
    ///
    /// ```asm
    /// UMULH <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn umulh(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        emit_data_proc_three(self, 1, 0, 0b110, xm, 0, 0b11111, xn, xd)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_madd() {
        let mut prod = TestProducer::new();

        let instr = prod.madd_32(3, 4, 2, 6);
        assert_eq!(instr, "madd w3, w4, w2, w6");

        let instr = prod.madd_64(3, 4, 2, 6);
        assert_eq!(instr, "madd x3, x4, x2, x6");
    }

    #[test]
    fn test_msub() {
        let mut prod = TestProducer::new();

        let instr = prod.msub_32(3, 4, 2, 6);
        assert_eq!(instr, "msub w3, w4, w2, w6");

        let instr = prod.msub_64(3, 4, 2, 6);
        assert_eq!(instr, "msub x3, x4, x2, x6");
    }

    #[test]
    fn test_smaddl() {
        let mut prod = TestProducer::new();

        let instr = prod.smaddl(3, 4, 2, 6);
        assert_eq!(instr, "smaddl x3, w4, w2, x6");
    }

    #[test]
    fn test_smsubl() {
        let mut prod = TestProducer::new();

        let instr = prod.smsubl(3, 4, 2, 6);
        assert_eq!(instr, "smsubl x3, w4, w2, x6");
    }

    #[test]
    fn test_smulh() {
        let mut prod = TestProducer::new();

        let instr = prod.smulh(3, 4, 2);
        assert_eq!(instr, "smulh x3, x4, x2");
    }

    #[test]
    fn test_umaddl() {
        let mut prod = TestProducer::new();

        let instr = prod.umaddl(3, 4, 2, 6);
        assert_eq!(instr, "umaddl x3, w4, w2, x6");
    }

    #[test]
    fn test_umsubl() {
        let mut prod = TestProducer::new();

        let instr = prod.umsubl(3, 4, 2, 6);
        assert_eq!(instr, "umsubl x3, w4, w2, x6");
    }

    #[test]
    fn test_umulh() {
        let mut prod = TestProducer::new();

        let instr = prod.umulh(3, 4, 2);
        assert_eq!(instr, "umulh x3, x4, x2");
    }
}
