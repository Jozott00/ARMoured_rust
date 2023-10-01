//! # [Conditional compare (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#condcmp_imm)
//!
//! Implements the following instructions:
//!  - [CCMN - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMN--immediate---Conditional-Compare-Negative--immediate--?lang=en)
//!  - [CCMP - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMP--immediate---Conditional-Compare--immediate--?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::{UImm4, UImm5};
use crate::types::condition::Condition;
use crate::types::encodable::Encodable;
use crate::types::Register;

#[inline(always)]
fn emit_cond_cmp_imm<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: u8,
    op: u8,
    s: u8,
    imm5: u8,
    cond: u8,
    o2: u8,
    rn: Register,
    o3: u8,
    nzcv: u8,
) -> T {
    let i = bseq_32!(sf:1 op:1 s:1 11010010 imm5:5 cond:4 1 o2:1 rn:5 o3:1 nzcv:4);
    proc.process(i)
}

/// # [Conditional compare (immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#condcmp_imm)
///
/// Implements the following instructions:
///  - [CCMN - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMN--immediate---Conditional-Compare-Negative--immediate--?lang=en)
///  - [CCMP - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMP--immediate---Conditional-Compare--immediate--?lang=en)
pub trait ConditionalCompareImmediate<T>: InstructionProcessor<T> {
    /// [CCMN - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMN--immediate---Conditional-Compare-Negative--immediate--?lang=en)
    ///
    /// Conditional Compare Negative (immediate) sets the value of the condition flags to the result of the comparison of a register value and a negated immediate value if the condition is TRUE, and an immediate value otherwise.
    ///
    /// ```asm
    /// CCMN <Wn>, #<imm>, #<nzcv>, <cond>
    /// ```
    #[inline(always)]
    fn ccmn_32_imm(&mut self, wn: Register, imm: UImm5, nzcv: UImm4, cond: Condition) -> T {
        emit_cond_cmp_imm(self, 0, 0, 1, imm, cond.encode(), 0, wn, 0, nzcv)
    }

    /// [CCMN - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMN--immediate---Conditional-Compare-Negative--immediate--?lang=en)
    ///
    /// Conditional Compare Negative (immediate) sets the value of the condition flags to the result of the comparison of a register value and a negated immediate value if the condition is TRUE, and an immediate value otherwise.
    ///
    /// ```asm
    /// CCMN <Xn>, #<imm>, #<nzcv>, <cond>
    /// ```
    #[inline(always)]
    fn ccmn_64_imm(&mut self, xn: Register, imm: UImm5, nzcv: UImm4, cond: Condition) -> T {
        emit_cond_cmp_imm(self, 1, 0, 1, imm, cond.encode(), 0, xn, 0, nzcv)
    }

    /// [CCMP - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMP--immediate---Conditional-Compare--immediate--?lang=en)
    ///
    /// Conditional Compare (immediate) sets the value of the condition flags to the result of the comparison of a register value and an immediate value if the condition is TRUE, and an immediate value otherwise.
    ///
    /// ```asm
    /// CCMP <Wn>, #<imm>, #<nzcv>, <cond>
    /// ```
    #[inline(always)]
    fn ccmp_32_imm(&mut self, wn: Register, imm: UImm5, nzcv: UImm4, cond: Condition) -> T {
        emit_cond_cmp_imm(self, 0, 1, 1, imm, cond.encode(), 0, wn, 0, nzcv)
    }

    /// [CCMP - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CCMP--immediate---Conditional-Compare--immediate--?lang=en)
    ///
    /// Conditional Compare (immediate) sets the value of the condition flags to the result of the comparison of a register value and an immediate value if the condition is TRUE, and an immediate value otherwise.
    ///
    /// ```asm
    /// CCMP <Xn>, #<imm>, #<nzcv>, <cond>
    /// ```
    #[inline(always)]
    fn ccmp_64_imm(&mut self, xn: Register, imm: UImm5, nzcv: UImm4, cond: Condition) -> T {
        emit_cond_cmp_imm(self, 1, 1, 1, imm, cond.encode(), 0, xn, 0, nzcv)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_ccmn() {
        let mut prod = TestProducer::new();

        let instr = prod.ccmn_32_imm(3, 4, 0b1101, Condition::AL);
        assert_eq!(instr, "ccmn w3, #0x4, #0xd, al");

        let instr = prod.ccmn_64_imm(3, 4, 0b1101, Condition::AL);
        assert_eq!(instr, "ccmn x3, #0x4, #0xd, al");
    }

    #[test]
    fn test_ccmp() {
        let mut prod = TestProducer::new();

        let instr = prod.ccmp_32_imm(3, 4, 0b1101, Condition::AL);
        assert_eq!(instr, "ccmp w3, #0x4, #0xd, al");

        let instr = prod.ccmp_64_imm(3, 4, 0b1101, Condition::AL);
        assert_eq!(instr, "ccmp x3, #0x4, #0xd, al");
    }
}
