//! # Data-processing (2 source)
//!
//! Implements the following instructions:
//! - [UDIV - Unsigned Divide](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UDIV--Unsigned-Divide-?lang=en)
//! - [SDIV - Signed Divide](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SDIV--Signed-Divide-?lang=en)
//! - [LSLV - Logical Shift Left Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSLV--Logical-Shift-Left-Variable-?lang=en)
//! - [LSL - Logical Shift Left (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSL--register---Logical-Shift-Left--register---an-alias-of-LSLV-?lang=en)
//! - [LSRV - Logical Shift Right Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSRV--Logical-Shift-Right-Variable-?lang=en)
//! - [LSR - Logical Shift Right (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSR--register---Logical-Shift-Right--register---an-alias-of-LSRV-?lang=en)
//! - [ASRV - Arithmetic Shift Right Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ASRV--Arithmetic-Shift-Right-Variable-?lang=en)
//! - [ASR - Arithmetic Shift Right (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ASR--register---Arithmetic-Shift-Right--register---an-alias-of-ASRV-?lang=en)
//! - [RORV - Rotate Right Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RORV--Rotate-Right-Variable-?lang=en)
//! - [ROR - Rotate Right (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ROR--register---Rotate-Right--register---an-alias-of-RORV-?lang=en)
//! - [CRC32B, CRC32H, CRC32W, CRC32X - checksum](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CRC32B--CRC32H--CRC32W--CRC32X--CRC32-checksum-?lang=en)
//! - [CRC32CB, CRC32CH, CRC32CW, CRC32CX - checksum](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CRC32CB--CRC32CH--CRC32CW--CRC32CX--CRC32C-checksum-?lang=en)
//! - [SUBP - Subtract Pointer](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBP--Subtract-Pointer-?lang=en)
//! - [IRG - Insert Random Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/IRG--Insert-Random-Tag-?lang=en)
//! - [GMI - Tag Mask Insert](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/GMI--Tag-Mask-Insert-?lang=en)
//! - [PACGA - Pointer Authentication Code - using Generic key](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACGA--Pointer-Authentication-Code--using-Generic-key-?lang=en)
//! - [SUBPS - Subtract Pointer - setting Flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBPS--Subtract-Pointer--setting-Flags-?lang=en)


use bit_seq::{bseq_32, bseq_8};
use crate::instruction_encoding::InstructionProcessor;
use crate::types::Register;

#[inline(always)]
fn emit_data_proc_two<P: InstructionProcessor<T>, T>(proc: &mut P, sf: u8, s: u8, rm: Register, opcode: u8, rn: Register, rd: Register) -> T {
    let i = bseq_32!(sf:1 0 s:1 11010110 rm:5 opcode:6 rn:5 rd:5);
    proc.process(i)
}

/// # Data-processing (2 source)
///
/// Implements the following instructions:
/// - [UDIV - Unsigned Divide](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UDIV--Unsigned-Divide-?lang=en)
/// - [SDIV - Signed Divide](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SDIV--Signed-Divide-?lang=en)
/// - [LSLV - Logical Shift Left Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSLV--Logical-Shift-Left-Variable-?lang=en)
/// - [LSL - Logical Shift Left (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSL--register---Logical-Shift-Left--register---an-alias-of-LSLV-?lang=en)
/// - [LSRV - Logical Shift Right Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSRV--Logical-Shift-Right-Variable-?lang=en)
/// - [LSR - Logical Shift Right (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSR--register---Logical-Shift-Right--register---an-alias-of-LSRV-?lang=en)
/// - [ASRV - Arithmetic Shift Right Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ASRV--Arithmetic-Shift-Right-Variable-?lang=en)
/// - [ASR - Arithmetic Shift Right (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ASR--register---Arithmetic-Shift-Right--register---an-alias-of-ASRV-?lang=en)
/// - [RORV - Rotate Right Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RORV--Rotate-Right-Variable-?lang=en)
/// - [ROR - Rotate Right (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ROR--register---Rotate-Right--register---an-alias-of-RORV-?lang=en)
/// - [CRC32B, CRC32H, CRC32W, CRC32X - checksum](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CRC32B--CRC32H--CRC32W--CRC32X--CRC32-checksum-?lang=en)
/// - [CRC32CB, CRC32CH, CRC32CW, CRC32CX - checksum](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CRC32CB--CRC32CH--CRC32CW--CRC32CX--CRC32C-checksum-?lang=en)
/// - [SUBP - Subtract Pointer](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBP--Subtract-Pointer-?lang=en)
/// - [IRG - Insert Random Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/IRG--Insert-Random-Tag-?lang=en)
/// - [GMI - Tag Mask Insert](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/GMI--Tag-Mask-Insert-?lang=en)
/// - [PACGA - Pointer Authentication Code - using Generic key](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACGA--Pointer-Authentication-Code--using-Generic-key-?lang=en)
/// - [SUBPS - Subtract Pointer - setting Flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBPS--Subtract-Pointer--setting-Flags-?lang=en)
pub trait DataProcessingTwoSource<T>: InstructionProcessor<T> {
    /// [UDIV - Unsigned Divide](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UDIV--Unsigned-Divide-?lang=en)
    ///
    /// Unsigned Divide divides an unsigned integer register value by another unsigned integer register value, and writes the result to the destination register. The condition flags are not affected.
    ///
    /// ```asm
    /// UDIV <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn udiv_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_data_proc_two(self, 0, 0, wm, 0b10, wn, wd)
    }

    /// [UDIV - Unsigned Divide](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/UDIV--Unsigned-Divide-?lang=en)
    ///
    /// Unsigned Divide divides an unsigned integer register value by another unsigned integer register value, and writes the result to the destination register. The condition flags are not affected.
    ///
    /// ```asm
    /// UDIV <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn udiv_64(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        emit_data_proc_two(self, 1, 0, xm, 0b10, xn, xd)
    }


    /// [SDIV - Signed Divide](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SDIV--Signed-Divide-?lang=en)
    ///
    /// Signed Divide divides a signed integer register value by another signed integer register value, and writes the result to the destination register. The condition flags are not affected.
    ///
    /// ```asm
    /// SDIV <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn sdiv_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_data_proc_two(self, 0, 0, wm, 0b11, wn, wd)
    }

    /// [SDIV - Signed Divide](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SDIV--Signed-Divide-?lang=en)
    ///
    /// Signed Divide divides a signed integer register value by another signed integer register value, and writes the result to the destination register. The condition flags are not affected.
    ///
    /// ```asm
    /// SDIV <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn sdiv_64(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        emit_data_proc_two(self, 1, 0, xm, 0b11, xn, xd)
    }

    /// [LSLV - Logical Shift Left Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSLV--Logical-Shift-Left-Variable-?lang=en)
    ///
    /// Logical Shift Left Variable shifts a register value left by a variable number of bits, shifting in zeros, and writes the result to the destination register. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is left-shifted.
    ///
    /// ```asm
    /// LSLV <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn lslv_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_data_proc_two(self, 0, 0, wm, 0b1000, wn, wd)
    }

    /// [LSLV - Logical Shift Left Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSLV--Logical-Shift-Left-Variable-?lang=en)
    ///
    /// Logical Shift Left Variable shifts a register value left by a variable number of bits, shifting in zeros, and writes the result to the destination register. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is left-shifted.
    ///
    /// ```asm
    /// LSLV <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn lslv_64(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        emit_data_proc_two(self, 1, 0, xm, 0b1000, xn, xd)
    }

    /// [LSRV - Logical Shift Right Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSRV--Logical-Shift-Right-Variable-?lang=en)
    ///
    /// Logical Shift Right Variable shifts a register value right by a variable number of bits, shifting in zeros, and writes the result to the destination register. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is right-shifted.
    ///
    /// ```asm
    /// LSRV <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn lsrv_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_data_proc_two(self, 0, 0, wm, 0b1001, wn, wd)
    }

    /// [LSRV - Logical Shift Right Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSRV--Logical-Shift-Right-Variable-?lang=en)
    ///
    /// Logical Shift Right Variable shifts a register value right by a variable number of bits, shifting in zeros, and writes the result to the destination register. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is right-shifted.
    ///
    /// ```asm
    /// LSRV <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn lsrv_64(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        emit_data_proc_two(self, 1, 0, xm, 0b1001, xn, xd)
    }

    /// [ASRV - Arithmetic Shift Right Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ASRV--Arithmetic-Shift-Right-Variable-?lang=en)
    ///
    /// Arithmetic Shift Right Variable shifts a register value right by a variable number of bits, shifting in copies of its sign bit, and writes the result to the destination register. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is right-shifted.
    ///
    /// ```asm
    /// ASRV <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn asrv_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_data_proc_two(self, 0, 0, wm, 0b1010, wn, wd)
    }

    /// [ASRV - Arithmetic Shift Right Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ASRV--Arithmetic-Shift-Right-Variable-?lang=en)
    ///
    /// Arithmetic Shift Right Variable shifts a register value right by a variable number of bits, shifting in copies of its sign bit, and writes the result to the destination register. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is right-shifted.
    ///
    /// ```asm
    /// ASRV <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn asrv_64(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        emit_data_proc_two(self, 1, 0, xm, 0b1010, xn, xd)
    }


    /// [RORV - Rotate Right Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RORV--Rotate-Right-Variable-?lang=en)
    ///
    /// Rotate Right Variable provides the value of the contents of a register rotated by a variable number of bits. The bits that are rotated off the right end are inserted into the vacated bit positions on the left. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is right-shifted.
    ///
    /// ```asm
    /// RORV <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn rorv_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        emit_data_proc_two(self, 0, 0, wm, 0b1011, wn, wd)
    }

    /// [RORV - Rotate Right Variable](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RORV--Rotate-Right-Variable-?lang=en)
    ///
    /// Rotate Right Variable provides the value of the contents of a register rotated by a variable number of bits. The bits that are rotated off the right end are inserted into the vacated bit positions on the left. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is right-shifted.
    ///
    /// ```asm
    /// RORV <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn rorv_64(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        emit_data_proc_two(self, 1, 0, xm, 0b1011, xn, xd)
    }


    /// [CRC32B - checksum](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CRC32B--CRC32H--CRC32W--CRC32X--CRC32-checksum-?lang=en)
    ///
    /// CRC32 checksum performs a cyclic redundancy check (CRC) calculation on a value held in a general-purpose register. It takes an input CRC value in the first source operand, performs a CRC on the input value in the second source operand, and returns the output CRC value. The second source operand can be 8, 16, 32, or 64 bits. To align with common usage, the bit order of the values is reversed as part of the operation, and the polynomial 0x04C11DB7 is used for the CRC calculation.
    ///
    /// In an Armv8.0 implementation, this is an optional instruction. From Armv8.1, it is mandatory for all implementations to implement this instruction.
    ///
    /// ```asm
    /// CRC32B <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn crc32b(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        let sz = 0b00;
        emit_data_proc_two(self, 0, 0, wm, bseq_8!(100 sz:2), wn, wd)
    }

    /// [CRC32H - checksum](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CRC32B--CRC32H--CRC32W--CRC32X--CRC32-checksum-?lang=en)
    ///
    /// CRC32 checksum performs a cyclic redundancy check (CRC) calculation on a value held in a general-purpose register. It takes an input CRC value in the first source operand, performs a CRC on the input value in the second source operand, and returns the output CRC value. The second source operand can be 8, 16, 32, or 64 bits. To align with common usage, the bit order of the values is reversed as part of the operation, and the polynomial 0x04C11DB7 is used for the CRC calculation.
    ///
    /// In an Armv8.0 implementation, this is an optional instruction. From Armv8.1, it is mandatory for all implementations to implement this instruction.
    ///
    /// ```asm
    /// CRC32H <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn crc32h(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        let sz = 0b01;
        emit_data_proc_two(self, 0, 0, wm, bseq_8!(100 sz:2), wn, wd)
    }

    /// [CRC32W - checksum](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CRC32B--CRC32H--CRC32W--CRC32X--CRC32-checksum-?lang=en)
    ///
    /// CRC32 checksum performs a cyclic redundancy check (CRC) calculation on a value held in a general-purpose register. It takes an input CRC value in the first source operand, performs a CRC on the input value in the second source operand, and returns the output CRC value. The second source operand can be 8, 16, 32, or 64 bits. To align with common usage, the bit order of the values is reversed as part of the operation, and the polynomial 0x04C11DB7 is used for the CRC calculation.
    ///
    /// In an Armv8.0 implementation, this is an optional instruction. From Armv8.1, it is mandatory for all implementations to implement this instruction.
    ///
    /// ```asm
    /// CRC32W <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn crc32w(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        let sz = 0b10;
        emit_data_proc_two(self, 0, 0, wm, bseq_8!(100 sz:2), wn, wd)
    }

    /// [CRC32X - checksum](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CRC32B--CRC32H--CRC32W--CRC32X--CRC32-checksum-?lang=en)
    ///
    /// CRC32 checksum performs a cyclic redundancy check (CRC) calculation on a value held in a general-purpose register. It takes an input CRC value in the first source operand, performs a CRC on the input value in the second source operand, and returns the output CRC value. The second source operand can be 8, 16, 32, or 64 bits. To align with common usage, the bit order of the values is reversed as part of the operation, and the polynomial 0x04C11DB7 is used for the CRC calculation.
    ///
    /// In an Armv8.0 implementation, this is an optional instruction. From Armv8.1, it is mandatory for all implementations to implement this instruction.
    ///
    /// ```asm
    /// CRC32X <Wd>, <Wn>, <Xm>
    /// ```
    #[inline(always)]
    fn crc32x(&mut self, wd: Register, wn: Register, xm: Register) -> T {
        let sz = 0b11;
        emit_data_proc_two(self, 1, 0, xm, bseq_8!(100 sz:2), wn, wd)
    }

    /// [CRC32CB - checksum](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CRC32CB--CRC32CH--CRC32CW--CRC32CX--CRC32C-checksum-?lang=en)
    ///
    /// CRC32 checksum performs a cyclic redundancy check (CRC) calculation on a value held in a general-purpose register. It takes an input CRC value in the first source operand, performs a CRC on the input value in the second source operand, and returns the output CRC value. The second source operand can be 8, 16, 32, or 64 bits. To align with common usage, the bit order of the values is reversed as part of the operation, and the polynomial 0x1EDC6F41 is used for the CRC calculation.
    ///
    /// In an Armv8.0 implementation, this is an optional instruction. From Armv8.1, it is mandatory for all implementations to implement this instruction.
    ///
    /// ```asm
    /// CRC32CB <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn crc32cb(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        let sz = 0b00;
        emit_data_proc_two(self, 0, 0, wm, bseq_8!(101 sz:2), wn, wd)
    }

    /// [CRC32CH - checksum](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CRC32CB--CRC32CH--CRC32CW--CRC32CX--CRC32C-checksum-?lang=en)
    ///
    /// CRC32 checksum performs a cyclic redundancy check (CRC) calculation on a value held in a general-purpose register. It takes an input CRC value in the first source operand, performs a CRC on the input value in the second source operand, and returns the output CRC value. The second source operand can be 8, 16, 32, or 64 bits. To align with common usage, the bit order of the values is reversed as part of the operation, and the polynomial 0x1EDC6F41 is used for the CRC calculation.
    ///
    /// In an Armv8.0 implementation, this is an optional instruction. From Armv8.1, it is mandatory for all implementations to implement this instruction.
    ///
    /// ```asm
    /// CRC32CH <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn crc32ch(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        let sz = 0b01;
        emit_data_proc_two(self, 0, 0, wm, bseq_8!(101 sz:2), wn, wd)
    }

    /// [CRC32CW - checksum](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CRC32CB--CRC32CH--CRC32CW--CRC32CX--CRC32C-checksum-?lang=en)
    ///
    /// CRC32 checksum performs a cyclic redundancy check (CRC) calculation on a value held in a general-purpose register. It takes an input CRC value in the first source operand, performs a CRC on the input value in the second source operand, and returns the output CRC value. The second source operand can be 8, 16, 32, or 64 bits. To align with common usage, the bit order of the values is reversed as part of the operation, and the polynomial 0x1EDC6F41 is used for the CRC calculation.
    ///
    /// In an Armv8.0 implementation, this is an optional instruction. From Armv8.1, it is mandatory for all implementations to implement this instruction.
    ///
    /// ```asm
    /// CRC32CW <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn crc32cw(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        let sz = 0b10;
        emit_data_proc_two(self, 0, 0, wm, bseq_8!(101 sz:2), wn, wd)
    }

    /// [CRC32CX - checksum](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CRC32CB--CRC32CH--CRC32CW--CRC32CX--CRC32C-checksum-?lang=en)
    ///
    /// CRC32 checksum performs a cyclic redundancy check (CRC) calculation on a value held in a general-purpose register. It takes an input CRC value in the first source operand, performs a CRC on the input value in the second source operand, and returns the output CRC value. The second source operand can be 8, 16, 32, or 64 bits. To align with common usage, the bit order of the values is reversed as part of the operation, and the polynomial 0x1EDC6F41 is used for the CRC calculation.
    ///
    /// In an Armv8.0 implementation, this is an optional instruction. From Armv8.1, it is mandatory for all implementations to implement this instruction.
    ///
    /// ```asm
    /// CRC32CX <Wd>, <Wn>, <Xm>
    /// ```
    #[inline(always)]
    fn crc32cx(&mut self, wd: Register, wn: Register, xm: Register) -> T {
        let sz = 0b11;
        emit_data_proc_two(self, 1, 0, xm, bseq_8!(101 sz:2), wn, wd)
    }

    /// [SUBP - Subtract Pointer](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBP--Subtract-Pointer-?lang=en)
    ///
    /// Subtract Pointer subtracts the 56-bit address held in the second source register from the 56-bit address held in the first source register, sign-extends the result to 64-bits, and writes the result to the destination register.
    ///
    /// ```asm
    /// SUBP <Xd>, <Xn|SP>, <Xm|SP>
    /// ```
    #[inline(always)]
    fn subp(&mut self, xd: Register, xn_sp: Register, xm_sp: Register) -> T {
        emit_data_proc_two(self, 1, 0, xm_sp, 0, xn_sp, xd)
    }

    /// [IRG - Insert Random Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/IRG--Insert-Random-Tag-?lang=en)
    ///
    /// Insert Random Tag inserts a random Logical Address Tag into the address in the first source register, and writes the result to the destination register. Any tags specified in the optional second source register or in GCR_EL1.Exclude are excluded from the selection of the random Logical Address Tag.
    ///
    /// ```asm
    /// IRG <Xd|SP>, <Xn|SP>, <Xm>
    /// ```
    #[inline(always)]
    fn irg(&mut self, xd_sp: Register, xn_sp: Register, xm: Register) -> T {
        emit_data_proc_two(self, 1, 0, xm, 0b100, xn_sp, xd_sp)
    }

    /// [GMI - Tag Mask Insert](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/GMI--Tag-Mask-Insert-?lang=en)
    ///
    /// Tag Mask Insert inserts the tag in the first source register into the excluded set specified in the second source register, writing the new excluded set to the destination register.
    ///
    /// ```asm
    /// GMI <Xd>, <Xn|SP>, <Xm>
    /// ```
    #[inline(always)]
    fn gmi(&mut self, xd: Register, xn_sp: Register, xm: Register) -> T {
        emit_data_proc_two(self, 1, 0, xm, 0b101, xn_sp, xd)
    }

    /// [PACGA - Pointer Authentication Code - using Generic key](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACGA--Pointer-Authentication-Code--using-Generic-key-?lang=en)
    ///
    /// Pointer Authentication Code, using Generic key. This instruction computes the pointer authentication code for an address in the first source register, using a modifier in the second source register, and the Generic key. The computed pointer authentication code is returned in the upper 32 bits of the destination register.
    ///
    /// ```asm
    /// PACGA <Xd>, <Xn>, <Xm|SP>
    /// ```
    #[inline(always)]
    fn pacga(&mut self, xd: Register, xn: Register, xm_sp: Register) -> T {
        emit_data_proc_two(self, 1, 0, xm_sp, 0b1100, xn, xd)
    }

    /// [SUBPS - Subtract Pointer - setting Flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBPS--Subtract-Pointer--setting-Flags-?lang=en)
    ///
    /// Subtract Pointer, setting Flags subtracts the 56-bit address held in the second source register from the 56-bit address held in the first source register, sign-extends the result to 64-bits, and writes the result to the destination register. It updates the condition flags based on the result of the subtraction.
    ///
    /// ```asm
    /// SUBPS <Xd>, <Xn|SP>, <Xm|SP>
    /// ```
    #[inline(always)]
    fn subps(&mut self, xd: Register, xn_sp: Register, xm_sp: Register) -> T {
        emit_data_proc_two(self, 1, 1, xm_sp, 0, xn_sp, xd)
    }


    // aliases

    /// [LSL - Logical Shift Left (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSL--register---Logical-Shift-Left--register---an-alias-of-LSLV-?lang=en)
    ///
    /// Logical Shift Left (register) shifts a register value left by a variable number of bits, shifting in zeros, and writes the result to the destination register. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is left-shifted.
    ///
    /// ```asm
    /// LSL <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn lsl_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        self.lslv_32(wd, wn, wm)
    }

    //// [LSL - Logical Shift Left (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSL--register---Logical-Shift-Left--register---an-alias-of-LSLV-?lang=en)
    ///
    /// Logical Shift Left (register) shifts a register value left by a variable number of bits, shifting in zeros, and writes the result to the destination register. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is left-shifted.
    ///
    /// ```asm
    /// LSL <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn lsl_64(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        self.lslv_64(xd, xn, xm)
    }

    /// [LSR - Logical Shift Right (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSR--register---Logical-Shift-Right--register---an-alias-of-LSRV-?lang=en)
    ///
    /// Logical Shift Right (register) shifts a register value right by a variable number of bits, shifting in zeros, and writes the result to the destination register. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is right-shifted.
    ///
    /// ```asm
    /// LSL <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn lsr_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        self.lsrv_32(wd, wn, wm)
    }

    //// [LSR - Logical Shift Right (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LSR--register---Logical-Shift-Right--register---an-alias-of-LSRV-?lang=en)
    ///
    /// Logical Shift Right (register) shifts a register value right by a variable number of bits, shifting in zeros, and writes the result to the destination register. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is right-shifted.
    ///
    /// ```asm
    /// LSL <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn lsr_64(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        self.lsrv_64(xd, xn, xm)
    }

    /// [ASR - Arithmetic Shift Right (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ASR--register---Arithmetic-Shift-Right--register---an-alias-of-ASRV-?lang=en)
    ///
    /// Arithmetic Shift Right (register) shifts a register value right by a variable number of bits, shifting in copies of its sign bit, and writes the result to the destination register. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is right-shifted.
    ///
    /// ```asm
    /// ASR <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn asr_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        self.asrv_32(wd, wn, wm)
    }

    /// [ASR - Arithmetic Shift Right (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ASR--register---Arithmetic-Shift-Right--register---an-alias-of-ASRV-?lang=en)
    ///
    /// Arithmetic Shift Right (register) shifts a register value right by a variable number of bits, shifting in copies of its sign bit, and writes the result to the destination register. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is right-shifted.
    ///
    /// ```asm
    /// ASR <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn asr_64(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        self.asrv_64(xd, xn, xm)
    }

    /// [ROR - Rotate Right (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ROR--register---Rotate-Right--register---an-alias-of-RORV-?lang=en)
    ///
    /// Rotate Right (register) provides the value of the contents of a register rotated by a variable number of bits. The bits that are rotated off the right end are inserted into the vacated bit positions on the left. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is right-shifted.
    ///
    /// ```asm
    /// ROR <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn ror_32(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        self.rorv_32(wd, wn, wm)
    }

    /// [ROR - Rotate Right (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ROR--register---Rotate-Right--register---an-alias-of-RORV-?lang=en)
    ///
    /// Rotate Right (register) provides the value of the contents of a register rotated by a variable number of bits. The bits that are rotated off the right end are inserted into the vacated bit positions on the left. The remainder obtained by dividing the second source register by the data size defines the number of bits by which the first source register is right-shifted.
    ///
    /// ```asm
    /// ROR <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn ror_64(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        self.rorv_64(xd, xn, xm)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_udiv() {
        let mut prod = TestProducer::new();

        let instr = prod.udiv_32(3, 4, 2);
        assert_eq!(instr, "udiv w3, w4, w2");

        let instr = prod.udiv_64(3, 4, 2);
        assert_eq!(instr, "udiv x3, x4, x2");
    }

    #[test]
    fn test_sdiv() {
        let mut prod = TestProducer::new();

        let instr = prod.sdiv_32(3, 4, 2);
        assert_eq!(instr, "sdiv w3, w4, w2");

        let instr = prod.sdiv_64(3, 4, 2);
        assert_eq!(instr, "sdiv x3, x4, x2");
    }

    #[test]
    fn test_lslv() {
        let mut prod = TestProducer::new();

        let instr = prod.lslv_32(3, 4, 2);
        assert_eq!(instr, "lsl w3, w4, w2");

        let instr = prod.lslv_64(3, 4, 2);
        assert_eq!(instr, "lsl x3, x4, x2");
    }

    #[test]
    fn test_lsl() {
        let mut prod = TestProducer::new();

        let instr = prod.lsl_32(3, 4, 2);
        assert_eq!(instr, "lsl w3, w4, w2");

        let instr = prod.lsl_64(3, 4, 2);
        assert_eq!(instr, "lsl x3, x4, x2");
    }

    #[test]
    fn test_lsrv() {
        let mut prod = TestProducer::new();

        let instr = prod.lsrv_32(3, 4, 2);
        assert_eq!(instr, "lsr w3, w4, w2");

        let instr = prod.lsrv_64(3, 4, 2);
        assert_eq!(instr, "lsr x3, x4, x2");
    }

    #[test]
    fn test_lsr() {
        let mut prod = TestProducer::new();

        let instr = prod.lsr_32(3, 4, 2);
        assert_eq!(instr, "lsr w3, w4, w2");

        let instr = prod.lsr_64(3, 4, 2);
        assert_eq!(instr, "lsr x3, x4, x2");
    }

    #[test]
    fn test_asrv() {
        let mut prod = TestProducer::new();

        let instr = prod.asrv_32(3, 4, 2);
        assert_eq!(instr, "asr w3, w4, w2");

        let instr = prod.asrv_64(3, 4, 2);
        assert_eq!(instr, "asr x3, x4, x2");
    }

    #[test]
    fn test_asr() {
        let mut prod = TestProducer::new();

        let instr = prod.asr_32(3, 4, 2);
        assert_eq!(instr, "asr w3, w4, w2");

        let instr = prod.asr_64(3, 4, 2);
        assert_eq!(instr, "asr x3, x4, x2");
    }

    #[test]
    fn test_rorv() {
        let mut prod = TestProducer::new();

        let instr = prod.rorv_32(3, 4, 2);
        assert_eq!(instr, "ror w3, w4, w2");

        let instr = prod.rorv_64(3, 4, 2);
        assert_eq!(instr, "ror x3, x4, x2");
    }

    #[test]
    fn test_ror() {
        let mut prod = TestProducer::new();

        let instr = prod.ror_32(3, 4, 2);
        assert_eq!(instr, "ror w3, w4, w2");

        let instr = prod.ror_64(3, 4, 2);
        assert_eq!(instr, "ror x3, x4, x2");
    }

    #[test]
    fn test_crc32x() {
        let mut prod = TestProducer::new();

        let instr = prod.crc32b(3, 4, 2);
        assert_eq!(instr, "crc32b w3, w4, w2");

        let instr = prod.crc32h(3, 4, 2);
        assert_eq!(instr, "crc32h w3, w4, w2");

        let instr = prod.crc32w(3, 4, 2);
        assert_eq!(instr, "crc32w w3, w4, w2");

        let instr = prod.crc32x(3, 4, 2);
        assert_eq!(instr, "crc32x w3, w4, x2");
    }

    #[test]
    fn test_crc32cx() {
        let mut prod = TestProducer::new();

        let instr = prod.crc32cb(3, 4, 2);
        assert_eq!(instr, "crc32cb w3, w4, w2");

        let instr = prod.crc32ch(3, 4, 2);
        assert_eq!(instr, "crc32ch w3, w4, w2");

        let instr = prod.crc32cw(3, 4, 2);
        assert_eq!(instr, "crc32cw w3, w4, w2");

        let instr = prod.crc32cx(3, 4, 2);
        assert_eq!(instr, "crc32cx w3, w4, x2");
    }

    #[test]
    fn test_subp() {
        let mut prod = TestProducer::new();

        let instr = prod.subp(3, 4, 2);
        assert_eq!(instr, "subp x3, x4, x2");
    }

    #[test]
    fn test_irg() {
        let mut prod = TestProducer::new();

        let instr = prod.irg(3, 4, 2);
        assert_eq!(instr, "irg x3, x4, x2");
    }

    #[test]
    fn test_gmi() {
        let mut prod = TestProducer::new();

        let instr = prod.gmi(3, 4, 2);
        assert_eq!(instr, "gmi x3, x4, x2");
    }

    #[test]
    fn test_pacga() {
        let mut prod = TestProducer::new();

        let instr = prod.pacga(3, 4, 2);
        assert_eq!(instr, "pacga x3, x4, x2");
    }

    #[test]
    fn test_subps() {
        let mut prod = TestProducer::new();

        let instr = prod.subps(3, 4, 2);
        assert_eq!(instr, "subps x3, x4, x2");
    }

}