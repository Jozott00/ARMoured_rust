//! # [Advanced SIMD load/store single structure](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#asisdlso)
//!
//! Implements the following instructions:
//!  - [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
//!  - [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
//!  - [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
//!  - [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
//!  - [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
//!  - [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
//!  - [LD1R - Load one single element structure and Replicate to all lanes - of one register - ](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1R--Load-one-single-element-structure-and-Replicate-to-all-lanes--of-one-register--?lang=en)
//!  - [LD3R - Load single 3 element structure and Replicate to all lanes of three registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3R--Load-single-3-element-structure-and-Replicate-to-all-lanes-of-three-registers-?lang=en)
//!  - [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
//!  - [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
//!  - [LD2R - Load single 2 element structure and Replicate to all lanes of two registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2R--Load-single-2-element-structure-and-Replicate-to-all-lanes-of-two-registers-?lang=en)
//!  - [LD4R - Load single 4 element structure and Replicate to all lanes of four registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4R--Load-single-4-element-structure-and-Replicate-to-all-lanes-of-four-registers-?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Register, UImm1, UImm2, UImm3, UImm4};
use crate::types::ArrSpecifier::{ArrSpec, ArrSpec1};

#[inline(always)]
fn emit_adv_ldr_str_all<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    q: u8,
    with_offset: bool,
    l: u8,
    r: u8,
    rm: Register,
    opcode: u8,
    s: u8,
    size: u8,
    rn: Register,
    rt: Register,
) -> T {
    let op = with_offset as u8;
    let r = bseq_32!(0 q:1 001101 op:1 l:1 r:1 rm:5 opcode:3 s:1 size:2 rn:5 rt:5);
    proc.process(r)
}

#[inline(always)]
fn emit_adv_ldr_str_plain<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    l: u8,
    r: u8,
    opcode: u8,
    op_size: u8,
    index: u8,
    rn: Register,
    vts: &[Register],
) -> T {
    debug_assert!(
        are_indices_sequential(vts),
        "vector registers must be sequential, were {:?}",
        vts
    );
    let (q, s, size) = q_s_size(index, op_size);
    emit_adv_ldr_str_all(proc, q, false, l, r, 0, opcode, s, size, rn, vts[0])
}

#[inline(always)]
fn emit_adv_ldr_str_off_reg<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    l: u8,
    r: u8,
    rm: Register,
    opcode: u8,
    op_size: u8,
    index: u8,
    rn: Register,
    vts: &[Register],
) -> T {
    debug_assert!(
        are_indices_sequential(vts),
        "vector registers must be sequential, were {:?}",
        vts
    );
    let (q, s, size) = q_s_size(index, op_size);
    emit_adv_ldr_str_all(proc, q, true, l, r, rm, opcode, s, size, rn, vts[0])
}

#[inline(always)]
fn emit_adv_ldr_str_off_imm<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    l: u8,
    r: u8,
    opcode: u8,
    op_size: u8,
    index: u8,
    rn: Register,
    vts: &[Register],
) -> T {
    debug_assert!(
        are_indices_sequential(vts),
        "vector registers must be sequential, were {:?}",
        vts
    );
    let (q, s, size) = q_s_size(index, op_size);
    emit_adv_ldr_str_all(proc, q, true, l, r, 0b11111, opcode, s, size, rn, vts[0])
}

#[inline(always)]
fn emit_adv_ldxr_plain<P: InstructionProcessor<T>, T, A: ArrSpec>(
    proc: &mut P,
    l: u8,
    r: u8,
    opcode: u8,
    t: A,
    rn: Register,
    vts: &[Register],
) -> T {
    debug_assert!(
        are_indices_sequential(vts),
        "vector registers must be sequential, were {:?}",
        vts
    );
    emit_adv_ldr_str_all(proc, t.q(), false, l, r, 0, opcode, 0, t.size(), rn, vts[0])
}

#[inline(always)]
fn emit_adv_ldxr_off_reg<P: InstructionProcessor<T>, T, A: ArrSpec>(
    proc: &mut P,
    l: u8,
    r: u8,
    rm: Register,
    opcode: u8,
    t: A,
    rn: Register,
    vts: &[Register],
) -> T {
    debug_assert!(
        are_indices_sequential(vts),
        "vector registers must be sequential, were {:?}",
        vts
    );
    emit_adv_ldr_str_all(proc, t.q(), true, l, r, rm, opcode, 0, t.size(), rn, vts[0])
}

#[inline(always)]
fn emit_adv_ldxr_off_imm<P: InstructionProcessor<T>, T, A: ArrSpec>(
    proc: &mut P,
    l: u8,
    r: u8,
    opcode: u8,
    t: A,
    rn: Register,
    vts: &[Register],
) -> T {
    debug_assert!(
        are_indices_sequential(vts),
        "vector registers must be sequential, were {:?}",
        vts
    );
    emit_adv_ldr_str_all(
        proc,
        t.q(),
        true,
        l,
        r,
        0b11111,
        opcode,
        0,
        t.size(),
        rn,
        vts[0],
    )
}

#[inline(always)]
fn get_imm_size(immediate: u8, smallest_imm: u8) -> u8 {
    match immediate / smallest_imm {
        1 => 0b00,
        2 => 0b01,
        4 => 0b10,
        _ => 0b11,
    }
}

fn are_indices_sequential(indices: &[Register]) -> bool {
    for i in 0..(indices.len() - 1) {
        if (indices[i] + 1) % 32 != indices[(i + 1) % indices.len()] {
            return false;
        }
    }
    true
}

#[inline(always)]
fn q_s_size(index: u8, op_size: u8) -> (u8, u8, u8) {
    match op_size {
        // 1 1 11
        8 => (index >> 3, index >> 2, index & 0b11),
        // 1 1 1
        16 => (index >> 2, index >> 1, index << 1),
        // 1 1 -
        32 => (index >> 1, index, 0b00),
        // 1 - -
        64 => (index, 0, 0b01),
        _ => {
            debug_assert!(true, "Major implementation error. Shouldnt be possible!");
            (index, 0, 0b01)
        }
    }
}

/// # [Advanced SIMD load/store single structure](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#asisdlso)
///
/// Implements the following instructions:
///  - [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
///  - [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
///  - [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
///  - [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
///  - [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
///  - [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
///  - [LD1R - Load one single element structure and Replicate to all lanes - of one register - ](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1R--Load-one-single-element-structure-and-Replicate-to-all-lanes--of-one-register--?lang=en)
///  - [LD3R - Load single 3 element structure and Replicate to all lanes of three registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3R--Load-single-3-element-structure-and-Replicate-to-all-lanes-of-three-registers-?lang=en)
///  - [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
///  - [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
///  - [LD2R - Load single 2 element structure and Replicate to all lanes of two registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2R--Load-single-2-element-structure-and-Replicate-to-all-lanes-of-two-registers-?lang=en)
///  - [LD4R - Load single 4 element structure and Replicate to all lanes of four registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4R--Load-single-4-element-structure-and-Replicate-to-all-lanes-of-four-registers-?lang=en)
pub trait AdvancedSIMDLoadStoreSingleStructures<T>: InstructionProcessor<T> {
    /// [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
    ///
    /// Store a single-element structure from one lane of one register. This instruction stores the specified element of a SIMD&FP register to memory.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.B }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st1_single_struct_bytes(&mut self, vt: Register, index: UImm4, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 0, 0, 0b00, 8, index, xn_sp, &[vt])
    }

    /// [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
    ///
    /// Store a single-element structure from one lane of one register. This instruction stores the specified element of a SIMD&FP register to memory.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.H }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st1_single_struct_half_words(&mut self, vt: Register, index: UImm3, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 0, 0, 0b010, 16, index, xn_sp, &[vt])
    }

    /// [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
    ///
    /// Store a single-element structure from one lane of one register. This instruction stores the specified element of a SIMD&FP register to memory.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.S }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st1_single_struct_single_words(&mut self, vt: Register, index: UImm2, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 0, 0, 0b100, 32, index, xn_sp, &[vt])
    }

    /// [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
    ///
    /// Store a single-element structure from one lane of one register. This instruction stores the specified element of a SIMD&FP register to memory.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.D }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st1_single_struct_double_words(&mut self, vt: Register, index: UImm1, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 0, 0, 0b100, 64, index, xn_sp, &[vt])
    }

    /// [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
    ///
    /// Store a single-element structure from one lane of one register. This instruction stores the specified element of a SIMD&FP register to memory.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.B }[<index>], [<Xn|SP>], #1
    /// ```
    #[inline(always)]
    fn st1_single_struct_bytes_offset_imm(
        &mut self,
        vt: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0, 0b000, 8, index, xn_sp, &[vt])
    }

    /// [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
    ///
    /// Store a single-element structure from one lane of one register. This instruction stores the specified element of a SIMD&FP register to memory.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.B }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st1_single_struct_bytes_offset_reg(
        &mut self,
        vt: Register,
        index: UImm4,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0, xm, 0b000, 8, index, xn_sp, &[vt])
    }

    /// [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
    ///
    /// Store a single-element structure from one lane of one register. This instruction stores the specified element of a SIMD&FP register to memory.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.H }[<index>], [<Xn|SP>], #2
    /// ```
    #[inline(always)]
    fn st1_single_struct_half_words_offset_imm(
        &mut self,
        vt: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0, 0b010, 16, index, xn_sp, &[vt])
    }

    /// [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
    ///
    /// Store a single-element structure from one lane of one register. This instruction stores the specified element of a SIMD&FP register to memory.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.H }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st1_single_struct_half_words_offset_reg(
        &mut self,
        vt: Register,
        index: UImm3,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0, xm, 0b010, 16, index, xn_sp, &[vt])
    }

    /// [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
    ///
    /// Store a single-element structure from one lane of one register. This instruction stores the specified element of a SIMD&FP register to memory.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.S }[<index>], [<Xn|SP>], #4
    /// ```
    #[inline(always)]
    fn st1_single_struct_single_words_offset_imm(
        &mut self,
        vt: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0, 0b100, 32, index, xn_sp, &[vt])
    }

    /// [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
    ///
    /// Store a single-element structure from one lane of one register. This instruction stores the specified element of a SIMD&FP register to memory.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.S }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st1_single_struct_single_words_offset_reg(
        &mut self,
        vt: Register,
        index: UImm2,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0, xm, 0b100, 32, index, xn_sp, &[vt])
    }

    /// [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
    ///
    /// Store a single-element structure from one lane of one register. This instruction stores the specified element of a SIMD&FP register to memory.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.D }[<index>], [<Xn|SP>], #8
    /// ```
    #[inline(always)]
    fn st1_single_struct_double_words_offset_imm(
        &mut self,
        vt: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0, 0b100, 64, index, xn_sp, &[vt])
    }

    /// [ST1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--single-structure---Store-a-single-element-structure-from-one-lane-of-one-register-?lang=en)
    ///
    /// Store a single-element structure from one lane of one register. This instruction stores the specified element of a SIMD&FP register to memory.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.D }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st1_single_struct_double_words_offset_reg(
        &mut self,
        vt: Register,
        index: UImm1,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0, xm, 0b100, 64, index, xn_sp, &[vt])
    }

    /// [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
    ///
    /// Store single 3-element structure from one lane of three registers. This instruction stores a 3-element structure to memory from corresponding elements of three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.B, <Vt2>.B, <Vt3>.B }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st3_single_struct_bytes(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 0, 0, 0b001, 8, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
    ///
    /// Store single 3-element structure from one lane of three registers. This instruction stores a 3-element structure to memory from corresponding elements of three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.H, <Vt2>.H, <Vt3>.H }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st3_single_struct_half_words(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 0, 0, 0b011, 16, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
    ///
    /// Store single 3-element structure from one lane of three registers. This instruction stores a 3-element structure to memory from corresponding elements of three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.S, <Vt2>.S, <Vt3>.S }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st3_single_struct_single_words(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 0, 0, 0b101, 32, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
    ///
    /// Store single 3-element structure from one lane of three registers. This instruction stores a 3-element structure to memory from corresponding elements of three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.D, <Vt2>.D, <Vt3>.D }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st3_single_struct_double_words(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 0, 0, 0b101, 64, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
    ///
    /// Store single 3-element structure from one lane of three registers. This instruction stores a 3-element structure to memory from corresponding elements of three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.B, <Vt2>.B, <Vt3>.B }[<index>], [<Xn|SP>], #3
    /// ```
    #[inline(always)]
    fn st3_single_struct_bytes_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0, 0b001, 8, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
    ///
    /// Store single 3-element structure from one lane of three registers. This instruction stores a 3-element structure to memory from corresponding elements of three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.B, <Vt2>.B, <Vt3>.B }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st3_single_struct_bytes_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm4,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0, xm, 0b001, 8, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
    ///
    /// Store single 3-element structure from one lane of three registers. This instruction stores a 3-element structure to memory from corresponding elements of three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.H, <Vt2>.H, <Vt3>.H }[<index>], [<Xn|SP>], #6
    /// ```
    #[inline(always)]
    fn st3_single_struct_half_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0, 0b011, 16, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
    ///
    /// Store single 3-element structure from one lane of three registers. This instruction stores a 3-element structure to memory from corresponding elements of three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.H, <Vt2>.H, <Vt3>.H }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st3_single_struct_half_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm3,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0, xm, 0b011, 16, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
    ///
    /// Store single 3-element structure from one lane of three registers. This instruction stores a 3-element structure to memory from corresponding elements of three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.S, <Vt2>.S, <Vt3>.S }[<index>], [<Xn|SP>], #12
    /// ```
    #[inline(always)]
    fn st3_single_struct_single_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0, 0b101, 32, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
    ///
    /// Store single 3-element structure from one lane of three registers. This instruction stores a 3-element structure to memory from corresponding elements of three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.S, <Vt2>.S, <Vt3>.S }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st3_single_struct_single_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm2,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0, xm, 0b101, 32, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
    ///
    /// Store single 3-element structure from one lane of three registers. This instruction stores a 3-element structure to memory from corresponding elements of three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.D, <Vt2>.D, <Vt3>.D }[<index>], [<Xn|SP>], #24
    /// ```
    #[inline(always)]
    fn st3_single_struct_double_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0, 0b101, 64, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [ST3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--single-structure---Store-single-3-element-structure-from-one-lane-of-three-registers-?lang=en)
    ///
    /// Store single 3-element structure from one lane of three registers. This instruction stores a 3-element structure to memory from corresponding elements of three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.D, <Vt2>.D, <Vt3>.D }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st3_single_struct_double_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm1,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0, xm, 0b101, 64, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
    ///
    /// Store single 2-element structure from one lane of two registers. This instruction stores a 2-element structure to memory from corresponding elements of two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.B, <Vt2>.B }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st2_single_struct_bytes(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 0, 1, 0b00, 8, index, xn_sp, &[vt, vt2])
    }

    /// [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
    ///
    /// Store single 2-element structure from one lane of two registers. This instruction stores a 2-element structure to memory from corresponding elements of two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.H, <Vt2>.H }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st2_single_struct_half_words(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 0, 1, 0b010, 16, index, xn_sp, &[vt, vt2])
    }

    /// [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
    ///
    /// Store single 2-element structure from one lane of two registers. This instruction stores a 2-element structure to memory from corresponding elements of two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.S, <Vt2>.S }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st2_single_struct_single_words(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 0, 1, 0b100, 32, index, xn_sp, &[vt, vt2])
    }

    /// [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
    ///
    /// Store single 2-element structure from one lane of two registers. This instruction stores a 2-element structure to memory from corresponding elements of two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.D, <Vt2>.D }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st2_single_struct_double_words(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 0, 1, 0b100, 64, index, xn_sp, &[vt, vt2])
    }

    /// [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
    ///
    /// Store single 2-element structure from one lane of two registers. This instruction stores a 2-element structure to memory from corresponding elements of two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.B, <Vt2>.B }[<index>], [<Xn|SP>], #2
    /// ```
    #[inline(always)]
    fn st2_single_struct_bytes_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 1, 0b000, 8, index, xn_sp, &[vt, vt2])
    }

    /// [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
    ///
    /// Store single 2-element structure from one lane of two registers. This instruction stores a 2-element structure to memory from corresponding elements of two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.B, <Vt2>.B }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st2_single_struct_bytes_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm4,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 1, xm, 0b000, 8, index, xn_sp, &[vt, vt2])
    }

    /// [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
    ///
    /// Store single 2-element structure from one lane of two registers. This instruction stores a 2-element structure to memory from corresponding elements of two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.H, <Vt2>.H }[<index>], [<Xn|SP>], #4
    /// ```
    #[inline(always)]
    fn st2_single_struct_half_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 1, 0b010, 16, index, xn_sp, &[vt, vt2])
    }

    /// [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
    ///
    /// Store single 2-element structure from one lane of two registers. This instruction stores a 2-element structure to memory from corresponding elements of two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.H, <Vt2>.H }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st2_single_struct_half_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm3,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 1, xm, 0b010, 16, index, xn_sp, &[vt, vt2])
    }

    /// [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
    ///
    /// Store single 2-element structure from one lane of two registers. This instruction stores a 2-element structure to memory from corresponding elements of two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.S, <Vt2>.S }[<index>], [<Xn|SP>], #8
    /// ```
    #[inline(always)]
    fn st2_single_struct_single_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 1, 0b100, 32, index, xn_sp, &[vt, vt2])
    }

    /// [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
    ///
    /// Store single 2-element structure from one lane of two registers. This instruction stores a 2-element structure to memory from corresponding elements of two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.S, <Vt2>.S }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st2_single_struct_single_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm2,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 1, xm, 0b100, 32, index, xn_sp, &[vt, vt2])
    }

    /// [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
    ///
    /// Store single 2-element structure from one lane of two registers. This instruction stores a 2-element structure to memory from corresponding elements of two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.D, <Vt2>.D }[<index>], [<Xn|SP>], #16
    /// ```
    #[inline(always)]
    fn st2_single_struct_double_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 1, 0b100, 64, index, xn_sp, &[vt, vt2])
    }

    /// [ST2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--single-structure---Store-single-2-element-structure-from-one-lane-of-two-registers-?lang=en)
    ///
    /// Store single 2-element structure from one lane of two registers. This instruction stores a 2-element structure to memory from corresponding elements of two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.D, <Vt2>.D }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st2_single_struct_double_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm1,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 1, xm, 0b100, 64, index, xn_sp, &[vt, vt2])
    }

    /// [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
    ///
    /// Store single 4-element structure from one lane of four registers. This instruction stores a 4-element structure to memory from corresponding elements of four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.B, <Vt2>.B, <Vt3>.B, <Vt4>.B }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st4_single_struct_bytes(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 0, 1, 0b001, 8, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
    ///
    /// Store single 4-element structure from one lane of four registers. This instruction stores a 4-element structure to memory from corresponding elements of four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.H, <Vt2>.H, <Vt3>.H, <Vt4>.H }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st4_single_struct_half_words(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 0, 1, 0b011, 16, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
    ///
    /// Store single 4-element structure from one lane of four registers. This instruction stores a 4-element structure to memory from corresponding elements of four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.S, <Vt2>.S, <Vt3>.S, <Vt4>.S }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st4_single_struct_single_words(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 0, 1, 0b101, 32, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
    ///
    /// Store single 4-element structure from one lane of four registers. This instruction stores a 4-element structure to memory from corresponding elements of four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.B, <Vt2>.B, <Vt3>.B, <Vt4>.B }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st4_single_struct_bytes_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm4,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 1, xm, 0b001, 8, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
    ///
    /// Store single 4-element structure from one lane of four registers. This instruction stores a 4-element structure to memory from corresponding elements of four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.D, <Vt2>.D, <Vt3>.D, <Vt4>.D }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st4_single_struct_double_words(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 0, 1, 0b101, 64, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
    ///
    /// Store single 4-element structure from one lane of four registers. This instruction stores a 4-element structure to memory from corresponding elements of four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.B, <Vt2>.B, <Vt3>.B, <Vt4>.B }[<index>], [<Xn|SP>], #4
    /// ```
    #[inline(always)]
    fn st4_single_struct_bytes_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 1, 0b001, 8, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
    ///
    /// Store single 4-element structure from one lane of four registers. This instruction stores a 4-element structure to memory from corresponding elements of four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.H, <Vt2>.H, <Vt3>.H, <Vt4>.H }[<index>], [<Xn|SP>], #8
    /// ```
    #[inline(always)]
    fn st4_single_struct_half_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 1, 0b011, 16, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
    ///
    /// Store single 4-element structure from one lane of four registers. This instruction stores a 4-element structure to memory from corresponding elements of four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.H, <Vt2>.H, <Vt3>.H, <Vt4>.H }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st4_single_struct_half_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm3,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(
            self,
            0,
            1,
            xm,
            0b011,
            16,
            index,
            xn_sp,
            &[vt, vt2, vt3, vt4],
        )
    }

    /// [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
    ///
    /// Store single 4-element structure from one lane of four registers. This instruction stores a 4-element structure to memory from corresponding elements of four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.S, <Vt2>.S, <Vt3>.S, <Vt4>.S }[<index>], [<Xn|SP>], #16
    /// ```
    #[inline(always)]
    fn st4_single_struct_single_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 1, 0b101, 32, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
    ///
    /// Store single 4-element structure from one lane of four registers. This instruction stores a 4-element structure to memory from corresponding elements of four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.S, <Vt2>.S, <Vt3>.S, <Vt4>.S }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st4_single_struct_single_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm2,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(
            self,
            0,
            1,
            xm,
            0b101,
            32,
            index,
            xn_sp,
            &[vt, vt2, vt3, vt4],
        )
    }

    /// [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
    ///
    /// Store single 4-element structure from one lane of four registers. This instruction stores a 4-element structure to memory from corresponding elements of four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.D, <Vt2>.D, <Vt3>.D, <Vt4>.D }[<index>], [<Xn|SP>], #32
    /// ```
    #[inline(always)]
    fn st4_single_struct_double_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 1, 0b101, 64, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [ST4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--single-structure---Store-single-4-element-structure-from-one-lane-of-four-registers-?lang=en)
    ///
    /// Store single 4-element structure from one lane of four registers. This instruction stores a 4-element structure to memory from corresponding elements of four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.D, <Vt2>.D, <Vt3>.D, <Vt4>.D }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st4_single_struct_double_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm1,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(
            self,
            0,
            1,
            xm,
            0b101,
            64,
            index,
            xn_sp,
            &[vt, vt2, vt3, vt4],
        )
    }

    /// [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
    ///
    /// Load one single-element structure to one lane of one register. This instruction loads a single-element structure from memory and writes the result to the specified lane of the SIMD&FP register without affecting the other bits of the register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.B }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld1_single_struct_bytes(&mut self, vt: Register, index: UImm4, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 1, 0, 0b00, 8, index, xn_sp, &[vt])
    }

    /// [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
    ///
    /// Load one single-element structure to one lane of one register. This instruction loads a single-element structure from memory and writes the result to the specified lane of the SIMD&FP register without affecting the other bits of the register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.H }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld1_single_struct_half_words(&mut self, vt: Register, index: UImm3, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 1, 0, 0b010, 16, index, xn_sp, &[vt])
    }

    /// [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
    ///
    /// Load one single-element structure to one lane of one register. This instruction loads a single-element structure from memory and writes the result to the specified lane of the SIMD&FP register without affecting the other bits of the register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.S }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld1_single_struct_single_words(&mut self, vt: Register, index: UImm2, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 1, 0, 0b100, 32, index, xn_sp, &[vt])
    }

    /// [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
    ///
    /// Load one single-element structure to one lane of one register. This instruction loads a single-element structure from memory and writes the result to the specified lane of the SIMD&FP register without affecting the other bits of the register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.D }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld1_single_struct_double_words(&mut self, vt: Register, index: UImm1, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 1, 0, 0b100, 64, index, xn_sp, &[vt])
    }

    /// [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
    ///
    /// Load one single-element structure to one lane of one register. This instruction loads a single-element structure from memory and writes the result to the specified lane of the SIMD&FP register without affecting the other bits of the register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.B }[<index>], [<Xn|SP>], #1
    /// ```
    #[inline(always)]
    fn ld1_single_struct_bytes_offset_imm(
        &mut self,
        vt: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0, 0b000, 8, index, xn_sp, &[vt])
    }

    /// [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
    ///
    /// Load one single-element structure to one lane of one register. This instruction loads a single-element structure from memory and writes the result to the specified lane of the SIMD&FP register without affecting the other bits of the register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.B }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld1_single_struct_bytes_offset_reg(
        &mut self,
        vt: Register,
        index: UImm4,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0, xm, 0b000, 8, index, xn_sp, &[vt])
    }

    /// [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
    ///
    /// Load one single-element structure to one lane of one register. This instruction loads a single-element structure from memory and writes the result to the specified lane of the SIMD&FP register without affecting the other bits of the register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.H }[<index>], [<Xn|SP>], #2
    /// ```
    #[inline(always)]
    fn ld1_single_struct_half_words_offset_imm(
        &mut self,
        vt: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0, 0b010, 16, index, xn_sp, &[vt])
    }

    /// [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
    ///
    /// Load one single-element structure to one lane of one register. This instruction loads a single-element structure from memory and writes the result to the specified lane of the SIMD&FP register without affecting the other bits of the register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.H }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld1_single_struct_half_words_offset_reg(
        &mut self,
        vt: Register,
        index: UImm3,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0, xm, 0b010, 16, index, xn_sp, &[vt])
    }

    /// [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
    ///
    /// Load one single-element structure to one lane of one register. This instruction loads a single-element structure from memory and writes the result to the specified lane of the SIMD&FP register without affecting the other bits of the register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.S }[<index>], [<Xn|SP>], #4
    /// ```
    #[inline(always)]
    fn ld1_single_struct_single_words_offset_imm(
        &mut self,
        vt: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0, 0b100, 32, index, xn_sp, &[vt])
    }

    /// [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
    ///
    /// Load one single-element structure to one lane of one register. This instruction loads a single-element structure from memory and writes the result to the specified lane of the SIMD&FP register without affecting the other bits of the register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.S }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld1_single_struct_single_words_offset_reg(
        &mut self,
        vt: Register,
        index: UImm2,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0, xm, 0b100, 32, index, xn_sp, &[vt])
    }

    /// [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
    ///
    /// Load one single-element structure to one lane of one register. This instruction loads a single-element structure from memory and writes the result to the specified lane of the SIMD&FP register without affecting the other bits of the register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.D }[<index>], [<Xn|SP>], #8
    /// ```
    #[inline(always)]
    fn ld1_single_struct_double_words_offset_imm(
        &mut self,
        vt: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0, 0b100, 64, index, xn_sp, &[vt])
    }

    /// [LD1 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--single-structure---Load-one-single-element-structure-to-one-lane-of-one-register-?lang=en)
    ///
    /// Load one single-element structure to one lane of one register. This instruction loads a single-element structure from memory and writes the result to the specified lane of the SIMD&FP register without affecting the other bits of the register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.D }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld1_single_struct_double_words_offset_reg(
        &mut self,
        vt: Register,
        index: UImm1,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0, xm, 0b100, 64, index, xn_sp, &[vt])
    }

    /// [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure to one lane of three registers. This instruction loads a 3-element structure from memory and writes the result to the corresponding elements of the three SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3 { <Vt>.B, <Vt2>.B, <Vt3>.B }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld3_single_struct_bytes(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 1, 0, 0b001, 8, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure to one lane of three registers. This instruction loads a 3-element structure from memory and writes the result to the corresponding elements of the three SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3 { <Vt>.H, <Vt2>.H, <Vt3>.H }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld3_single_struct_half_words(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 1, 0, 0b011, 16, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure to one lane of three registers. This instruction loads a 3-element structure from memory and writes the result to the corresponding elements of the three SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3 { <Vt>.S, <Vt2>.S, <Vt3>.S }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld3_single_struct_single_words(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 1, 0, 0b101, 32, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure to one lane of three registers. This instruction loads a 3-element structure from memory and writes the result to the corresponding elements of the three SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3 { <Vt>.D, <Vt2>.D, <Vt3>.D }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld3_single_struct_double_words(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 1, 0, 0b101, 64, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure to one lane of three registers. This instruction loads a 3-element structure from memory and writes the result to the corresponding elements of the three SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3 { <Vt>.B, <Vt2>.B, <Vt3>.B }[<index>], [<Xn|SP>], #3
    /// ```
    #[inline(always)]
    fn ld3_single_struct_bytes_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0, 0b001, 8, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure to one lane of three registers. This instruction loads a 3-element structure from memory and writes the result to the corresponding elements of the three SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3 { <Vt>.B, <Vt2>.B, <Vt3>.B }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld3_single_struct_bytes_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm4,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0, xm, 0b001, 8, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure to one lane of three registers. This instruction loads a 3-element structure from memory and writes the result to the corresponding elements of the three SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3 { <Vt>.H, <Vt2>.H, <Vt3>.H }[<index>], [<Xn|SP>], #6
    /// ```
    #[inline(always)]
    fn ld3_single_struct_half_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0, 0b011, 16, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure to one lane of three registers. This instruction loads a 3-element structure from memory and writes the result to the corresponding elements of the three SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3 { <Vt>.H, <Vt2>.H, <Vt3>.H }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld3_single_struct_half_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm3,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0, xm, 0b011, 16, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure to one lane of three registers. This instruction loads a 3-element structure from memory and writes the result to the corresponding elements of the three SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3 { <Vt>.S, <Vt2>.S, <Vt3>.S }[<index>], [<Xn|SP>], #12
    /// ```
    #[inline(always)]
    fn ld3_single_struct_single_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0, 0b101, 32, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure to one lane of three registers. This instruction loads a 3-element structure from memory and writes the result to the corresponding elements of the three SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3 { <Vt>.S, <Vt2>.S, <Vt3>.S }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld3_single_struct_single_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm2,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0, xm, 0b101, 32, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure to one lane of three registers. This instruction loads a 3-element structure from memory and writes the result to the corresponding elements of the three SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3 { <Vt>.D, <Vt2>.D, <Vt3>.D }[<index>], [<Xn|SP>], #24
    /// ```
    #[inline(always)]
    fn ld3_single_struct_double_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0, 0b101, 64, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD3 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--single-structure---Load-single-3-element-structure-to-one-lane-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure to one lane of three registers. This instruction loads a 3-element structure from memory and writes the result to the corresponding elements of the three SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3 { <Vt>.D, <Vt2>.D, <Vt3>.D }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld3_single_struct_double_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        index: UImm1,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0, xm, 0b101, 64, index, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD1R - Load one single element structure and Replicate to all lanes - of one register - ](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1R--Load-one-single-element-structure-and-Replicate-to-all-lanes--of-one-register--?lang=en)
    ///
    /// Load one single-element structure and Replicate to all lanes (of one register). This instruction loads a single-element structure from memory and replicates the structure to all the lanes of the SIMD&FP register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1R { <Vt>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld1r(&mut self, vt: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldxr_plain(self, 1, 0, 0b110, t, xn_sp, &[vt])
    }

    /// [LD1R - Load one single element structure and Replicate to all lanes - of one register - ](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1R--Load-one-single-element-structure-and-Replicate-to-all-lanes--of-one-register--?lang=en)
    ///
    /// Load one single-element structure and Replicate to all lanes (of one register). This instruction loads a single-element structure from memory and replicates the structure to all the lanes of the SIMD&FP register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1R { <Vt>.<T> }, [<Xn|SP>], <imm>
    /// ```
    ///
    /// *Note*: `<imm>` implicitly given by the chosen arrangement specification `t`
    #[inline(always)]
    fn ld1r_offset_imm(&mut self, vt: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldxr_off_imm(self, 1, 0, 0b110, t, xn_sp, &[vt])
    }

    /// [LD1R - Load one single element structure and Replicate to all lanes - of one register - ](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1R--Load-one-single-element-structure-and-Replicate-to-all-lanes--of-one-register--?lang=en)
    ///
    /// Load one single-element structure and Replicate to all lanes (of one register). This instruction loads a single-element structure from memory and replicates the structure to all the lanes of the SIMD&FP register.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1R { <Vt>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld1r_offset_reg(&mut self, vt: Register, t: ArrSpec1, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldxr_off_reg(self, 1, 0, xm, 0b110, t, xn_sp, &[vt])
    }

    /// [LD3R - Load single 3 element structure and Replicate to all lanes of three registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3R--Load-single-3-element-structure-and-Replicate-to-all-lanes-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure and Replicate to all lanes of three registers. This instruction loads a 3-element structure from memory and replicates the structure to all the lanes of the three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3R { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld3r(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        t: ArrSpec1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldxr_plain(self, 1, 0, 0b111, t, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD3R - Load single 3 element structure and Replicate to all lanes of three registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3R--Load-single-3-element-structure-and-Replicate-to-all-lanes-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure and Replicate to all lanes of three registers. This instruction loads a 3-element structure from memory and replicates the structure to all the lanes of the three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3R { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>], <imm>
    /// ```
    ///
    /// *Note*: `<imm>` implicitly given by the chosen arrangement specification `t`
    #[inline(always)]
    fn ld3r_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        t: ArrSpec1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldxr_off_imm(self, 1, 0, 0b111, t, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD3R - Load single 3 element structure and Replicate to all lanes of three registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3R--Load-single-3-element-structure-and-Replicate-to-all-lanes-of-three-registers-?lang=en)
    ///
    /// Load single 3-element structure and Replicate to all lanes of three registers. This instruction loads a 3-element structure from memory and replicates the structure to all the lanes of the three SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD3R { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld3r_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        t: ArrSpec1,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldxr_off_reg(self, 1, 0, xm, 0b111, t, xn_sp, &[vt, vt2, vt3])
    }

    /// [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure to one lane of two registers. This instruction loads a 2-element structure from memory and writes the result to the corresponding elements of the two SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2 { <Vt>.B, <Vt2>.B }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld2_single_struct_bytes(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 1, 1, 0b00, 8, index, xn_sp, &[vt, vt2])
    }

    /// [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure to one lane of two registers. This instruction loads a 2-element structure from memory and writes the result to the corresponding elements of the two SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2 { <Vt>.H, <Vt2>.H }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld2_single_struct_half_words(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 1, 1, 0b010, 16, index, xn_sp, &[vt, vt2])
    }

    /// [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure to one lane of two registers. This instruction loads a 2-element structure from memory and writes the result to the corresponding elements of the two SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2 { <Vt>.S, <Vt2>.S }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld2_single_struct_single_words(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 1, 1, 0b100, 32, index, xn_sp, &[vt, vt2])
    }

    /// [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure to one lane of two registers. This instruction loads a 2-element structure from memory and writes the result to the corresponding elements of the two SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2 { <Vt>.D, <Vt2>.D }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld2_single_struct_double_words(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 1, 1, 0b100, 64, index, xn_sp, &[vt, vt2])
    }

    /// [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure to one lane of two registers. This instruction loads a 2-element structure from memory and writes the result to the corresponding elements of the two SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2 { <Vt>.B, <Vt2>.B }[<index>], [<Xn|SP>], #2
    /// ```
    #[inline(always)]
    fn ld2_single_struct_bytes_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 1, 0b000, 8, index, xn_sp, &[vt, vt2])
    }

    /// [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure to one lane of two registers. This instruction loads a 2-element structure from memory and writes the result to the corresponding elements of the two SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2 { <Vt>.B, <Vt2>.B }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld2_single_struct_bytes_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm4,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 1, xm, 0b000, 8, index, xn_sp, &[vt, vt2])
    }

    /// [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure to one lane of two registers. This instruction loads a 2-element structure from memory and writes the result to the corresponding elements of the two SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2 { <Vt>.H, <Vt2>.H }[<index>], [<Xn|SP>], #4
    /// ```
    #[inline(always)]
    fn ld2_single_struct_half_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 1, 0b010, 16, index, xn_sp, &[vt, vt2])
    }

    /// [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure to one lane of two registers. This instruction loads a 2-element structure from memory and writes the result to the corresponding elements of the two SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2 { <Vt>.H, <Vt2>.H }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld2_single_struct_half_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm3,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 1, xm, 0b010, 16, index, xn_sp, &[vt, vt2])
    }

    /// [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure to one lane of two registers. This instruction loads a 2-element structure from memory and writes the result to the corresponding elements of the two SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2 { <Vt>.S, <Vt2>.S }[<index>], [<Xn|SP>], #8
    /// ```
    #[inline(always)]
    fn ld2_single_struct_single_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 1, 0b100, 32, index, xn_sp, &[vt, vt2])
    }

    /// [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure to one lane of two registers. This instruction loads a 2-element structure from memory and writes the result to the corresponding elements of the two SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2 { <Vt>.S, <Vt2>.S }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld2_single_struct_single_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm2,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 1, xm, 0b100, 32, index, xn_sp, &[vt, vt2])
    }

    /// [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure to one lane of two registers. This instruction loads a 2-element structure from memory and writes the result to the corresponding elements of the two SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2 { <Vt>.D, <Vt2>.D }[<index>], [<Xn|SP>], #16
    /// ```
    #[inline(always)]
    fn ld2_single_struct_double_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 1, 0b100, 64, index, xn_sp, &[vt, vt2])
    }

    /// [LD2 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--single-structure---Load-single-2-element-structure-to-one-lane-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure to one lane of two registers. This instruction loads a 2-element structure from memory and writes the result to the corresponding elements of the two SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2 { <Vt>.D, <Vt2>.D }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld2_single_struct_double_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        index: UImm1,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 1, xm, 0b100, 64, index, xn_sp, &[vt, vt2])
    }

    /// [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure to one lane of four registers. This instruction loads a 4-element structure from memory and writes the result to the corresponding elements of the four SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4 { <Vt>.B, <Vt2>.B, <Vt3>.B, <Vt4>.B }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld4_single_struct_bytes(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 1, 1, 0b001, 8, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure to one lane of four registers. This instruction loads a 4-element structure from memory and writes the result to the corresponding elements of the four SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4 { <Vt>.H, <Vt2>.H, <Vt3>.H, <Vt4>.H }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld4_single_struct_half_words(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 1, 1, 0b011, 16, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure to one lane of four registers. This instruction loads a 4-element structure from memory and writes the result to the corresponding elements of the four SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4 { <Vt>.S, <Vt2>.S, <Vt3>.S, <Vt4>.S }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld4_single_struct_single_words(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 1, 1, 0b101, 32, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure to one lane of four registers. This instruction loads a 4-element structure from memory and writes the result to the corresponding elements of the four SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4 { <Vt>.D, <Vt2>.D, <Vt3>.D, <Vt4>.D }[<index>], [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld4_single_struct_double_words(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_plain(self, 1, 1, 0b101, 64, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure to one lane of four registers. This instruction loads a 4-element structure from memory and writes the result to the corresponding elements of the four SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4 { <Vt>.B, <Vt2>.B, <Vt3>.B, <Vt4>.B }[<index>], [<Xn|SP>], #4
    /// ```
    #[inline(always)]
    fn ld4_single_struct_bytes_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm4,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 1, 0b001, 8, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure to one lane of four registers. This instruction loads a 4-element structure from memory and writes the result to the corresponding elements of the four SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4 { <Vt>.B, <Vt2>.B, <Vt3>.B, <Vt4>.B }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld4_single_struct_bytes_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm4,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 1, xm, 0b001, 8, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure to one lane of four registers. This instruction loads a 4-element structure from memory and writes the result to the corresponding elements of the four SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4 { <Vt>.H, <Vt2>.H, <Vt3>.H, <Vt4>.H }[<index>], [<Xn|SP>], #8
    /// ```
    #[inline(always)]
    fn ld4_single_struct_half_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm3,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 1, 0b011, 16, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure to one lane of four registers. This instruction loads a 4-element structure from memory and writes the result to the corresponding elements of the four SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4 { <Vt>.H, <Vt2>.H, <Vt3>.H, <Vt4>.H }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld4_single_struct_half_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm3,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(
            self,
            1,
            1,
            xm,
            0b011,
            16,
            index,
            xn_sp,
            &[vt, vt2, vt3, vt4],
        )
    }

    /// [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure to one lane of four registers. This instruction loads a 4-element structure from memory and writes the result to the corresponding elements of the four SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4 { <Vt>.S, <Vt2>.S, <Vt3>.S, <Vt4>.S }[<index>], [<Xn|SP>], #16
    /// ```
    #[inline(always)]
    fn ld4_single_struct_single_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm2,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 1, 0b101, 32, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure to one lane of four registers. This instruction loads a 4-element structure from memory and writes the result to the corresponding elements of the four SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4 { <Vt>.S, <Vt2>.S, <Vt3>.S, <Vt4>.S }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld4_single_struct_single_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm2,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(
            self,
            1,
            1,
            xm,
            0b101,
            32,
            index,
            xn_sp,
            &[vt, vt2, vt3, vt4],
        )
    }

    /// [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure to one lane of four registers. This instruction loads a 4-element structure from memory and writes the result to the corresponding elements of the four SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4 { <Vt>.D, <Vt2>.D, <Vt3>.D, <Vt4>.D }[<index>], [<Xn|SP>], #32
    /// ```
    #[inline(always)]
    fn ld4_single_struct_double_words_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 1, 0b101, 64, index, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [LD4 - single structure](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--single-structure---Load-single-4-element-structure-to-one-lane-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure to one lane of four registers. This instruction loads a 4-element structure from memory and writes the result to the corresponding elements of the four SIMD&FP registers without affecting the other bits of the registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4 { <Vt>.D, <Vt2>.D, <Vt3>.D, <Vt4>.D }[<index>], [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld4_single_struct_double_words_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        index: UImm1,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldr_str_off_reg(
            self,
            1,
            1,
            xm,
            0b101,
            64,
            index,
            xn_sp,
            &[vt, vt2, vt3, vt4],
        )
    }

    /// [LD2R - Load single 2 element structure and Replicate to all lanes of two registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2R--Load-single-2-element-structure-and-Replicate-to-all-lanes-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure and Replicate to all lanes of two registers. This instruction loads a 2-element structure from memory and replicates the structure to all the lanes of the two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2R { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld2r(&mut self, vt: Register, vt2: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldxr_plain(self, 1, 1, 0b110, t, xn_sp, &[vt, vt2])
    }

    /// [LD2R - Load single 2 element structure and Replicate to all lanes of two registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2R--Load-single-2-element-structure-and-Replicate-to-all-lanes-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure and Replicate to all lanes of two registers. This instruction loads a 2-element structure from memory and replicates the structure to all the lanes of the two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2R { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>], <imm>
    /// ```
    ///
    /// *Note*: `<imm>` implicitly given by the chosen arrangement specification `t`
    #[inline(always)]
    fn ld2r_offset_imm(&mut self, vt: Register, vt2: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldxr_off_imm(self, 1, 1, 0b110, t, xn_sp, &[vt, vt2])
    }

    /// [LD2R - Load single 2 element structure and Replicate to all lanes of two registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2R--Load-single-2-element-structure-and-Replicate-to-all-lanes-of-two-registers-?lang=en)
    ///
    /// Load single 2-element structure and Replicate to all lanes of two registers. This instruction loads a 2-element structure from memory and replicates the structure to all the lanes of the two SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD2R { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld2r_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        t: ArrSpec1,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldxr_off_reg(self, 1, 1, xm, 0b110, t, xn_sp, &[vt, vt2])
    }

    /// [LD4R - Load single 4 element structure and Replicate to all lanes of four registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4R--Load-single-4-element-structure-and-Replicate-to-all-lanes-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure and Replicate to all lanes of four registers. This instruction loads a 4-element structure from memory and replicates the structure to all the lanes of the four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4R { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld4r(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        t: ArrSpec1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldxr_plain(self, 1, 1, 0b111, t, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [LD4R - Load single 4 element structure and Replicate to all lanes of four registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4R--Load-single-4-element-structure-and-Replicate-to-all-lanes-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure and Replicate to all lanes of four registers. This instruction loads a 4-element structure from memory and replicates the structure to all the lanes of the four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4R { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>], <imm>
    /// ```
    ///
    /// *Note*: `<imm>` implicitly given by the chosen arrangement specification `t`
    #[inline(always)]
    fn ld4r_offset_imm(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        t: ArrSpec1,
        xn_sp: Register,
    ) -> T {
        emit_adv_ldxr_off_imm(self, 1, 1, 0b111, t, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [LD4R - Load single 4 element structure and Replicate to all lanes of four registers](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4R--Load-single-4-element-structure-and-Replicate-to-all-lanes-of-four-registers-?lang=en)
    ///
    /// Load single 4-element structure and Replicate to all lanes of four registers. This instruction loads a 4-element structure from memory and replicates the structure to all the lanes of the four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD4R { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld4r_offset_reg(
        &mut self,
        vt: Register,
        vt2: Register,
        vt3: Register,
        vt4: Register,
        t: ArrSpec1,
        xn_sp: Register,
        xm: Register,
    ) -> T {
        emit_adv_ldxr_off_reg(self, 1, 1, xm, 0b111, t, xn_sp, &[vt, vt2, vt3, vt4])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_st1() {
        let mut prod = TestProducer::new();

        let instr = prod.st1_single_struct_bytes(1, 0b1111, 2);
        assert_eq!(instr, "st1 {v1.b}[15], [x2]");

        let instr = prod.st1_single_struct_bytes_offset_imm(1, 0b1111, 2);
        assert_eq!(instr, "st1 {v1.b}[15], [x2], #0x1");

        let instr = prod.st1_single_struct_bytes_offset_reg(1, 0b1111, 2, 3);
        assert_eq!(instr, "st1 {v1.b}[15], [x2], x3");

        let instr = prod.st1_single_struct_half_words(1, 0b111, 2);
        assert_eq!(instr, "st1 {v1.h}[7], [x2]");

        let instr = prod.st1_single_struct_half_words_offset_imm(1, 0b111, 2);
        assert_eq!(instr, "st1 {v1.h}[7], [x2], #0x2");

        let instr = prod.st1_single_struct_half_words_offset_reg(1, 0b111, 2, 3);
        assert_eq!(instr, "st1 {v1.h}[7], [x2], x3");

        let instr = prod.st1_single_struct_single_words(1, 0b111, 2);
        assert_eq!(instr, "st1 {v1.s}[3], [x2]");

        let instr = prod.st1_single_struct_single_words_offset_imm(1, 0b111, 2);
        assert_eq!(instr, "st1 {v1.s}[3], [x2], #0x4");

        let instr = prod.st1_single_struct_single_words_offset_reg(1, 0b111, 2, 3);
        assert_eq!(instr, "st1 {v1.s}[3], [x2], x3");

        let instr = prod.st1_single_struct_double_words(1, 0b111, 2);
        assert_eq!(instr, "st1 {v1.d}[1], [x2]");

        let instr = prod.st1_single_struct_double_words_offset_imm(1, 0b111, 2);
        assert_eq!(instr, "st1 {v1.d}[1], [x2], #0x8");

        let instr = prod.st1_single_struct_double_words_offset_reg(1, 0b111, 2, 3);
        assert_eq!(instr, "st1 {v1.d}[1], [x2], x3");
    }

    #[test]
    fn test_st3() {
        let mut prod = TestProducer::new();

        let instr = prod.st3_single_struct_bytes(1, 2, 3, 0b1111, 2);
        assert_eq!(instr, "st3 {v1.b, v2.b, v3.b}[15], [x2]");

        let instr = prod.st3_single_struct_bytes_offset_imm(1, 2, 3, 0b1111, 2);
        assert_eq!(instr, "st3 {v1.b, v2.b, v3.b}[15], [x2], #0x3");

        let instr = prod.st3_single_struct_bytes_offset_reg(1, 2, 3, 0b1111, 2, 3);
        assert_eq!(instr, "st3 {v1.b, v2.b, v3.b}[15], [x2], x3");

        let instr = prod.st3_single_struct_half_words(1, 2, 3, 0b111, 2);
        assert_eq!(instr, "st3 {v1.h, v2.h, v3.h}[7], [x2]");

        let instr = prod.st3_single_struct_half_words_offset_imm(1, 2, 3, 0b111, 2);
        assert_eq!(instr, "st3 {v1.h, v2.h, v3.h}[7], [x2], #0x6");

        let instr = prod.st3_single_struct_half_words_offset_reg(1, 2, 3, 0b111, 2, 3);
        assert_eq!(instr, "st3 {v1.h, v2.h, v3.h}[7], [x2], x3");

        let instr = prod.st3_single_struct_single_words(1, 2, 3, 0b111, 2);
        assert_eq!(instr, "st3 {v1.s, v2.s, v3.s}[3], [x2]");

        let instr = prod.st3_single_struct_single_words_offset_imm(1, 2, 3, 0b111, 2);
        assert_eq!(instr, "st3 {v1.s, v2.s, v3.s}[3], [x2], #0xc");

        let instr = prod.st3_single_struct_single_words_offset_reg(1, 2, 3, 0b111, 2, 3);
        assert_eq!(instr, "st3 {v1.s, v2.s, v3.s}[3], [x2], x3");

        let instr = prod.st3_single_struct_double_words(1, 2, 3, 0b111, 2);
        assert_eq!(instr, "st3 {v1.d, v2.d, v3.d}[1], [x2]");

        let instr = prod.st3_single_struct_double_words_offset_imm(1, 2, 3, 0b111, 2);
        assert_eq!(instr, "st3 {v1.d, v2.d, v3.d}[1], [x2], #0x18");

        let instr = prod.st3_single_struct_double_words_offset_reg(1, 2, 3, 0b111, 2, 3);
        assert_eq!(instr, "st3 {v1.d, v2.d, v3.d}[1], [x2], x3");
    }

    #[test]
    fn test_st2() {
        let mut prod = TestProducer::new();

        let instr = prod.st2_single_struct_bytes(1, 2, 0b1111, 2);
        assert_eq!(instr, "st2 {v1.b, v2.b}[15], [x2]");

        let instr = prod.st2_single_struct_bytes_offset_imm(1, 2, 0b1111, 2);
        assert_eq!(instr, "st2 {v1.b, v2.b}[15], [x2], #0x2");

        let instr = prod.st2_single_struct_bytes_offset_reg(1, 2, 0b1111, 2, 3);
        assert_eq!(instr, "st2 {v1.b, v2.b}[15], [x2], x3");

        let instr = prod.st2_single_struct_half_words(1, 2, 0b111, 2);
        assert_eq!(instr, "st2 {v1.h, v2.h}[7], [x2]");

        let instr = prod.st2_single_struct_half_words_offset_imm(1, 2, 0b111, 2);
        assert_eq!(instr, "st2 {v1.h, v2.h}[7], [x2], #0x4");

        let instr = prod.st2_single_struct_half_words_offset_reg(1, 2, 0b111, 2, 3);
        assert_eq!(instr, "st2 {v1.h, v2.h}[7], [x2], x3");

        let instr = prod.st2_single_struct_single_words(1, 2, 0b111, 2);
        assert_eq!(instr, "st2 {v1.s, v2.s}[3], [x2]");

        let instr = prod.st2_single_struct_single_words_offset_imm(1, 2, 0b111, 2);
        assert_eq!(instr, "st2 {v1.s, v2.s}[3], [x2], #0x8");

        let instr = prod.st2_single_struct_single_words_offset_reg(1, 2, 0b111, 2, 3);
        assert_eq!(instr, "st2 {v1.s, v2.s}[3], [x2], x3");

        let instr = prod.st2_single_struct_double_words(1, 2, 0b111, 2);
        assert_eq!(instr, "st2 {v1.d, v2.d}[1], [x2]");

        let instr = prod.st2_single_struct_double_words_offset_imm(1, 2, 0b111, 2);
        assert_eq!(instr, "st2 {v1.d, v2.d}[1], [x2], #0x10");

        let instr = prod.st2_single_struct_double_words_offset_reg(1, 2, 0b111, 2, 3);
        assert_eq!(instr, "st2 {v1.d, v2.d}[1], [x2], x3");
    }

    #[test]
    fn test_st4() {
        let mut prod = TestProducer::new();

        let instr = prod.st4_single_struct_bytes(1, 2, 3, 4, 0b1111, 2);
        assert_eq!(instr, "st4 {v1.b, v2.b, v3.b, v4.b}[15], [x2]");

        let instr = prod.st4_single_struct_bytes_offset_imm(1, 2, 3, 4, 0b1111, 2);
        assert_eq!(instr, "st4 {v1.b, v2.b, v3.b, v4.b}[15], [x2], #0x4");

        let instr = prod.st4_single_struct_bytes_offset_reg(1, 2, 3, 4, 0b1111, 2, 3);
        assert_eq!(instr, "st4 {v1.b, v2.b, v3.b, v4.b}[15], [x2], x3");

        let instr = prod.st4_single_struct_half_words(1, 2, 3, 4, 0b111, 2);
        assert_eq!(instr, "st4 {v1.h, v2.h, v3.h, v4.h}[7], [x2]");

        let instr = prod.st4_single_struct_half_words_offset_imm(1, 2, 3, 4, 0b111, 2);
        assert_eq!(instr, "st4 {v1.h, v2.h, v3.h, v4.h}[7], [x2], #0x8");

        let instr = prod.st4_single_struct_half_words_offset_reg(1, 2, 3, 4, 0b111, 2, 3);
        assert_eq!(instr, "st4 {v1.h, v2.h, v3.h, v4.h}[7], [x2], x3");

        let instr = prod.st4_single_struct_single_words(1, 2, 3, 4, 0b111, 2);
        assert_eq!(instr, "st4 {v1.s, v2.s, v3.s, v4.s}[3], [x2]");

        let instr = prod.st4_single_struct_single_words_offset_imm(1, 2, 3, 4, 0b111, 2);
        assert_eq!(instr, "st4 {v1.s, v2.s, v3.s, v4.s}[3], [x2], #0x10");

        let instr = prod.st4_single_struct_single_words_offset_reg(1, 2, 3, 4, 0b111, 2, 3);
        assert_eq!(instr, "st4 {v1.s, v2.s, v3.s, v4.s}[3], [x2], x3");

        let instr = prod.st4_single_struct_double_words(1, 2, 3, 4, 0b111, 2);
        assert_eq!(instr, "st4 {v1.d, v2.d, v3.d, v4.d}[1], [x2]");

        let instr = prod.st4_single_struct_double_words_offset_imm(1, 2, 3, 4, 0b111, 2);
        assert_eq!(instr, "st4 {v1.d, v2.d, v3.d, v4.d}[1], [x2], #0x20");

        let instr = prod.st4_single_struct_double_words_offset_reg(1, 2, 3, 4, 0b111, 2, 3);
        assert_eq!(instr, "st4 {v1.d, v2.d, v3.d, v4.d}[1], [x2], x3");
    }

    #[test]
    fn test_ld1r() {
        let mut prod = TestProducer::new();

        let instr = prod.ld1r(1, ArrSpec1::T1D, 2);
        assert_eq!(instr, "ld1r {v1.1d}, [x2]");

        let instr = prod.ld1r_offset_imm(1, ArrSpec1::T4H, 2);
        assert_eq!(instr, "ld1r {v1.4h}, [x2], #0x2");

        let instr = prod.ld1r_offset_reg(1, ArrSpec1::T1D, 2, 3);
        assert_eq!(instr, "ld1r {v1.1d}, [x2], x3");
    }

    #[test]
    fn test_ld2r() {
        let mut prod = TestProducer::new();

        let instr = prod.ld2r(1, 2, ArrSpec1::T1D, 2);
        assert_eq!(instr, "ld2r {v1.1d, v2.1d}, [x2]");

        let instr = prod.ld2r_offset_imm(1, 2, ArrSpec1::T1D, 2);
        assert_eq!(instr, "ld2r {v1.1d, v2.1d}, [x2], #0x10");

        let instr = prod.ld2r_offset_reg(1, 2, ArrSpec1::T1D, 2, 3);
        assert_eq!(instr, "ld2r {v1.1d, v2.1d}, [x2], x3");
    }

    #[test]
    fn test_ld3r() {
        let mut prod = TestProducer::new();

        let instr = prod.ld3r(1, 2, 3, ArrSpec1::T1D, 2);
        assert_eq!(instr, "ld3r {v1.1d, v2.1d, v3.1d}, [x2]");

        let instr = prod.ld3r_offset_imm(1, 2, 3, ArrSpec1::T1D, 2);
        assert_eq!(instr, "ld3r {v1.1d, v2.1d, v3.1d}, [x2], #0x18");

        let instr = prod.ld3r_offset_reg(1, 2, 3, ArrSpec1::T1D, 2, 3);
        assert_eq!(instr, "ld3r {v1.1d, v2.1d, v3.1d}, [x2], x3");
    }

    #[test]
    fn test_ld4r() {
        let mut prod = TestProducer::new();

        let instr = prod.ld4r(1, 2, 3, 4, ArrSpec1::T1D, 2);
        assert_eq!(instr, "ld4r {v1.1d, v2.1d, v3.1d, v4.1d}, [x2]");

        let instr = prod.ld4r_offset_imm(1, 2, 3, 4, ArrSpec1::T1D, 2);
        assert_eq!(instr, "ld4r {v1.1d, v2.1d, v3.1d, v4.1d}, [x2], #0x20");

        let instr = prod.ld4r_offset_reg(1, 2, 3, 4, ArrSpec1::T1D, 2, 3);
        assert_eq!(instr, "ld4r {v1.1d, v2.1d, v3.1d, v4.1d}, [x2], x3");
    }
}
