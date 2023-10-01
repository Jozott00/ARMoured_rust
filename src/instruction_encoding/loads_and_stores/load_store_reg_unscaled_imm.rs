//! # [Load/store register (unscaled immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldst_unscaled)
//!
//! Implements the following instructions:
//!  - [STURB - Store Register Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STURB--Store-Register-Byte--unscaled--?lang=en)
//!  - [LDURB - Load Register Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURB--Load-Register-Byte--unscaled--?lang=en)
//!  - [LDURSB - Load Register Signed Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURSB--Load-Register-Signed-Byte--unscaled--?lang=en)
//!  - [STUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STUR--SIMD-FP---Store-SIMD-FP-register--unscaled-offset--?lang=en)
//!  - [LDUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDUR--SIMD-FP---Load-SIMD-FP-Register--unscaled-offset--?lang=en)
//!  - [STURH - Store Register Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STURH--Store-Register-Halfword--unscaled--?lang=en)
//!  - [LDURH - Load Register Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURH--Load-Register-Halfword--unscaled--?lang=en)
//!  - [LDURSH - Load Register Signed Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURSH--Load-Register-Signed-Halfword--unscaled--?lang=en)
//!  - [STUR - Store Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STUR--Store-Register--unscaled--?lang=en)
//!  - [LDUR - Load Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUR--Load-Register--unscaled--?lang=en)
//!  - [LDURSW - Load Register Signed Word - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURSW--Load-Register-Signed-Word--unscaled--?lang=en)
//!  - [PRFUM - Prefetch Memory - unscaled offset - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PRFUM--Prefetch-Memory--unscaled-offset--?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Imm9, Register, UImm5};
use crate::types::prefetch_memory::PrfOp;

#[inline(always)]
fn emit_ld_st<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    size: u8,
    v: u8,
    opc: u8,
    imm9: u16,
    rn: Register,
    rt: Register,
) -> T {
    let r = bseq_32!(size:2 111 v:1 00 opc:2 0 imm9:9 00 rn:5 rt:5);
    proc.process(r)
}

#[inline(always)]
fn emit_ld_st_checked<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    size: u8,
    v: u8,
    opc: u8,
    simm: Imm9,
    rn: Register,
    rt: Register,
) -> T {
    emit_ld_st(proc, size, v, opc, simm as u16, rn, rt)
}

/// # [Load/store register (unscaled immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldst_unscaled)
///
/// Implements the following instructions:
///  - [STURB - Store Register Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STURB--Store-Register-Byte--unscaled--?lang=en)
///  - [LDURB - Load Register Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURB--Load-Register-Byte--unscaled--?lang=en)
///  - [LDURSB - Load Register Signed Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURSB--Load-Register-Signed-Byte--unscaled--?lang=en)
///  - [STUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STUR--SIMD-FP---Store-SIMD-FP-register--unscaled-offset--?lang=en)
///  - [LDUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDUR--SIMD-FP---Load-SIMD-FP-Register--unscaled-offset--?lang=en)
///  - [STURH - Store Register Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STURH--Store-Register-Halfword--unscaled--?lang=en)
///  - [LDURH - Load Register Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURH--Load-Register-Halfword--unscaled--?lang=en)
///  - [LDURSH - Load Register Signed Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURSH--Load-Register-Signed-Halfword--unscaled--?lang=en)
///  - [STUR - Store Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STUR--Store-Register--unscaled--?lang=en)
///  - [LDUR - Load Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUR--Load-Register--unscaled--?lang=en)
///  - [LDURSW - Load Register Signed Word - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURSW--Load-Register-Signed-Word--unscaled--?lang=en)
///  - [PRFUM - Prefetch Memory - unscaled offset - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PRFUM--Prefetch-Memory--unscaled-offset--?lang=en)
pub trait LoadStoreRegisterUnscaledImmediate<T>: InstructionProcessor<T> {
    /// [STURB - Store Register Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STURB--Store-Register-Byte--unscaled--?lang=en)
    ///
    /// Store Register Byte (unscaled) calculates an address from a base register value and an immediate offset, and stores a byte to the calculated address, from a 32-bit register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STURB <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn sturb(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b00, 0, 0b00, simm, xn_sp, wt)
    }

    /// [LDURB - Load Register Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURB--Load-Register-Byte--unscaled--?lang=en)
    ///
    /// Load Register Byte (unscaled) calculates an address from a base register and an immediate offset, loads a byte from memory, zero-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDURB <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldurb(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b00, 0, 0b01, simm, xn_sp, wt)
    }

    /// [LDURSB - Load Register Signed Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURSB--Load-Register-Signed-Byte--unscaled--?lang=en)
    ///
    /// Load Register Signed Byte (unscaled) calculates an address from a base register and an immediate offset, loads a signed byte from memory, sign-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDURSB <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldursb_32(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b00, 0, 0b11, simm, xn_sp, wt)
    }

    /// [LDURSB - Load Register Signed Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURSB--Load-Register-Signed-Byte--unscaled--?lang=en)
    ///
    /// Load Register Signed Byte (unscaled) calculates an address from a base register and an immediate offset, loads a signed byte from memory, sign-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDURSB <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldursb_64(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b00, 0, 0b10, simm, xn_sp, xt)
    }

    /// [STUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STUR--SIMD-FP---Store-SIMD-FP-register--unscaled-offset--?lang=en)
    ///
    /// Store SIMD&FP register (unscaled offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an optional immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STUR <Bt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stur_8_simd(&mut self, bt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b00, 1, 0b00, simm, xn_sp, bt)
    }

    /// [STUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STUR--SIMD-FP---Store-SIMD-FP-register--unscaled-offset--?lang=en)
    ///
    /// Store SIMD&FP register (unscaled offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an optional immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STUR <Ht>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stur_16_simd(&mut self, ht: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b01, 1, 0b00, simm, xn_sp, ht)
    }

    /// [STUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STUR--SIMD-FP---Store-SIMD-FP-register--unscaled-offset--?lang=en)
    ///
    /// Store SIMD&FP register (unscaled offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an optional immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STUR <St>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stur_32_simd(&mut self, st: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b10, 1, 0b00, simm, xn_sp, st)
    }

    /// [STUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STUR--SIMD-FP---Store-SIMD-FP-register--unscaled-offset--?lang=en)
    ///
    /// Store SIMD&FP register (unscaled offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an optional immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STUR <Dt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stur_64_simd(&mut self, dt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b11, 1, 0b00, simm, xn_sp, dt)
    }

    /// [STUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STUR--SIMD-FP---Store-SIMD-FP-register--unscaled-offset--?lang=en)
    ///
    /// Store SIMD&FP register (unscaled offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an optional immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STUR <Qt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stur_128_simd(&mut self, qt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b00, 1, 0b10, simm, xn_sp, qt)
    }

    /// [LDUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDUR--SIMD-FP---Load-SIMD-FP-Register--unscaled-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (unscaled offset). This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from a base register value and an optional immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDUR <Bt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldur_8_simd(&mut self, bt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b00, 1, 0b01, simm, xn_sp, bt)
    }

    /// [LDUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDUR--SIMD-FP---Load-SIMD-FP-Register--unscaled-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (unscaled offset). This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from a base register value and an optional immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDUR <Ht>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldur_16_simd(&mut self, ht: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b01, 1, 0b01, simm, xn_sp, ht)
    }

    /// [LDUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDUR--SIMD-FP---Load-SIMD-FP-Register--unscaled-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (unscaled offset). This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from a base register value and an optional immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDUR <St>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldur_32_simd(&mut self, st: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b10, 1, 0b01, simm, xn_sp, st)
    }

    /// [LDUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDUR--SIMD-FP---Load-SIMD-FP-Register--unscaled-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (unscaled offset). This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from a base register value and an optional immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDUR <Dt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldur_64_simd(&mut self, dt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b11, 1, 0b01, simm, xn_sp, dt)
    }

    /// [LDUR - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDUR--SIMD-FP---Load-SIMD-FP-Register--unscaled-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (unscaled offset). This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from a base register value and an optional immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDUR <Qt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldur_128_simd(&mut self, qt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b00, 1, 0b11, simm, xn_sp, qt)
    }

    /// [STURH - Store Register Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STURH--Store-Register-Halfword--unscaled--?lang=en)
    ///
    /// Store Register Halfword (unscaled) calculates an address from a base register value and an immediate offset, and stores a halfword to the calculated address, from a 32-bit register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STURH <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn sturh(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b01, 0, 0b00, simm, xn_sp, wt)
    }

    /// [LDURH - Load Register Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURH--Load-Register-Halfword--unscaled--?lang=en)
    ///
    /// Load Register Halfword (unscaled) calculates an address from a base register and an immediate offset, loads a halfword from memory, zero-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDURH <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldurh(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b01, 0, 0b01, simm, xn_sp, wt)
    }

    /// [LDURSH - Load Register Signed Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURSH--Load-Register-Signed-Halfword--unscaled--?lang=en)
    ///
    /// Load Register Signed Halfword (unscaled) calculates an address from a base register and an immediate offset, loads a signed halfword from memory, sign-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDURSH <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldursh_32(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b01, 0, 0b11, simm, xn_sp, wt)
    }

    /// [LDURSH - Load Register Signed Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURSH--Load-Register-Signed-Halfword--unscaled--?lang=en)
    ///
    /// Load Register Signed Halfword (unscaled) calculates an address from a base register and an immediate offset, loads a signed halfword from memory, sign-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDURSH <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldursh_64(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b01, 0, 0b10, simm, xn_sp, xt)
    }

    /// [STUR - Store Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STUR--Store-Register--unscaled--?lang=en)
    ///
    /// Store Register (unscaled) calculates an address from a base register value and an immediate offset, and stores a 32-bit word or a 64-bit doubleword to the calculated address, from a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STUR <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stur_32(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b10, 0, 0b00, simm, xn_sp, wt)
    }

    /// [STUR - Store Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STUR--Store-Register--unscaled--?lang=en)
    ///
    /// Store Register (unscaled) calculates an address from a base register value and an immediate offset, and stores a 32-bit word or a 64-bit doubleword to the calculated address, from a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STUR <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stur_64(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b11, 0, 0b00, simm, xn_sp, xt)
    }

    /// [LDUR - Load Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUR--Load-Register--unscaled--?lang=en)
    ///
    /// Load Register (unscaled) calculates an address from a base register and an immediate offset, loads a 32-bit word or 64-bit doubleword from memory, zero-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDUR <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldur_32(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b10, 0, 0b01, simm, xn_sp, wt)
    }

    /// [LDUR - Load Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUR--Load-Register--unscaled--?lang=en)
    ///
    /// Load Register (unscaled) calculates an address from a base register and an immediate offset, loads a 32-bit word or 64-bit doubleword from memory, zero-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDUR <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldur_64(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b11, 0, 0b01, simm, xn_sp, xt)
    }

    /// [LDURSW - Load Register Signed Word - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDURSW--Load-Register-Signed-Word--unscaled--?lang=en)
    ///
    /// Load Register Signed Word (unscaled) calculates an address from a base register and an immediate offset, loads a signed word from memory, sign-extends it, and writes it to a register. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDURSW <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldursw(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b10, 0, 0b10, simm, xn_sp, xt)
    }

    /// [PRFUM - Prefetch Memory - unscaled offset - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PRFUM--Prefetch-Memory--unscaled-offset--?lang=en)
    ///
    /// Prefetch Memory (unscaled offset) signals the memory system that data memory accesses from a specified address are likely to occur in the near future. The memory system can respond by taking actions that are expected to speed up the memory accesses when they do occur, such as preloading the cache line containing the specified address into one or more caches.
    ///
    /// The effect of an PRFUM instruction is implementation defined. For more information, see Prefetch memory.
    ///
    /// ```asm
    /// PRFUM (<prfop>|#<imm5>), [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn prfum_prfop(&mut self, prfop: PrfOp, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b11, 0, 0b10, simm, xn_sp, prfop.encode())
    }

    /// [PRFUM - Prefetch Memory - unscaled offset - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PRFUM--Prefetch-Memory--unscaled-offset--?lang=en)
    ///
    /// Prefetch Memory (unscaled offset) signals the memory system that data memory accesses from a specified address are likely to occur in the near future. The memory system can respond by taking actions that are expected to speed up the memory accesses when they do occur, such as preloading the cache line containing the specified address into one or more caches.
    ///
    /// The effect of an PRFUM instruction is implementation defined. For more information, see Prefetch memory.
    ///
    /// ```asm
    /// PRFUM (<prfop>|#<imm5>), [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn prfum_custom(&mut self, imm5: UImm5, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_checked(self, 0b11, 0, 0b10, simm, xn_sp, imm5)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;
    use crate::types::prefetch_memory::{PrfPolicy, PrfTarget, PrfType};

    use super::*;

    #[test]
    fn test_sturb() {
        let mut prod = TestProducer::new();

        let instr = prod.sturb(2, 3, -256);
        assert_eq!(instr, "sturb w2, [x3, #0xffffffffffffff00]");
        let instr = prod.sturb(2, 3, -257);
        assert_eq!(instr, "sturb w2, [x3, #0xff]");
    }

    #[test]
    fn test_ldurb() {
        let mut prod = TestProducer::new();

        let instr = prod.ldurb(2, 3, -256);
        assert_eq!(instr, "ldurb w2, [x3, #0xffffffffffffff00]");
        let instr = prod.ldurb(2, 3, -257);
        assert_eq!(instr, "ldurb w2, [x3, #0xff]");
    }

    #[test]
    fn test_ldursb() {
        let mut prod = TestProducer::new();

        let instr = prod.ldursb_32(2, 3, -256);
        assert_eq!(instr, "ldursb w2, [x3, #0xffffffffffffff00]");

        let instr = prod.ldursb_64(2, 3, -257);
        assert_eq!(instr, "ldursb x2, [x3, #0xff]");
    }

    #[test]
    fn test_stur_simd() {
        let mut prod = TestProducer::new();

        let instr = prod.stur_8_simd(2, 3, -256);
        assert_eq!(instr, "stur b2, [x3, #0xffffffffffffff00]");

        let instr = prod.stur_16_simd(2, 3, -256);
        assert_eq!(instr, "stur h2, [x3, #0xffffffffffffff00]");

        let instr = prod.stur_32_simd(2, 3, -256);
        assert_eq!(instr, "stur s2, [x3, #0xffffffffffffff00]");

        let instr = prod.stur_64_simd(2, 3, -257);
        assert_eq!(instr, "stur d2, [x3, #0xff]");

        let instr = prod.stur_128_simd(2, 3, -257);
        assert_eq!(instr, "stur q2, [x3, #0xff]");
    }

    #[test]
    fn test_ldur_simd() {
        let mut prod = TestProducer::new();

        let instr = prod.ldur_8_simd(2, 3, -256);
        assert_eq!(instr, "ldur b2, [x3, #0xffffffffffffff00]");

        let instr = prod.ldur_16_simd(2, 3, -256);
        assert_eq!(instr, "ldur h2, [x3, #0xffffffffffffff00]");

        let instr = prod.ldur_32_simd(2, 3, -256);
        assert_eq!(instr, "ldur s2, [x3, #0xffffffffffffff00]");

        let instr = prod.ldur_64_simd(2, 3, -257);
        assert_eq!(instr, "ldur d2, [x3, #0xff]");

        let instr = prod.ldur_128_simd(2, 3, -257);
        assert_eq!(instr, "ldur q2, [x3, #0xff]");
    }

    #[test]
    fn test_sturh() {
        let mut prod = TestProducer::new();

        let instr = prod.sturh(2, 3, -256);
        assert_eq!(instr, "sturh w2, [x3, #0xffffffffffffff00]");
        let instr = prod.sturh(2, 3, -257);
        assert_eq!(instr, "sturh w2, [x3, #0xff]");
    }

    #[test]
    fn test_ldurh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldurh(2, 3, -256);
        assert_eq!(instr, "ldurh w2, [x3, #0xffffffffffffff00]");
        let instr = prod.ldurh(2, 3, -257);
        assert_eq!(instr, "ldurh w2, [x3, #0xff]");
    }

    #[test]
    fn test_ldursh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldursh_32(2, 3, -256);
        assert_eq!(instr, "ldursh w2, [x3, #0xffffffffffffff00]");

        let instr = prod.ldursh_64(2, 3, -257);
        assert_eq!(instr, "ldursh x2, [x3, #0xff]");
    }

    #[test]
    fn test_stur() {
        let mut prod = TestProducer::new();

        let instr = prod.stur_32(2, 3, -256);
        assert_eq!(instr, "stur w2, [x3, #0xffffffffffffff00]");

        let instr = prod.stur_64(2, 3, -257);
        assert_eq!(instr, "stur x2, [x3, #0xff]");
    }

    #[test]
    fn test_ldur() {
        let mut prod = TestProducer::new();

        let instr = prod.ldur_32(2, 3, -256);
        assert_eq!(instr, "ldur w2, [x3, #0xffffffffffffff00]");

        let instr = prod.ldur_64(2, 3, -257);
        assert_eq!(instr, "ldur x2, [x3, #0xff]");
    }

    #[test]
    fn test_ldursw() {
        let mut prod = TestProducer::new();

        let instr = prod.ldursw(2, 3, -256);
        assert_eq!(instr, "ldursw x2, [x3, #0xffffffffffffff00]");
        let instr = prod.ldursw(2, 3, -257);
        assert_eq!(instr, "ldursw x2, [x3, #0xff]");
    }

    #[test]
    fn test_prfum() {
        let mut prod = TestProducer::new();

        let prfop = PrfOp(PrfType::PLD, PrfTarget::L2, PrfPolicy::STRM);
        let instr = prod.prfum_prfop(prfop, 3, -256);
        assert_eq!(instr, "prfum pldl2strm, [x3, #0xffffffffffffff00]");

        let instr = prod.prfum_custom(prfop.encode(), 3, -256);
        assert_eq!(instr, "prfum pldl2strm, [x3, #0xffffffffffffff00]");
    }
}
