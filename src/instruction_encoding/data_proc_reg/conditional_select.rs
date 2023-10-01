//! # [Conditional select](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#condsel)
//!
//! Implements the following instructions:
//!  - [CSEL - Conditional Select](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSEL--Conditional-Select-?lang=en)
//!  - [CSINC - Conditional Select Increment](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSINC--Conditional-Select-Increment-?lang=en)
//!  - [CSINV - Conditional Select Invert](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSINV--Conditional-Select-Invert-?lang=en)
//!  - [CSNEG - Conditional Select Negation](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSNEG--Conditional-Select-Negation-?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::condition::Condition;
use crate::types::encodable::Encodable;
use crate::types::Register;

#[inline(always)]
fn emit_cond_sel<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: u8,
    op: u8,
    s: u8,
    rm: Register,
    cond: u8,
    op2: u8,
    rn: Register,
    rd: Register,
) -> T {
    let i = bseq_32!(sf:1 op:1 s:1 11010100 rm:5 cond:4 op2:2 rn:5 rd:5);
    proc.process(i)
}

/// # [Conditional select](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#condsel)
///
/// Implements the following instructions:
///  - [CSEL - Conditional Select](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSEL--Conditional-Select-?lang=en)
///  - [CSINC - Conditional Select Increment](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSINC--Conditional-Select-Increment-?lang=en)
///  - [CSINV - Conditional Select Invert](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSINV--Conditional-Select-Invert-?lang=en)
///  - [CSNEG - Conditional Select Negation](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSNEG--Conditional-Select-Negation-?lang=en)
pub trait ConditionalSelect<T>: InstructionProcessor<T> {
    /// [CSEL - Conditional Select](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSEL--Conditional-Select-?lang=en)
    ///
    /// If the condition is true, Conditional Select writes the value of the first source register to the destination register. If the condition is false, it writes the value of the second source register to the destination register.
    ///
    /// ```asm
    /// CSEL <Wd>, <Wn>, <Wm>, <cond>
    /// ```
    #[inline(always)]
    fn csel_32(&mut self, wd: Register, wn: Register, wm: Register, cond: Condition) -> T {
        emit_cond_sel(self, 0, 0, 0, wm, cond.encode(), 0b00, wn, wd)
    }

    /// [CSEL - Conditional Select](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSEL--Conditional-Select-?lang=en)
    ///
    /// If the condition is true, Conditional Select writes the value of the first source register to the destination register. If the condition is false, it writes the value of the second source register to the destination register.
    ///
    /// ```asm
    /// CSEL <Xd>, <Xn>, <Xm>, <cond>
    /// ```
    #[inline(always)]
    fn csel_64(&mut self, xd: Register, xn: Register, xm: Register, cond: Condition) -> T {
        emit_cond_sel(self, 1, 0, 0, xm, cond.encode(), 0b00, xn, xd)
    }

    /// [CSINC - Conditional Select Increment](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSINC--Conditional-Select-Increment-?lang=en)
    ///
    /// Conditional Select Increment returns, in the destination register, the value of the first source register if the condition is TRUE, and otherwise returns the value of the second source register incremented by 1.
    ///
    /// This instruction is used by the aliases CINC, and CSET.
    ///
    /// ```asm
    /// CSINC <Wd>, <Wn>, <Wm>, <cond>
    /// ```
    #[inline(always)]
    fn csinc_32(&mut self, wd: Register, wn: Register, wm: Register, cond: Condition) -> T {
        emit_cond_sel(self, 0, 0, 0, wm, cond.encode(), 0b01, wn, wd)
    }

    /// [CSINC - Conditional Select Increment](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSINC--Conditional-Select-Increment-?lang=en)
    ///
    /// Conditional Select Increment returns, in the destination register, the value of the first source register if the condition is TRUE, and otherwise returns the value of the second source register incremented by 1.
    ///
    /// This instruction is used by the aliases CINC, and CSET.
    ///
    /// ```asm
    /// CSINC <Xd>, <Xn>, <Xm>, <cond>
    /// ```
    #[inline(always)]
    fn csinc_64(&mut self, xd: Register, xn: Register, xm: Register, cond: Condition) -> T {
        emit_cond_sel(self, 1, 0, 0, xm, cond.encode(), 0b01, xn, xd)
    }

    /// [CSINV - Conditional Select Invert](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSINV--Conditional-Select-Invert-?lang=en)
    ///
    /// Conditional Select Invert returns, in the destination register, the value of the first source register if the condition is TRUE, and otherwise returns the bitwise inversion value of the second source register.
    ///
    /// This instruction is used by the aliases CINV, and CSETM.
    ///
    /// ```asm
    /// CSINV <Wd>, <Wn>, <Wm>, <cond>
    /// ```
    #[inline(always)]
    fn csinv_32(&mut self, wd: Register, wn: Register, wm: Register, cond: Condition) -> T {
        emit_cond_sel(self, 0, 1, 0, wm, cond.encode(), 0b00, wn, wd)
    }

    /// [CSINV - Conditional Select Invert](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSINV--Conditional-Select-Invert-?lang=en)
    ///
    /// Conditional Select Invert returns, in the destination register, the value of the first source register if the condition is TRUE, and otherwise returns the bitwise inversion value of the second source register.
    ///
    /// This instruction is used by the aliases CINV, and CSETM.
    ///
    /// ```asm
    /// CSINV <Xd>, <Xn>, <Xm>, <cond>
    /// ```
    #[inline(always)]
    fn csinv_64(&mut self, xd: Register, xn: Register, xm: Register, cond: Condition) -> T {
        emit_cond_sel(self, 1, 1, 0, xm, cond.encode(), 0b00, xn, xd)
    }

    /// [CSNEG - Conditional Select Negation](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSNEG--Conditional-Select-Negation-?lang=en)
    ///
    /// Conditional Select Negation returns, in the destination register, the value of the first source register if the condition is TRUE, and otherwise returns the negated value of the second source register.
    ///
    /// This instruction is used by the alias CNEG.
    ///
    /// ```asm
    /// CSNEG <Wd>, <Wn>, <Wm>, <cond>
    /// ```
    #[inline(always)]
    fn csneg_32(&mut self, wd: Register, wn: Register, wm: Register, cond: Condition) -> T {
        emit_cond_sel(self, 0, 1, 0, wm, cond.encode(), 0b01, wn, wd)
    }

    /// [CSNEG - Conditional Select Negation](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CSNEG--Conditional-Select-Negation-?lang=en)
    ///
    /// Conditional Select Negation returns, in the destination register, the value of the first source register if the condition is TRUE, and otherwise returns the negated value of the second source register.
    ///
    /// This instruction is used by the alias CNEG.
    ///
    /// ```asm
    /// CSNEG <Xd>, <Xn>, <Xm>, <cond>
    /// ```
    #[inline(always)]
    fn csneg_64(&mut self, xd: Register, xn: Register, xm: Register, cond: Condition) -> T {
        emit_cond_sel(self, 1, 1, 0, xm, cond.encode(), 0b01, xn, xd)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_csel() {
        let mut prod = TestProducer::new();

        let instr = prod.csel_32(3, 4, 5, Condition::AL);
        assert_eq!(instr, "csel w3, w4, w5, al");

        let instr = prod.csel_64(3, 4, 5, Condition::AL);
        assert_eq!(instr, "csel x3, x4, x5, al");
    }

    #[test]
    fn test_csinc() {
        let mut prod = TestProducer::new();

        let instr = prod.csinc_32(3, 4, 5, Condition::AL);
        assert_eq!(instr, "csinc w3, w4, w5, al");

        let instr = prod.csinc_64(3, 4, 5, Condition::AL);
        assert_eq!(instr, "csinc x3, x4, x5, al");
    }

    #[test]
    fn test_csinv() {
        let mut prod = TestProducer::new();

        let instr = prod.csinv_32(3, 4, 5, Condition::AL);
        assert_eq!(instr, "csinv w3, w4, w5, al");

        let instr = prod.csinv_64(3, 4, 5, Condition::AL);
        assert_eq!(instr, "csinv x3, x4, x5, al");
    }

    #[test]
    fn test_csneg() {
        let mut prod = TestProducer::new();

        let instr = prod.csneg_32(3, 4, 5, Condition::AL);
        assert_eq!(instr, "csneg w3, w4, w5, al");

        let instr = prod.csneg_64(3, 4, 5, Condition::AL);
        assert_eq!(instr, "csneg x3, x4, x5, al");
    }
}
