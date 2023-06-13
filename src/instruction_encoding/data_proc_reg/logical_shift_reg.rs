//! # Logical (shifted register)
//!
//! Implements the following instructions:
//!  - [AND - shifted register -  Bitwise AND - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AND--shifted-register---Bitwise-AND--shifted-register--?lang=en)
//!  - [BIC - shifted register -  Bitwise Bit Clear - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BIC--shifted-register---Bitwise-Bit-Clear--shifted-register--?lang=en)
//!  - [ORR - shifted register -  Bitwise OR - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ORR--shifted-register---Bitwise-OR--shifted-register--?lang=en)
//!  - [ORN - shifted register -  Bitwise OR NOT - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ORN--shifted-register---Bitwise-OR-NOT--shifted-register--?lang=en)
//!  - [EOR - shifted register -  Bitwise Exclusive OR - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EOR--shifted-register---Bitwise-Exclusive-OR--shifted-register--?lang=en)
//!  - [EON - shifted register -  Bitwise Exclusive OR NOT - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EON--shifted-register---Bitwise-Exclusive-OR-NOT--shifted-register--?lang=en)
//!  - [ANDS - shifted register -  Bitwise AND - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ANDS--shifted-register---Bitwise-AND--shifted-register---setting-flags-?lang=en)
//!  - [BICS - shifted register -  Bitwise Bit Clear - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BICS--shifted-register---Bitwise-Bit-Clear--shifted-register---setting-flags-?lang=en)



use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Register, UImm5, UImm6};
use crate::types::shifts::Shift;

fn emit_log_shift<P: InstructionProcessor<T>, T>(proc: &mut P, sf: u8, opc: u8, shift: u8, n: u8, rm: Register, imm6: u8, rn: Register, rd: Register) -> T {
    let i = bseq_32!(sf:1 opc:2 01010 shift:2 n:1 rm:5 imm6:6 rn:5 rd:5);
    proc.process(i)
}

fn emit_log_opt_shift<P: InstructionProcessor<T>, T>(proc: &mut P, sf: u8, opc: u8, shift: Option<Shift<u8>>, n: u8, rm: Register, rn: Register, rd: Register) -> T {
    let (shift, imm6): (u8, u8) = shift.unwrap_or(Shift::LSL(0)).into();
    emit_log_shift(proc, sf, opc, shift.into(), n, rm, imm6, rn, rd)
}

/// # Logical (shifted register)
///
/// Implements the following instructions:
///  - [AND - shifted register -  Bitwise AND - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AND--shifted-register---Bitwise-AND--shifted-register--?lang=en)
///  - [BIC - shifted register -  Bitwise Bit Clear - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BIC--shifted-register---Bitwise-Bit-Clear--shifted-register--?lang=en)
///  - [ORR - shifted register -  Bitwise OR - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ORR--shifted-register---Bitwise-OR--shifted-register--?lang=en)
///  - [ORN - shifted register -  Bitwise OR NOT - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ORN--shifted-register---Bitwise-OR-NOT--shifted-register--?lang=en)
///  - [EOR - shifted register -  Bitwise Exclusive OR - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EOR--shifted-register---Bitwise-Exclusive-OR--shifted-register--?lang=en)
///  - [EON - shifted register -  Bitwise Exclusive OR NOT - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EON--shifted-register---Bitwise-Exclusive-OR-NOT--shifted-register--?lang=en)
///  - [ANDS - shifted register -  Bitwise AND - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ANDS--shifted-register---Bitwise-AND--shifted-register---setting-flags-?lang=en)
///  - [BICS - shifted register -  Bitwise Bit Clear - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BICS--shifted-register---Bitwise-Bit-Clear--shifted-register---setting-flags-?lang=en)
pub trait LogicalShiftRegister<T>: InstructionProcessor<T> {
    /// [AND - shifted register -  Bitwise AND - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AND--shifted-register---Bitwise-AND--shifted-register--?lang=en)
    ///
    /// Bitwise AND (shifted register) performs a bitwise AND of a register value and an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// AND <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
    /// ```
    fn and_32(&mut self, wd: Register, wn: Register, wm: Register, shift: Option<Shift<UImm5>>) -> T {
        emit_log_opt_shift(self, 0, 0, shift, 0, wm, wn, wd)
    }


    /// [AND - shifted register -  Bitwise AND - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/AND--shifted-register---Bitwise-AND--shifted-register--?lang=en)
    ///
    /// Bitwise AND (shifted register) performs a bitwise AND of a register value and an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// AND <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
    /// ```
    fn and_64(&mut self, xd: Register, xn: Register, xm: Register, shift: Option<Shift<UImm6>>) -> T {
        emit_log_opt_shift(self, 1, 0, shift, 0, xm, xn, xd)
    }


    /// [BIC - shifted register -  Bitwise Bit Clear - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BIC--shifted-register---Bitwise-Bit-Clear--shifted-register--?lang=en)
    ///
    /// Bitwise Bit Clear (shifted register) performs a bitwise AND of a register value and the complement of an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// BIC <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
    /// ```
    fn bic_32(&mut self, wd: Register, wn: Register, wm: Register, shift: Option<Shift<UImm5>>) -> T {
        emit_log_opt_shift(self, 0, 0, shift, 1, wm, wn, wd)
    }


    /// [BIC - shifted register -  Bitwise Bit Clear - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BIC--shifted-register---Bitwise-Bit-Clear--shifted-register--?lang=en)
    ///
    /// Bitwise Bit Clear (shifted register) performs a bitwise AND of a register value and the complement of an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// BIC <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
    /// ```
    fn bic_64(&mut self, xd: Register, xn: Register, xm: Register, shift: Option<Shift<UImm6>>) -> T {
        emit_log_opt_shift(self, 1, 0, shift, 1, xm, xn, xd)
    }


    /// [ORR - shifted register -  Bitwise OR - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ORR--shifted-register---Bitwise-OR--shifted-register--?lang=en)
    ///
    /// Bitwise OR (shifted register) performs a bitwise (inclusive) OR of a register value and an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias MOV (register).
    ///
    /// ```asm
    /// ORR <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
    /// ```
    fn orr_32(&mut self, wd: Register, wn: Register, wm: Register, shift: Option<Shift<UImm5>>) -> T {
        emit_log_opt_shift(self, 0, 0b01, shift, 0, wm, wn, wd)
    }


    /// [ORR - shifted register -  Bitwise OR - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ORR--shifted-register---Bitwise-OR--shifted-register--?lang=en)
    ///
    /// Bitwise OR (shifted register) performs a bitwise (inclusive) OR of a register value and an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias MOV (register).
    ///
    /// ```asm
    /// ORR <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
    /// ```
    fn orr_64(&mut self, xd: Register, xn: Register, xm: Register, shift: Option<Shift<UImm6>>) -> T {
        emit_log_opt_shift(self, 1, 0b01, shift, 0, xm, xn, xd)
    }


    /// [ORN - shifted register -  Bitwise OR NOT - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ORN--shifted-register---Bitwise-OR-NOT--shifted-register--?lang=en)
    ///
    /// Bitwise OR NOT (shifted register) performs a bitwise (inclusive) OR of a register value and the complement of an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias MVN.
    ///
    /// ```asm
    /// ORN <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
    /// ```
    fn orn_32(&mut self, wd: Register, wn: Register, wm: Register, shift: Option<Shift<UImm5>>) -> T {
        emit_log_opt_shift(self, 0, 0b01, shift, 1, wm, wn, wd)
    }


    /// [ORN - shifted register -  Bitwise OR NOT - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ORN--shifted-register---Bitwise-OR-NOT--shifted-register--?lang=en)
    ///
    /// Bitwise OR NOT (shifted register) performs a bitwise (inclusive) OR of a register value and the complement of an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// This instruction is used by the alias MVN.
    ///
    /// ```asm
    /// ORN <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
    /// ```
    fn orn_64(&mut self, xd: Register, xn: Register, xm: Register, shift: Option<Shift<UImm6>>) -> T {
        emit_log_opt_shift(self, 1, 0b01, shift, 1, xm, xn, xd)
    }


    /// [EOR - shifted register -  Bitwise Exclusive OR - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EOR--shifted-register---Bitwise-Exclusive-OR--shifted-register--?lang=en)
    ///
    /// Bitwise Exclusive OR (shifted register) performs a bitwise Exclusive OR of a register value and an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// EOR <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
    /// ```
    fn eor_32(&mut self, wd: Register, wn: Register, wm: Register, shift: Option<Shift<UImm5>>) -> T {
        emit_log_opt_shift(self, 0, 0b10, shift, 0, wm, wn, wd)
    }


    /// [EOR - shifted register -  Bitwise Exclusive OR - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EOR--shifted-register---Bitwise-Exclusive-OR--shifted-register--?lang=en)
    ///
    /// Bitwise Exclusive OR (shifted register) performs a bitwise Exclusive OR of a register value and an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// EOR <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
    /// ```
    fn eor_64(&mut self, xd: Register, xn: Register, xm: Register, shift: Option<Shift<UImm6>>) -> T {
        emit_log_opt_shift(self, 1, 0b10, shift, 0, xm, xn, xd)
    }


    /// [EON - shifted register -  Bitwise Exclusive OR NOT - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EON--shifted-register---Bitwise-Exclusive-OR-NOT--shifted-register--?lang=en)
    ///
    /// Bitwise Exclusive OR NOT (shifted register) performs a bitwise Exclusive OR NOT of a register value and an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// EON <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
    /// ```
    fn eon_32(&mut self, wd: Register, wn: Register, wm: Register, shift: Option<Shift<UImm5>>) -> T {
        emit_log_opt_shift(self, 0, 0b10, shift, 1, wm, wn, wd)
    }


    /// [EON - shifted register -  Bitwise Exclusive OR NOT - shifted register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/EON--shifted-register---Bitwise-Exclusive-OR-NOT--shifted-register--?lang=en)
    ///
    /// Bitwise Exclusive OR NOT (shifted register) performs a bitwise Exclusive OR NOT of a register value and an optionally-shifted register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// EON <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
    /// ```
    fn eon_64(&mut self, xd: Register, xn: Register, xm: Register, shift: Option<Shift<UImm6>>) -> T {
        emit_log_opt_shift(self, 1, 0b10, shift, 1, xm, xn, xd)
    }


    /// [ANDS - shifted register -  Bitwise AND - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ANDS--shifted-register---Bitwise-AND--shifted-register---setting-flags-?lang=en)
    ///
    /// Bitwise AND (shifted register), setting flags, performs a bitwise AND of a register value and an optionally-shifted register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the alias TST (shifted register).
    ///
    /// ```asm
    /// ANDS <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
    /// ```
    fn ands_32(&mut self, wd: Register, wn: Register, wm: Register, shift: Option<Shift<UImm5>>) -> T {
        emit_log_opt_shift(self, 0, 0b11, shift, 0, wm, wn, wd)
    }


    /// [ANDS - shifted register -  Bitwise AND - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ANDS--shifted-register---Bitwise-AND--shifted-register---setting-flags-?lang=en)
    ///
    /// Bitwise AND (shifted register), setting flags, performs a bitwise AND of a register value and an optionally-shifted register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the alias TST (shifted register).
    ///
    /// ```asm
    /// ANDS <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
    /// ```
    fn ands_64(&mut self, xd: Register, xn: Register, xm: Register, shift: Option<Shift<UImm6>>) -> T {
        emit_log_opt_shift(self, 1, 0b11, shift, 0, xm, xn, xd)
    }


    /// [BICS - shifted register -  Bitwise Bit Clear - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BICS--shifted-register---Bitwise-Bit-Clear--shifted-register---setting-flags-?lang=en)
    ///
    /// Bitwise Bit Clear (shifted register), setting flags, performs a bitwise AND of a register value and the complement of an optionally-shifted register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// ```asm
    /// BICS <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
    /// ```
    fn bics_32(&mut self, wd: Register, wn: Register, wm: Register, shift: Option<Shift<UImm5>>) -> T {
        emit_log_opt_shift(self, 0, 0b11, shift, 1, wm, wn, wd)
    }


    /// [BICS - shifted register -  Bitwise Bit Clear - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BICS--shifted-register---Bitwise-Bit-Clear--shifted-register---setting-flags-?lang=en)
    ///
    /// Bitwise Bit Clear (shifted register), setting flags, performs a bitwise AND of a register value and the complement of an optionally-shifted register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// ```asm
    /// BICS <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
    /// ```
    fn bics_64(&mut self, xd: Register, xn: Register, xm: Register, shift: Option<Shift<UImm6>>) -> T {
        emit_log_opt_shift(self, 1, 0b11, shift, 1, xm, xn, xd)
    }
}


#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_and() {
        let mut prod = TestProducer::new();

        let instr = prod.and_32(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "and w3, w4, w5, lsl #0x6");

        let instr = prod.and_32(3, 4, 5, None);
        assert_eq!(instr, "and w3, w4, w5");

        let instr = prod.and_64(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "and x3, x4, x5, lsl #0x6");

        let instr = prod.and_64(3, 4, 5, None);
        assert_eq!(instr, "and x3, x4, x5");
    }

    #[test]
    fn test_bic() {
        let mut prod = TestProducer::new();

        let instr = prod.bic_32(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "bic w3, w4, w5, lsl #0x6");

        let instr = prod.bic_32(3, 4, 5, None);
        assert_eq!(instr, "bic w3, w4, w5");

        let instr = prod.bic_64(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "bic x3, x4, x5, lsl #0x6");

        let instr = prod.bic_64(3, 4, 5, None);
        assert_eq!(instr, "bic x3, x4, x5");
    }

    #[test]
    fn test_orr() {
        let mut prod = TestProducer::new();

        let instr = prod.orr_32(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "orr w3, w4, w5, lsl #0x6");

        let instr = prod.orr_32(3, 4, 5, None);
        assert_eq!(instr, "orr w3, w4, w5");

        let instr = prod.orr_64(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "orr x3, x4, x5, lsl #0x6");

        let instr = prod.orr_64(3, 4, 5, None);
        assert_eq!(instr, "orr x3, x4, x5");
    }

    #[test]
    fn test_orn() {
        let mut prod = TestProducer::new();

        let instr = prod.orn_32(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "orn w3, w4, w5, lsl #0x6");

        let instr = prod.orn_32(3, 4, 5, None);
        assert_eq!(instr, "orn w3, w4, w5");

        let instr = prod.orn_64(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "orn x3, x4, x5, lsl #0x6");

        let instr = prod.orn_64(3, 4, 5, None);
        assert_eq!(instr, "orn x3, x4, x5");
    }

    #[test]
    fn test_eor() {
        let mut prod = TestProducer::new();

        let instr = prod.eor_32(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "eor w3, w4, w5, lsl #0x6");

        let instr = prod.eor_32(3, 4, 5, None);
        assert_eq!(instr, "eor w3, w4, w5");

        let instr = prod.eor_64(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "eor x3, x4, x5, lsl #0x6");

        let instr = prod.eor_64(3, 4, 5, None);
        assert_eq!(instr, "eor x3, x4, x5");
    }

    #[test]
    fn test_eon() {
        let mut prod = TestProducer::new();

        let instr = prod.eon_32(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "eon w3, w4, w5, lsl #0x6");

        let instr = prod.eon_32(3, 4, 5, None);
        assert_eq!(instr, "eon w3, w4, w5");

        let instr = prod.eon_64(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "eon x3, x4, x5, lsl #0x6");

        let instr = prod.eon_64(3, 4, 5, None);
        assert_eq!(instr, "eon x3, x4, x5");
    }

    #[test]
    fn test_ands() {
        let mut prod = TestProducer::new();

        let instr = prod.ands_32(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "ands w3, w4, w5, lsl #0x6");

        let instr = prod.ands_32(3, 4, 5, None);
        assert_eq!(instr, "ands w3, w4, w5");

        let instr = prod.ands_64(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "ands x3, x4, x5, lsl #0x6");

        let instr = prod.ands_64(3, 4, 5, None);
        assert_eq!(instr, "ands x3, x4, x5");
    }

    #[test]
    fn test_bics() {
        let mut prod = TestProducer::new();

        let instr = prod.bics_32(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "bics w3, w4, w5, lsl #0x6");

        let instr = prod.bics_32(3, 4, 5, None);
        assert_eq!(instr, "bics w3, w4, w5");

        let instr = prod.bics_64(3, 4, 5, Some(Shift::LSL(6)));
        assert_eq!(instr, "bics x3, x4, x5, lsl #0x6");

        let instr = prod.bics_64(3, 4, 5, None);
        assert_eq!(instr, "bics x3, x4, x5");
    }
}