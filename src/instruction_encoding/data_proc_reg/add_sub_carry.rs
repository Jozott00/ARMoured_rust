//! # [Add/subtract (with carry)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#addsub_carry)
//!
//! Implements the following instructions:
//!  - [ADC - Add with Carry](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADC--Add-with-Carry-?lang=en)
//!  - [ADCS - Add with Carry - setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADCS--Add-with-Carry--setting-flags-?lang=en)
//!  - [SBC - Subtract with Carry](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SBC--Subtract-with-Carry-?lang=en)
//!  - [SBCS - Subtract with Carry - setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SBCS--Subtract-with-Carry--setting-flags-?lang=en)



use bit_seq::bseq_32;
use crate::types::Register;
use crate::instruction_encoding::InstructionProcessor;

#[inline(always)]
fn emit_bitfield<P: InstructionProcessor<T>, T>(proc: &mut P, sf: u8, op: u8, s: u8, rm: Register, rn: Register, rd: Register) -> T {
    let i = bseq_32!(sf:1 op:1 s:1 11010000 rm:5 0:6 rn:5 rd:5);
    proc.process(i)
}

/// # [Add/subtract (with carry)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#addsub_carry)
///
/// Implements the following instructions:
///  - [ADC - Add with Carry](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADC--Add-with-Carry-?lang=en)
///  - [ADCS - Add with Carry - setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADCS--Add-with-Carry--setting-flags-?lang=en)
///  - [SBC - Subtract with Carry](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SBC--Subtract-with-Carry-?lang=en)
///  - [SBCS - Subtract with Carry - setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SBCS--Subtract-with-Carry--setting-flags-?lang=en)
pub trait AddSubtractWithCarry<T>: InstructionProcessor<T> {
    /// [ADC - Add with Carry](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADC--Add-with-Carry-?lang=en)
    ///
    /// Add with Carry adds two register values and the Carry flag value, and writes the result to the destination register.
    ///
    /// ```asm
    /// ADC <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn adc_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_bitfield(self, 0, 0, 0, wm, wn, wd)
    }


    /// [ADC - Add with Carry](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADC--Add-with-Carry-?lang=en)
    ///
    /// Add with Carry adds two register values and the Carry flag value, and writes the result to the destination register.
    ///
    /// ```asm
    /// ADC <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn adc_64(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_bitfield(self, 1, 0, 0, wm, wn, wd)
    }


    /// [ADCS - Add with Carry - setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADCS--Add-with-Carry--setting-flags-?lang=en)
    ///
    /// Add with Carry, setting flags, adds two register values and the Carry flag value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// ```asm
    /// ADCS <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn adcs_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_bitfield(self, 0, 0, 1, wm, wn, wd)
    }


    /// [ADCS - Add with Carry - setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADCS--Add-with-Carry--setting-flags-?lang=en)
    ///
    /// Add with Carry, setting flags, adds two register values and the Carry flag value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// ```asm
    /// ADCS <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn adcs_64(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_bitfield(self, 1, 0, 1, wm, wn, wd)
    }

    /// [SBC - Subtract with Carry](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SBC--Subtract-with-Carry-?lang=en)
    ///
    /// Subtract with Carry subtracts a register value and the value of NOT (Carry flag) from a register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias NGC.
    ///
    /// ```asm
    /// SBC <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn sbc_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_bitfield(self, 0, 1, 0, wm, wn, wd)
    }


    /// [SBC - Subtract with Carry](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SBC--Subtract-with-Carry-?lang=en)
    ///
    /// Subtract with Carry subtracts a register value and the value of NOT (Carry flag) from a register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias NGC.
    ///
    /// ```asm
    /// SBC <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn sbc_64(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_bitfield(self, 1, 1, 0, wm, wn, wd)
    }


    /// [SBCS - Subtract with Carry - setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SBCS--Subtract-with-Carry--setting-flags-?lang=en)
    ///
    /// Subtract with Carry, setting flags, subtracts a register value and the value of NOT (Carry flag) from a register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the alias NGCS.
    ///
    /// ```asm
    /// SBCS <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn sbcs_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_bitfield(self, 0, 1, 1, wm, wn, wd)
    }


    /// [SBCS - Subtract with Carry - setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SBCS--Subtract-with-Carry--setting-flags-?lang=en)
    ///
    /// Subtract with Carry, setting flags, subtracts a register value and the value of NOT (Carry flag) from a register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the alias NGCS.
    ///
    /// ```asm
    /// SBCS <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn sbcs_64(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_bitfield(self, 1, 1, 1, wm, wn, wd)
    }
}


#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_adc() {
        let mut prod = TestProducer::new();

        let instr = prod.adc_32(3, 4, 5);
        assert_eq!(instr, "adc w3, w4, w5");

        let instr = prod.adc_64(3, 4, 5);
        assert_eq!(instr, "adc x3, x4, x5");
    }

    #[test]
    fn test_adcs() {
        let mut prod = TestProducer::new();

        let instr = prod.adcs_32(3, 4, 5);
        assert_eq!(instr, "adcs w3, w4, w5");

        let instr = prod.adcs_64(3, 4, 5);
        assert_eq!(instr, "adcs x3, x4, x5");
    }

    #[test]
    fn test_sbc() {
        let mut prod = TestProducer::new();

        let instr = prod.sbc_32(3, 4, 5);
        assert_eq!(instr, "sbc w3, w4, w5");

        let instr = prod.sbc_64(3, 4, 5);
        assert_eq!(instr, "sbc x3, x4, x5");
    }

    #[test]
    fn test_sbcs() {
        let mut prod = TestProducer::new();

        let instr = prod.sbcs_32(3, 4, 5);
        assert_eq!(instr, "sbcs w3, w4, w5");

        let instr = prod.sbcs_64(3, 4, 5);
        assert_eq!(instr, "sbcs x3, x4, x5");
    }
}