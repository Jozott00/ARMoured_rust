//! # [Load/store register (immediate post-indexed)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldst_immpost)
//!
//! Implements the following instructions:
//!  - [STRB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRB--immediate---Store-Register-Byte--immediate--?lang=en)
//!  - [LDRB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRB--immediate---Load-Register-Byte--immediate--?lang=en)
//!  - [LDRSB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--immediate---Load-Register-Signed-Byte--immediate--?lang=en)
//!  - [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
//!  - [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
//!  - [STRH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRH--immediate---Store-Register-Halfword--immediate--?lang=en)
//!  - [LDRH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRH--immediate---Load-Register-Halfword--immediate--?lang=en)
//!  - [LDRSH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSH--immediate---Load-Register-Signed-Halfword--immediate--?lang=en)
//!  - [STR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STR--immediate---Store-Register--immediate--?lang=en)
//!  - [LDR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDR--immediate---Load-Register--immediate--?lang=en)
//!  - [LDRSW - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSW--immediate---Load-Register-Signed-Word--immediate--?lang=en)
///  - [PRFM - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PRFM--immediate---Prefetch-Memory--immediate--?lang=en)


use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Register, UImm12, UImm13, UImm14, UImm15, UImm16, UImm5};
use crate::types::prefetch_memory::PrfOp;

#[inline(always)]
fn emit_load_store_offset<P: InstructionProcessor<T>, T>(proc: &mut P, size: u8, V: u8, opc: u8, pimm: u16, rn: Register, rt: Register) -> T {
    debug_assert!(0 <= pimm && pimm <= 4095, "pimm must be in range 0 to 4095");
    let r = bseq_32!(size:2 111 V:1 01 opc:2 pimm:12 rn:5 rt:5);
    proc.process(r)
}

/// # [Load/store register (immediate post-indexed)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldst_immpost)
///
/// Implements the following instructions:
///  - [STRB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRB--immediate---Store-Register-Byte--immediate--?lang=en)
///  - [LDRB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRB--immediate---Load-Register-Byte--immediate--?lang=en)
///  - [LDRSB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--immediate---Load-Register-Signed-Byte--immediate--?lang=en)
///  - [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
///  - [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
///  - [STRH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRH--immediate---Store-Register-Halfword--immediate--?lang=en)
///  - [LDRH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRH--immediate---Load-Register-Halfword--immediate--?lang=en)
///  - [LDRSH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSH--immediate---Load-Register-Signed-Halfword--immediate--?lang=en)
///  - [STR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STR--immediate---Store-Register--immediate--?lang=en)
///  - [LDR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDR--immediate---Load-Register--immediate--?lang=en)
///  - [LDRSW - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSW--immediate---Load-Register-Signed-Word--immediate--?lang=en)
/// ///  - [PRFM - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PRFM--immediate---Prefetch-Memory--immediate--?lang=en)
pub trait LoadStoreRegisterUnsignedImmediate<T>: InstructionProcessor<T> {
    /// [STRB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRB--immediate---Store-Register-Byte--immediate--?lang=en)
    ///
    /// Store Register Byte (immediate) stores the least significant byte of a 32-bit register to memory. The address that is used for the store is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// STRB <Wt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn strb_imm_unsigned_offset(&mut self, wt: Register, xn_sp: Register, pimm: UImm12) -> T {
        emit_load_store_offset(self, 0, 0, 0, pimm, xn_sp, wt)
    }


    /// [LDRB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRB--immediate---Load-Register-Byte--immediate--?lang=en)
    ///
    /// Load Register Byte (immediate) loads a byte from memory, zero-extends it, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRB <Wt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldrb_imm_unsigned_offset(&mut self, wt: Register, xn_sp: Register, pimm: UImm12) -> T {
        emit_load_store_offset(self, 0, 0, 0b01, pimm, xn_sp, wt)
    }


    /// [LDRSB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--immediate---Load-Register-Signed-Byte--immediate--?lang=en)
    ///
    /// Load Register Signed Byte (immediate) loads a byte from memory, sign-extends it to either 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSB <Wt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldrsb_32_imm_unsigned_offset(&mut self, wt: Register, xn_sp: Register, pimm: UImm12) -> T {
        emit_load_store_offset(self, 0, 0, 0b11, pimm, xn_sp, wt)
    }


    /// [LDRSB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--immediate---Load-Register-Signed-Byte--immediate--?lang=en)
    ///
    /// Load Register Signed Byte (immediate) loads a byte from memory, sign-extends it to either 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSB <Xt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldrsb_64_imm_unsigned_offset(&mut self, xt: Register, xn_sp: Register, pimm: UImm12) -> T {
        emit_load_store_offset(self, 0, 0, 0b10, pimm, xn_sp, xt)
    }


    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Bt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn str_8_imm_simd_unsigned_offset(&mut self, bt: Register, xn_sp: Register, pimm: UImm12) -> T {
        emit_load_store_offset(self, 0b00, 1, 0b00, pimm, xn_sp, bt)
    }


    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Ht>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn str_16_imm_simd_unsigned_offset(&mut self, ht: Register, xn_sp: Register, pimm: UImm13) -> T {
        debug_assert!(pimm <= 8190, "pimm must be in range 0 to 8190, was {}", pimm);
        debug_assert!(pimm % 2 == 0, "pimm must be multiply of 2, was {}", pimm);
        let pimm = pimm / 2;
        emit_load_store_offset(self, 0b01, 1, 0b00, pimm, xn_sp, ht)
    }


    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <St>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn str_32_imm_simd_unsigned_offset(&mut self, st: Register, xn_sp: Register, pimm: UImm14) -> T {
        debug_assert!(pimm <= 16380, "pimm must be in range 0 to 16380, was {}", pimm);
        debug_assert!(pimm % 4 == 0, "pimm must be multiply of 4, was {}", pimm);
        let pimm = pimm / 4;
        emit_load_store_offset(self, 0b10, 1, 0b00, pimm, xn_sp, st)
    }


    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Dt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn str_64_imm_simd_unsigned_offset(&mut self, dt: Register, xn_sp: Register, pimm: UImm15) -> T {
        debug_assert!(pimm <= 32760, "pimm must be in range 0 to 32760, was {}", pimm);
        debug_assert!(pimm % 8 == 0, "pimm must be multiply of 8, was {}", pimm);
        let pimm = pimm / 8;
        emit_load_store_offset(self, 0b11, 1, 0b00, pimm, xn_sp, dt)
    }


    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Qt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn str_128_imm_simd_unsigned_offset(&mut self, qt: Register, xn_sp: Register, pimm: UImm16) -> T {
        debug_assert!(pimm <= 65520, "pimm must be in range 0 to 65520, was {}", pimm);
        debug_assert!(pimm % 16 == 0, "pimm must be multiply of 16, was {}", pimm);
        let pimm = pimm / 16;
        emit_load_store_offset(self, 0b00, 1, 0b10, pimm, xn_sp, qt)
    }


    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Bt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldr_8_imm_simd_unsigned_offset(&mut self, bt: Register, xn_sp: Register, pimm: UImm12) -> T {
        emit_load_store_offset(self, 0b00, 1, 0b01, pimm, xn_sp, bt)
    }

    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Ht>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldr_16_imm_simd_unsigned_offset(&mut self, ht: Register, xn_sp: Register, pimm: UImm13) -> T {
        debug_assert!(pimm <= 8190, "pimm must be in range 0 to 8190, was {}", pimm);
        debug_assert!(pimm % 2 == 0, "pimm must be multiply of 2, was {}", pimm);
        let pimm = pimm / 2;
        emit_load_store_offset(self, 0b01, 1, 0b01, pimm, xn_sp, ht)
    }

    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <St>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldr_32_imm_simd_unsigned_offset(&mut self, st: Register, xn_sp: Register, pimm: UImm14) -> T {
        debug_assert!(pimm <= 16380, "pimm must be in range 0 to 16380, was {}", pimm);
        debug_assert!(pimm % 4 == 0, "pimm must be multiply of 4, was {}", pimm);
        let pimm = pimm / 4;
        emit_load_store_offset(self, 0b10, 1, 0b01, pimm, xn_sp, st)
    }


    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Dt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldr_64_imm_simd_unsigned_offset(&mut self, dt: Register, xn_sp: Register, pimm: UImm15) -> T {
        debug_assert!(pimm <= 32760, "pimm must be in range 0 to 32760, was {}", pimm);
        debug_assert!(pimm % 8 == 0, "pimm must be multiply of 8, was {}", pimm);
        let pimm = pimm / 8;
        emit_load_store_offset(self, 0b11, 1, 0b01, pimm, xn_sp, dt)
    }


    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Qt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldr_128_imm_simd_unsigned_offset(&mut self, qt: Register, xn_sp: Register, pimm: UImm16) -> T {
        debug_assert!(pimm <= 65520, "pimm must be in range 0 to 65520, was {}", pimm);
        debug_assert!(pimm % 16 == 0, "pimm must be multiply of 16, was {}", pimm);
        let pimm = pimm / 16;
        emit_load_store_offset(self, 0b00, 1, 0b11, pimm, xn_sp, qt)
    }


    /// [STRH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRH--immediate---Store-Register-Halfword--immediate--?lang=en)
    ///
    /// Store Register Halfword (immediate) stores the least significant halfword of a 32-bit register to memory. The address that is used for the store is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// STRH <Wt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn strh_imm_unsigned_offset(&mut self, wt: Register, xn_sp: Register, pimm: UImm13) -> T {
        debug_assert!(pimm <= 8190, "pimm must be in range 0 to 8190, was {}", pimm);
        debug_assert!(pimm % 2 == 0, "pimm must be multiply of 2, was {}", pimm);
        let pimm = pimm / 2;
        emit_load_store_offset(self, 0b01, 0, 0, pimm, xn_sp, wt)
    }


    /// [LDRH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRH--immediate---Load-Register-Halfword--immediate--?lang=en)
    ///
    /// Load Register Halfword (immediate) loads a halfword from memory, zero-extends it, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRH <Wt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldrh_imm_unsigned_offset(&mut self, wt: Register, xn_sp: Register, pimm: UImm13) -> T {
        debug_assert!(pimm <= 8190, "pimm must be in range 0 to 8190, was {}", pimm);
        debug_assert!(pimm % 2 == 0, "pimm must be multiply of 2, was {}", pimm);
        let pimm = pimm / 2;
        emit_load_store_offset(self, 0b01, 0, 0b01, pimm, xn_sp, wt)
    }

    /// [LDRSH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSH--immediate---Load-Register-Signed-Halfword--immediate--?lang=en)
    ///
    /// Load Register Signed Halfword (immediate) loads a halfword from memory, sign-extends it to 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSH <Wt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldrsh_32_imm_unsigned_offset(&mut self, wt: Register, xn_sp: Register, pimm: UImm13) -> T {
        debug_assert!(pimm <= 8190, "pimm must be in range 0 to 8190, was {}", pimm);
        debug_assert!(pimm % 2 == 0, "pimm must be multiply of 2, was {}", pimm);
        let pimm = pimm / 2;
        emit_load_store_offset(self, 0b01, 0, 0b11, pimm, xn_sp, wt)
    }


    /// [LDRSH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSH--immediate---Load-Register-Signed-Halfword--immediate--?lang=en)
    ///
    /// Load Register Signed Halfword (immediate) loads a halfword from memory, sign-extends it to 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSH <Xt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldrsh_64_imm_unsigned_offset(&mut self, xt: Register, xn_sp: Register, pimm: UImm13) -> T {
        debug_assert!(pimm <= 8190, "pimm must be in range 0 to 8190, was {}", pimm);
        debug_assert!(pimm % 2 == 0, "pimm must be multiply of 2, was {}", pimm);
        let pimm = pimm / 2;
        emit_load_store_offset(self, 0b01, 0, 0b10, pimm, xn_sp, xt)
    }


    /// [STR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STR--immediate---Store-Register--immediate--?lang=en)
    ///
    /// Store Register (immediate) stores a word or a doubleword from a register to memory. The address that is used for the store is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// STR <Wt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn str_32_imm_unsigned_offset(&mut self, wt: Register, xn_sp: Register, pimm: UImm14) -> T {
        debug_assert!(pimm <= 16380, "pimm must be in range 0 to 16380, was {}", pimm);
        debug_assert!(pimm % 4 == 0, "pimm must be multiply of 2, was {}", pimm);
        let pimm = pimm / 4;
        emit_load_store_offset(self, 0b10, 0, 0b00, pimm, xn_sp, wt)
    }

    /// [STR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STR--immediate---Store-Register--immediate--?lang=en)
    ///
    /// Store Register (immediate) stores a word or a doubleword from a register to memory. The address that is used for the store is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// STR <Xt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn str_64_imm_unsigned_offset(&mut self, xt: Register, xn_sp: Register, pimm: UImm15) -> T {
        debug_assert!(pimm <= 32760, "pimm must be in range 0 to 32760, was {}", pimm);
        debug_assert!(pimm % 8 == 0, "pimm must be multiply of 2, was {}", pimm);
        let pimm = pimm / 8;
        emit_load_store_offset(self, 0b11, 0, 0b00, pimm, xn_sp, xt)
    }


    /// [LDR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDR--immediate---Load-Register--immediate--?lang=en)
    ///
    /// Load Register (immediate) loads a word or doubleword from memory and writes it to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes. The Unsigned offset variant scales the immediate offset value by the size of the value accessed before adding it to the base register value.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDR <Wt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldr_32_imm_unsigned_offset(&mut self, wt: Register, xn_sp: Register, pimm: UImm12) -> T {
        debug_assert!(pimm <= 16380, "pimm must be in range 0 to 16380, was {}", pimm);
        debug_assert!(pimm % 4 == 0, "pimm must be multiply of 2, was {}", pimm);
        let pimm = pimm / 4;
        emit_load_store_offset(self, 0b10, 0, 0b01, pimm, xn_sp, wt)
    }


    /// [LDR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDR--immediate---Load-Register--immediate--?lang=en)
    ///
    /// Load Register (immediate) loads a word or doubleword from memory and writes it to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes. The Unsigned offset variant scales the immediate offset value by the size of the value accessed before adding it to the base register value.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDR <Xt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldr_64_imm_unsigned_offset(&mut self, xt: Register, xn_sp: Register, pimm: UImm14) -> T {
        debug_assert!(pimm % 8 == 0, "pimm must be multiply of 2, was {}", pimm);
        let pimm = pimm / 8;
        emit_load_store_offset(self, 0b11, 0, 0b01, pimm, xn_sp, xt)
    }


    /// [LDRSW - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSW--immediate---Load-Register-Signed-Word--immediate--?lang=en)
    ///
    /// Load Register Signed Word (immediate) loads a word from memory, sign-extends it to 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSW <Xt>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn ldrsw_imm_unsigned_offset(&mut self, xt: Register, xn_sp: Register, pimm: UImm12) -> T {
        debug_assert!(pimm <= 16380, "pimm must be in range 0 to 16380, was {}", pimm);
        debug_assert!(pimm % 4 == 0, "pimm must be multiply of 4, was {}", pimm);
        let pimm = pimm / 4;
        emit_load_store_offset(self, 0b10, 0, 0b10, pimm, xn_sp, xt)
    }

    /// [PRFM - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PRFM--immediate---Prefetch-Memory--immediate--?lang=en)
    ///
    /// Prefetch Memory (immediate) signals the memory system that data memory accesses from a specified address are likely to occur in the near future. The memory system can respond by taking actions that are expected to speed up the memory accesses when they do occur, such as preloading the cache line containing the specified address into one or more caches.
    ///
    /// The effect of an PRFM instruction is implementation defined. For more information, see Prefetch memory.
    ///
    /// For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// PRFM <prfop>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn prfm_imm_prfop(&mut self, prfop: PrfOp, xn_sp: Register, pimm: UImm15) -> T {
        debug_assert!(pimm <= 32760, "pimm must be in range 0 to 32760, was {}", pimm);
        debug_assert!(pimm % 8 == 0, "pimm must be multiply of 8, was {}", pimm);
        let pimm = pimm / 8;
        emit_load_store_offset(self, 0b11, 0, 0b10, pimm, xn_sp, prfop.encode())
    }

    /// [PRFM - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/PRFM--immediate---Prefetch-Memory--immediate--?lang=en)
    ///
    /// Prefetch Memory (immediate) signals the memory system that data memory accesses from a specified address are likely to occur in the near future. The memory system can respond by taking actions that are expected to speed up the memory accesses when they do occur, such as preloading the cache line containing the specified address into one or more caches.
    ///
    /// The effect of an PRFM instruction is implementation defined. For more information, see Prefetch memory.
    ///
    /// For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// PRFM #<imm5>, [<Xn|SP>{, #<pimm>}]
    /// ```
    #[inline(always)]
    fn prfm_imm_custom(&mut self, imm5: UImm5, xn_sp: Register, pimm: UImm15) -> T {
        debug_assert!(pimm <= 32760, "pimm must be in range 0 to 32760, was {}", pimm);
        debug_assert!(pimm % 8 == 0, "pimm must be multiply of 8, was {}", pimm);
        let pimm = pimm / 8;
        emit_load_store_offset(self, 0b11, 0, 0b10, pimm, xn_sp, imm5)
    }
}


#[cfg(test)]
mod tests {
    use bad64::decode;
    use super::*;
    use crate::mc_memory::MockMemory;
    use crate::instruction_emitter::MockEmitter;
    use crate::{stream_mock, assert_panic};
    use crate::types::InstructionPointer;
    use crate::types::prefetch_memory::PrfPolicy::{KEEP, STRM};
    use crate::types::prefetch_memory::PrfTarget::{L1, L2, L3};
    use crate::types::prefetch_memory::PrfType::{PLD, PLI, PST};
    use crate::instruction_stream::InstrStream;

    #[test]
    fn test_strb_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.strb_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "strb w0, [x3]");

            let instr = stream.strb_imm_unsigned_offset(0, 0b11111, 4095);
            assert_eq!(instr.to_string(), "strb w0, [sp, #0xfff]");

            assert_panic!("Should panic: imm out of range"; stream.strb_imm_unsigned_offset(0, 3, 4096));
        })
    }

    #[test]
    fn test_ldrb_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldrb_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldrb w0, [x3]");

            let instr = stream.ldrb_imm_unsigned_offset(0, 0b11111, 4095);
            assert_eq!(instr.to_string(), "ldrb w0, [sp, #0xfff]");

            assert_panic!("Should panic: imm out of range"; stream.ldrb_imm_unsigned_offset(0, 3, 4096));
        })
    }

    #[test]
    fn test_ldrsb_32_imm_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldrsb_32_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldrsb w0, [x3]");

            let instr = stream.ldrsb_32_imm_unsigned_offset(0, 0b11111, 4095);
            assert_eq!(instr.to_string(), "ldrsb w0, [sp, #0xfff]");

            assert_panic!("Should panic: imm out of range"; stream.ldrsb_32_imm_unsigned_offset(0, 3, 4096));
        })
    }

    #[test]
    fn test_ldrsb_64_imm_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldrsb_64_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldrsb x0, [x3]");

            let instr = stream.ldrsb_64_imm_unsigned_offset(0, 0b11111, 4095);
            assert_eq!(instr.to_string(), "ldrsb x0, [sp, #0xfff]");

            assert_panic!("Should panic: imm out of range"; stream.ldrsb_64_imm_unsigned_offset(0, 3, 4096));
        })
    }

    #[test]
    fn test_strh_imm_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.strh_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "strh w0, [x3]");

            let instr = stream.strh_imm_unsigned_offset(0, 0b11111, 2);
            assert_eq!(instr.to_string(), "strh w0, [sp, #0x2]");

            let instr = stream.strh_imm_unsigned_offset(0, 0b11111, 8190);
            assert_eq!(instr.to_string(), "strh w0, [sp, #0x1ffe]");

            assert_panic!("Should panic: pimm not multiply of 2"; stream.strh_imm_unsigned_offset(0, 3, 1));
            assert_panic!("Should panic: imm out of range"; stream.strh_imm_unsigned_offset(0, 3, 8191));
        })
    }

    #[test]
    fn test_ldrh_imm_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldrh_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldrh w0, [x3]");

            let instr = stream.ldrh_imm_unsigned_offset(0, 0b11111, 2);
            assert_eq!(instr.to_string(), "ldrh w0, [sp, #0x2]");

            let instr = stream.ldrh_imm_unsigned_offset(0, 0b11111, 8190);
            assert_eq!(instr.to_string(), "ldrh w0, [sp, #0x1ffe]");

            assert_panic!("Should panic: pimm not multiply of 2"; stream.ldrh_imm_unsigned_offset(0, 3, 1));
            assert_panic!("Should panic: imm out of range"; stream.ldrh_imm_unsigned_offset(0, 3, 8191));
        })
    }


    #[test]
    fn test_ldrsh_32_imm_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldrsh_32_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldrsh w0, [x3]");

            let instr = stream.ldrsh_32_imm_unsigned_offset(0, 3, 2);
            assert_eq!(instr.to_string(), "ldrsh w0, [x3, #0x2]");

            let instr = stream.ldrsh_32_imm_unsigned_offset(0, 0b11111, 8190);
            assert_eq!(instr.to_string(), "ldrsh w0, [sp, #0x1ffe]");

            assert_panic!("Should panic: pimm not multiply of 2"; stream.ldrsh_32_imm_unsigned_offset(0, 3, 1));
            assert_panic!("Should panic: imm out of range"; stream.ldrsh_32_imm_unsigned_offset(0, 3, 8191));
        })
    }


    #[test]
    fn test_ldrsh_64_imm_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldrsh_64_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldrsh x0, [x3]");

            let instr = stream.ldrsh_64_imm_unsigned_offset(0, 3, 2);
            assert_eq!(instr.to_string(), "ldrsh x0, [x3, #0x2]");

            let instr = stream.ldrsh_64_imm_unsigned_offset(0, 0b11111, 8190);
            assert_eq!(instr.to_string(), "ldrsh x0, [sp, #0x1ffe]");

            assert_panic!("Should panic: pimm not multiply of 2"; stream.ldrsh_64_imm_unsigned_offset(0, 3, 1));
            assert_panic!("Should panic: imm out of range"; stream.ldrsh_64_imm_unsigned_offset(0, 3, 8191));
        })
    }

    #[test]
    fn test_str_32_imm_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.str_32_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "str w0, [x3]");

            let instr = stream.str_32_imm_unsigned_offset(0, 3, 4);
            assert_eq!(instr.to_string(), "str w0, [x3, #0x4]");

            let instr = stream.str_32_imm_unsigned_offset(0, 0b11111, 16380);
            assert_eq!(instr.to_string(), "str w0, [sp, #0x3ffc]");

            assert_panic!("Should panic: pimm not multiply of 4"; stream.str_32_imm_unsigned_offset(0, 3, 1));
            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_unsigned_offset(0, 3, 16381));
        })
    }

    #[test]
    fn test_str_64_imm_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.str_64_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "str x0, [x3]");

            let instr = stream.str_64_imm_unsigned_offset(0, 3, 8);
            assert_eq!(instr.to_string(), "str x0, [x3, #0x8]");

            let instr = stream.str_64_imm_unsigned_offset(0, 0b11111, 32760);
            assert_eq!(instr.to_string(), "str x0, [sp, #0x7ff8]");

            assert_panic!("Should panic: pimm not multiply of 8"; stream.str_64_imm_unsigned_offset(0, 3, 1));
            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_unsigned_offset(0, 3, 32761));
        })
    }


    #[test]
    fn test_ldr_32_imm_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_32_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldr w0, [x3]");

            let instr = stream.ldr_32_imm_unsigned_offset(0, 3, 4);
            assert_eq!(instr.to_string(), "ldr w0, [x3, #0x4]");

            let instr = stream.ldr_32_imm_unsigned_offset(0, 0b11111, 16380);
            assert_eq!(instr.to_string(), "ldr w0, [sp, #0x3ffc]");

            assert_panic!("Should panic: pimm not multiply of 4"; stream.ldr_32_imm_unsigned_offset(0, 3, 1));
            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_unsigned_offset(0, 3, 16381));
        })
    }

    #[test]
    fn test_ldr_64_imm_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_64_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldr x0, [x3]");

            let instr = stream.ldr_64_imm_unsigned_offset(0, 3, 8);
            assert_eq!(instr.to_string(), "ldr x0, [x3, #0x8]");

            let instr = stream.ldr_64_imm_unsigned_offset(0, 0b11111, 32760);
            assert_eq!(instr.to_string(), "ldr x0, [sp, #0x7ff8]");

            assert_panic!("Should panic: pimm not multiply of 8"; stream.ldr_64_imm_unsigned_offset(0, 3, 1));
            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_unsigned_offset(0, 3, 32761));
        })
    }

    #[test]
    fn test_str_8_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.str_8_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "str b0, [x3]");

            let instr = stream.str_8_imm_simd_unsigned_offset(0, 0b11111, 4095);
            assert_eq!(instr.to_string(), "str b0, [sp, #0xfff]");

            assert_panic!("Should panic: imm out of range"; stream.str_8_imm_simd_unsigned_offset(0, 3, 4096));
        })
    }

    #[test]
    fn test_ldrsw_imm_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldrsw_imm_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldrsw x0, [x3]");

            let instr = stream.ldrsw_imm_unsigned_offset(0, 0b11111, 4);
            assert_eq!(instr.to_string(), "ldrsw x0, [sp, #0x4]");

            let instr = stream.ldrsw_imm_unsigned_offset(0, 0b11111, 16380);
            assert_eq!(instr.to_string(), "ldrsw x0, [sp, #0x3ffc]");

            assert_panic!("Should panic: pimm not multiply of 4"; stream.ldrsw_imm_unsigned_offset(0, 3, 1));
            assert_panic!("Should panic: imm out of range"; stream.ldrsw_imm_unsigned_offset(0, 3, 16381));
        })
    }

    #[test]
    fn test_prfm_imm_prfop() {
        stream_mock!(stream, {

            let prfop = PrfOp(PLD, L1, KEEP);
            let instr = stream.prfm_imm_prfop(prfop, 3, 0x0);
            assert_eq!(instr.to_string(), "prfm pldl1keep, [x3]");

            let prfop = PrfOp(PLI, L2, STRM);
            let instr = stream.prfm_imm_prfop(prfop, 3, 0x8);
            assert_eq!(instr.to_string(), "prfm plil2strm, [x3, #0x8]");

            let prfop = PrfOp(PST, L3, KEEP);
            let instr = stream.prfm_imm_prfop(prfop, 3, 32760);
            assert_eq!(instr.to_string(), "prfm pstl3keep, [x3, #0x7ff8]");


            assert_panic!("Should panic: pimm not multiply of 8"; stream.prfm_imm_prfop(prfop, 3, 1));
            assert_panic!("Should panic: imm out of range"; stream.prfm_imm_prfop(prfop, 3, 32761));
        })
    }

    #[test]
    fn test_prfm_imm_custom() {
        stream_mock!(stream, {

            let cust = PrfOp(PLD, L1, KEEP).encode();
            let instr = stream.prfm_imm_custom(cust, 3, 0x0);
            assert_eq!(instr.to_string(), "prfm pldl1keep, [x3]");

            let instr = stream.prfm_imm_custom(cust, 3, 0x8);
            assert_eq!(instr.to_string(), "prfm pldl1keep, [x3, #0x8]");

            let instr = stream.prfm_imm_custom(cust, 3, 32760);
            assert_eq!(instr.to_string(), "prfm pldl1keep, [x3, #0x7ff8]");

            assert_panic!("Should panic: custom out of range"; stream.prfm_imm_custom(32, 3, 32761));
            assert_panic!("Should panic: pimm not multiply of 8"; stream.prfm_imm_custom(0, 3, 1));
            assert_panic!("Should panic: imm out of range"; stream.prfm_imm_custom(0, 3, 32761));
        })
    }

    #[test]
    fn test_str_16_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.str_16_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "str h0, [x3]");

            let instr = stream.str_16_imm_simd_unsigned_offset(0, 3, 2);
            assert_eq!(instr.to_string(), "str h0, [x3, #0x2]");

            let instr = stream.str_16_imm_simd_unsigned_offset(0, 0b11111, 8190);
            assert_eq!(instr.to_string(), "str h0, [sp, #0x1ffe]");

            assert_panic!("Should panic: imm out of range"; stream.str_16_imm_simd_unsigned_offset(0, 3, 8191));
            assert_panic!("Should panic: no multiply of 2"; stream.str_16_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    #[test]
    fn test_str_32_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.str_32_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "str s0, [x3]");

            let instr = stream.str_32_imm_simd_unsigned_offset(0, 3, 4);
            assert_eq!(instr.to_string(), "str s0, [x3, #0x4]");

            let instr = stream.str_32_imm_simd_unsigned_offset(0, 0b11111, 16380);
            assert_eq!(instr.to_string(), "str s0, [sp, #0x3ffc]");

            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_simd_unsigned_offset(0, 3, 16381));
            assert_panic!("Should panic: no multiply of 4"; stream.str_32_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    #[test]
    fn test_str_64_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.str_64_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "str d0, [x3]");

            let instr = stream.str_64_imm_simd_unsigned_offset(0, 3, 8);
            assert_eq!(instr.to_string(), "str d0, [x3, #0x8]");

            let instr = stream.str_64_imm_simd_unsigned_offset(0, 0b11111, 32760);
            assert_eq!(instr.to_string(), "str d0, [sp, #0x7ff8]");

            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_simd_unsigned_offset(0, 3, 32761));
            assert_panic!("Should panic: no multiply of 8"; stream.str_64_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    #[test]
    fn test_str_128_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.str_128_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "str q0, [x3]");

            let instr = stream.str_128_imm_simd_unsigned_offset(0, 3, 16);
            assert_eq!(instr.to_string(), "str q0, [x3, #0x10]");

            let instr = stream.str_128_imm_simd_unsigned_offset(0, 0b11111, 65520);
            assert_eq!(instr.to_string(), "str q0, [sp, #0xfff0]");

            assert_panic!("Should panic: imm out of range"; stream.str_128_imm_simd_unsigned_offset(0, 3, 65521));
            assert_panic!("Should panic: no multiply of 8"; stream.str_128_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    #[test]
    fn test_ldr_8_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_8_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldr b0, [x3]");

            let instr = stream.ldr_8_imm_simd_unsigned_offset(0, 0b11111, 4095);
            assert_eq!(instr.to_string(), "ldr b0, [sp, #0xfff]");

            assert_panic!("Should panic: imm out of range"; stream.ldr_8_imm_simd_unsigned_offset(0, 3, 4096));
        })
    }

    #[test]
    fn test_ldr_16_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_16_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldr h0, [x3]");

            let instr = stream.ldr_16_imm_simd_unsigned_offset(0, 3, 2);
            assert_eq!(instr.to_string(), "ldr h0, [x3, #0x2]");

            let instr = stream.ldr_16_imm_simd_unsigned_offset(0, 0b11111, 8190);
            assert_eq!(instr.to_string(), "ldr h0, [sp, #0x1ffe]");

            assert_panic!("Should panic: imm out of range"; stream.ldr_16_imm_simd_unsigned_offset(0, 3, 8191));
            assert_panic!("Should panic: no multiply of 2"; stream.ldr_16_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    #[test]
    fn test_ldr_32_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_32_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldr s0, [x3]");

            let instr = stream.ldr_32_imm_simd_unsigned_offset(0, 3, 4);
            assert_eq!(instr.to_string(), "ldr s0, [x3, #0x4]");

            let instr = stream.ldr_32_imm_simd_unsigned_offset(0, 0b11111, 16380);
            assert_eq!(instr.to_string(), "ldr s0, [sp, #0x3ffc]");

            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_simd_unsigned_offset(0, 3, 16381));
            assert_panic!("Should panic: no multiply of 4"; stream.ldr_32_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    #[test]
    fn test_ldr_64_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_64_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldr d0, [x3]");

            let instr = stream.ldr_64_imm_simd_unsigned_offset(0, 3, 8);
            assert_eq!(instr.to_string(), "ldr d0, [x3, #0x8]");

            let instr = stream.ldr_64_imm_simd_unsigned_offset(0, 0b11111, 32760);
            assert_eq!(instr.to_string(), "ldr d0, [sp, #0x7ff8]");

            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_simd_unsigned_offset(0, 3, 32761));
            assert_panic!("Should panic: no multiply of 8"; stream.ldr_64_imm_simd_unsigned_offset(0, 3, 1));
        })
    }

    #[test]
    fn test_ldr_128_imm_simd_unsigned_offset() {
        stream_mock!(stream, {
            let instr = stream.ldr_128_imm_simd_unsigned_offset(0, 3, 0);
            assert_eq!(instr.to_string(), "ldr q0, [x3]");

            let instr = stream.ldr_128_imm_simd_unsigned_offset(0, 3, 16);
            assert_eq!(instr.to_string(), "ldr q0, [x3, #0x10]");

            let instr = stream.ldr_128_imm_simd_unsigned_offset(0, 0b11111, 65520);
            assert_eq!(instr.to_string(), "ldr q0, [sp, #0xfff0]");

            assert_panic!("Should panic: imm out of range"; stream.ldr_128_imm_simd_unsigned_offset(0, 3, 65521));
            assert_panic!("Should panic: no multiply of 8"; stream.ldr_128_imm_simd_unsigned_offset(0, 3, 1));
        })
    }
}
