//! # Data Processing (Register)
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



use crate::instruction_encoding::InstructionProcessor;

/// # Data Processing (Register)
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
    fn f0(&mut self) -> T {
        todo!()
    }


    /// [RBIT - Reverse Bits](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/RBIT--Reverse-Bits-?lang=en)
    ///
    /// Reverse Bits reverses the bit order in a register.
    ///
    /// ```asm
    /// RBIT <Xd>, <Xn>
    /// ```
    fn f1(&mut self) -> T {
        todo!()
    }


    /// [REV16 - Reverse bytes in 16 bit halfwords](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV16--Reverse-bytes-in-16-bit-halfwords-?lang=en)
    ///
    /// Reverse bytes in 16-bit halfwords reverses the byte order in each 16-bit halfword of a register.
    ///
    /// ```asm
    /// REV16 <Wd>, <Wn>
    /// ```
    fn f2(&mut self) -> T {
        todo!()
    }


    /// [REV16 - Reverse bytes in 16 bit halfwords](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV16--Reverse-bytes-in-16-bit-halfwords-?lang=en)
    ///
    /// Reverse bytes in 16-bit halfwords reverses the byte order in each 16-bit halfword of a register.
    ///
    /// ```asm
    /// REV16 <Xd>, <Xn>
    /// ```
    fn f3(&mut self) -> T {
        todo!()
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
    fn f4(&mut self) -> T {
        todo!()
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
    fn f5(&mut self) -> T {
        todo!()
    }


    /// [CLZ - Count Leading Zeros](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLZ--Count-Leading-Zeros-?lang=en)
    ///
    /// Count Leading Zeros counts the number of binary zero bits before the first binary one bit in the value of the source register, and writes the result to the destination register.
    ///
    /// ```asm
    /// CLZ <Wd>, <Wn>
    /// ```
    fn f6(&mut self) -> T {
        todo!()
    }


    /// [CLZ - Count Leading Zeros](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLZ--Count-Leading-Zeros-?lang=en)
    ///
    /// Count Leading Zeros counts the number of binary zero bits before the first binary one bit in the value of the source register, and writes the result to the destination register.
    ///
    /// ```asm
    /// CLZ <Xd>, <Xn>
    /// ```
    fn f7(&mut self) -> T {
        todo!()
    }


    /// [CLS - Count Leading Sign bits](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLS--Count-Leading-Sign-bits-?lang=en)
    ///
    /// Count Leading Sign bits counts the number of leading bits of the source register that have the same value as the most significant bit of the register, and writes the result to the destination register. This count does not include the most significant bit of the source register.
    ///
    /// ```asm
    /// CLS <Wd>, <Wn>
    /// ```
    fn f8(&mut self) -> T {
        todo!()
    }


    /// [CLS - Count Leading Sign bits](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CLS--Count-Leading-Sign-bits-?lang=en)
    ///
    /// Count Leading Sign bits counts the number of leading bits of the source register that have the same value as the most significant bit of the register, and writes the result to the destination register. This count does not include the most significant bit of the source register.
    ///
    /// ```asm
    /// CLS <Xd>, <Xn>
    /// ```
    fn f9(&mut self) -> T {
        todo!()
    }


    /// [REV32 - Reverse bytes in 32 bit words](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/REV32--Reverse-bytes-in-32-bit-words-?lang=en)
    ///
    /// Reverse bytes in 32-bit words reverses the byte order in each 32-bit word of a register.
    ///
    /// ```asm
    /// REV32 <Xd>, <Xn>
    /// ```
    fn f10(&mut self) -> T {
        todo!()
    }


    /// [PACIA - PACIA1716 - PACIASP - PACIAZ - PACIZA - Pointer Authentication Code for Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIA--PACIA1716--PACIASP--PACIAZ--PACIZA--Pointer-Authentication-Code-for-Instruction-address--using-key-A-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key A. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key A.
    ///
    /// The address is:
    ///
    /// ```asm
    /// PACIA <Xd>, <Xn|SP>
    /// ```
    fn f11(&mut self) -> T {
        todo!()
    }


    /// [PACIA - PACIA1716 - PACIASP - PACIAZ - PACIZA - Pointer Authentication Code for Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIA--PACIA1716--PACIASP--PACIAZ--PACIZA--Pointer-Authentication-Code-for-Instruction-address--using-key-A-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key A. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key A.
    ///
    /// The address is:
    ///
    /// ```asm
    /// PACIZA <Xd>
    /// ```
    fn f12(&mut self) -> T {
        todo!()
    }


    /// [PACIA - PACIA1716 - PACIASP - PACIAZ - PACIZA - Pointer Authentication Code for Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIA--PACIA1716--PACIASP--PACIAZ--PACIZA--Pointer-Authentication-Code-for-Instruction-address--using-key-A-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key A. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key A.
    ///
    /// The address is:
    ///
    /// ```asm
    /// PACIA1716
    /// ```
    fn f13(&mut self) -> T {
        todo!()
    }


    /// [PACIA - PACIA1716 - PACIASP - PACIAZ - PACIZA - Pointer Authentication Code for Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIA--PACIA1716--PACIASP--PACIAZ--PACIZA--Pointer-Authentication-Code-for-Instruction-address--using-key-A-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key A. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key A.
    ///
    /// The address is:
    ///
    /// ```asm
    /// PACIASP
    /// ```
    fn f14(&mut self) -> T {
        todo!()
    }


    /// [PACIA - PACIA1716 - PACIASP - PACIAZ - PACIZA - Pointer Authentication Code for Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIA--PACIA1716--PACIASP--PACIAZ--PACIZA--Pointer-Authentication-Code-for-Instruction-address--using-key-A-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key A. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key A.
    ///
    /// The address is:
    ///
    /// ```asm
    /// PACIAZ
    /// ```
    fn f15(&mut self) -> T {
        todo!()
    }


    /// [PACIB - PACIB1716 - PACIBSP - PACIBZ - PACIZB - Pointer Authentication Code for Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIB--PACIB1716--PACIBSP--PACIBZ--PACIZB--Pointer-Authentication-Code-for-Instruction-address--using-key-B-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key B. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key B.
    ///
    /// The address is:
    ///
    /// ```asm
    /// PACIB <Xd>, <Xn|SP>
    /// ```
    fn f16(&mut self) -> T {
        todo!()
    }


    /// [PACIB - PACIB1716 - PACIBSP - PACIBZ - PACIZB - Pointer Authentication Code for Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIB--PACIB1716--PACIBSP--PACIBZ--PACIZB--Pointer-Authentication-Code-for-Instruction-address--using-key-B-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key B. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key B.
    ///
    /// The address is:
    ///
    /// ```asm
    /// PACIZB <Xd>
    /// ```
    fn f17(&mut self) -> T {
        todo!()
    }


    /// [PACIB - PACIB1716 - PACIBSP - PACIBZ - PACIZB - Pointer Authentication Code for Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIB--PACIB1716--PACIBSP--PACIBZ--PACIZB--Pointer-Authentication-Code-for-Instruction-address--using-key-B-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key B. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key B.
    ///
    /// The address is:
    ///
    /// ```asm
    /// PACIB1716
    /// ```
    fn f18(&mut self) -> T {
        todo!()
    }


    /// [PACIB - PACIB1716 - PACIBSP - PACIBZ - PACIZB - Pointer Authentication Code for Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIB--PACIB1716--PACIBSP--PACIBZ--PACIZB--Pointer-Authentication-Code-for-Instruction-address--using-key-B-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key B. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key B.
    ///
    /// The address is:
    ///
    /// ```asm
    /// PACIBSP
    /// ```
    fn f19(&mut self) -> T {
        todo!()
    }


    /// [PACIB - PACIB1716 - PACIBSP - PACIBZ - PACIZB - Pointer Authentication Code for Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PACIB--PACIB1716--PACIBSP--PACIBZ--PACIZB--Pointer-Authentication-Code-for-Instruction-address--using-key-B-?lang=en)
    ///
    /// Pointer Authentication Code for Instruction address, using key B. This instruction computes and inserts a pointer authentication code for an instruction address, using a modifier and key B.
    ///
    /// The address is:
    ///
    /// ```asm
    /// PACIBZ
    /// ```
    fn f20(&mut self) -> T {
        todo!()
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
    fn f21(&mut self) -> T {
        todo!()
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
    fn f22(&mut self) -> T {
        todo!()
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
    fn f23(&mut self) -> T {
        todo!()
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
    fn f24(&mut self) -> T {
        todo!()
    }


    /// [AUTIA - AUTIA1716 - AUTIASP - AUTIAZ - AUTIZA - Authenticate Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIA--AUTIA1716--AUTIASP--AUTIAZ--AUTIZA--Authenticate-Instruction-address--using-key-A-?lang=en)
    ///
    /// Authenticate Instruction address, using key A. This instruction authenticates an instruction address, using a modifier and key A.
    ///
    /// The address is:
    ///
    /// ```asm
    /// AUTIA <Xd>, <Xn|SP>
    /// ```
    fn f25(&mut self) -> T {
        todo!()
    }


    /// [AUTIA - AUTIA1716 - AUTIASP - AUTIAZ - AUTIZA - Authenticate Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIA--AUTIA1716--AUTIASP--AUTIAZ--AUTIZA--Authenticate-Instruction-address--using-key-A-?lang=en)
    ///
    /// Authenticate Instruction address, using key A. This instruction authenticates an instruction address, using a modifier and key A.
    ///
    /// The address is:
    ///
    /// ```asm
    /// AUTIZA <Xd>
    /// ```
    fn f26(&mut self) -> T {
        todo!()
    }


    /// [AUTIA - AUTIA1716 - AUTIASP - AUTIAZ - AUTIZA - Authenticate Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIA--AUTIA1716--AUTIASP--AUTIAZ--AUTIZA--Authenticate-Instruction-address--using-key-A-?lang=en)
    ///
    /// Authenticate Instruction address, using key A. This instruction authenticates an instruction address, using a modifier and key A.
    ///
    /// The address is:
    ///
    /// ```asm
    /// AUTIA1716
    /// ```
    fn f27(&mut self) -> T {
        todo!()
    }


    /// [AUTIA - AUTIA1716 - AUTIASP - AUTIAZ - AUTIZA - Authenticate Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIA--AUTIA1716--AUTIASP--AUTIAZ--AUTIZA--Authenticate-Instruction-address--using-key-A-?lang=en)
    ///
    /// Authenticate Instruction address, using key A. This instruction authenticates an instruction address, using a modifier and key A.
    ///
    /// The address is:
    ///
    /// ```asm
    /// AUTIASP
    /// ```
    fn f28(&mut self) -> T {
        todo!()
    }


    /// [AUTIA - AUTIA1716 - AUTIASP - AUTIAZ - AUTIZA - Authenticate Instruction address - using key A](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIA--AUTIA1716--AUTIASP--AUTIAZ--AUTIZA--Authenticate-Instruction-address--using-key-A-?lang=en)
    ///
    /// Authenticate Instruction address, using key A. This instruction authenticates an instruction address, using a modifier and key A.
    ///
    /// The address is:
    ///
    /// ```asm
    /// AUTIAZ
    /// ```
    fn f29(&mut self) -> T {
        todo!()
    }


    /// [AUTIB - AUTIB1716 - AUTIBSP - AUTIBZ - AUTIZB - Authenticate Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIB--AUTIB1716--AUTIBSP--AUTIBZ--AUTIZB--Authenticate-Instruction-address--using-key-B-?lang=en)
    ///
    /// Authenticate Instruction address, using key B. This instruction authenticates an instruction address, using a modifier and key B.
    ///
    /// The address is:
    ///
    /// ```asm
    /// AUTIB <Xd>, <Xn|SP>
    /// ```
    fn f30(&mut self) -> T {
        todo!()
    }


    /// [AUTIB - AUTIB1716 - AUTIBSP - AUTIBZ - AUTIZB - Authenticate Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIB--AUTIB1716--AUTIBSP--AUTIBZ--AUTIZB--Authenticate-Instruction-address--using-key-B-?lang=en)
    ///
    /// Authenticate Instruction address, using key B. This instruction authenticates an instruction address, using a modifier and key B.
    ///
    /// The address is:
    ///
    /// ```asm
    /// AUTIZB <Xd>
    /// ```
    fn f31(&mut self) -> T {
        todo!()
    }


    /// [AUTIB - AUTIB1716 - AUTIBSP - AUTIBZ - AUTIZB - Authenticate Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIB--AUTIB1716--AUTIBSP--AUTIBZ--AUTIZB--Authenticate-Instruction-address--using-key-B-?lang=en)
    ///
    /// Authenticate Instruction address, using key B. This instruction authenticates an instruction address, using a modifier and key B.
    ///
    /// The address is:
    ///
    /// ```asm
    /// AUTIB1716
    /// ```
    fn f32(&mut self) -> T {
        todo!()
    }


    /// [AUTIB - AUTIB1716 - AUTIBSP - AUTIBZ - AUTIZB - Authenticate Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIB--AUTIB1716--AUTIBSP--AUTIBZ--AUTIZB--Authenticate-Instruction-address--using-key-B-?lang=en)
    ///
    /// Authenticate Instruction address, using key B. This instruction authenticates an instruction address, using a modifier and key B.
    ///
    /// The address is:
    ///
    /// ```asm
    /// AUTIBSP
    /// ```
    fn f33(&mut self) -> T {
        todo!()
    }


    /// [AUTIB - AUTIB1716 - AUTIBSP - AUTIBZ - AUTIZB - Authenticate Instruction address - using key B](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AUTIB--AUTIB1716--AUTIBSP--AUTIBZ--AUTIZB--Authenticate-Instruction-address--using-key-B-?lang=en)
    ///
    /// Authenticate Instruction address, using key B. This instruction authenticates an instruction address, using a modifier and key B.
    ///
    /// The address is:
    ///
    /// ```asm
    /// AUTIBZ
    /// ```
    fn f34(&mut self) -> T {
        todo!()
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
    fn f35(&mut self) -> T {
        todo!()
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
    fn f36(&mut self) -> T {
        todo!()
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
    fn f37(&mut self) -> T {
        todo!()
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
    fn f38(&mut self) -> T {
        todo!()
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
    fn f39(&mut self) -> T {
        todo!()
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
    fn f40(&mut self) -> T {
        todo!()
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
    fn f41(&mut self) -> T {
        todo!()
    }
}
