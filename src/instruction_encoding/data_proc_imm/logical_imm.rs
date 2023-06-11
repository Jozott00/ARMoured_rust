//! # Logical (immediate)
//!
//! Implements the following instructions:
//! - [AND](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AND--immediate---Bitwise-AND--immediate--?lang=en)
//! - [ORR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ORR--immediate---Bitwise-OR--immediate--?lang=en)
//! - [EOR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EOR--immediate---Bitwise-Exclusive-OR--immediate--?lang=en)
//! - [ANDS](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ANDS--immediate---Bitwise-AND--immediate---setting-flags-?lang=en)

use bit_seq::{bseq_32, bseq_64, bseq_8};
use crate::instruction_emitter::Emitter;
use crate::instruction_encoding::InstructionProcessor;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::{Imm12, Imm13, Imm32, Imm6, Imm64, Register, UImm32, UImm64};
use crate::types::bitmask_immediate::BitmaskImmediate;

/// Encodes and emits a logical instruction with an immediate value.
/// This is a helper function used by logical instruction variants that accept an immediate.
///
/// # Arguments
///
/// * `sf` - Operand size. If sf = 0, 32-bit operands are used. If sf = 1, 64-bit operands are used.
/// * `opc` - Opcode to determine the specific logical operation.
/// * `bit_mask` - A BitmaskImmediate object representing the immediate value in the appropriate format for logical instructions.
/// * `rn` - The source register.
/// * `rd` - The destination register, where the result of the operation will be stored.
#[inline(always)]
fn emit_logical_imm<P: InstructionProcessor<T>, T>(proc: &mut P, sf: u8, opc: u8, bit_mask: &BitmaskImmediate, rn: Register, rd: Register) -> T {
    let nrs_mask = bit_mask.as_u16();
    let r = bseq_32!(sf:1 opc:2 100100 nrs_mask:13 rn:5 rd:5);
    proc.emit(r)
}

pub trait LogicalImmediate<T>: InstructionProcessor<T> {
    /// [AND](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AND--immediate---Bitwise-AND--immediate--?lang=en) \
    /// Encodes and emits a 32-bit AND operation with an immediate value.
    /// This function will fail if the immediate value cannot be encoded as a valid logical immediate.
    ///
    /// ```asm
    /// AND <Wd|WSP>, <Wn>, #<imm>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register, where the result of the operation will be stored.
    /// * `rn` - The source register.
    /// * `imm` - An immediate 32-bit value to be logically ANDed with the value in the source register.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` if the operation was successful.
    /// * `Err(())` if the immediate value could not be encoded as a valid logical immediate.
    #[inline(always)]
    fn and_32_imm(&mut self, wd: Register, wn: Register, imm: UImm32) -> Result<T, ()> {
        let mask_64 = bseq_64!(imm:32 imm:32);
        let bit_mask = BitmaskImmediate::try_from(mask_64)?;
        Ok(emit_logical_imm(self, 0, 0b00, &bit_mask, wn, wd))
    }

    /// [AND](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AND--immediate---Bitwise-AND--immediate--?lang=en) \
    /// Encodes and emits a 64-bit AND operation with an immediate value.
    /// This function will fail if the immediate value cannot be encoded as a valid logical immediate.
    ///
    /// ```asm
    /// AND <Xd|SP>, <Xn>, #<imm>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register, where the result of the operation will be stored.
    /// * `rn` - The source register.
    /// * `imm` - An immediate 64-bit value to be logically ANDed with the value in the source register.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` if the operation was successful.
    /// * `Err(())` if the immediate value could not be encoded as a valid logical immediate.
    #[inline(always)]
    fn and_64_imm(&mut self, xd: Register, xn: Register, imm: UImm64) -> Result<T, ()> {
        let bit_mask = BitmaskImmediate::try_from(imm)?;
        Ok(emit_logical_imm(self, 1, 0b00, &bit_mask, xn, xd))
    }

    /// [ORR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ORR--immediate---Bitwise-OR--immediate--?lang=en) \
    /// Encodes and emits a 32-bit ORR operation with an immediate value.
    /// This function will fail if the immediate value cannot be encoded as a valid logical immediate.
    ///
    /// ```asm
    /// ORR <Wd|WSP>, <Wn>, #<imm>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register, where the result of the operation will be stored.
    /// * `rn` - The source register.
    /// * `imm` - An immediate 32-bit value to be logically ORed with the value in the source register.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` if the operation was successful.
    /// * `Err(())` if the immediate value could not be encoded as a valid logical immediate.
    #[inline(always)]
    fn orr_32_imm(&mut self, wd: Register, wn: Register, imm: UImm32) -> Result<T, ()> {
        let mask_64 = bseq_64!(imm:32 imm:32);
        let bit_mask = BitmaskImmediate::try_from(mask_64)?;
        Ok(emit_logical_imm(self, 0, 0b01, &bit_mask, wn, wd))
    }

    /// [ORR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ORR--immediate---Bitwise-OR--immediate--?lang=en) \
    /// Encodes and emits a 64-bit ORR operation with an immediate value.
    /// This function will fail if the immediate value cannot be encoded as a valid logical immediate.
    ///
    /// ```asm
    /// ORR <Xd|SP>, <Xn>, #<imm>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register, where the result of the operation will be stored.
    /// * `rn` - The source register.
    /// * `imm` - An immediate 64-bit value to be logically ORed with the value in the source register.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` if the operation was successful.
    /// * `Err(())` if the immediate value could not be encoded as a valid logical immediate.
    #[inline(always)]
    fn orr_64_imm(&mut self, xd: Register, xn: Register, imm: UImm64) -> Result<T, ()> {
        let bit_mask = BitmaskImmediate::try_from(imm)?;
        Ok(emit_logical_imm(self, 1, 0b01, &bit_mask, xn, xd))
    }

    /// [EOR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EOR--immediate---Bitwise-Exclusive-OR--immediate--?lang=en) \
    /// Encodes and emits a 32-bit EOR operation with an immediate value.
    /// This function will fail if the immediate value cannot be encoded as a valid logical immediate.
    ///
    /// ```asm
    /// EOR <Wd|WSP>, <Wn>, #<imm>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register, where the result of the operation will be stored.
    /// * `rn` - The source register.
    /// * `imm` - An immediate 32-bit value to be logically XORed with the value in the source register.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` if the operation was successful.
    /// * `Err(())` if the immediate value could not be encoded as a valid logical immediate.
    #[inline(always)]
    fn eor_32_imm(&mut self, wd: Register, wn: Register, imm: UImm32) -> Result<T, ()> {
        let mask_64 = bseq_64!(imm:32 imm:32);
        let bit_mask = BitmaskImmediate::try_from(mask_64)?;
        Ok(emit_logical_imm(self, 0, 0b10, &bit_mask, wn, wd))
    }

    /// [EOR](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EOR--immediate---Bitwise-Exclusive-OR--immediate--?lang=en) \
    /// Encodes and emits a 64-bit EOR operation with an immediate value.
    /// This function will fail if the immediate value cannot be encoded as a valid logical immediate.
    ///
    /// ```asm
    /// EOR <Xd|SP>, <Xn>, #<imm>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register, where the result of the operation will be stored.
    /// * `rn` - The source register.
    /// * `imm` - An immediate 64-bit value to be logically XORed with the value in the source register.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` if the operation was successful.
    /// * `Err(())` if the immediate value could not be encoded as a valid logical immediate.
    #[inline(always)]
    fn eor_64_imm(&mut self, xd: Register, xn: Register, imm: UImm64) -> Result<T, ()> {
        let bit_mask = BitmaskImmediate::try_from(imm)?;
        Ok(emit_logical_imm(self, 1, 0b10, &bit_mask, xn, xd))
    }

    /// [ANDS](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ANDS--immediate---Bitwise-AND--immediate---setting-flags-?lang=en) \
    /// Encodes and emits a 32-bit ANDS operation with an immediate value.
    /// This function will fail if the immediate value cannot be encoded as a valid logical immediate.
    ///
    /// ```asm
    /// ANDS <Wd|WSP>, <WXn>, #<imm>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register, where the result of the operation will be stored.
    /// * `rn` - The source register.
    /// * `imm` - An immediate 32-bit value to be logically ANDed with the value in the source register.
    ///            The condition flags will be updated based on the result.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` if the operation was successful.
    /// * `Err(())` if the immediate value could not be encoded as a valid logical immediate.
    #[inline(always)]
    fn ands_32_imm(&mut self, wd: Register, wn: Register, imm: UImm32) -> Result<T, ()> {
        let mask_64 = bseq_64!(imm:32 imm:32);
        let bit_mask = BitmaskImmediate::try_from(mask_64)?;
        Ok(emit_logical_imm(self, 0, 0b11, &bit_mask, wn, wd))
    }

    /// [ANDS](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ANDS--immediate---Bitwise-AND--immediate---setting-flags-?lang=en) \
    /// Encodes and emits a 64-bit ANDS operation with an immediate value.
    /// This function will fail if the immediate value cannot be encoded as a valid logical immediate.
    ///
    /// ```asm
    /// AND <Xd|SP>, <Xn>, #<imm>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `rd` - The destination register, where the result of the operation will be stored.
    /// * `rn` - The source register.
    /// * `imm` - An immediate 64-bit value to be logically ANDed with the value in the source register.
    ///            The condition flags will be updated based on the result.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` if the operation was successful.
    /// * `Err(())` if the immediate value could not be encoded as a valid logical immediate.
    #[inline(always)]
    fn ands_64_imm(&mut self, xd: Register, xn: Register, imm: UImm64) -> Result<T, ()> {
        let bit_mask = BitmaskImmediate::try_from(imm)?;
        Ok(emit_logical_imm(self, 1, 0b11, &bit_mask, xn, xd))
    }
}


#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_and_x() {
        let mut prod = TestProducer::new();

        let instr = prod.and_32_imm(2, 4, 0b111100);
        assert_eq!(instr, Ok("and w2, w4, #0x3c".to_string()));

        let instr = prod.and_32_imm(2, 4, 0b11100111100);
        assert_eq!(instr, Err(()));

        let instr = prod.and_64_imm(2, 4, 0b111100);
        assert_eq!(instr, Ok("and x2, x4, #0x3c".to_string()));

        let instr = prod.and_64_imm(2, 4, 0b11100111100);
        assert_eq!(instr, Err(()));
    }

    #[test]
    fn test_orr_x() {
        let mut prod = TestProducer::new();

        let instr = prod.orr_32_imm(2, 4, 0b111100);
        assert_eq!(instr, Ok("orr w2, w4, #0x3c".to_string()));

        let instr = prod.orr_32_imm(2, 4, 0b11100111100);
        assert_eq!(instr, Err(()));

        let instr = prod.orr_64_imm(2, 4, 0b111100);
        assert_eq!(instr, Ok("orr x2, x4, #0x3c".to_string()));

        let instr = prod.orr_64_imm(2, 4, 0b11100111100);
        assert_eq!(instr, Err(()));
    }

    #[test]
    fn test_eor_x() {
        let mut prod = TestProducer::new();

        let instr = prod.eor_32_imm(2, 4, 0b111100);
        assert_eq!(instr, Ok("eor w2, w4, #0x3c".to_string()));

        let instr = prod.eor_32_imm(2, 4, 0b11100111100);
        assert_eq!(instr, Err(()));

        let instr = prod.eor_64_imm(2, 4, 0b111100);
        assert_eq!(instr, Ok("eor x2, x4, #0x3c".to_string()));

        let instr = prod.eor_64_imm(2, 4, 0b11100111100);
        assert_eq!(instr, Err(()));
    }

    #[test]
    fn test_ands_x() {
        let mut prod = TestProducer::new();

        let instr = prod.ands_32_imm(2, 4, 0b111100);
        assert_eq!(instr, Ok("ands w2, w4, #0x3c".to_string()));

        let instr = prod.ands_32_imm(2, 4, 0b11100111100);
        assert_eq!(instr, Err(()));

        let instr = prod.ands_64_imm(2, 4, 0b111100);
        assert_eq!(instr, Ok("ands x2, x4, #0x3c".to_string()));

        let instr = prod.ands_64_imm(2, 4, 0b11100111100);
        assert_eq!(instr, Err(()));
    }
}