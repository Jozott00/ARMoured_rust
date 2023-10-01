//! # Data-processing (1 source)
//!
//! Implements the following instructions:
//!  - [RBIT - Reverse Bits](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RBIT--Reverse-Bits-?lang=en)
//!  - [REV16 - Reverse bytes in 16 bit halfwords](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV16--Reverse-bytes-in-16-bit-halfwords-?lang=en)
//!  - [REV - Reverse Bytes](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV--Reverse-Bytes-?lang=en)
//!  - [CLZ - Count Leading Zeros](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLZ--Count-Leading-Zeros-?lang=en)
//!  - [CLS - Count Leading Sign bits](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLS--Count-Leading-Sign-bits-?lang=en)
//!  - [REV32 - Reverse bytes in 32 bit words](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV32--Reverse-bytes-in-32-bit-words-?lang=en)
//!  - [PACIA - PACIA1716 - PACIASP - PACIAZ - PACIZA - Pointer Authentication Code for Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIA--PACIA1716--PACIASP--PACIAZ--PACIZA--Pointer-Authentication-Code-for-Instruction-address--using-key-A-?lang=en)
//!  - [PACIB - PACIB1716 - PACIBSP - PACIBZ - PACIZB - Pointer Authentication Code for Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIB--PACIB1716--PACIBSP--PACIBZ--PACIZB--Pointer-Authentication-Code-for-Instruction-address--using-key-B-?lang=en)
//!  - [PACDA - PACDZA - Pointer Authentication Code for Data address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACDA--PACDZA--Pointer-Authentication-Code-for-Data-address--using-key-A-?lang=en)
//!  - [PACDB - PACDZB - Pointer Authentication Code for Data address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACDB--PACDZB--Pointer-Authentication-Code-for-Data-address--using-key-B-?lang=en)
//!  - [AUTIA - AUTIA1716 - AUTIASP - AUTIAZ - AUTIZA - Authenticate Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIA--AUTIA1716--AUTIASP--AUTIAZ--AUTIZA--Authenticate-Instruction-address--using-key-A-?lang=en)
//!  - [AUTIB - AUTIB1716 - AUTIBSP - AUTIBZ - AUTIZB - Authenticate Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIB--AUTIB1716--AUTIBSP--AUTIBZ--AUTIZB--Authenticate-Instruction-address--using-key-B-?lang=en)
//!  - [AUTDA - AUTDZA - Authenticate Data address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTDA--AUTDZA--Authenticate-Data-address--using-key-A-?lang=en)
//!  - [AUTDB - AUTDZB - Authenticate Data address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTDB--AUTDZB--Authenticate-Data-address--using-key-B-?lang=en)
//!  - [XPACD - XPACI - XPACLRI - Strip Pointer Authentication Code](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/XPACD--XPACI--XPACLRI--Strip-Pointer-Authentication-Code-?lang=en)

use bit_seq::{bseq_32, bseq_8};

use crate::instruction_encoding::InstructionProcessor;
use crate::types::Register;

#[inline(always)]
fn emit_data_proc_one<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: u8,
    s: u8,
    opcode2: u8,
    opcode: u8,
    rn: Register,
    rd: Register,
) -> T {
    let i = bseq_32!(sf:1 1 s:1 11010110 opcode2:5 opcode:6 rn:5 rd:5);
    proc.process(i)
}

#[inline(always)]
fn emit_sys_instrs<P: InstructionProcessor<T>, T>(proc: &mut P, crm: u8, op2: u8) -> T {
    let i = bseq_32!(110 10101 0:6 110010 crm:4 op2:3 !0:5);
    proc.process(i)
}

/// # Data-processing (1 source)
///
/// Implements the following instructions:
///  - [RBIT - Reverse Bits](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RBIT--Reverse-Bits-?lang=en)
///  - [REV16 - Reverse bytes in 16 bit halfwords](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV16--Reverse-bytes-in-16-bit-halfwords-?lang=en)
///  - [REV - Reverse Bytes](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV--Reverse-Bytes-?lang=en)
///  - [CLZ - Count Leading Zeros](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLZ--Count-Leading-Zeros-?lang=en)
///  - [CLS - Count Leading Sign bits](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLS--Count-Leading-Sign-bits-?lang=en)
///  - [REV32 - Reverse bytes in 32 bit words](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV32--Reverse-bytes-in-32-bit-words-?lang=en)
///  - [PACIA - PACIA1716 - PACIASP - PACIAZ - PACIZA - Pointer Authentication Code for Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIA--PACIA1716--PACIASP--PACIAZ--PACIZA--Pointer-Authentication-Code-for-Instruction-address--using-key-A-?lang=en)
///  - [PACIB - PACIB1716 - PACIBSP - PACIBZ - PACIZB - Pointer Authentication Code for Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIB--PACIB1716--PACIBSP--PACIBZ--PACIZB--Pointer-Authentication-Code-for-Instruction-address--using-key-B-?lang=en)
///  - [PACDA - PACDZA - Pointer Authentication Code for Data address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACDA--PACDZA--Pointer-Authentication-Code-for-Data-address--using-key-A-?lang=en)
///  - [PACDB - PACDZB - Pointer Authentication Code for Data address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACDB--PACDZB--Pointer-Authentication-Code-for-Data-address--using-key-B-?lang=en)
///  - [AUTIA - AUTIA1716 - AUTIASP - AUTIAZ - AUTIZA - Authenticate Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIA--AUTIA1716--AUTIASP--AUTIAZ--AUTIZA--Authenticate-Instruction-address--using-key-A-?lang=en)
///  - [AUTIB - AUTIB1716 - AUTIBSP - AUTIBZ - AUTIZB - Authenticate Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIB--AUTIB1716--AUTIBSP--AUTIBZ--AUTIZB--Authenticate-Instruction-address--using-key-B-?lang=en)
///  - [AUTDA - AUTDZA - Authenticate Data address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTDA--AUTDZA--Authenticate-Data-address--using-key-A-?lang=en)
///  - [AUTDB - AUTDZB - Authenticate Data address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTDB--AUTDZB--Authenticate-Data-address--using-key-B-?lang=en)
///  - [XPACD - XPACI - XPACLRI - Strip Pointer Authentication Code](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/XPACD--XPACI--XPACLRI--Strip-Pointer-Authentication-Code-?lang=en)
pub trait DataProcessingOneSource<T>: InstructionProcessor<T> {
    /// [RBIT - Reverse Bits](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RBIT--Reverse-Bits-?lang=en)
    ///
    /// Reverse Bits reverses the bit order in a register.
    ///
    /// ```asm
    /// RBIT <Wd>, <Wn>
    /// ```
    #[inline(always)]
    fn rbit_32(&mut self, wd: Register, wn: Register) -> T {
        emit_data_proc_one(self, 0, 0, 0, 0, wn, wd)
    }

    /// [RBIT - Reverse Bits](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RBIT--Reverse-Bits-?lang=en)
    ///
    /// Reverse Bits reverses the bit order in a register.
    ///
    /// ```asm
    /// RBIT <Xd>, <Xn>
    /// ```
    #[inline(always)]
    fn rbit_64(&mut self, xd: Register, xn: Register) -> T {
        emit_data_proc_one(self, 1, 0, 0, 0, xn, xd)
    }

    /// [REV16 - Reverse bytes in 16 bit halfwords](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV16--Reverse-bytes-in-16-bit-halfwords-?lang=en)
    ///
    /// Reverse bytes in 16-bit halfwords reverses the byte order in each 16-bit halfword of a register.
    ///
    /// ```asm
    /// REV16 <Wd>, <Wn>
    /// ```
    #[inline(always)]
    fn rev16_32(&mut self, wd: Register, wn: Register) -> T {
        emit_data_proc_one(self, 0, 0, 0, 1, wn, wd)
    }

    /// [REV16 - Reverse bytes in 16 bit halfwords](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV16--Reverse-bytes-in-16-bit-halfwords-?lang=en)
    ///
    /// Reverse bytes in 16-bit halfwords reverses the byte order in each 16-bit halfword of a register.
    ///
    /// ```asm
    /// REV16 <Xd>, <Xn>
    /// ```
    #[inline(always)]
    fn rev16_64(&mut self, xd: Register, xn: Register) -> T {
        emit_data_proc_one(self, 1, 0, 0, 1, xn, xd)
    }

    /// [REV - Reverse Bytes](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV--Reverse-Bytes-?lang=en)
    ///
    /// Reverse Bytes reverses the byte order in a register.
    ///
    /// This instruction is used by the pseudo-instruction REV64.
    ///
    /// ```asm
    /// REV <Wd>, <Wn>
    /// ```
    #[inline(always)]
    fn rev_32(&mut self, wd: Register, wn: Register) -> T {
        emit_data_proc_one(self, 0, 0, 0, 0b10, wn, wd)
    }

    /// [REV - Reverse Bytes](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV--Reverse-Bytes-?lang=en)
    ///
    /// Reverse Bytes reverses the byte order in a register.
    ///
    /// This instruction is used by the pseudo-instruction REV64.
    ///
    /// ```asm
    /// REV <Xd>, <Xn>
    /// ```
    #[inline(always)]
    fn rev_64(&mut self, xd: Register, xn: Register) -> T {
        emit_data_proc_one(self, 1, 0, 0, 0b11, xn, xd)
    }

    /// [CLZ - Count Leading Zeros](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLZ--Count-Leading-Zeros-?lang=en)
    ///
    /// Count Leading Zeros counts the number of binary zero bits before the first binary one bit in the value of the source register, and writes the result to the destination register.
    ///
    /// ```asm
    /// CLZ <Wd>, <Wn>
    /// ```
    #[inline(always)]
    fn clz_32(&mut self, wd: Register, wn: Register) -> T {
        emit_data_proc_one(self, 0, 0, 0, 0b100, wn, wd)
    }

    /// [CLZ - Count Leading Zeros](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLZ--Count-Leading-Zeros-?lang=en)
    ///
    /// Count Leading Zeros counts the number of binary zero bits before the first binary one bit in the value of the source register, and writes the result to the destination register.
    ///
    /// ```asm
    /// CLZ <Xd>, <Xn>
    /// ```
    #[inline(always)]
    fn clz_64(&mut self, xd: Register, xn: Register) -> T {
        emit_data_proc_one(self, 1, 0, 0, 0b100, xn, xd)
    }

    /// [CLS - Count Leading Sign bits](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLS--Count-Leading-Sign-bits-?lang=en)
    ///
    /// Count Leading Sign bits counts the number of leading bits of the source register that have the same value as the most significant bit of the register, and writes the result to the destination register. This count does not include the most significant bit of the source register.
    ///
    /// ```asm
    /// CLS <Wd>, <Wn>
    /// ```
    #[inline(always)]
    fn cls_32(&mut self, wd: Register, wn: Register) -> T {
        emit_data_proc_one(self, 0, 0, 0, 0b101, wn, wd)
    }

    /// [CLS - Count Leading Sign bits](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLS--Count-Leading-Sign-bits-?lang=en)
    ///
    /// Count Leading Sign bits counts the number of leading bits of the source register that have the same value as the most significant bit of the register, and writes the result to the destination register. This count does not include the most significant bit of the source register.
    ///
    /// ```asm
    /// CLS <Xd>, <Xn>
    /// ```
    #[inline(always)]
    fn cls_64(&mut self, xd: Register, xn: Register) -> T {
        emit_data_proc_one(self, 1, 0, 0, 0b101, xn, xd)
    }

    /// [REV32 - Reverse bytes in 32 bit words](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV32--Reverse-bytes-in-32-bit-words-?lang=en)
    ///
    /// Reverse bytes in 32-bit words reverses the byte order in each 32-bit word of a register.
    ///
    /// ```asm
    /// REV32 <Xd>, <Xn>
    /// ```
    #[inline(always)]
    fn rev32(&mut self, xd: Register, xn: Register) -> T {
        emit_data_proc_one(self, 1, 0, 0, 0b10, xn, xd)
    }

    /// [PACIA - PACIA1716 - PACIASP - PACIAZ - PACIZA - Pointer Authentication Code for Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIA--PACIA1716--PACIASP--PACIAZ--PACIZA--Pointer-Authentication-Code-for-Instruction-address--using-key-A-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key A. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key A.
    ///
    /// ```asm
    /// PACIA <Xd>, <Xn|SP>
    /// ```
    #[inline(always)]
    fn pacia(&mut self, xd: Register, xn_sp: Register) -> T {
        let z = 0;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 000), xn_sp, xd)
    }

    /// [PACIA - PACIA1716 - PACIASP - PACIAZ - PACIZA - Pointer Authentication Code for Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIA--PACIA1716--PACIASP--PACIAZ--PACIZA--Pointer-Authentication-Code-for-Instruction-address--using-key-A-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key A. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key A.
    ///
    /// ```asm
    /// PACIZA <Xd>
    /// ```
    #[inline(always)]
    fn paciza(&mut self, xd: Register) -> T {
        let z = 1;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 000), 0b11111, xd)
    }

    /// [PACIA - PACIA1716 - PACIASP - PACIAZ - PACIZA - Pointer Authentication Code for Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIA--PACIA1716--PACIASP--PACIAZ--PACIZA--Pointer-Authentication-Code-for-Instruction-address--using-key-A-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key A. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key A.
    ///
    /// ```asm
    /// PACIA1716
    /// ```
    #[inline(always)]
    fn pacia1716(&mut self) -> T {
        emit_sys_instrs(self, 1, 0)
    }

    /// [PACIA - PACIA1716 - PACIASP - PACIAZ - PACIZA - Pointer Authentication Code for Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIA--PACIA1716--PACIASP--PACIAZ--PACIZA--Pointer-Authentication-Code-for-Instruction-address--using-key-A-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key A. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key A.
    ///
    /// ```asm
    /// PACIASP
    /// ```
    #[inline(always)]
    fn paciasp(&mut self) -> T {
        emit_sys_instrs(self, 0b11, 1)
    }

    /// [PACIA - PACIA1716 - PACIASP - PACIAZ - PACIZA - Pointer Authentication Code for Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIA--PACIA1716--PACIASP--PACIAZ--PACIZA--Pointer-Authentication-Code-for-Instruction-address--using-key-A-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key A. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key A.
    ///
    /// ```asm
    /// PACIAZ
    /// ```
    #[inline(always)]
    fn paciaz(&mut self) -> T {
        emit_sys_instrs(self, 0b11, 0)
    }

    /// [PACIB - PACIB1716 - PACIBSP - PACIBZ - PACIZB - Pointer Authentication Code for Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIB--PACIB1716--PACIBSP--PACIBZ--PACIZB--Pointer-Authentication-Code-for-Instruction-address--using-key-B-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key B. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key B.
    ///
    /// ```asm
    /// PACIB <Xd>, <Xn|SP>
    /// ```
    #[inline(always)]
    fn pacib(&mut self, xd: Register, xn_sp: Register) -> T {
        let z = 0;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 001), xn_sp, xd)
    }

    /// [PACIB - PACIB1716 - PACIBSP - PACIBZ - PACIZB - Pointer Authentication Code for Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIB--PACIB1716--PACIBSP--PACIBZ--PACIZB--Pointer-Authentication-Code-for-Instruction-address--using-key-B-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key B. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key B.
    ///
    /// ```asm
    /// PACIZB <Xd>
    /// ```
    #[inline(always)]
    fn pacizb(&mut self, xd: Register) -> T {
        let z = 1;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 001), 0b11111, xd)
    }

    /// [PACIB - PACIB1716 - PACIBSP - PACIBZ - PACIZB - Pointer Authentication Code for Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIB--PACIB1716--PACIBSP--PACIBZ--PACIZB--Pointer-Authentication-Code-for-Instruction-address--using-key-B-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key B. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key B.
    ///
    /// ```asm
    /// PACIB1716
    /// ```
    #[inline(always)]
    fn pacib1716(&mut self) -> T {
        emit_sys_instrs(self, 1, 0b10)
    }

    /// [PACIB - PACIB1716 - PACIBSP - PACIBZ - PACIZB - Pointer Authentication Code for Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIB--PACIB1716--PACIBSP--PACIBZ--PACIZB--Pointer-Authentication-Code-for-Instruction-address--using-key-B-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key B. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key B.
    ///
    /// ```asm
    /// PACIBSP
    /// ```
    #[inline(always)]
    fn pacibsp(&mut self) -> T {
        emit_sys_instrs(self, 0b11, 0b11)
    }

    /// [PACIB - PACIB1716 - PACIBSP - PACIBZ - PACIZB - Pointer Authentication Code for Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIB--PACIB1716--PACIBSP--PACIBZ--PACIZB--Pointer-Authentication-Code-for-Instruction-address--using-key-B-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key B. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key B.
    ///
    /// ```asm
    /// PACIBZ
    /// ```
    #[inline(always)]
    fn pacibz(&mut self) -> T {
        emit_sys_instrs(self, 0b11, 0b10)
    }

    /// [PACDA - PACDZA - Pointer Authentication Code for Data address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACDA--PACDZA--Pointer-Authentication-Code-for-Data-address--using-key-A-?lang=en)
    ///
    /// Pointer Authentication Code for Data address, using key A. This instruction computes and inserts a pointer authentication code for a data address, using a modifier and key A.
    ///
    /// The address is in the general-purpose register that is specified by <Xd>.
    ///
    /// ```asm
    /// PACDA <Xd>, <Xn|SP>
    /// ```
    #[inline(always)]
    fn pacda(&mut self, xd: Register, xn_sp: Register) -> T {
        let z = 0;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 010), xn_sp, xd)
    }

    /// [PACDA - PACDZA - Pointer Authentication Code for Data address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACDA--PACDZA--Pointer-Authentication-Code-for-Data-address--using-key-A-?lang=en)
    ///
    /// Pointer Authentication Code for Data address, using key A. This instruction computes and inserts a pointer authentication code for a data address, using a modifier and key A.
    ///
    /// The address is in the general-purpose register that is specified by <Xd>.
    ///
    /// ```asm
    /// PACDZA <Xd>
    /// ```
    #[inline(always)]
    fn pacdza(&mut self, xd: Register) -> T {
        let z = 1;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 010), 0b11111, xd)
    }

    /// [PACDB - PACDZB - Pointer Authentication Code for Data address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACDB--PACDZB--Pointer-Authentication-Code-for-Data-address--using-key-B-?lang=en)
    ///
    /// Pointer Authentication Code for Data address, using key B. This instruction computes and inserts a pointer authentication code for a data address, using a modifier and key B.
    ///
    /// The address is in the general-purpose register that is specified by <Xd>.
    ///
    /// ```asm
    /// PACDB <Xd>, <Xn|SP>
    /// ```
    #[inline(always)]
    fn pacdb(&mut self, xd: Register, xn_sp: Register) -> T {
        let z = 0;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 011), xn_sp, xd)
    }

    /// [PACDB - PACDZB - Pointer Authentication Code for Data address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACDB--PACDZB--Pointer-Authentication-Code-for-Data-address--using-key-B-?lang=en)
    ///
    /// Pointer Authentication Code for Data address, using key B. This instruction computes and inserts a pointer authentication code for a data address, using a modifier and key B.
    ///
    /// The address is in the general-purpose register that is specified by <Xd>.
    ///
    /// ```asm
    /// PACDZB <Xd>
    /// ```
    #[inline(always)]
    fn pacdzb(&mut self, xd: Register) -> T {
        let z = 1;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 011), 0b11111, xd)
    }

    /// [AUTIA - AUTIA1716 - AUTIASP - AUTIAZ - AUTIZA - Authenticate Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIA--AUTIA1716--AUTIASP--AUTIAZ--AUTIZA--Authenticate-Instruction-address--using-key-A-?lang=en)
    ///
    /// Authenticate Instruction address, using key A. This instruction authenticates an instruction address, using a modifier and key A.
    ///
    /// ```asm
    /// AUTIA <Xd>, <Xn|SP>
    /// ```
    #[inline(always)]
    fn autia(&mut self, xd: Register, xn_sp: Register) -> T {
        let z = 0;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 100), xn_sp, xd)
    }

    /// [AUTIA - AUTIA1716 - AUTIASP - AUTIAZ - AUTIZA - Authenticate Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIA--AUTIA1716--AUTIASP--AUTIAZ--AUTIZA--Authenticate-Instruction-address--using-key-A-?lang=en)
    ///
    /// Authenticate Instruction address, using key A. This instruction authenticates an instruction address, using a modifier and key A.
    ///
    /// ```asm
    /// AUTIZA <Xd>
    /// ```
    #[inline(always)]
    fn autiza(&mut self, xd: Register) -> T {
        let z = 1;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 100), 0b11111, xd)
    }

    /// [AUTIA - AUTIA1716 - AUTIASP - AUTIAZ - AUTIZA - Authenticate Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIA--AUTIA1716--AUTIASP--AUTIAZ--AUTIZA--Authenticate-Instruction-address--using-key-A-?lang=en)
    ///
    /// Authenticate Instruction address, using key A. This instruction authenticates an instruction address, using a modifier and key A.
    ///
    /// ```asm
    /// AUTIA1716
    /// ```
    #[inline(always)]
    fn autia1716(&mut self) -> T {
        emit_sys_instrs(self, 1, 0b100)
    }

    /// [AUTIA - AUTIA1716 - AUTIASP - AUTIAZ - AUTIZA - Authenticate Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIA--AUTIA1716--AUTIASP--AUTIAZ--AUTIZA--Authenticate-Instruction-address--using-key-A-?lang=en)
    ///
    /// Authenticate Instruction address, using key A. This instruction authenticates an instruction address, using a modifier and key A.
    ///
    /// ```asm
    /// AUTIASP
    /// ```
    #[inline(always)]
    fn autiasp(&mut self) -> T {
        emit_sys_instrs(self, 0b11, 0b101)
    }

    /// [AUTIA - AUTIA1716 - AUTIASP - AUTIAZ - AUTIZA - Authenticate Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIA--AUTIA1716--AUTIASP--AUTIAZ--AUTIZA--Authenticate-Instruction-address--using-key-A-?lang=en)
    ///
    /// Authenticate Instruction address, using key A. This instruction authenticates an instruction address, using a modifier and key A.
    ///
    /// ```asm
    /// AUTIAZ
    /// ```
    #[inline(always)]
    fn autiaz(&mut self) -> T {
        emit_sys_instrs(self, 0b11, 0b100)
    }

    /// [AUTIB - AUTIB1716 - AUTIBSP - AUTIBZ - AUTIZB - Authenticate Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIB--AUTIB1716--AUTIBSP--AUTIBZ--AUTIZB--Authenticate-Instruction-address--using-key-B-?lang=en)
    ///
    /// Authenticate Instruction address, using key B. This instruction authenticates an instruction address, using a modifier and key B.
    ///
    /// ```asm
    /// AUTIB <Xd>, <Xn|SP>
    /// ```
    #[inline(always)]
    fn autib(&mut self, xd: Register, xn_sp: Register) -> T {
        let z = 0;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 101), xn_sp, xd)
    }

    /// [AUTIB - AUTIB1716 - AUTIBSP - AUTIBZ - AUTIZB - Authenticate Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIB--AUTIB1716--AUTIBSP--AUTIBZ--AUTIZB--Authenticate-Instruction-address--using-key-B-?lang=en)
    ///
    /// Authenticate Instruction address, using key B. This instruction authenticates an instruction address, using a modifier and key B.
    ///
    /// ```asm
    /// AUTIZB <Xd>
    /// ```
    #[inline(always)]
    fn autizb(&mut self, xd: Register) -> T {
        let z = 1;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 101), 0b11111, xd)
    }

    /// [AUTIB - AUTIB1716 - AUTIBSP - AUTIBZ - AUTIZB - Authenticate Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIB--AUTIB1716--AUTIBSP--AUTIBZ--AUTIZB--Authenticate-Instruction-address--using-key-B-?lang=en)
    ///
    /// Authenticate Instruction address, using key B. This instruction authenticates an instruction address, using a modifier and key B.
    ///
    /// ```asm
    /// AUTIB1716
    /// ```
    #[inline(always)]
    fn autib1716(&mut self) -> T {
        emit_sys_instrs(self, 0b1, 0b110)
    }

    /// [AUTIB - AUTIB1716 - AUTIBSP - AUTIBZ - AUTIZB - Authenticate Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIB--AUTIB1716--AUTIBSP--AUTIBZ--AUTIZB--Authenticate-Instruction-address--using-key-B-?lang=en)
    ///
    /// Authenticate Instruction address, using key B. This instruction authenticates an instruction address, using a modifier and key B.
    ///
    /// ```asm
    /// AUTIBSP
    /// ```
    #[inline(always)]
    fn autibsp(&mut self) -> T {
        emit_sys_instrs(self, 0b11, 0b111)
    }

    /// [AUTIB - AUTIB1716 - AUTIBSP - AUTIBZ - AUTIZB - Authenticate Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIB--AUTIB1716--AUTIBSP--AUTIBZ--AUTIZB--Authenticate-Instruction-address--using-key-B-?lang=en)
    ///
    /// Authenticate Instruction address, using key B. This instruction authenticates an instruction address, using a modifier and key B.
    ///
    /// ```asm
    /// AUTIBZ
    /// ```
    #[inline(always)]
    fn autibz(&mut self) -> T {
        emit_sys_instrs(self, 0b11, 0b110)
    }

    /// [AUTDA - AUTDZA - Authenticate Data address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTDA--AUTDZA--Authenticate-Data-address--using-key-A-?lang=en)
    ///
    /// Authenticate Data address, using key A. This instruction authenticates a data address, using a modifier and key A.
    ///
    /// The address is in the general-purpose register that is specified by <Xd>.
    ///
    /// ```asm
    /// AUTDA <Xd>, <Xn|SP>
    /// ```
    #[inline(always)]
    fn autda(&mut self, xd: Register, xn_sp: Register) -> T {
        let z = 0;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 110), xn_sp, xd)
    }

    /// [AUTDA - AUTDZA - Authenticate Data address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTDA--AUTDZA--Authenticate-Data-address--using-key-A-?lang=en)
    ///
    /// Authenticate Data address, using key A. This instruction authenticates a data address, using a modifier and key A.
    ///
    /// The address is in the general-purpose register that is specified by <Xd>.
    ///
    /// ```asm
    /// AUTDZA <Xd>
    /// ```
    #[inline(always)]
    fn autdza(&mut self, xd: Register) -> T {
        let z = 1;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 110), 0b11111, xd)
    }

    /// [AUTDB - AUTDZB - Authenticate Data address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTDB--AUTDZB--Authenticate-Data-address--using-key-B-?lang=en)
    ///
    /// Authenticate Data address, using key B. This instruction authenticates a data address, using a modifier and key B.
    ///
    /// The address is in the general-purpose register that is specified by <Xd>.
    ///
    /// ```asm
    /// AUTDB <Xd>, <Xn|SP>
    /// ```
    #[inline(always)]
    fn autdb(&mut self, xd: Register, xn_sp: Register) -> T {
        let z = 0;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 111), xn_sp, xd)
    }

    /// [AUTDB - AUTDZB - Authenticate Data address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTDB--AUTDZB--Authenticate-Data-address--using-key-B-?lang=en)
    ///
    /// Authenticate Data address, using key B. This instruction authenticates a data address, using a modifier and key B.
    ///
    /// The address is in the general-purpose register that is specified by <Xd>.
    ///
    /// ```asm
    /// AUTDZB <Xd>
    /// ```
    #[inline(always)]
    fn autdzb(&mut self, xd: Register) -> T {
        let z = 1;
        emit_data_proc_one(self, 1, 0, 1, bseq_8!(z:1 111), 0b11111, xd)
    }

    /// [XPACD - XPACI - XPACLRI - Strip Pointer Authentication Code](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/XPACD--XPACI--XPACLRI--Strip-Pointer-Authentication-Code-?lang=en)
    ///
    /// Strip Pointer Authentication Code. This instruction removes the pointer authentication code from an address. The address is in the specified general-purpose register for XPACI and XPACD, and is in LR for XPACLRI.
    ///
    /// The XPACD instruction is used for data addresses, and XPACI and XPACLRI are used for instruction addresses.
    ///
    /// ```asm
    /// XPACD <Xd>
    /// ```
    #[inline(always)]
    fn xpacd(&mut self, xd: Register) -> T {
        emit_data_proc_one(self, 1, 0, 1, 0b10001, 0b11111, xd)
    }

    /// [XPACD - XPACI - XPACLRI - Strip Pointer Authentication Code](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/XPACD--XPACI--XPACLRI--Strip-Pointer-Authentication-Code-?lang=en)
    ///
    /// Strip Pointer Authentication Code. This instruction removes the pointer authentication code from an address. The address is in the specified general-purpose register for XPACI and XPACD, and is in LR for XPACLRI.
    ///
    /// The XPACD instruction is used for data addresses, and XPACI and XPACLRI are used for instruction addresses.
    ///
    /// ```asm
    /// XPACI <Xd>
    /// ```
    #[inline(always)]
    fn xpaci(&mut self, xd: Register) -> T {
        emit_data_proc_one(self, 1, 0, 1, 0b10000, 0b11111, xd)
    }

    /// [XPACD - XPACI - XPACLRI - Strip Pointer Authentication Code](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/XPACD--XPACI--XPACLRI--Strip-Pointer-Authentication-Code-?lang=en)
    ///
    /// Strip Pointer Authentication Code. This instruction removes the pointer authentication code from an address. The address is in the specified general-purpose register for XPACI and XPACD, and is in LR for XPACLRI.
    ///
    /// The XPACD instruction is used for data addresses, and XPACI and XPACLRI are used for instruction addresses.
    ///
    /// ```asm
    /// XPACLRI
    /// ```
    #[inline(always)]
    fn xpaclri(&mut self) -> T {
        emit_sys_instrs(self, 0, 0b111)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_rbit() {
        let mut prod = TestProducer::new();

        let instr = prod.rbit_32(3, 4);
        assert_eq!(instr, "rbit w3, w4");

        let instr = prod.rbit_64(3, 4);
        assert_eq!(instr, "rbit x3, x4");
    }

    #[test]
    fn test_rev16() {
        let mut prod = TestProducer::new();

        let instr = prod.rev16_32(3, 4);
        assert_eq!(instr, "rev16 w3, w4");

        let instr = prod.rev16_64(3, 4);
        assert_eq!(instr, "rev16 x3, x4");
    }

    #[test]
    fn test_rev() {
        let mut prod = TestProducer::new();

        let instr = prod.rev_32(3, 4);
        assert_eq!(instr, "rev w3, w4");

        let instr = prod.rev_64(3, 4);
        assert_eq!(instr, "rev x3, x4");
    }

    #[test]
    fn test_clz() {
        let mut prod = TestProducer::new();

        let instr = prod.clz_32(3, 4);
        assert_eq!(instr, "clz w3, w4");

        let instr = prod.clz_64(3, 4);
        assert_eq!(instr, "clz x3, x4");
    }

    #[test]
    fn test_cls() {
        let mut prod = TestProducer::new();

        let instr = prod.cls_32(3, 4);
        assert_eq!(instr, "cls w3, w4");

        let instr = prod.cls_64(3, 4);
        assert_eq!(instr, "cls x3, x4");
    }

    #[test]
    fn test_rev32() {
        let mut prod = TestProducer::new();
        let instr = prod.rev32(3, 4);
        assert_eq!(instr, "rev32 x3, x4");
    }

    #[test]
    fn test_pacia() {
        let mut prod = TestProducer::new();

        let instr = prod.pacia(3, 4);
        assert_eq!(instr, "pacia x3, x4");

        let instr = prod.paciza(3);
        assert_eq!(instr, "paciza x3");

        let instr = prod.pacia1716();
        assert_eq!(instr, "pacia1716");

        let instr = prod.paciasp();
        assert_eq!(instr, "paciasp");

        let instr = prod.paciaz();
        assert_eq!(instr, "paciaz");
    }

    #[test]
    fn test_pacib() {
        let mut prod = TestProducer::new();

        let instr = prod.pacib(3, 4);
        assert_eq!(instr, "pacib x3, x4");

        let instr = prod.pacizb(3);
        assert_eq!(instr, "pacizb x3");

        let instr = prod.pacib1716();
        assert_eq!(instr, "pacib1716");

        let instr = prod.pacibsp();
        assert_eq!(instr, "pacibsp");

        let instr = prod.pacibz();
        assert_eq!(instr, "pacibz");
    }

    #[test]
    fn test_pacda() {
        let mut prod = TestProducer::new();

        let instr = prod.pacda(3, 4);
        assert_eq!(instr, "pacda x3, x4");

        let instr = prod.pacdza(3);
        assert_eq!(instr, "pacdza x3");
    }

    #[test]
    fn test_pacdb() {
        let mut prod = TestProducer::new();

        let instr = prod.pacdb(3, 4);
        assert_eq!(instr, "pacdb x3, x4");

        let instr = prod.pacdzb(3);
        assert_eq!(instr, "pacdzb x3");
    }

    #[test]
    fn test_autia() {
        let mut prod = TestProducer::new();

        let instr = prod.autia(3, 4);
        assert_eq!(instr, "autia x3, x4");

        let instr = prod.autiza(3);
        assert_eq!(instr, "autiza x3");

        let instr = prod.autia1716();
        assert_eq!(instr, "autia1716");

        let instr = prod.autiasp();
        assert_eq!(instr, "autiasp");

        let instr = prod.autiaz();
        assert_eq!(instr, "autiaz");
    }

    #[test]
    fn test_autib() {
        let mut prod = TestProducer::new();

        let instr = prod.autib(3, 4);
        assert_eq!(instr, "autib x3, x4");

        let instr = prod.autizb(3);
        assert_eq!(instr, "autizb x3");

        let instr = prod.autib1716();
        assert_eq!(instr, "autib1716");

        let instr = prod.autibsp();
        assert_eq!(instr, "autibsp");

        let instr = prod.autibz();
        assert_eq!(instr, "autibz");
    }

    #[test]
    fn test_autda() {
        let mut prod = TestProducer::new();

        let instr = prod.autda(3, 4);
        assert_eq!(instr, "autda x3, x4");

        let instr = prod.autdza(3);
        assert_eq!(instr, "autdza x3");
    }

    #[test]
    fn test_autdb() {
        let mut prod = TestProducer::new();

        let instr = prod.autdb(3, 4);
        assert_eq!(instr, "autdb x3, x4");

        let instr = prod.autdzb(3);
        assert_eq!(instr, "autdzb x3");
    }

    #[test]
    fn test_xpac() {
        let mut prod = TestProducer::new();

        let instr = prod.xpacd(3);
        assert_eq!(instr, "xpacd x3");

        let instr = prod.xpaci(3);
        assert_eq!(instr, "xpaci x3");

        let instr = prod.xpaclri();
        assert_eq!(instr, "xpaclri");
    }
}
