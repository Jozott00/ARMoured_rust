//! # Add/subtract (extended register)
//!
//! Implements the following instructions:
//!  - [ADD - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--extended-register---Add--extended-register--?lang=en)
//!  - [ADDS - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--extended-register---Add--extended-register---setting-flags-?lang=en)
//!  - [SUB - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--extended-register---Subtract--extended-register--?lang=en)
//!  - [SUBS - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--extended-register---Subtract--extended-register---setting-flags-?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Register, UImm2};
use crate::types::extends::RegExtend;

#[inline(always)]
fn emit_add_sub_ext<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: u8,
    op: u8,
    s: u8,
    opt: u8,
    rm: Register,
    option: u8,
    imm3: u8,
    rn: Register,
    rd: Register,
) -> T {
    let r = bseq_32!(sf:1 op:1 s:1 01011 opt:2 1 rm:5 option:3 imm3:3 rn:5 rd:5);
    proc.process(r)
}

#[inline(always)]
fn emit_add_sub_ext_opt_shift<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    sf: u8,
    op: u8,
    s: u8,
    opt: u8,
    rm: Register,
    extend: RegExtend,
    amount: Option<UImm2>,
    rn: Register,
    rd: Register,
) -> T {
    let amount = amount.unwrap_or(0);
    debug_assert!(
        amount <= 4,
        "shift amount must be in range 0 to 4, was {}",
        amount
    );
    emit_add_sub_ext(proc, sf, op, s, opt, rm, extend.into(), amount, rn, rd)
}

/// # Add/subtract (extended register)
///
/// Implements the following instructions:
///  - [ADD - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--extended-register---Add--extended-register--?lang=en)
///  - [ADDS - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--extended-register---Add--extended-register---setting-flags-?lang=en)
///  - [SUB - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--extended-register---Subtract--extended-register--?lang=en)
///  - [SUBS - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--extended-register---Subtract--extended-register---setting-flags-?lang=en)
pub trait AddSubtractExtendedRegister<T>: InstructionProcessor<T> {
    /// [ADD - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--extended-register---Add--extended-register--?lang=en)
    ///
    /// Add (extended register) adds a register value and a sign or zero-extended register value, followed by an optional left shift amount, and writes the result to the destination register. The argument that is extended from the `<Rm>` register can be a byte, halfword, word, or doubleword.
    ///
    /// ```asm
    /// ADD <Wd|WSP>, <Wn|WSP>, <Wm>, <extend> {#<amount>}
    /// ```
    #[inline(always)]
    fn add_32_reg_extend(
        &mut self,
        wd_wsp: Register,
        wn_wsp: Register,
        wm: Register,
        extend: RegExtend,
        amount: Option<UImm2>,
    ) -> T {
        emit_add_sub_ext_opt_shift(self, 0, 0, 0, 0b00, wm, extend, amount, wn_wsp, wd_wsp)
    }

    /// [ADD - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--extended-register---Add--extended-register--?lang=en)
    ///
    /// Add (extended register) adds a register value and a sign or zero-extended register value, followed by an optional left shift amount, and writes the result to the destination register. The argument that is extended from the `<Rm>` register can be a byte, halfword, word, or doubleword.
    ///
    /// ```asm
    /// ADD <Xd|SP>, <Xn|SP>, <R><m>, <extend> {#<amount>}
    /// ```
    ///
    /// **Note**: The `<R>` in `<R><m>` is implicitly given by the chosen `<extend>`. [`RegExtend`] also does not support `LSL`
    /// as it simplifies the api.
    #[inline(always)]
    fn add_64_reg_extend(
        &mut self,
        xd_sp: Register,
        xn_sp: Register,
        m: Register,
        extend: RegExtend,
        amount: Option<UImm2>,
    ) -> T {
        emit_add_sub_ext_opt_shift(self, 1, 0, 0, 0b00, m, extend, amount, xn_sp, xd_sp)
    }

    /// [ADDS - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--extended-register---Add--extended-register---setting-flags-?lang=en)
    ///
    /// Add (extended register), setting flags, adds a register value and a sign or zero-extended register value, followed by an optional left shift amount, and writes the result to the destination register. The argument that is extended from the `<Rm>` register can be a byte, halfword, word, or doubleword. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the alias CMN (extended register).
    ///
    /// ```asm
    /// ADDS <Wd>, <Wn|WSP>, <Wm>{, <extend> {#<amount>}}
    /// ```
    #[inline(always)]
    fn adds_32_reg_extend(
        &mut self,
        wd_wsp: Register,
        wn_wsp: Register,
        wm: Register,
        extend: RegExtend,
        amount: Option<UImm2>,
    ) -> T {
        emit_add_sub_ext_opt_shift(self, 0, 0, 1, 0b00, wm, extend, amount, wn_wsp, wd_wsp)
    }

    /// [ADDS - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--extended-register---Add--extended-register---setting-flags-?lang=en)
    ///
    /// Add (extended register), setting flags, adds a register value and a sign or zero-extended register value, followed by an optional left shift amount, and writes the result to the destination register. The argument that is extended from the `<Rm>` register can be a byte, halfword, word, or doubleword. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the alias CMN (extended register).
    ///
    /// ```asm
    /// ADDS <Xd>, <Xn|SP>, <R><m>{, <extend> {#<amount>}}
    /// ```
    ///
    /// **Note**: The `<R>` in `<R><m>` is implicitly given by the chosen `<extend>`. [`RegExtend`] also does not support `LSL`
    /// as it simplifies the api.
    #[inline(always)]
    fn adds_64_reg_extend(
        &mut self,
        xd_sp: Register,
        xn_sp: Register,
        m: Register,
        extend: RegExtend,
        amount: Option<UImm2>,
    ) -> T {
        emit_add_sub_ext_opt_shift(self, 1, 0, 1, 0b00, m, extend, amount, xn_sp, xd_sp)
    }

    /// [SUB - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--extended-register---Subtract--extended-register--?lang=en)
    ///
    /// Subtract (extended register) subtracts a sign or zero-extended register value, followed by an optional left shift amount, from a register value, and writes the result to the destination register. The argument that is extended from the `<Rm>` register can be a byte, halfword, word, or doubleword.
    ///
    /// ```asm
    /// SUB <Wd|WSP>, <Wn|WSP>, <Wm>{, <extend> {#<amount>}}
    /// ```
    #[inline(always)]
    fn sub_32_reg_extend(
        &mut self,
        wd_wsp: Register,
        wn_wsp: Register,
        wm: Register,
        extend: RegExtend,
        amount: Option<UImm2>,
    ) -> T {
        emit_add_sub_ext_opt_shift(self, 0, 1, 0, 0b00, wm, extend, amount, wn_wsp, wd_wsp)
    }

    /// [SUB - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--extended-register---Subtract--extended-register--?lang=en)
    ///
    /// Subtract (extended register) subtracts a sign or zero-extended register value, followed by an optional left shift amount, from a register value, and writes the result to the destination register. The argument that is extended from the `<Rm>` register can be a byte, halfword, word, or doubleword.
    ///
    /// ```asm
    /// SUB <Xd|SP>, <Xn|SP>, <R><m>{, <extend> {#<amount>}}
    /// ```
    ///
    /// **Note**: The `<R>` in `<R><m>` is implicitly given by the chosen `<extend>`. [`RegExtend`] also does not support `LSL`
    /// as it simplifies the api.
    #[inline(always)]
    fn sub_64_reg_extend(
        &mut self,
        xd_sp: Register,
        xn_sp: Register,
        m: Register,
        extend: RegExtend,
        amount: Option<UImm2>,
    ) -> T {
        emit_add_sub_ext_opt_shift(self, 1, 1, 0, 0b00, m, extend, amount, xn_sp, xd_sp)
    }

    /// [SUBS - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--extended-register---Subtract--extended-register---setting-flags-?lang=en)
    ///
    /// Subtract (extended register), setting flags, subtracts a sign or zero-extended register value, followed by an optional left shift amount, from a register value, and writes the result to the destination register. The argument that is extended from the `<Rm>` register can be a byte, halfword, word, or doubleword. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the alias CMP (extended register).
    ///
    /// ```asm
    /// SUBS <Wd>, <Wn|WSP>, <Wm>{, <extend> {#<amount>}}
    /// ```
    #[inline(always)]
    fn subs_32_reg_extend(
        &mut self,
        wd_wsp: Register,
        wn_wsp: Register,
        wm: Register,
        extend: RegExtend,
        amount: Option<UImm2>,
    ) -> T {
        emit_add_sub_ext_opt_shift(self, 0, 1, 1, 0b00, wm, extend, amount, wn_wsp, wd_wsp)
    }

    /// [SUBS - extended register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--extended-register---Subtract--extended-register---setting-flags-?lang=en)
    ///
    /// Subtract (extended register), setting flags, subtracts a sign or zero-extended register value, followed by an optional left shift amount, from a register value, and writes the result to the destination register. The argument that is extended from the `<Rm>` register can be a byte, halfword, word, or doubleword. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the alias CMP (extended register).
    ///
    /// ```asm
    /// SUBS <Xd>, <Xn|SP>, <R><m>{, <extend> {#<amount>}}
    /// ```
    ///
    /// **Note**: The `<R>` in `<R><m>` is implicitly given by the chosen `<extend>`. We also [`RegExtend`] does not support `LSL`
    /// as it simplifies the api.
    #[inline(always)]
    fn subs_64_reg_extend(
        &mut self,
        xd_sp: Register,
        xn_sp: Register,
        m: Register,
        extend: RegExtend,
        amount: Option<UImm2>,
    ) -> T {
        emit_add_sub_ext_opt_shift(self, 1, 1, 1, 0b00, m, extend, amount, xn_sp, xd_sp)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_add() {
        let mut prod = TestProducer::new();

        let instr = prod.add_32_reg_extend(2, 3, 9, RegExtend::SXTB, None);
        assert_eq!(instr, "add w2, w3, w9, sxtb");

        let instr = prod.add_32_reg_extend(2, 3, 9, RegExtend::UXTW, Some(3));
        assert_eq!(instr, "add w2, w3, w9, uxtw #0x3");

        let instr = prod.add_64_reg_extend(2, 3, 9, RegExtend::SXTB, None);
        assert_eq!(instr, "add x2, x3, w9, sxtb");

        let instr = prod.add_64_reg_extend(2, 3, 31, RegExtend::UXTX, Some(3));
        assert_eq!(instr, "add x2, x3, xzr, uxtx #0x3");
    }

    #[test]
    fn test_adds() {
        let mut prod = TestProducer::new();

        let instr = prod.adds_32_reg_extend(2, 3, 9, RegExtend::SXTB, None);
        assert_eq!(instr, "adds w2, w3, w9, sxtb");

        let instr = prod.adds_32_reg_extend(2, 3, 9, RegExtend::UXTW, Some(3));
        assert_eq!(instr, "adds w2, w3, w9, uxtw #0x3");

        let instr = prod.adds_64_reg_extend(2, 3, 9, RegExtend::SXTB, None);
        assert_eq!(instr, "adds x2, x3, w9, sxtb");

        let instr = prod.adds_64_reg_extend(2, 3, 31, RegExtend::UXTX, Some(3));
        assert_eq!(instr, "adds x2, x3, xzr, uxtx #0x3");
    }

    #[test]
    fn test_sub() {
        let mut prod = TestProducer::new();

        let instr = prod.sub_32_reg_extend(2, 3, 9, RegExtend::SXTB, None);
        assert_eq!(instr, "sub w2, w3, w9, sxtb");

        let instr = prod.sub_32_reg_extend(2, 3, 9, RegExtend::UXTW, Some(3));
        assert_eq!(instr, "sub w2, w3, w9, uxtw #0x3");

        let instr = prod.sub_64_reg_extend(2, 3, 9, RegExtend::SXTB, None);
        assert_eq!(instr, "sub x2, x3, w9, sxtb");

        let instr = prod.sub_64_reg_extend(2, 3, 31, RegExtend::UXTX, Some(3));
        assert_eq!(instr, "sub x2, x3, xzr, uxtx #0x3");
    }

    #[test]
    fn test_subs() {
        let mut prod = TestProducer::new();

        let instr = prod.subs_32_reg_extend(2, 3, 9, RegExtend::SXTB, None);
        assert_eq!(instr, "subs w2, w3, w9, sxtb");

        let instr = prod.subs_32_reg_extend(2, 3, 9, RegExtend::UXTW, Some(3));
        assert_eq!(instr, "subs w2, w3, w9, uxtw #0x3");

        let instr = prod.subs_64_reg_extend(2, 3, 9, RegExtend::SXTB, None);
        assert_eq!(instr, "subs x2, x3, w9, sxtb");

        let instr = prod.subs_64_reg_extend(2, 3, 31, RegExtend::UXTX, Some(3));
        assert_eq!(instr, "subs x2, x3, xzr, uxtx #0x3");
    }
}
