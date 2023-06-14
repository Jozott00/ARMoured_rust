//! # [Conditional compare (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#condcmp_reg)
//!
//! Implements the following instructions:
//!  - [CCMN - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMN--register---Conditional-Compare-Negative--register--?lang=en)
//!  - [CCMP - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMP--register---Conditional-Compare--register--?lang=en)



use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Register, UImm4};
use crate::types::condition::Condition;
use crate::types::encodable::Encodable;

#[inline(always)]
fn emit_cond_cmp_reg<P: InstructionProcessor<T>, T>(proc: &mut P, sf: u8, op: u8, s: u8, rm: Register, cond: u8, o2: u8, rn: Register, o3: u8, nzcv: u8) -> T {
    let i = bseq_32!(sf:1 op:1 s:1 11010010 rm:5 cond:4 0 o2:1 rn:5 o3:1 nzcv:4);
    proc.process(i)
}


/// # [Conditional compare (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#condcmp_reg)
///
/// Implements the following instructions:
///  - [CCMN - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMN--register---Conditional-Compare-Negative--register--?lang=en)
///  - [CCMP - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMP--register---Conditional-Compare--register--?lang=en)
pub trait ConditionalCompareRegister<T>: InstructionProcessor<T> {
    /// [CCMN - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMN--register---Conditional-Compare-Negative--register--?lang=en)
    ///
    /// Conditional Compare Negative (register) sets the value of the condition flags to the result of the comparison of a register value and the inverse of another register value if the condition is TRUE, and an immediate value otherwise.
    ///
    /// ```asm
    /// CCMN <Wn>, <Wm>, #<nzcv>, <cond>
    /// ```
    #[inline(always)]
    fn ccmn_32_reg(&mut self, wn: Register, wm: Register, nzvc: UImm4, cond: Condition) -> T {
        emit_cond_cmp_reg(self, 0, 0, 1, wm, cond.encode(), 0, wn, 0, nzvc)
    }


    /// [CCMN - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMN--register---Conditional-Compare-Negative--register--?lang=en)
    ///
    /// Conditional Compare Negative (register) sets the value of the condition flags to the result of the comparison of a register value and the inverse of another register value if the condition is TRUE, and an immediate value otherwise.
    ///
    /// ```asm
    /// CCMN <Xn>, <Xm>, #<nzcv>, <cond>
    /// ```
    #[inline(always)]
    fn ccmn_64_reg(&mut self, xn: Register, xm: Register, nzvc: UImm4, cond: Condition) -> T {
        emit_cond_cmp_reg(self, 1, 0, 1, xm, cond.encode(), 0, xn, 0, nzvc)
    }


    /// [CCMP - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMP--register---Conditional-Compare--register--?lang=en)
    ///
    /// Conditional Compare (register) sets the value of the condition flags to the result of the comparison of two registers if the condition is TRUE, and an immediate value otherwise.
    ///
    /// ```asm
    /// CCMP <Wn>, <Wm>, #<nzcv>, <cond>
    /// ```
    #[inline(always)]
    fn ccmp_32_reg(&mut self, wn: Register, wm: Register, nzvc: UImm4, cond: Condition) -> T {
        emit_cond_cmp_reg(self, 0, 1, 1, wm, cond.encode(), 0, wn, 0, nzvc)
    }


    /// [CCMP - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMP--register---Conditional-Compare--register--?lang=en)
    ///
    /// Conditional Compare (register) sets the value of the condition flags to the result of the comparison of two registers if the condition is TRUE, and an immediate value otherwise.
    ///
    /// ```asm
    /// CCMP <Xn>, <Xm>, #<nzcv>, <cond>
    /// ```
    #[inline(always)]
    fn ccmp_64_reg(&mut self, xn: Register, xm: Register, nzvc: UImm4, cond: Condition) -> T {
        emit_cond_cmp_reg(self, 1, 1, 1, xm, cond.encode(), 0, xn, 0, nzvc)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_ccmn() {
        let mut prod = TestProducer::new();

        let instr = prod.ccmn_32_reg(3, 4, 0b1101, Condition::AL);
        assert_eq!(instr, "ccmn w3, w4, #0xd, al");

        let instr = prod.ccmn_64_reg(3, 4, 0b1101, Condition::AL);
        assert_eq!(instr, "ccmn x3, x4, #0xd, al");
    }

    #[test]
    fn test_ccmp() {
        let mut prod = TestProducer::new();

        let instr = prod.ccmp_32_reg(3, 4, 0b1101, Condition::AL);
        assert_eq!(instr, "ccmp w3, w4, #0xd, al");

        let instr = prod.ccmp_64_reg(3, 4, 0b1101, Condition::AL);
        assert_eq!(instr, "ccmp x3, x4, #0xd, al");
    }
}