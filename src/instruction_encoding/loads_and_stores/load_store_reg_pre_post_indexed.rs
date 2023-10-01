//! # [Load/store register (immediate pre- and post-indexed)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldst_immpost)
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

use bit_seq::bseq_32;

use crate::instruction_emitter::Emitter;
use crate::instruction_encoding::InstructionProcessor;
use crate::mc_memory::Memory;
use crate::types::{Imm9, Register};

/// # Arguments
/// - mode: if pre (`0b11`) or post (`0b01`) index. Between `imm9` and `Rn` in encoding.
#[inline(always)]
fn emit_load_store_pre_post<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    size: u8,
    V: u8,
    opc: u8,
    simm: Imm9,
    mode: u8,
    wt: Register,
    xn_sp: Register,
) -> T {
    debug_assert!(
        -256 <= simm && simm <= 255,
        "simm must be in range -256 to 255"
    );
    let r = bseq_32!(size:2 111 V:1 00 opc:2 0 simm:9 mode:2 xn_sp:5 wt:5);
    proc.process(r)
}

/// # [Load/store register (immediate pre- and post-indexed)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldst_immpost)
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
pub trait LoadStoreRegisterPrePostIndexed<T>: InstructionProcessor<T> {
    // API methods

    // STRB instructions
    /// [STRB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRB--immediate---Store-Register-Byte--immediate--?lang=en)
    ///
    /// Store Register Byte (immediate) stores the least significant byte of a 32-bit register to memory. The address that is used for the store is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// STRB <Wt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn strb_imm_post_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0, 0, 0, simm, 0b01, wt, xn_sp)
    }

    /// [STRB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRB--immediate---Store-Register-Byte--immediate--?lang=en)
    ///
    /// Store Register Byte (immediate) stores the least significant byte of a 32-bit register to memory. The address that is used for the store is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// STRB <Wt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn strb_imm_pre_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0, 0, 0, simm, 0b11, wt, xn_sp)
    }

    // LDRB instructions

    /// [LDRB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRB--immediate---Load-Register-Byte--immediate--?lang=en)
    ///
    /// Load Register Byte (immediate) loads a byte from memory, zero-extends it, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRB <Wt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldrb_imm_post_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0, 0, 0b01, simm, 0b01, wt, xn_sp)
    }

    /// [LDRB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRB--immediate---Load-Register-Byte--immediate--?lang=en)
    ///
    /// Load Register Byte (immediate) loads a byte from memory, zero-extends it, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRB <Wt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldrb_imm_pre_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0, 0, 0b01, simm, 0b11, wt, xn_sp)
    }

    // LDRSB instructions

    /// [LDRSB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--immediate---Load-Register-Signed-Byte--immediate--?lang=en)
    ///
    /// Load Register Signed Byte (immediate) loads a byte from memory, sign-extends it to either 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSB <Wt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldrsb_32_imm_post_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0, 0, 0b11, simm, 0b01, wt, xn_sp)
    }

    /// [LDRSB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--immediate---Load-Register-Signed-Byte--immediate--?lang=en)
    ///
    /// Load Register Signed Byte (immediate) loads a byte from memory, sign-extends it to either 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSB <Wt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldrsb_32_imm_pre_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0, 0, 0b11, simm, 0b11, wt, xn_sp)
    }

    /// [LDRSB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--immediate---Load-Register-Signed-Byte--immediate--?lang=en)
    ///
    /// Load Register Signed Byte (immediate) loads a byte from memory, sign-extends it to either 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSB <Xt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldrsb_64_imm_post_index(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0, 0, 0b10, simm, 0b01, xt, xn_sp)
    }

    /// [LDRSB - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSB--immediate---Load-Register-Signed-Byte--immediate--?lang=en)
    ///
    /// Load Register Signed Byte (immediate) loads a byte from memory, sign-extends it to either 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSB <Xt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldrsb_64_imm_pre_index(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0, 0, 0b10, simm, 0b11, xt, xn_sp)
    }

    // STRH Imm instructions

    /// [STRH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRH--immediate---Store-Register-Halfword--immediate--?lang=en)
    ///
    /// Store Register Halfword (immediate) stores the least significant halfword of a 32-bit register to memory. The address that is used for the store is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// STRH <Wt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn strh_imm_post_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b01, 0, 0, simm, 0b01, wt, xn_sp)
    }

    /// [STRH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STRH--immediate---Store-Register-Halfword--immediate--?lang=en)
    ///
    /// Store Register Halfword (immediate) stores the least significant halfword of a 32-bit register to memory. The address that is used for the store is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// STRH <Wt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn strh_imm_pre_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b01, 0, 0, simm, 0b11, wt, xn_sp)
    }

    // LDRH Imm instructions

    /// [LDRH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRH--immediate---Load-Register-Halfword--immediate--?lang=en)
    ///
    /// Load Register Halfword (immediate) loads a halfword from memory, zero-extends it, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRH <Wt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldrh_imm_post_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b01, 0, 0b01, simm, 0b01, wt, xn_sp)
    }

    /// [LDRH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRH--immediate---Load-Register-Halfword--immediate--?lang=en)
    ///
    /// Load Register Halfword (immediate) loads a halfword from memory, zero-extends it, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRH <Wt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldrh_imm_pre_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b01, 0, 0b01, simm, 0b11, wt, xn_sp)
    }

    // LDRH Imm instructions

    /// [LDRSH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSH--immediate---Load-Register-Signed-Halfword--immediate--?lang=en)
    ///
    /// Load Register Signed Halfword (immediate) loads a halfword from memory, sign-extends it to 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSH <Wt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldrsh_32_imm_post_index(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b01, 0, 0b11, simm, 0b01, xt, xn_sp)
    }

    /// [LDRSH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSH--immediate---Load-Register-Signed-Halfword--immediate--?lang=en)
    ///
    /// Load Register Signed Halfword (immediate) loads a halfword from memory, sign-extends it to 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSH <Wt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldrsh_32_imm_pre_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b01, 0, 0b11, simm, 0b11, wt, xn_sp)
    }

    /// [LDRSH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSH--immediate---Load-Register-Signed-Halfword--immediate--?lang=en)
    ///
    /// Load Register Signed Halfword (immediate) loads a halfword from memory, sign-extends it to 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSH <Xt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldrsh_64_imm_post_index(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b01, 0, 0b10, simm, 0b01, xt, xn_sp)
    }

    /// [LDRSH - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSH--immediate---Load-Register-Signed-Halfword--immediate--?lang=en)
    ///
    /// Load Register Signed Halfword (immediate) loads a halfword from memory, sign-extends it to 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSH <Xt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldrsh_64_imm_pre_index(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b01, 0, 0b10, simm, 0b11, xt, xn_sp)
    }

    // STR Imm instructions

    /// [STR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STR--immediate---Store-Register--immediate--?lang=en)
    ///
    /// Store Register (immediate) stores a word or a doubleword from a register to memory. The address that is used for the store is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// STR <Wt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn str_32_imm_post_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b10, 0, 0b00, simm, 0b01, wt, xn_sp)
    }

    /// [STR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STR--immediate---Store-Register--immediate--?lang=en)
    ///
    /// Store Register (immediate) stores a word or a doubleword from a register to memory. The address that is used for the store is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// STR <Wt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn str_32_imm_pre_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b10, 0, 0b00, simm, 0b11, wt, xn_sp)
    }

    /// [STR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STR--immediate---Store-Register--immediate--?lang=en)
    ///
    /// Store Register (immediate) stores a word or a doubleword from a register to memory. The address that is used for the store is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// STR <Xt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn str_64_imm_post_index(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b11, 0, 0b00, simm, 0b01, xt, xn_sp)
    }

    /// [STR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STR--immediate---Store-Register--immediate--?lang=en)
    ///
    /// Store Register (immediate) stores a word or a doubleword from a register to memory. The address that is used for the store is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// STR <Xt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn str_64_imm_pre_index(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b11, 0, 0b00, simm, 0b11, xt, xn_sp)
    }

    // LDR Imm instructions

    /// [LDR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDR--immediate---Load-Register--immediate--?lang=en)
    ///
    /// Load Register (immediate) loads a word or doubleword from memory and writes it to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes. The Unsigned offset variant scales the immediate offset value by the size of the value accessed before adding it to the base register value.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDR <Wt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldr_32_imm_post_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b10, 0, 0b01, simm, 0b01, wt, xn_sp)
    }

    /// [LDR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDR--immediate---Load-Register--immediate--?lang=en)
    ///
    /// Load Register (immediate) loads a word or doubleword from memory and writes it to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes. The Unsigned offset variant scales the immediate offset value by the size of the value accessed before adding it to the base register value.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDR <Wt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldr_32_imm_pre_index(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b10, 0, 0b01, simm, 0b11, wt, xn_sp)
    }

    /// [LDR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDR--immediate---Load-Register--immediate--?lang=en)
    ///
    /// Load Register (immediate) loads a word or doubleword from memory and writes it to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes. The Unsigned offset variant scales the immediate offset value by the size of the value accessed before adding it to the base register value.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDR <Xt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldr_64_imm_post_index(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b11, 0, 0b01, simm, 0b01, xt, xn_sp)
    }

    /// [LDR - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDR--immediate---Load-Register--immediate--?lang=en)
    ///
    /// Load Register (immediate) loads a word or doubleword from memory and writes it to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes. The Unsigned offset variant scales the immediate offset value by the size of the value accessed before adding it to the base register value.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDR <Xt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldr_64_imm_pre_index(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b11, 0, 0b01, simm, 0b11, xt, xn_sp)
    }

    // LDRSW Imm instructions

    /// [LDRSW - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSW--immediate---Load-Register-Signed-Word--immediate--?lang=en)
    ///
    /// Load Register Signed Word (immediate) loads a word from memory, sign-extends it to 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSW <Xt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldrsw_imm_post_index(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b10, 0, 0b10, simm, 0b01, xt, xn_sp)
    }

    /// [LDRSW - immediate](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRSW--immediate---Load-Register-Signed-Word--immediate--?lang=en)
    ///
    /// Load Register Signed Word (immediate) loads a word from memory, sign-extends it to 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Unsigned offset
    ///
    /// ```asm
    /// LDRSW <Xt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldrsw_imm_pre_index(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b10, 0, 0b10, simm, 0b11, xt, xn_sp)
    }

    // STR Imm SIMD&FP instructions

    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Bt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn str_8_imm_simd_post_index(&mut self, bt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b00, 1, 0b00, simm, 0b01, bt, xn_sp)
    }

    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Bt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn str_8_imm_simd_pre_index(&mut self, bt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b00, 1, 0b00, simm, 0b11, bt, xn_sp)
    }

    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Ht>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn str_16_imm_simd_post_index(&mut self, ht: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b01, 1, 0b00, simm, 0b01, ht, xn_sp)
    }

    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Ht>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn str_16_imm_simd_pre_index(&mut self, ht: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b01, 1, 0b00, simm, 0b11, ht, xn_sp)
    }

    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <St>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn str_32_imm_simd_post_index(&mut self, st: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b10, 1, 0b00, simm, 0b01, st, xn_sp)
    }

    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <St>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn str_32_imm_simd_pre_index(&mut self, st: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b10, 1, 0b00, simm, 0b11, st, xn_sp)
    }

    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Dt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn str_64_imm_simd_post_index(&mut self, dt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b11, 1, 0b00, simm, 0b01, dt, xn_sp)
    }

    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Dt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn str_64_imm_simd_pre_index(&mut self, dt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b11, 1, 0b00, simm, 0b11, dt, xn_sp)
    }

    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Qt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn str_128_imm_simd_post_index(&mut self, qt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b00, 1, 0b10, simm, 0b01, qt, xn_sp)
    }

    /// [STR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STR--immediate--SIMD-FP---Store-SIMD-FP-register--immediate-offset--?lang=en)
    ///
    /// Store SIMD&FP register (immediate offset). This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STR <Qt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn str_128_imm_simd_pre_index(&mut self, qt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b00, 1, 0b10, simm, 0b11, qt, xn_sp)
    }

    // LDR Imm SIMD&FP instructions

    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Bt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldr_8_imm_simd_post_index(&mut self, bt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b00, 1, 0b01, simm, 0b01, bt, xn_sp)
    }

    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Bt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldr_8_imm_simd_pre_index(&mut self, bt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b00, 1, 0b01, simm, 0b11, bt, xn_sp)
    }

    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Ht>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldr_16_imm_simd_post_index(&mut self, ht: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b01, 1, 0b01, simm, 0b01, ht, xn_sp)
    }

    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Ht>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldr_16_imm_simd_pre_index(&mut self, ht: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b01, 1, 0b01, simm, 0b11, ht, xn_sp)
    }

    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <St>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldr_32_imm_simd_post_index(&mut self, st: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b10, 1, 0b01, simm, 0b01, st, xn_sp)
    }

    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <St>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldr_32_imm_simd_pre_index(&mut self, st: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b10, 1, 0b01, simm, 0b11, st, xn_sp)
    }

    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Dt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldr_64_imm_simd_post_index(&mut self, dt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b11, 1, 0b01, simm, 0b01, dt, xn_sp)
    }

    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Dt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldr_64_imm_simd_pre_index(&mut self, dt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b11, 1, 0b01, simm, 0b11, dt, xn_sp)
    }

    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Qt>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn ldr_128_imm_simd_post_index(&mut self, qt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b00, 1, 0b11, simm, 0b01, qt, xn_sp)
    }

    /// [LDR - immediate - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDR--immediate--SIMD-FP---Load-SIMD-FP-Register--immediate-offset--?lang=en)
    ///
    /// Load SIMD&FP Register (immediate offset). This instruction loads an element from memory, and writes the result as a scalar to the SIMD&FP register. The address that is used for the load is calculated from a base register value, a signed immediate offset, and an optional offset that is a multiple of the element size.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDR <Qt>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn ldr_128_imm_simd_pre_index(&mut self, qt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_load_store_pre_post(self, 0b00, 1, 0b11, simm, 0b11, qt, xn_sp)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_panic, stream_mock};
    use crate::instruction_emitter::MockEmitter;
    use crate::instruction_stream::InstrStream;
    use crate::mc_memory::MockMemory;
    use crate::types::InstructionPointer;

    use super::*;

// STRB Test Cases

    #[test]
    fn test_strb_pre_index() {
        stream_mock!(stream, {
            let instr = stream.strb_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "strb w0, [x3, #0xffffffffffffff00]!");

            let instr = stream.strb_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "strb w0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.strb_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.strb_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_strb_post_index() {
        stream_mock!(stream, {
            let instr = stream.strb_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "strb w0, [x3], #0xffffffffffffff00");

            let instr = stream.strb_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "strb w0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.strb_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.strb_imm_post_index(0, 3, 256));
        })
    }

    // LDRB Test Cases

    #[test]
    fn test_ldrb_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldrb_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrb w0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldrb_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrb w0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldrb_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrb_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrb_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldrb_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrb w0, [x3], #0xffffffffffffff00");

            let instr = stream.ldrb_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrb w0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldrb_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrb_imm_post_index(0, 3, 256));
        })
    }

    // LDRSB Test Cases

    #[test]
    fn test_ldrsb_32_imm_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsb_32_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsb w0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldrsb_32_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsb w0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldrsb_32_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsb_32_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrsb_32_imm_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsb_32_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsb w0, [x3], #0xffffffffffffff00");

            let instr = stream.ldrsb_32_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsb w0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldrsb_32_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsb_32_imm_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrsb_64_imm_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsb_64_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsb x0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldrsb_64_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsb x0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldrsb_64_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsb_64_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrsb_64_imm_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsb_64_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsb x0, [x3], #0xffffffffffffff00");

            let instr = stream.ldrsb_64_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsb x0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldrsb_64_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsb_64_imm_post_index(0, 3, 256));
        })
    }

    // STRH Imm Test Cases

    #[test]
    fn test_strh_imm_pre_index() {
        stream_mock!(stream, {
            let instr = stream.strh_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "strh w0, [x3, #0xffffffffffffff00]!");

            let instr = stream.strh_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "strh w0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.strh_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.strh_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_strh_imm_post_index() {
        stream_mock!(stream, {
            let instr = stream.strh_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "strh w0, [x3], #0xffffffffffffff00");

            let instr = stream.strh_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "strh w0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.strh_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.strh_imm_post_index(0, 3, 256));
        })
    }

    // LDRH Imm Test Cases

    #[test]
    fn test_ldrh_imm_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldrh_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrh w0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldrh_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrh w0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldrh_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrh_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrh_imm_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldrh_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrh w0, [x3], #0xffffffffffffff00");

            let instr = stream.ldrh_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrh w0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldrh_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrh_imm_post_index(0, 3, 256));
        })
    }

    // LDRSH Test Cases

    #[test]
    fn test_ldrsh_32_imm_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsh_32_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsh w0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldrsh_32_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsh w0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldrsh_32_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsh_32_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrsh_32_imm_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsh_32_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsh w0, [x3], #0xffffffffffffff00");

            let instr = stream.ldrsh_32_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsh w0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldrsh_32_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsh_32_imm_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrsh_64_imm_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsh_64_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsh x0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldrsh_64_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsh x0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldrsh_64_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsh_64_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrsh_64_imm_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsh_64_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsh x0, [x3], #0xffffffffffffff00");

            let instr = stream.ldrsh_64_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsh x0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldrsh_64_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsh_64_imm_post_index(0, 3, 256));
        })
    }

    // STR Imm Test Cases

    #[test]
    fn test_str_32_imm_pre_index() {
        stream_mock!(stream, {
            let instr = stream.str_32_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str w0, [x3, #0xffffffffffffff00]!");

            let instr = stream.str_32_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str w0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_32_imm_post_index() {
        stream_mock!(stream, {
            let instr = stream.str_32_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str w0, [x3], #0xffffffffffffff00");

            let instr = stream.str_32_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str w0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_64_imm_pre_index() {
        stream_mock!(stream, {
            let instr = stream.str_64_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str x0, [x3, #0xffffffffffffff00]!");

            let instr = stream.str_64_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str x0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_64_imm_post_index() {
        stream_mock!(stream, {
            let instr = stream.str_64_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str x0, [x3], #0xffffffffffffff00");

            let instr = stream.str_64_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str x0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_post_index(0, 3, 256));
        })
    }

    // LDR Imm Test Cases

    #[test]
    fn test_ldr_32_imm_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_32_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr w0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldr_32_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr w0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_32_imm_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_32_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr w0, [x3], #0xffffffffffffff00");

            let instr = stream.ldr_32_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr w0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_post_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_64_imm_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_64_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr x0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldr_64_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr x0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_64_imm_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_64_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr x0, [x3], #0xffffffffffffff00");

            let instr = stream.ldr_64_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr x0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_post_index(0, 3, 256));
        })
    }

    // STR Imm SIMD&FP Test Cases

    #[test]
    fn test_str_8_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.str_8_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str b0, [x3, #0xffffffffffffff00]!");

            let instr = stream.str_8_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str b0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.str_8_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_8_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_8_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.str_8_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str b0, [x3], #0xffffffffffffff00");

            let instr = stream.str_8_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str b0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.str_8_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_8_imm_simd_post_index(0, 3, 256));
        })
    }

    // LDRSW Imm Test Cases

    #[test]
    fn test_ldrsw_imm_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsw_imm_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsw x0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldrsw_imm_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsw x0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldrsw_imm_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsw_imm_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldrsw_imm_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldrsw_imm_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldrsw x0, [x3], #0xffffffffffffff00");

            let instr = stream.ldrsw_imm_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldrsw x0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldrsw_imm_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldrsw_imm_post_index(0, 3, 256));
        })
    }

    // PRFM Imm Test Cases

    // STR 16 bit imm simd&fp

    #[test]
    fn test_str_16_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.str_16_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str h0, [x3, #0xffffffffffffff00]!");

            let instr = stream.str_16_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str h0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.str_16_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_16_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_16_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.str_16_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str h0, [x3], #0xffffffffffffff00");

            let instr = stream.str_16_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str h0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.str_16_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_16_imm_simd_post_index(0, 3, 256));
        })
    }

    // STR 32 bit imm simd&fp

    #[test]
    fn test_str_32_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.str_32_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str s0, [x3, #0xffffffffffffff00]!");

            let instr = stream.str_32_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str s0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_32_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.str_32_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str s0, [x3], #0xffffffffffffff00");

            let instr = stream.str_32_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str s0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_32_imm_simd_post_index(0, 3, 256));
        })
    }

    // STR 64 bit imm simd&fp

    #[test]
    fn test_str_64_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.str_64_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str d0, [x3, #0xffffffffffffff00]!");

            let instr = stream.str_64_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str d0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_64_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.str_64_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str d0, [x3], #0xffffffffffffff00");

            let instr = stream.str_64_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str d0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_64_imm_simd_post_index(0, 3, 256));
        })
    }

    // STR 128 bit imm simd&fp

    #[test]
    fn test_str_128_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.str_128_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str q0, [x3, #0xffffffffffffff00]!");

            let instr = stream.str_128_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str q0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.str_128_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_128_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_str_128_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.str_128_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "str q0, [x3], #0xffffffffffffff00");

            let instr = stream.str_128_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "str q0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.str_128_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.str_128_imm_simd_post_index(0, 3, 256));
        })
    }

    // LDR Test Cases

    #[test]
    fn test_ldr_8_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_8_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr b0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldr_8_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr b0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldr_8_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_8_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_8_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_8_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr b0, [x3], #0xffffffffffffff00");

            let instr = stream.ldr_8_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr b0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldr_8_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_8_imm_simd_post_index(0, 3, 256));
        })
    }

    // LDR 16 bit imm simd&fp

    #[test]
    fn test_ldr_16_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_16_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr h0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldr_16_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr h0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldr_16_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_16_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_16_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_16_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr h0, [x3], #0xffffffffffffff00");

            let instr = stream.ldr_16_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr h0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldr_16_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_16_imm_simd_post_index(0, 3, 256));
        })
    }

    // LDR 32 bit imm simd&fp

    #[test]
    fn test_ldr_32_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_32_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr s0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldr_32_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr s0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_32_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_32_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr s0, [x3], #0xffffffffffffff00");

            let instr = stream.ldr_32_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr s0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_32_imm_simd_post_index(0, 3, 256));
        })
    }

    // LDR 64 bit imm simd&fp

    #[test]
    fn test_ldr_64_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_64_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr d0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldr_64_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr d0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_64_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_64_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr d0, [x3], #0xffffffffffffff00");

            let instr = stream.ldr_64_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr d0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_64_imm_simd_post_index(0, 3, 256));
        })
    }

    // LDR 128 bit imm simd&fp

    #[test]
    fn test_ldr_128_imm_simd_pre_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_128_imm_simd_pre_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr q0, [x3, #0xffffffffffffff00]!");

            let instr = stream.ldr_128_imm_simd_pre_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr q0, [sp, #0xff]!");

            assert_panic!("Should panic: imm out of range"; stream.ldr_128_imm_simd_pre_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_128_imm_simd_pre_index(0, 3, 256));
        })
    }

    #[test]
    fn test_ldr_128_imm_simd_post_index() {
        stream_mock!(stream, {
            let instr = stream.ldr_128_imm_simd_post_index(0, 3, -256);
            assert_eq!(instr.to_string(), "ldr q0, [x3], #0xffffffffffffff00");

            let instr = stream.ldr_128_imm_simd_post_index(0, 0b11111, 255);
            assert_eq!(instr.to_string(), "ldr q0, [sp], #0xff");

            assert_panic!("Should panic: imm out of range"; stream.ldr_128_imm_simd_post_index(0, 3, -257));
            assert_panic!("Should panic: imm out of range"; stream.ldr_128_imm_simd_post_index(0, 3, 256));
        })
    }
}
