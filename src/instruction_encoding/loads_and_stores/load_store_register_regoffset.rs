//! # [Load/store register (register offset)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldst_regoff)
//!
//! Implements the following instructions:
//!  - [STRB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRB--register---Store-Register-Byte--register--?lang=en)
//!  - [LDRB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRB--register---Load-Register-Byte--register--?lang=en)
//!  - [LDRSB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--register---Load-Register-Signed-Byte--register--?lang=en)
//!  - [STR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--register--SIMD-FP---Store-SIMD-FP-register--register-offset--?lang=en)
//!  - [LDR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--register--SIMD-FP---Load-SIMD-FP-Register--register-offset--?lang=en)
//!  - [STRH - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRH--register---Store-Register-Halfword--register--?lang=en)
//!  - [LDRH - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRH--register---Load-Register-Halfword--register--?lang=en)
//!  - [LDRSH - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSH--register---Load-Register-Signed-Halfword--register--?lang=en)
//!  - [STR - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STR--register---Store-Register--register--?lang=en)
//!  - [LDR - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDR--register---Load-Register--register--?lang=en)
//!  - [LDRSW - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSW--register---Load-Register-Signed-Word--register--?lang=en)
//!  - [PRFM - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PRFM--register---Prefetch-Memory--register--?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Register, UImm1, UImm2, UImm3, UImm5};
use crate::types::extends::{RegExtend, RegExtendLSL};
use crate::types::prefetch_memory::PrfOp;

#[inline(always)]
fn emit_ld_st_reg_off<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    size: u8,
    v: u8,
    opc: u8,
    rm: Register,
    option: u8,
    s: u8,
    rn: Register,
    rt: Register,
) -> T {
    let r = bseq_32!(size:2 111 v:1 00 opc:2 1 rm:5 option:3 s:1 10 rn:5 rt:5);
    proc.process(r)
}

/// # [Load/store register (register offset)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldst_regoff)
///
/// Implements the following instructions:
///  - [STRB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRB--register---Store-Register-Byte--register--?lang=en)
///  - [LDRB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRB--register---Load-Register-Byte--register--?lang=en)
///  - [LDRSB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--register---Load-Register-Signed-Byte--register--?lang=en)
///  - [STR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--register--SIMD-FP---Store-SIMD-FP-register--register-offset--?lang=en)
///  - [LDR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--register--SIMD-FP---Load-SIMD-FP-Register--register-offset--?lang=en)
///  - [STRH - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRH--register---Store-Register-Halfword--register--?lang=en)
///  - [LDRH - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRH--register---Load-Register-Halfword--register--?lang=en)
///  - [LDRSH - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSH--register---Load-Register-Signed-Halfword--register--?lang=en)
///  - [STR - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STR--register---Store-Register--register--?lang=en)
///  - [LDR - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDR--register---Load-Register--register--?lang=en)
///  - [LDRSW - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSW--register---Load-Register-Signed-Word--register--?lang=en)
///  - [PRFM - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PRFM--register---Prefetch-Memory--register--?lang=en)
pub trait LoadStoreRegisterRegisterOffset<T>: InstructionProcessor<T> {
    /// [STRB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRB--register---Store-Register-Byte--register--?lang=en)
    ///
    /// Store Register Byte (register) calculates an address from a base register value and an offset register value, and stores a byte from a 32-bit register to the calculated address. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// The instruction uses an offset addressing mode, that calculates the address used for the memory access from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// ```asm
    /// STRB <Wt>, [<Xn|SP>, (<Wm>|<Xm>), <extend> {<amount>}]
    /// ```
    ///
    /// *Note:* whether `wm_xm` is `Wm` or `Xm` depends on the used extend.
    #[inline(always)]
    fn strb_reg_extend_reg(
        &mut self,
        wt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtend,
        amount: bool,
    ) -> T {
        debug_assert!(
            [RegExtend::UXTW, RegExtend::SXTW, RegExtend::SXTX].contains(&extend),
            "extend must be one of uxtw, sxtw and sxtx, was {}",
            extend
        );
        emit_ld_st_reg_off(
            self,
            0b00,
            0,
            0b00,
            wm_xm,
            extend.into(),
            amount.into(),
            xn_sp,
            wt,
        )
    }

    /// [STRB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRB--register---Store-Register-Byte--register--?lang=en)
    ///
    /// Store Register Byte (register) calculates an address from a base register value and an offset register value, and stores a byte from a 32-bit register to the calculated address. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// The instruction uses an offset addressing mode, that calculates the address used for the memory access from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// ```asm
    /// STRB <Wt>, [<Xn|SP>, <Xm>{, LSL <amount>}]
    /// ```
    #[inline(always)]
    fn strb_reg_shift_reg(
        &mut self,
        wt: Register,
        xn_sp: Register,
        xm: Register,
        amount: bool,
    ) -> T {
        emit_ld_st_reg_off(self, 0b00, 0, 0b00, xm, 0b011, amount.into(), xn_sp, wt)
    }

    /// [LDRB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRB--register---Load-Register-Byte--register--?lang=en)
    ///
    /// Load Register Byte (register) calculates an address from a base register value and an offset register value, loads a byte from memory, zero-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDRB <Wt>, [<Xn|SP>, (<Wm>|<Xm>), <extend> {<amount>}]
    /// ```
    #[inline(always)]
    fn ldrb_reg_extend_reg(
        &mut self,
        wt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtend,
        amount: bool,
    ) -> T {
        debug_assert!(
            [RegExtend::UXTW, RegExtend::SXTW, RegExtend::SXTX].contains(&extend),
            "extend must be one of uxtw, sxtw and sxtx, was {}",
            extend
        );
        emit_ld_st_reg_off(
            self,
            0b00,
            0,
            0b01,
            wm_xm,
            extend.into(),
            amount.into(),
            xn_sp,
            wt,
        )
    }

    /// [LDRB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRB--register---Load-Register-Byte--register--?lang=en)
    ///
    /// Load Register Byte (register) calculates an address from a base register value and an offset register value, loads a byte from memory, zero-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDRB <Wt>, [<Xn|SP>, <Xm>{, LSL <amount>}]
    /// ```
    #[inline(always)]
    fn ldrb_reg_shift_reg(
        &mut self,
        wt: Register,
        xn_sp: Register,
        xm: Register,
        amount: bool,
    ) -> T {
        emit_ld_st_reg_off(self, 0b00, 0, 0b01, xm, 0b011, amount.into(), xn_sp, wt)
    }

    /// [LDRSB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--register---Load-Register-Signed-Byte--register--?lang=en)
    ///
    /// Load Register Signed Byte (register) calculates an address from a base register value and an offset register value, loads a byte from memory, sign-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDRSB <Wt>, [<Xn|SP>, (<Wm>|<Xm>), <extend> {<amount>}]
    /// ```
    #[inline(always)]
    fn ldrsb_32_reg_extend_reg(
        &mut self,
        wt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtend,
        amount: bool,
    ) -> T {
        debug_assert!(
            [RegExtend::UXTW, RegExtend::SXTW, RegExtend::SXTX].contains(&extend),
            "extend must be one of uxtw, sxtw and sxtx, was {}",
            extend
        );
        emit_ld_st_reg_off(
            self,
            0b00,
            0,
            0b11,
            wm_xm,
            extend.into(),
            amount.into(),
            xn_sp,
            wt,
        )
    }

    /// [LDRSB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--register---Load-Register-Signed-Byte--register--?lang=en)
    ///
    /// Load Register Signed Byte (register) calculates an address from a base register value and an offset register value, loads a byte from memory, sign-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDRSB <Wt>, [<Xn|SP>, <Xm>{, LSL <amount>}]
    /// ```
    #[inline(always)]
    fn ldrsb_32_reg_shift_reg(
        &mut self,
        wt: Register,
        xn_sp: Register,
        xm: Register,
        amount: bool,
    ) -> T {
        emit_ld_st_reg_off(self, 0b00, 0, 0b11, xm, 0b011, amount.into(), xn_sp, wt)
    }

    /// [LDRSB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--register---Load-Register-Signed-Byte--register--?lang=en)
    ///
    /// Load Register Signed Byte (register) calculates an address from a base register value and an offset register value, loads a byte from memory, sign-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDRSB <Xt>, [<Xn|SP>, (<Wm>|<Xm>), <extend> {<amount>}]
    /// ```
    #[inline(always)]
    fn ldrsb_64_reg_extend_reg(
        &mut self,
        xt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtend,
        amount: bool,
    ) -> T {
        debug_assert!(
            [RegExtend::UXTW, RegExtend::SXTW, RegExtend::SXTX].contains(&extend),
            "extend must be one of uxtw, sxtw and sxtx, was {}",
            extend
        );
        emit_ld_st_reg_off(
            self,
            0b00,
            0,
            0b10,
            wm_xm,
            extend.into(),
            amount.into(),
            xn_sp,
            xt,
        )
    }

    /// [LDRSB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--register---Load-Register-Signed-Byte--register--?lang=en)
    ///
    /// Load Register Signed Byte (register) calculates an address from a base register value and an offset register value, loads a byte from memory, sign-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDRSB <Xt>, [<Xn|SP>, <Xm>{, LSL <amount>}]
    /// ```
    #[inline(always)]
    fn ldrsb_64_reg_shift_reg(
        &mut self,
        xt: Register,
        xn_sp: Register,
        xm: Register,
        amount: bool,
    ) -> T {
        emit_ld_st_reg_off(self, 0b00, 0, 0b10, xm, 0b011, amount.into(), xn_sp, xt)
    }

    /// [STR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--register--SIMD-FP---Store-SIMD-FP-register--register-offset--?lang=en)
    ///
    /// Store SIMD&FP register (register offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Bt>, [<Xn|SP>, (<Wm>|<Xm>), <extend> {<amount>}]
    /// ```
    #[inline(always)]
    fn str_8_simd_reg_extend_reg(
        &mut self,
        bt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtend,
        amount: bool,
    ) -> T {
        debug_assert!(
            [RegExtend::UXTW, RegExtend::SXTW, RegExtend::SXTX].contains(&extend),
            "extend must be one of uxtw, sxtw and sxtx, was {}",
            extend
        );
        emit_ld_st_reg_off(
            self,
            0b00,
            1,
            0b00,
            wm_xm,
            extend.into(),
            amount.into(),
            xn_sp,
            bt,
        )
    }

    /// [STR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--register--SIMD-FP---Store-SIMD-FP-register--register-offset--?lang=en)
    ///
    /// Store SIMD&FP register (register offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Bt>, [<Xn|SP>, <Xm>{, LSL <amount>}]
    /// ```
    #[inline(always)]
    fn str_8_simd_reg_shift_reg(
        &mut self,
        bt: Register,
        xn_sp: Register,
        xm: Register,
        amount: bool,
    ) -> T {
        emit_ld_st_reg_off(self, 0b00, 1, 0b00, xm, 0b011, amount.into(), xn_sp, bt)
    }

    /// [STR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--register--SIMD-FP---Store-SIMD-FP-register--register-offset--?lang=en)
    ///
    /// Store SIMD&FP register (register offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Ht>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn str_16_simd_reg(
        &mut self,
        ht: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm1,
    ) -> T {
        debug_assert!(
            [0, 1].contains(&amount),
            "amount must be either 0 or 1, was {}",
            amount
        );
        emit_ld_st_reg_off(self, 0b01, 1, 0b00, wm_xm, extend.into(), amount, xn_sp, ht)
    }

    /// [STR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--register--SIMD-FP---Store-SIMD-FP-register--register-offset--?lang=en)
    ///
    /// Store SIMD&FP register (register offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <St>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn str_32_simd_reg(
        &mut self,
        st: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm2,
    ) -> T {
        debug_assert!(
            [0, 2].contains(&amount),
            "amount must be either 0 or 2, was {}",
            amount
        );
        let amount = amount / 2;
        emit_ld_st_reg_off(self, 0b10, 1, 0b00, wm_xm, extend.into(), amount, xn_sp, st)
    }
    /// [STR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--register--SIMD-FP---Store-SIMD-FP-register--register-offset--?lang=en)
    ///
    /// Store SIMD&FP register (register offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Dt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn str_64_simd_reg(
        &mut self,
        dt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm3,
    ) -> T {
        debug_assert!(
            [0, 3].contains(&amount),
            "amount must be either 0 or 3, was {}",
            amount
        );
        let amount = amount / 3;
        emit_ld_st_reg_off(self, 0b11, 1, 0b00, wm_xm, extend.into(), amount, xn_sp, dt)
    }

    /// [STR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--register--SIMD-FP---Store-SIMD-FP-register--register-offset--?lang=en)
    ///
    /// Store SIMD&FP register (register offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Qt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn str_128_simd_reg(
        &mut self,
        qt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm1,
    ) -> T {
        debug_assert!(
            [0, 4].contains(&amount),
            "amount must be either 0 or 4, was {}",
            amount
        );
        let amount = amount / 4;
        emit_ld_st_reg_off(self, 0b00, 1, 0b10, wm_xm, extend.into(), amount, xn_sp, qt)
    }

    /// [LDR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--register--SIMD-FP---Load-SIMD-FP-Register--register-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (register offset). This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Bt>, [<Xn|SP>, (<Wm>|<Xm>), <extend> {<amount>}]
    /// ```
    #[inline(always)]
    fn ldr_8_simd_reg_extend_reg(
        &mut self,
        bt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtend,
        amount: bool,
    ) -> T {
        debug_assert!(
            [RegExtend::UXTW, RegExtend::SXTW, RegExtend::SXTX].contains(&extend),
            "extend must be one of uxtw, sxtw and sxtx, was {}",
            extend
        );
        emit_ld_st_reg_off(
            self,
            0b00,
            1,
            0b01,
            wm_xm,
            extend.into(),
            amount.into(),
            xn_sp,
            bt,
        )
    }

    /// [LDR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--register--SIMD-FP---Load-SIMD-FP-Register--register-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (register offset). This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Bt>, [<Xn|SP>, <Xm>{, LSL <amount>}]
    /// ```
    #[inline(always)]
    fn ldr_8_simd_reg_shift_reg(
        &mut self,
        bt: Register,
        xn_sp: Register,
        xm: Register,
        amount: bool,
    ) -> T {
        emit_ld_st_reg_off(self, 0b00, 1, 0b01, xm, 0b011, amount.into(), xn_sp, bt)
    }

    /// [LDR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--register--SIMD-FP---Load-SIMD-FP-Register--register-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (register offset). This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Ht>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn ldr_16_simd_reg(
        &mut self,
        ht: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm1,
    ) -> T {
        debug_assert!(
            [0, 1].contains(&amount),
            "amount must be either 0 or 1, was {}",
            amount
        );
        emit_ld_st_reg_off(self, 0b01, 1, 0b01, wm_xm, extend.into(), amount, xn_sp, ht)
    }

    /// [LDR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--register--SIMD-FP---Load-SIMD-FP-Register--register-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (register offset). This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <St>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn ldr_32_simd_reg(
        &mut self,
        st: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm1,
    ) -> T {
        debug_assert!(
            [0, 2].contains(&amount),
            "amount must be either 0 or 2, was {}",
            amount
        );
        let amount = amount / 2;
        emit_ld_st_reg_off(self, 0b10, 1, 0b01, wm_xm, extend.into(), amount, xn_sp, st)
    }

    /// [LDR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--register--SIMD-FP---Load-SIMD-FP-Register--register-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (register offset). This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Dt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn ldr_64_simd_reg(
        &mut self,
        dt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm3,
    ) -> T {
        debug_assert!(
            [0, 3].contains(&amount),
            "amount must be either 0 or 3, was {}",
            amount
        );
        let amount = amount / 3;
        emit_ld_st_reg_off(self, 0b11, 1, 0b01, wm_xm, extend.into(), amount, xn_sp, dt)
    }

    /// [LDR - register - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--register--SIMD-FP---Load-SIMD-FP-Register--register-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (register offset). This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Qt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn ldr_128_simd_reg(
        &mut self,
        qt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm1,
    ) -> T {
        debug_assert!(
            [0, 4].contains(&amount),
            "amount must be either 0 or 4, was {}",
            amount
        );
        let amount = amount / 4;
        emit_ld_st_reg_off(self, 0b00, 1, 0b11, wm_xm, extend.into(), amount, xn_sp, qt)
    }

    /// [STRH - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRH--register---Store-Register-Halfword--register--?lang=en)
    ///
    /// Store Register Halfword (register) calculates an address from a base register value and an offset register value, and stores a halfword from a 32-bit register to the calculated address. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// The instruction uses an offset addressing mode, that calculates the address used for the memory access from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// ```asm
    /// STRH <Wt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn strh_reg(
        &mut self,
        wt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm1,
    ) -> T {
        debug_assert!(
            [0, 1].contains(&amount),
            "amount must be either 0 or 1, was {}",
            amount
        );
        emit_ld_st_reg_off(self, 0b01, 0, 0b00, wm_xm, extend.into(), amount, xn_sp, wt)
    }

    /// [LDRH - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRH--register---Load-Register-Halfword--register--?lang=en)
    ///
    /// Load Register Halfword (register) calculates an address from a base register value and an offset register value, loads a halfword from memory, zero-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDRH <Wt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn ldrh_reg(
        &mut self,
        wt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm1,
    ) -> T {
        debug_assert!(
            [0, 1].contains(&amount),
            "amount must be either 0 or 1, was {}",
            amount
        );
        emit_ld_st_reg_off(self, 0b01, 0, 0b01, wm_xm, extend.into(), amount, xn_sp, wt)
    }

    /// [LDRSH - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSH--register---Load-Register-Signed-Halfword--register--?lang=en)
    ///
    /// Load Register Signed Halfword (register) calculates an address from a base register value and an offset register value, loads a halfword from memory, sign-extends it, and writes it to a register. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDRSH <Wt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn ldrsh_32_reg(
        &mut self,
        wt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm1,
    ) -> T {
        debug_assert!(
            [0, 1].contains(&amount),
            "amount must be either 0 or 1, was {}",
            amount
        );
        emit_ld_st_reg_off(self, 0b01, 0, 0b11, wm_xm, extend.into(), amount, xn_sp, wt)
    }

    /// [LDRSH - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSH--register---Load-Register-Signed-Halfword--register--?lang=en)
    ///
    /// Load Register Signed Halfword (register) calculates an address from a base register value and an offset register value, loads a halfword from memory, sign-extends it, and writes it to a register. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDRSH <Xt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn ldrsh_64_reg(
        &mut self,
        xt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm1,
    ) -> T {
        debug_assert!(
            [0, 1].contains(&amount),
            "amount must be either 0 or 1, was {}",
            amount
        );
        emit_ld_st_reg_off(self, 0b01, 0, 0b10, wm_xm, extend.into(), amount, xn_sp, xt)
    }

    /// [STR - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STR--register---Store-Register--register--?lang=en)
    ///
    /// Store Register (register) calculates an address from a base register value and an offset register value, and stores a 32-bit word or a 64-bit doubleword to the calculated address, from a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// The instruction uses an offset addressing mode, that calculates the address used for the memory access from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// ```asm
    /// STR <Wt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn str_32_reg(
        &mut self,
        wt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm1,
    ) -> T {
        debug_assert!(
            [0, 2].contains(&amount),
            "amount must be either 0 or 2, was {}",
            amount
        );
        let amount = amount / 2;
        emit_ld_st_reg_off(self, 0b10, 0, 0b00, wm_xm, extend.into(), amount, xn_sp, wt)
    }

    /// [STR - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STR--register---Store-Register--register--?lang=en)
    ///
    /// Store Register (register) calculates an address from a base register value and an offset register value, and stores a 32-bit word or a 64-bit doubleword to the calculated address, from a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// The instruction uses an offset addressing mode, that calculates the address used for the memory access from a base register value and an offset register value. The offset can be optionally shifted and extended.
    ///
    /// ```asm
    /// STR <Xt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn str_64_reg(
        &mut self,
        xt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm1,
    ) -> T {
        debug_assert!(
            [0, 3].contains(&amount),
            "amount must be either 0 or 3, was {}",
            amount
        );
        let amount = amount / 3;
        emit_ld_st_reg_off(self, 0b11, 0, 0b00, wm_xm, extend.into(), amount, xn_sp, xt)
    }

    /// [LDR - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDR--register---Load-Register--register--?lang=en)
    ///
    /// Load Register (register) calculates an address from a base register value and an offset register value, loads a word from memory, and writes it to a register. The offset register value can optionally be shifted and extended. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDR <Wt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn ldr_32_reg(
        &mut self,
        wt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm1,
    ) -> T {
        debug_assert!(
            [0, 2].contains(&amount),
            "amount must be either 0 or 2, was {}",
            amount
        );
        let amount = amount / 2;
        emit_ld_st_reg_off(self, 0b10, 0, 0b01, wm_xm, extend.into(), amount, xn_sp, wt)
    }

    /// [LDR - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDR--register---Load-Register--register--?lang=en)
    ///
    /// Load Register (register) calculates an address from a base register value and an offset register value, loads a word from memory, and writes it to a register. The offset register value can optionally be shifted and extended. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDR <Xt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn ldr_64_reg(
        &mut self,
        xt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm1,
    ) -> T {
        debug_assert!(
            [0, 3].contains(&amount),
            "amount must be either 0 or 3, was {}",
            amount
        );
        let amount = amount / 3;
        emit_ld_st_reg_off(self, 0b11, 0, 0b01, wm_xm, extend.into(), amount, xn_sp, xt)
    }

    /// [LDRSW - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSW--register---Load-Register-Signed-Word--register--?lang=en)
    ///
    /// Load Register Signed Word (register) calculates an address from a base register value and an offset register value, loads a word from memory, sign-extends it to form a 64-bit value, and writes it to a register. The offset register value can be shifted left by 0 or 2 bits. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDRSW <Xt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn ldrsw_reg(
        &mut self,
        xt: Register,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm2,
    ) -> T {
        debug_assert!(
            [0, 2].contains(&amount),
            "amount must be either 0 or 2, was {}",
            amount
        );
        let amount = amount / 2;
        emit_ld_st_reg_off(self, 0b10, 0, 0b10, wm_xm, extend.into(), amount, xn_sp, xt)
    }

    /// [PRFM - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PRFM--register---Prefetch-Memory--register--?lang=en)
    ///
    /// Prefetch Memory (register) signals the memory system that data memory accesses from a specified address are likely to occur in the near future. The memory system can respond by taking actions that are expected to speed up the memory accesses when they do occur, such as preloading the cache line containing the specified address into one or more caches.
    ///
    /// The effect of an PRFM instruction is implementation defined. For more information, see Prefetch memory.
    ///
    /// ```asm
    /// PRFM (<prfop>|#<imm5>), [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn prfm_reg_prfop(
        &mut self,
        prfop: PrfOp,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm2,
    ) -> T {
        self.prfm_reg_custom(prfop.encode(), xn_sp, wm_xm, extend, amount)
    }

    /// [PRFM - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PRFM--register---Prefetch-Memory--register--?lang=en)
    ///
    /// Prefetch Memory (register) signals the memory system that data memory accesses from a specified address are likely to occur in the near future. The memory system can respond by taking actions that are expected to speed up the memory accesses when they do occur, such as preloading the cache line containing the specified address into one or more caches.
    ///
    /// The effect of an PRFM instruction is implementation defined. For more information, see Prefetch memory.
    ///
    /// ```asm
    /// PRFM #<imm5>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
    /// ```
    #[inline(always)]
    fn prfm_reg_custom(
        &mut self,
        imm5: UImm5,
        xn_sp: Register,
        wm_xm: Register,
        extend: RegExtendLSL,
        amount: UImm2,
    ) -> T {
        debug_assert!(
            [0, 3].contains(&amount),
            "amount must be either 0 or 3, was {}",
            amount
        );
        let amount = amount / 3;
        emit_ld_st_reg_off(
            self,
            0b11,
            0,
            0b10,
            wm_xm,
            extend.into(),
            amount,
            xn_sp,
            imm5,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;
    use crate::types::prefetch_memory::{PrfPolicy, PrfTarget, PrfType};

    use super::*;

    #[test]
    fn test_strb() {
        let mut prod = TestProducer::new();

        let instr = prod.strb_reg_extend_reg(2, 3, 4, RegExtend::SXTW, false);
        assert_eq!(instr, "strb w2, [x3, w4, sxtw]");

        let instr = prod.strb_reg_extend_reg(2, 3, 4, RegExtend::SXTX, true);
        assert_eq!(instr, "strb w2, [x3, x4, sxtx]");

        assert_panic!("Should panic: non valid extend"; prod.strb_reg_extend_reg(2, 3, 4, RegExtend::UXTH, true));

        let instr = prod.strb_reg_shift_reg(2, 3, 4, true);
        assert_eq!(instr, "strb w2, [x3, x4, lsl #0x0]");
    }

    #[test]
    fn test_ldrb() {
        let mut prod = TestProducer::new();

        let instr = prod.ldrb_reg_extend_reg(2, 3, 4, RegExtend::UXTW, false);
        assert_eq!(instr, "ldrb w2, [x3, w4, uxtw]");

        let instr = prod.ldrb_reg_extend_reg(2, 3, 4, RegExtend::SXTX, true);
        assert_eq!(instr, "ldrb w2, [x3, x4, sxtx]");

        assert_panic!("Should panic: non valid extend"; prod.ldrb_reg_extend_reg(2, 3, 4, RegExtend::UXTH, true));

        let instr = prod.ldrb_reg_shift_reg(2, 3, 4, true);
        assert_eq!(instr, "ldrb w2, [x3, x4, lsl #0x0]");
    }

    #[test]
    fn test_ldrsb() {
        let mut prod = TestProducer::new();

        let instr = prod.ldrsb_32_reg_extend_reg(2, 3, 4, RegExtend::UXTW, false);
        assert_eq!(instr, "ldrsb w2, [x3, w4, uxtw]");

        let instr = prod.ldrsb_32_reg_extend_reg(2, 3, 4, RegExtend::SXTX, true);
        assert_eq!(instr, "ldrsb w2, [x3, x4, sxtx]");

        assert_panic!("Should panic: non valid extend"; prod.ldrsb_32_reg_extend_reg(2, 3, 4, RegExtend::UXTH, true));

        let instr = prod.ldrsb_64_reg_shift_reg(2, 3, 4, true);
        assert_eq!(instr, "ldrsb x2, [x3, x4, lsl #0x0]");

        let instr = prod.ldrsb_64_reg_extend_reg(2, 3, 4, RegExtend::UXTW, false);
        assert_eq!(instr, "ldrsb x2, [x3, w4, uxtw]");

        let instr = prod.ldrsb_64_reg_extend_reg(2, 3, 4, RegExtend::SXTX, true);
        assert_eq!(instr, "ldrsb x2, [x3, x4, sxtx]");

        assert_panic!("Should panic: non valid extend"; prod.ldrsb_64_reg_extend_reg(2, 3, 4, RegExtend::UXTH, true));

        let instr = prod.ldrsb_64_reg_shift_reg(2, 3, 4, true);
        assert_eq!(instr, "ldrsb x2, [x3, x4, lsl #0x0]");
    }

    #[test]
    fn test_str_simd() {
        let mut prod = TestProducer::new();

        let instr = prod.str_8_simd_reg_extend_reg(2, 3, 4, RegExtend::UXTW, false);
        assert_eq!(instr, "str b2, [x3, w4, uxtw]");

        let instr = prod.str_8_simd_reg_extend_reg(2, 3, 4, RegExtend::SXTX, true);
        assert_eq!(instr, "str b2, [x3, x4, sxtx]");

        assert_panic!("Should panic: non valid extend"; prod.str_8_simd_reg_extend_reg(2, 3, 4, RegExtend::UXTH, true));

        let instr = prod.str_8_simd_reg_shift_reg(2, 3, 4, true);
        assert_eq!(instr, "str b2, [x3, x4, lsl #0x0]");

        let instr = prod.str_16_simd_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(instr, "str h2, [x3, w4, uxtw]");

        let instr = prod.str_16_simd_reg(2, 3, 4, RegExtendLSL::LSL, 1);
        assert_eq!(instr, "str h2, [x3, x4, lsl #0x1]");

        assert_panic!("Should panic: invalid amount"; prod.str_16_simd_reg(2, 3, 4, RegExtendLSL::SXTW, 2));

        let instr = prod.str_32_simd_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(instr, "str s2, [x3, w4, uxtw]");

        let instr = prod.str_32_simd_reg(2, 3, 4, RegExtendLSL::LSL, 2);
        assert_eq!(instr, "str s2, [x3, x4, lsl #0x2]");

        assert_panic!("Should panic: invalid amount"; prod.str_32_simd_reg(2, 3, 4, RegExtendLSL::SXTW, 1));

        let instr = prod.str_64_simd_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(instr, "str d2, [x3, w4, uxtw]");

        let instr = prod.str_64_simd_reg(2, 3, 4, RegExtendLSL::LSL, 3);
        assert_eq!(instr, "str d2, [x3, x4, lsl #0x3]");

        assert_panic!("Should panic: invalid amount"; prod.str_64_simd_reg(2, 3, 4, RegExtendLSL::SXTW, 1));

        let instr = prod.str_128_simd_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(instr, "str q2, [x3, w4, uxtw]");

        let instr = prod.str_128_simd_reg(2, 3, 4, RegExtendLSL::LSL, 4);
        assert_eq!(instr, "str q2, [x3, x4, lsl #0x4]");

        assert_panic!("Should panic: invalid amount"; prod.str_128_simd_reg(2, 3, 4, RegExtendLSL::SXTW, 1));
    }

    #[test]
    fn test_ldr_simd() {
        let mut prod = TestProducer::new();

        let instr = prod.ldr_8_simd_reg_extend_reg(2, 3, 4, RegExtend::UXTW, false);
        assert_eq!(instr, "ldr b2, [x3, w4, uxtw]");

        let instr = prod.ldr_8_simd_reg_extend_reg(2, 3, 4, RegExtend::SXTX, true);
        assert_eq!(instr, "ldr b2, [x3, x4, sxtx]");

        assert_panic!("Should panic: non valid extend"; prod.ldr_8_simd_reg_extend_reg(2, 3, 4, RegExtend::UXTH, true));

        let instr = prod.ldr_8_simd_reg_shift_reg(2, 3, 4, true);
        assert_eq!(instr, "ldr b2, [x3, x4, lsl #0x0]");

        let instr = prod.ldr_16_simd_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(instr, "ldr h2, [x3, w4, uxtw]");

        let instr = prod.ldr_16_simd_reg(2, 3, 4, RegExtendLSL::LSL, 1);
        assert_eq!(instr, "ldr h2, [x3, x4, lsl #0x1]");

        assert_panic!("Should panic: invalid amount"; prod.ldr_16_simd_reg(2, 3, 4, RegExtendLSL::SXTW, 2));

        let instr = prod.ldr_32_simd_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(instr, "ldr s2, [x3, w4, uxtw]");

        let instr = prod.ldr_32_simd_reg(2, 3, 4, RegExtendLSL::LSL, 2);
        assert_eq!(instr, "ldr s2, [x3, x4, lsl #0x2]");

        assert_panic!("Should panic: invalid amount"; prod.ldr_32_simd_reg(2, 3, 4, RegExtendLSL::SXTW, 1));

        let instr = prod.ldr_64_simd_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(instr, "ldr d2, [x3, w4, uxtw]");

        let instr = prod.ldr_64_simd_reg(2, 3, 4, RegExtendLSL::LSL, 3);
        assert_eq!(instr, "ldr d2, [x3, x4, lsl #0x3]");

        assert_panic!("Should panic: invalid amount"; prod.ldr_64_simd_reg(2, 3, 4, RegExtendLSL::SXTW, 1));

        let instr = prod.ldr_128_simd_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(instr, "ldr q2, [x3, w4, uxtw]");

        let instr = prod.ldr_128_simd_reg(2, 3, 4, RegExtendLSL::LSL, 4);
        assert_eq!(instr, "ldr q2, [x3, x4, lsl #0x4]");

        assert_panic!("Should panic: invalid amount"; prod.ldr_128_simd_reg(2, 3, 4, RegExtendLSL::SXTW, 1));
    }

    #[test]
    fn test_strh() {
        let mut prod = TestProducer::new();

        let instr = prod.strh_reg(2, 3, 4, RegExtendLSL::SXTW, 0);
        assert_eq!(instr, "strh w2, [x3, w4, sxtw]");

        let instr = prod.strh_reg(2, 3, 4, RegExtendLSL::SXTX, 1);
        assert_eq!(instr, "strh w2, [x3, x4, sxtx #0x1]");

        assert_panic!("Should panic: invalid amount"; prod.strh_reg(2, 3, 4, RegExtendLSL::UXTW, 2));
    }

    #[test]
    fn test_ldrh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldrh_reg(2, 3, 4, RegExtendLSL::SXTW, 0);
        assert_eq!(instr, "ldrh w2, [x3, w4, sxtw]");

        let instr = prod.ldrh_reg(2, 3, 4, RegExtendLSL::SXTX, 1);
        assert_eq!(instr, "ldrh w2, [x3, x4, sxtx #0x1]");

        assert_panic!("Should panic: invalid amount"; prod.ldrh_reg(2, 3, 4, RegExtendLSL::UXTW, 2));
    }

    #[test]
    fn test_ldrsh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldrsh_32_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(instr, "ldrsh w2, [x3, w4, uxtw]");

        let instr = prod.ldrsh_32_reg(2, 3, 4, RegExtendLSL::SXTX, 1);
        assert_eq!(instr, "ldrsh w2, [x3, x4, sxtx #0x1]");

        assert_panic!("Should panic: invalid amount"; prod.ldrsh_32_reg(2, 3, 4, RegExtendLSL::SXTX, 2));

        let instr = prod.ldrsh_64_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(instr, "ldrsh x2, [x3, w4, uxtw]");

        let instr = prod.ldrsh_64_reg(2, 3, 4, RegExtendLSL::SXTX, 1);
        assert_eq!(instr, "ldrsh x2, [x3, x4, sxtx #0x1]");

        assert_panic!("Should panic: invalid amount"; prod.ldrsh_64_reg(2, 3, 4, RegExtendLSL::SXTW, 2));
    }

    #[test]
    fn test_str() {
        let mut prod = TestProducer::new();

        let instr = prod.str_32_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(instr, "str w2, [x3, w4, uxtw]");

        let instr = prod.str_32_reg(2, 3, 4, RegExtendLSL::SXTX, 2);
        assert_eq!(instr, "str w2, [x3, x4, sxtx #0x2]");

        assert_panic!("Should panic: invalid amount"; prod.str_32_reg(2, 3, 4, RegExtendLSL::SXTX, 1));

        let instr = prod.str_64_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(instr, "str x2, [x3, w4, uxtw]");

        let instr = prod.str_64_reg(2, 3, 4, RegExtendLSL::SXTX, 3);
        assert_eq!(instr, "str x2, [x3, x4, sxtx #0x3]");

        assert_panic!("Should panic: invalid amount"; prod.str_64_reg(2, 3, 4, RegExtendLSL::SXTW, 2));
    }

    #[test]
    fn test_ldr() {
        let mut prod = TestProducer::new();

        let inldr = prod.ldr_32_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(inldr, "ldr w2, [x3, w4, uxtw]");

        let inldr = prod.ldr_32_reg(2, 3, 4, RegExtendLSL::SXTX, 2);
        assert_eq!(inldr, "ldr w2, [x3, x4, sxtx #0x2]");

        assert_panic!("Should panic: invalid amount"; prod.ldr_32_reg(2, 3, 4, RegExtendLSL::SXTX, 1));

        let inldr = prod.ldr_64_reg(2, 3, 4, RegExtendLSL::UXTW, 0);
        assert_eq!(inldr, "ldr x2, [x3, w4, uxtw]");

        let inldr = prod.ldr_64_reg(2, 3, 4, RegExtendLSL::SXTX, 3);
        assert_eq!(inldr, "ldr x2, [x3, x4, sxtx #0x3]");

        assert_panic!("Should panic:  invalid amount"; prod.ldr_64_reg(2, 3, 4, RegExtendLSL::SXTW, 2));
    }

    #[test]
    fn test_ldrsw() {
        let mut prod = TestProducer::new();

        let instr = prod.ldrsw_reg(2, 3, 4, RegExtendLSL::SXTW, 0);
        assert_eq!(instr, "ldrsw x2, [x3, w4, sxtw]");

        let instr = prod.ldrsw_reg(2, 3, 4, RegExtendLSL::SXTX, 2);
        assert_eq!(instr, "ldrsw x2, [x3, x4, sxtx #0x2]");

        assert_panic!("Should panic:  invalid amount"; prod.ldrsw_reg(2, 3, 4, RegExtendLSL::UXTW, 1));
    }

    #[test]
    fn test_prfm() {
        let mut prod = TestProducer::new();

        let prfop = PrfOp(PrfType::PLI, PrfTarget::L3, PrfPolicy::KEEP);
        let instr = prod.prfm_reg_prfop(prfop, 3, 4, RegExtendLSL::SXTW, 0);
        assert_eq!(instr, "prfm plil3keep, [x3, w4, sxtw]");

        let instr = prod.prfm_reg_custom(prfop.encode(), 3, 4, RegExtendLSL::SXTX, 3);
        assert_eq!(instr, "prfm plil3keep, [x3, x4, sxtx #0x3]");

        assert_panic!("Should panic: invalid amount"; prod.prfm_reg_prfop(prfop, 3, 4, RegExtendLSL::UXTW, 2));
    }
}
