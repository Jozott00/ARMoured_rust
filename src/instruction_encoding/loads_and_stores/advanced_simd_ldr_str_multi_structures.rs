//! # [Advanced SIMD load/store multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#asisdlse)
//!
//! Implements the following instructions:
//!  - [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
//!  - [ST2 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--multiple-structures---Store-multiple-2-element-structures-from-two-registers-?lang=en)
//!  - [ST3 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--multiple-structures---Store-multiple-3-element-structures-from-three-registers-?lang=en)
//!  - [ST4 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--multiple-structures---Store-multiple-4-element-structures-from-four-registers-?lang=en)
//!  - [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
//!  - [LD2 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--multiple-structures---Load-multiple-2-element-structures-to-two-registers-?lang=en)
//!  - [LD3 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--multiple-structures---Load-multiple-3-element-structures-to-three-registers-?lang=en)
//!  - [LD4 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--multiple-structures---Load-multiple-4-element-structures-to-four-registers-?lang=en)



use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Register, UImm6};
use crate::types::ArrSpecifier::{ArrSpec, ArrSpecX, ArrSpec1};

#[inline(always)]
fn emit_adv_ldr_str_all<P: InstructionProcessor<T>, T>(proc: &mut P, q: u8, with_offset: bool, l: u8, rm: Register, opcode: u8, size: u8, rn: Register, rt: Register) -> T {
    let op = with_offset as u8;
    let r = bseq_32!(0 q:1 001100 op:1 l:1 0 rm:5 opcode:4 size:2 rn:5 rt:5);
    //                     0 1  001100   1   0   0 11111 0000 11 00000 00000
    //                     0 1  001100   0   0   0 11111 0000 11 00000 00001
    proc.process(r)
}

#[inline(always)]
fn emit_adv_ldr_str_plain<P: InstructionProcessor<T>, T, A: ArrSpec>(proc: &mut P, l: u8, opcode: u8, t: A, rn: Register, vts: &[Register]) -> T {
    debug_assert!(are_indices_sequential(vts), "vector registers must be sequential, were {:?}", vts);
    emit_adv_ldr_str_all(proc, t.q(), false, l, 0, opcode, t.size(), rn, vts[0])
}

#[inline(always)]
fn emit_adv_ldr_str_off_reg<P: InstructionProcessor<T>, T, A: ArrSpec>(proc: &mut P, l: u8, opcode: u8, rm: Register, t: A, rn: Register, vts: &[Register]) -> T {
    debug_assert!(are_indices_sequential(vts), "vector registers must be sequential, were {:?}", vts);
    emit_adv_ldr_str_all(proc, t.q(), true, l, rm, opcode, t.size(), rn, vts[0])
}

#[inline(always)]
fn emit_adv_ldr_str_off_imm<P: InstructionProcessor<T>, T, A: ArrSpec>(proc: &mut P, l: u8, opcode: u8, t: A, rn: Register, vts: &[Register]) -> T {
    debug_assert!(are_indices_sequential(vts), "vector registers must be sequential, were {:?}", vts);
    emit_adv_ldr_str_all(proc, t.q(), true, l, 0b11111, opcode, t.size(), rn, vts[0])
}

#[cfg(debug_assertions)]
fn are_indices_sequential(indices: &[Register]) -> bool {
    for i in 0..(indices.len() - 1) {
        if (indices[i] + 1) % 32 != indices[(i + 1) % indices.len()] {
            return false;
        }
    }
    true
}

/// # [Advanced SIMD load/store multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#asisdlse)
///
/// Implements the following instructions:
///  - [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
///  - [ST2 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--multiple-structures---Store-multiple-2-element-structures-from-two-registers-?lang=en)
///  - [ST3 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--multiple-structures---Store-multiple-3-element-structures-from-three-registers-?lang=en)
///  - [ST4 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--multiple-structures---Store-multiple-4-element-structures-from-four-registers-?lang=en)
///  - [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
///  - [LD2 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--multiple-structures---Load-multiple-2-element-structures-to-two-registers-?lang=en)
///  - [LD3 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--multiple-structures---Load-multiple-3-element-structures-to-three-registers-?lang=en)
///  - [LD4 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--multiple-structures---Load-multiple-4-element-structures-to-four-registers-?lang=en)
pub trait AdvancedSimdLoadStoreMultipleStructures<T>: InstructionProcessor<T> {
    /// [ST4 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--multiple-structures---Store-multiple-4-element-structures-from-four-registers-?lang=en)
    ///
    /// Store multiple 4-element structures from four registers. This instruction stores multiple 4-element structures to memory from four SIMD&FP registers, with interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st4_multi_structs(&mut self, vt: Register, vt2: Register, vt3: Register, vt4: Register, t: ArrSpecX, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 0, 0b0000, t, xn_sp, &[vt, vt2, vt3, vt4])
    }


    /// [ST4 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--multiple-structures---Store-multiple-4-element-structures-from-four-registers-?lang=en)
    ///
    /// Store multiple 4-element structures from four registers. This instruction stores multiple 4-element structures to memory from four SIMD&FP registers, with interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>], <imm>
    /// ```
    ///
    /// *Note*: The register is implicitly given by the chose arrange spec
    #[inline(always)]
    fn st4_multi_structs_offset_imm(&mut self, vt: Register, vt2: Register, vt3: Register, vt4: Register, t: ArrSpecX, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0b0000, t, xn_sp, &[vt, vt2, vt3, vt4])
    }


    /// [ST4 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST4--multiple-structures---Store-multiple-4-element-structures-from-four-registers-?lang=en)
    ///
    /// Store multiple 4-element structures from four registers. This instruction stores multiple 4-element structures to memory from four SIMD&FP registers, with interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST4 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st4_multi_structs_offset_reg(&mut self, vt: Register, vt2: Register, vt3: Register, vt4: Register, t: ArrSpecX, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0b0000, xm, t, xn_sp, &[vt, vt2, vt3, vt4])
    }


    /// [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
    ///
    /// Store multiple single-element structures from one, two, three, or four registers. This instruction stores elements to memory from one, two, three, or four SIMD&FP registers, without interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st1_multi_structs_one_reg(&mut self, vt: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 0, 0b0111, t, xn_sp, &[vt])
    }


    /// [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
    ///
    /// Store multiple single-element structures from one, two, three, or four registers. This instruction stores elements to memory from one, two, three, or four SIMD&FP registers, without interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st1_multi_structs_two_reg(&mut self, vt: Register, vt2: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 0, 0b1010, t, xn_sp, &[vt, vt2])
    }


    /// [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
    ///
    /// Store multiple single-element structures from one, two, three, or four registers. This instruction stores elements to memory from one, two, three, or four SIMD&FP registers, without interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st1_multi_structs_three_reg(&mut self, vt: Register, vt2: Register, vt3: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 0, 0b0110, t, xn_sp, &[vt, vt2, vt3])
    }


    /// [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
    ///
    /// Store multiple single-element structures from one, two, three, or four registers. This instruction stores elements to memory from one, two, three, or four SIMD&FP registers, without interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st1_multi_structs_four_reg(&mut self, vt: Register, vt2: Register, vt3: Register, vt4: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 0, 0b0010, t, xn_sp, &[vt, vt2, vt3, vt4])
    }


    /// [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
    ///
    /// Store multiple single-element structures from one, two, three, or four registers. This instruction stores elements to memory from one, two, three, or four SIMD&FP registers, without interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.<T> }, [<Xn|SP>], <imm>
    /// ```
    #[inline(always)]
    fn st1_multi_structs_one_reg_offset_imm(&mut self, vt: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0b0111, t, xn_sp, &[vt])
    }


    /// [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
    ///
    /// Store multiple single-element structures from one, two, three, or four registers. This instruction stores elements to memory from one, two, three, or four SIMD&FP registers, without interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st1_multi_structs_one_reg_offset_reg(&mut self, vt: Register, t: ArrSpec1, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0b0111, xm, t, xn_sp, &[vt])
    }


    /// [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
    ///
    /// Store multiple single-element structures from one, two, three, or four registers. This instruction stores elements to memory from one, two, three, or four SIMD&FP registers, without interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>], <imm>
    /// ```
    #[inline(always)]
    fn st1_multi_structs_two_reg_offset_imm(&mut self, vt: Register, vt2: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0b1010, t, xn_sp, &[vt, vt2])
    }


    /// [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
    ///
    /// Store multiple single-element structures from one, two, three, or four registers. This instruction stores elements to memory from one, two, three, or four SIMD&FP registers, without interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st1_multi_structs_two_reg_offset_reg(&mut self, vt: Register, vt2: Register, t: ArrSpec1, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0b1010, xm, t, xn_sp, &[vt, vt2])
    }


    /// [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
    ///
    /// Store multiple single-element structures from one, two, three, or four registers. This instruction stores elements to memory from one, two, three, or four SIMD&FP registers, without interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>], <imm>
    /// ```
    #[inline(always)]
    fn st1_multi_structs_three_reg_offset_imm(&mut self, vt: Register, vt2: Register, vt3: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0b0110, t, xn_sp, &[vt, vt2, vt3])
    }


    /// [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
    ///
    /// Store multiple single-element structures from one, two, three, or four registers. This instruction stores elements to memory from one, two, three, or four SIMD&FP registers, without interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st1_multi_structs_three_reg_offset_reg(&mut self, vt: Register, vt2: Register, vt3: Register, t: ArrSpec1, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0b0110, xm, t, xn_sp, &[vt, vt2, vt3])
    }


    /// [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
    ///
    /// Store multiple single-element structures from one, two, three, or four registers. This instruction stores elements to memory from one, two, three, or four SIMD&FP registers, without interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>], <imm>
    /// ```
    #[inline(always)]
    fn st1_multi_structs_four_reg_offset_imm(&mut self, vt: Register, vt2: Register, vt3: Register, vt4: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0b0010, t, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [ST1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST1--multiple-structures---Store-multiple-single-element-structures-from-one--two--three--or-four-registers-?lang=en)
    ///
    /// Store multiple single-element structures from one, two, three, or four registers. This instruction stores elements to memory from one, two, three, or four SIMD&FP registers, without interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST1 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st1_multi_structs_four_reg_offset_reg(&mut self, vt: Register, vt2: Register, vt3: Register, vt4: Register, t: ArrSpec1, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0b0010, xm, t, xn_sp, &[vt, vt2, vt3, vt4])
    }


    /// [ST3 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--multiple-structures---Store-multiple-3-element-structures-from-three-registers-?lang=en)
    ///
    /// Store multiple 3-element structures from three registers. This instruction stores multiple 3-element structures to memory from three SIMD&FP registers, with interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st3_multi_structs(&mut self, vt: Register, vt2: Register, vt3: Register, t: ArrSpecX, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 0, 0b0100, t, xn_sp, &[vt, vt2, vt3])
    }


    /// [ST3 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--multiple-structures---Store-multiple-3-element-structures-from-three-registers-?lang=en)
    ///
    /// Store multiple 3-element structures from three registers. This instruction stores multiple 3-element structures to memory from three SIMD&FP registers, with interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>], <imm>
    /// ```
    #[inline(always)]
    fn st3_multi_structs_offset_imm(&mut self, vt: Register, vt2: Register, vt3: Register, t: ArrSpecX, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0b0100, t, xn_sp, &[vt, vt2, vt3])
    }


    /// [ST3 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST3--multiple-structures---Store-multiple-3-element-structures-from-three-registers-?lang=en)
    ///
    /// Store multiple 3-element structures from three registers. This instruction stores multiple 3-element structures to memory from three SIMD&FP registers, with interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST3 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st3_multi_structs_offset_reg(&mut self, vt: Register, vt2: Register, vt3: Register, t: ArrSpecX, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0b0100, xm, t, xn_sp, &[vt, vt2, vt3])
    }


    /// [ST2 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--multiple-structures---Store-multiple-2-element-structures-from-two-registers-?lang=en)
    ///
    /// Store multiple 2-element structures from two registers. This instruction stores multiple 2-element structures from two SIMD&FP registers to memory, with interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn st2_multi_structs(&mut self, vt: Register, vt2: Register, t: ArrSpecX, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 0, 0b1000, t, xn_sp, &[vt, vt2])
    }


    /// [ST2 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--multiple-structures---Store-multiple-2-element-structures-from-two-registers-?lang=en)
    ///
    /// Store multiple 2-element structures from two registers. This instruction stores multiple 2-element structures from two SIMD&FP registers to memory, with interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>], <imm>
    /// ```
    #[inline(always)]
    fn st2_multi_structs_offset_imm(&mut self, vt: Register, vt2: Register, t: ArrSpecX, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 0, 0b1000, t, xn_sp, &[vt, vt2])
    }


    /// [ST2 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/ST2--multiple-structures---Store-multiple-2-element-structures-from-two-registers-?lang=en)
    ///
    /// Store multiple 2-element structures from two registers. This instruction stores multiple 2-element structures from two SIMD&FP registers to memory, with interleaving. Every element of each register is stored.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// ST2 { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn st2_multi_structs_offset_reg(&mut self, vt: Register, vt2: Register, t: ArrSpecX, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 0, 0b1000, xm, t, xn_sp, &[vt, vt2])
    }


    /// [LD4 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--multiple-structures---Load-multiple-4-element-structures-to-four-registers-?lang=en)
    ///
    /// Load multiple 4-element structures to four registers. This instruction loads multiple 4-element structures from memory and writes the result to the four SIMD&FP registers, with de-interleaving.
    ///
    /// For an example of de-interleaving, see LD3 (multiple structures).
    ///
    /// ```asm
    /// LD4 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld4_multi_structs(&mut self, vt: Register, vt2: Register, vt3: Register, vt4: Register, t: ArrSpecX, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 1, 0b0000, t, xn_sp, &[vt, vt2, vt3, vt4])
    }


    /// [LD4 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--multiple-structures---Load-multiple-4-element-structures-to-four-registers-?lang=en)
    ///
    /// Load multiple 4-element structures to four registers. This instruction loads multiple 4-element structures from memory and writes the result to the four SIMD&FP registers, with de-interleaving.
    ///
    /// For an example of de-interleaving, see LD3 (multiple structures).
    ///
    /// ```asm
    /// LD4 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>], <imm>
    /// ```
    #[inline(always)]
    fn ld4_multi_structs_offset_imm(&mut self, vt: Register, vt2: Register, vt3: Register, vt4: Register, t: ArrSpecX, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0b0000, t, xn_sp, &[vt, vt2, vt3, vt4])
    }


    /// [LD4 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD4--multiple-structures---Load-multiple-4-element-structures-to-four-registers-?lang=en)
    ///
    /// Load multiple 4-element structures to four registers. This instruction loads multiple 4-element structures from memory and writes the result to the four SIMD&FP registers, with de-interleaving.
    ///
    /// For an example of de-interleaving, see LD3 (multiple structures).
    ///
    /// ```asm
    /// LD4 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld4_multi_structs_offset_reg(&mut self, vt: Register, vt2: Register, vt3: Register, vt4: Register, t: ArrSpecX, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0b0000, xm, t, xn_sp, &[vt, vt2, vt3, vt4])
    }


    /// [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
    ///
    /// Load multiple single-element structures to one, two, three, or four registers. This instruction loads multiple single-element structures from memory and writes the result to one, two, three, or four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld1_multi_structs_one_reg(&mut self, vt: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 1, 0b0111, t, xn_sp, &[vt])
    }


    /// [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
    ///
    /// Load multiple single-element structures to one, two, three, or four registers. This instruction loads multiple single-element structures from memory and writes the result to one, two, three, or four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld1_multi_structs_two_reg(&mut self, vt: Register, vt2: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 1, 0b1010, t, xn_sp, &[vt, vt2])
    }


    /// [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
    ///
    /// Load multiple single-element structures to one, two, three, or four registers. This instruction loads multiple single-element structures from memory and writes the result to one, two, three, or four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld1_multi_structs_three_reg(&mut self, vt: Register, vt2: Register, vt3: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 1, 0b0110, t, xn_sp, &[vt, vt2, vt3])
    }


    /// [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
    ///
    /// Load multiple single-element structures to one, two, three, or four registers. This instruction loads multiple single-element structures from memory and writes the result to one, two, three, or four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld1_multi_structs_four_reg(&mut self, vt: Register, vt2: Register, vt3: Register, vt4: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 1, 0b0010, t, xn_sp, &[vt, vt2, vt3, vt4])
    }

    /// [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
    ///
    /// Load multiple single-element structures to one, two, three, or four registers. This instruction loads multiple single-element structures from memory and writes the result to one, two, three, or four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.<T> }, [<Xn|SP>], <imm>
    /// ```
    #[inline(always)]
    fn ld1_multi_structs_one_reg_offset_imm(&mut self, vt: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0b0111, t, xn_sp, &[vt])
    }


    /// [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
    ///
    /// Load multiple single-element structures to one, two, three, or four registers. This instruction loads multiple single-element structures from memory and writes the result to one, two, three, or four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld1_multi_structs_one_reg_offset_reg(&mut self, vt: Register, t: ArrSpec1, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0b0111, xm, t, xn_sp, &[vt])
    }


    /// [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
    ///
    /// Load multiple single-element structures to one, two, three, or four registers. This instruction loads multiple single-element structures from memory and writes the result to one, two, three, or four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>], <imm>
    /// ```
    #[inline(always)]
    fn ld1_multi_structs_two_reg_offset_imm(&mut self, vt: Register, vt2: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0b1010, t, xn_sp, &[vt, vt2])
    }


    /// [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
    ///
    /// Load multiple single-element structures to one, two, three, or four registers. This instruction loads multiple single-element structures from memory and writes the result to one, two, three, or four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld1_multi_structs_two_reg_offset_reg(&mut self, vt: Register, vt2: Register, t: ArrSpec1, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0b1010, xm, t, xn_sp, &[vt, vt2])
    }


    /// [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
    ///
    /// Load multiple single-element structures to one, two, three, or four registers. This instruction loads multiple single-element structures from memory and writes the result to one, two, three, or four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>], <imm>
    /// ```
    #[inline(always)]
    fn ld1_multi_structs_three_reg_offset_imm(&mut self, vt: Register, vt2: Register, vt3: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0b0110, t, xn_sp, &[vt, vt2, vt3])
    }


    /// [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
    ///
    /// Load multiple single-element structures to one, two, three, or four registers. This instruction loads multiple single-element structures from memory and writes the result to one, two, three, or four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld1_multi_structs_three_reg_offset_reg(&mut self, vt: Register, vt2: Register, vt3: Register, t: ArrSpec1, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0b0110, xm, t, xn_sp, &[vt, vt2, vt3])
    }


    /// [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
    ///
    /// Load multiple single-element structures to one, two, three, or four registers. This instruction loads multiple single-element structures from memory and writes the result to one, two, three, or four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>], <imm>
    /// ```
    #[inline(always)]
    fn ld1_multi_structs_four_reg_offset_imm(&mut self, vt: Register, vt2: Register, vt3: Register, vt4: Register, t: ArrSpec1, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0b0010, t, xn_sp, &[vt, vt2, vt3, vt4])
    }


    /// [LD1 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD1--multiple-structures---Load-multiple-single-element-structures-to-one--two--three--or-four-registers-?lang=en)
    ///
    /// Load multiple single-element structures to one, two, three, or four registers. This instruction loads multiple single-element structures from memory and writes the result to one, two, three, or four SIMD&FP registers.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LD1 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T>, <Vt4>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld1_multi_structs_four_reg_offset_reg(&mut self, vt: Register, vt2: Register, vt3: Register, vt4: Register, t: ArrSpec1, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0b0010, xm, t, xn_sp, &[vt, vt2, vt3, vt4])
    }


    /// [LD3 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--multiple-structures---Load-multiple-3-element-structures-to-three-registers-?lang=en)
    ///
    /// Load multiple 3-element structures to three registers. This instruction loads multiple 3-element structures from memory and writes the result to the three SIMD&FP registers, with de-interleaving.
    ///
    ///  The following figure shows an example of the operation of de-interleaving of a LD3.16 (multiple 3-element structures) instruction:.
    ///
    /// ```asm
    /// LD3 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld3_multi_structs(&mut self, vt: Register, vt2: Register, vt3: Register, t: ArrSpecX, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 1, 0b0100, t, xn_sp, &[vt, vt2, vt3])
    }


    /// [LD3 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--multiple-structures---Load-multiple-3-element-structures-to-three-registers-?lang=en)
    ///
    /// Load multiple 3-element structures to three registers. This instruction loads multiple 3-element structures from memory and writes the result to the three SIMD&FP registers, with de-interleaving.
    ///
    ///  The following figure shows an example of the operation of de-interleaving of a LD3.16 (multiple 3-element structures) instruction:.
    ///
    /// ```asm
    /// LD3 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>], <imm>
    /// ```
    #[inline(always)]
    fn ld3_multi_structs_offset_imm(&mut self, vt: Register, vt2: Register, vt3: Register, t: ArrSpecX, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0b0100, t, xn_sp, &[vt, vt2, vt3])
    }


    /// [LD3 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD3--multiple-structures---Load-multiple-3-element-structures-to-three-registers-?lang=en)
    ///
    /// Load multiple 3-element structures to three registers. This instruction loads multiple 3-element structures from memory and writes the result to the three SIMD&FP registers, with de-interleaving.
    ///
    ///  The following figure shows an example of the operation of de-interleaving of a LD3.16 (multiple 3-element structures) instruction:.
    ///
    /// ```asm
    /// LD3 { <Vt>.<T>, <Vt2>.<T>, <Vt3>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld3_multi_structs_offset_reg(&mut self, vt: Register, vt2: Register, vt3: Register, t: ArrSpecX, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0b0100, xm, t, xn_sp, &[vt, vt2, vt3])
    }


    /// [LD2 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--multiple-structures---Load-multiple-2-element-structures-to-two-registers-?lang=en)
    ///
    /// Load multiple 2-element structures to two registers. This instruction loads multiple 2-element structures from memory and writes the result to the two SIMD&FP registers, with de-interleaving.
    ///
    /// For an example of de-interleaving, see LD3 (multiple structures).
    ///
    /// ```asm
    /// LD2 { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ld2_multi_structs(&mut self, vt: Register, vt2: Register, t: ArrSpecX, xn_sp: Register) -> T {
        emit_adv_ldr_str_plain(self, 1, 0b1000, t, xn_sp, &[vt, vt2])
    }


    /// [LD2 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--multiple-structures---Load-multiple-2-element-structures-to-two-registers-?lang=en)
    ///
    /// Load multiple 2-element structures to two registers. This instruction loads multiple 2-element structures from memory and writes the result to the two SIMD&FP registers, with de-interleaving.
    ///
    /// For an example of de-interleaving, see LD3 (multiple structures).
    ///
    /// ```asm
    /// LD2 { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>], <imm>
    /// ```
    #[inline(always)]
    fn ld2_multi_structs_offset_imm(&mut self, vt: Register, vt2: Register, t: ArrSpecX, xn_sp: Register) -> T {
        emit_adv_ldr_str_off_imm(self, 1, 0b1000, t, xn_sp, &[vt, vt2])
    }


    /// [LD2 - multiple structures](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LD2--multiple-structures---Load-multiple-2-element-structures-to-two-registers-?lang=en)
    ///
    /// Load multiple 2-element structures to two registers. This instruction loads multiple 2-element structures from memory and writes the result to the two SIMD&FP registers, with de-interleaving.
    ///
    /// For an example of de-interleaving, see LD3 (multiple structures).
    ///
    /// ```asm
    /// LD2 { <Vt>.<T>, <Vt2>.<T> }, [<Xn|SP>], <Xm>
    /// ```
    #[inline(always)]
    fn ld2_multi_structs_offset_reg(&mut self, vt: Register, vt2: Register, t: ArrSpecX, xn_sp: Register, xm: Register) -> T {
        emit_adv_ldr_str_off_reg(self, 1, 0b1000, xm, t, xn_sp, &[vt, vt2])
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_st4() {
        let mut prod = TestProducer::new();

        let instr = prod.st4_multi_structs(1, 2, 3, 4, ArrSpecX::T4H, 0);
        assert_eq!(instr, "st4 {v1.4h, v2.4h, v3.4h, v4.4h}, [x0]");

        assert_panic!("Should panic: vectors not sequential"; prod.st4_multi_structs(0, 2, 3, 4, ArrSpecX::T4H, 0));

        let instr = prod.st4_multi_structs_offset_imm(1, 2, 3, 4, ArrSpecX::T2D, 0);
        assert_eq!(instr, "st4 {v1.2d, v2.2d, v3.2d, v4.2d}, [x0], #0x40");

        assert_panic!("Should panic: vectors not sequential"; prod.st4_multi_structs_offset_imm(0, 2, 3, 4, ArrSpecX::T4H, 0));

        let instr = prod.st4_multi_structs_offset_reg(1, 2, 3, 4, ArrSpecX::T8B, 0, 3);
        assert_eq!(instr, "st4 {v1.8b, v2.8b, v3.8b, v4.8b}, [x0], x3");

        assert_panic!("Should panic: vectors not sequential"; prod.st4_multi_structs_offset_reg(0, 2, 3, 4, ArrSpecX::T4H, 0, 3));
    }

    #[test]
    fn test_st1_one_reg() {
        let mut prod = TestProducer::new();

        let instr = prod.st1_multi_structs_one_reg(1, ArrSpec1::T4H, 0);
        assert_eq!(instr, "st1 {v1.4h}, [x0]");

        let instr = prod.st1_multi_structs_one_reg_offset_imm(1, ArrSpec1::T2D, 0);
        assert_eq!(instr, "st1 {v1.2d}, [x0], #0x10");

        let instr = prod.st1_multi_structs_one_reg_offset_reg(1, ArrSpec1::T8B, 0, 3);
        assert_eq!(instr, "st1 {v1.8b}, [x0], x3");
    }

    #[test]
    fn test_st1_two_reg() {
        let mut prod = TestProducer::new();

        let instr = prod.st1_multi_structs_two_reg(1, 2, ArrSpec1::T8B, 0);
        assert_eq!(instr, "st1 {v1.8b, v2.8b}, [x0]");

        assert_panic!("Should panic: vectors not sequential"; prod.st1_multi_structs_two_reg(0, 2, ArrSpec1::T4H, 0));

        let instr = prod.st1_multi_structs_two_reg_offset_imm(1, 2, ArrSpec1::T1D, 0);
        assert_eq!(instr, "st1 {v1.1d, v2.1d}, [x0], #0x10");

        assert_panic!("Should panic: vectors not sequential"; prod.st1_multi_structs_two_reg_offset_imm(0, 2, ArrSpec1::T4H, 0));

        let instr = prod.st1_multi_structs_two_reg_offset_reg(1, 2, ArrSpec1::T2S, 0, 3);
        assert_eq!(instr, "st1 {v1.2s, v2.2s}, [x0], x3");

        assert_panic!("Should panic: vectors not sequential"; prod.st1_multi_structs_two_reg_offset_reg(0, 2, ArrSpec1::T4H, 0, 3));
    }

    #[test]
    fn test_st1_three_reg() {
        let mut prod = TestProducer::new();

        let instr = prod.st1_multi_structs_three_reg(1, 2, 3, ArrSpec1::T4S, 0);
        assert_eq!(instr, "st1 {v1.4s, v2.4s, v3.4s}, [x0]");

        assert_panic!("Should panic: vectors not sequential"; prod.st1_multi_structs_three_reg(0, 2, 3, ArrSpec1::T4H, 0));

        let instr = prod.st1_multi_structs_three_reg_offset_imm(1, 2, 3, ArrSpec1::T16B, 0);
        assert_eq!(instr, "st1 {v1.16b, v2.16b, v3.16b}, [x0], #0x30");

        assert_panic!("Should panic: vectors not sequential"; prod.st1_multi_structs_three_reg_offset_imm(0, 2, 3, ArrSpec1::T4H, 0));

        let instr = prod.st1_multi_structs_three_reg_offset_reg(1, 2, 3, ArrSpec1::T8B, 0, 3);
        assert_eq!(instr, "st1 {v1.8b, v2.8b, v3.8b}, [x0], x3");

        assert_panic!("Should panic: vectors not sequential"; prod.st1_multi_structs_three_reg_offset_reg(0, 2, 3, ArrSpec1::T4H, 0, 3));
    }

    #[test]
    fn test_st1_four_reg() {
        let mut prod = TestProducer::new();

        let instr = prod.st1_multi_structs_four_reg(1, 2, 3, 4, ArrSpec1::T4H, 0);
        assert_eq!(instr, "st1 {v1.4h, v2.4h, v3.4h, v4.4h}, [x0]");

        assert_panic!("Should panic: vectors not sequential"; prod.st1_multi_structs_four_reg(0, 2, 3, 4, ArrSpec1::T4H, 0));

        let instr = prod.st1_multi_structs_four_reg_offset_imm(1, 2, 3, 4, ArrSpec1::T2D, 0);
        assert_eq!(instr, "st1 {v1.2d, v2.2d, v3.2d, v4.2d}, [x0], #0x40");

        assert_panic!("Should panic: vectors not sequential"; prod.st1_multi_structs_four_reg_offset_imm(0, 2, 3, 4, ArrSpec1::T4H, 0));

        let instr = prod.st1_multi_structs_four_reg_offset_reg(1, 2, 3, 4, ArrSpec1::T8B, 0, 3);
        assert_eq!(instr, "st1 {v1.8b, v2.8b, v3.8b, v4.8b}, [x0], x3");

        assert_panic!("Should panic: vectors not sequential"; prod.st1_multi_structs_four_reg_offset_reg(0, 2, 3, 4, ArrSpec1::T4H, 0, 3));
    }

    #[test]
    fn test_st3() {
        let mut prod = TestProducer::new();

        let instr = prod.st3_multi_structs(1, 2, 3, ArrSpecX::T4S, 0);
        assert_eq!(instr, "st3 {v1.4s, v2.4s, v3.4s}, [x0]");

        assert_panic!("Should panic: vectors not sequential"; prod.st3_multi_structs(0, 2, 3, ArrSpecX::T4H, 0));

        let instr = prod.st3_multi_structs_offset_imm(1, 2, 3, ArrSpecX::T16B, 0);
        assert_eq!(instr, "st3 {v1.16b, v2.16b, v3.16b}, [x0], #0x30");

        assert_panic!("Should panic: vectors not sequential"; prod.st3_multi_structs_offset_imm(0, 2, 3, ArrSpecX::T4H, 0));

        let instr = prod.st3_multi_structs_offset_reg(1, 2, 3, ArrSpecX::T8B, 0, 3);
        assert_eq!(instr, "st3 {v1.8b, v2.8b, v3.8b}, [x0], x3");

        assert_panic!("Should panic: vectors not sequential"; prod.st3_multi_structs_offset_reg(0, 2, 3, ArrSpecX::T4H, 0, 3));
    }

    #[test]
    fn test_st2() {
        let mut prod = TestProducer::new();

        let instr = prod.st2_multi_structs(1, 2, ArrSpecX::T8B, 0);
        assert_eq!(instr, "st2 {v1.8b, v2.8b}, [x0]");

        assert_panic!("Should panic: vectors not sequential"; prod.st2_multi_structs(0, 2, ArrSpecX::T4H, 0));

        let instr = prod.st2_multi_structs_offset_imm(1, 2, ArrSpecX::T4H, 0);
        assert_eq!(instr, "st2 {v1.4h, v2.4h}, [x0], #0x10");

        assert_panic!("Should panic: vectors not sequential"; prod.st2_multi_structs_offset_imm(0, 2, ArrSpecX::T4H, 0));

        let instr = prod.st2_multi_structs_offset_reg(1, 2, ArrSpecX::T2S, 0, 3);
        assert_eq!(instr, "st2 {v1.2s, v2.2s}, [x0], x3");

        assert_panic!("Should panic: vectors not sequential"; prod.st2_multi_structs_offset_reg(0, 2, ArrSpecX::T4H, 0, 3));
    }

    #[test]
    fn test_ld4() {
        let mut prod = TestProducer::new();

        let instr = prod.ld4_multi_structs(1, 2, 3, 4, ArrSpecX::T4H, 0);
        assert_eq!(instr, "ld4 {v1.4h, v2.4h, v3.4h, v4.4h}, [x0]");

        assert_panic!("Should panic: vectors not sequential"; prod.ld4_multi_structs(0, 2, 3, 4, ArrSpecX::T4H, 0));

        let instr = prod.ld4_multi_structs_offset_imm(1, 2, 3, 4, ArrSpecX::T2D, 0);
        assert_eq!(instr, "ld4 {v1.2d, v2.2d, v3.2d, v4.2d}, [x0], #0x40");

        assert_panic!("Should panic: vectors not sequential"; prod.ld4_multi_structs_offset_imm(0, 2, 3, 4, ArrSpecX::T4H, 0));

        let instr = prod.ld4_multi_structs_offset_reg(1, 2, 3, 4, ArrSpecX::T8B, 0, 3);
        assert_eq!(instr, "ld4 {v1.8b, v2.8b, v3.8b, v4.8b}, [x0], x3");

        assert_panic!("Should panic: vectors not sequential"; prod.ld4_multi_structs_offset_reg(0, 2, 3, 4, ArrSpecX::T4H, 0, 3));
    }

    #[test]
    fn test_ld1_one_reg() {
        let mut prod = TestProducer::new();

        let instr = prod.ld1_multi_structs_one_reg(1, ArrSpec1::T4H, 0);
        assert_eq!(instr, "ld1 {v1.4h}, [x0]");

        let instr = prod.ld1_multi_structs_one_reg_offset_imm(1, ArrSpec1::T2D, 0);
        assert_eq!(instr, "ld1 {v1.2d}, [x0], #0x10");

        let instr = prod.ld1_multi_structs_one_reg_offset_reg(1, ArrSpec1::T8B, 0, 3);
        assert_eq!(instr, "ld1 {v1.8b}, [x0], x3");
    }

    #[test]
    fn test_ld1_two_reg() {
        let mut prod = TestProducer::new();

        let instr = prod.ld1_multi_structs_two_reg(1, 2, ArrSpec1::T8B, 0);
        assert_eq!(instr, "ld1 {v1.8b, v2.8b}, [x0]");

        assert_panic!("Should panic: vectors not sequential"; prod.ld1_multi_structs_two_reg(0, 2, ArrSpec1::T4H, 0));

        let instr = prod.ld1_multi_structs_two_reg_offset_imm(1, 2, ArrSpec1::T1D, 0);
        assert_eq!(instr, "ld1 {v1.1d, v2.1d}, [x0], #0x10");

        assert_panic!("Should panic: vectors not sequential"; prod.ld1_multi_structs_two_reg_offset_imm(0, 2, ArrSpec1::T4H, 0));

        let instr = prod.ld1_multi_structs_two_reg_offset_reg(1, 2, ArrSpec1::T2S, 0, 3);
        assert_eq!(instr, "ld1 {v1.2s, v2.2s}, [x0], x3");

        assert_panic!("Should panic: vectors not sequential"; prod.ld1_multi_structs_two_reg_offset_reg(0, 2, ArrSpec1::T4H, 0, 3));
    }

    #[test]
    fn test_ld1_three_reg() {
        let mut prod = TestProducer::new();

        let instr = prod.ld1_multi_structs_three_reg(1, 2, 3, ArrSpec1::T4S, 0);
        assert_eq!(instr, "ld1 {v1.4s, v2.4s, v3.4s}, [x0]");

        assert_panic!("Should panic: vectors not sequential"; prod.ld1_multi_structs_three_reg(0, 2, 3, ArrSpec1::T4H, 0));

        let instr = prod.ld1_multi_structs_three_reg_offset_imm(1, 2, 3, ArrSpec1::T16B, 0);
        assert_eq!(instr, "ld1 {v1.16b, v2.16b, v3.16b}, [x0], #0x30");

        assert_panic!("Should panic: vectors not sequential"; prod.ld1_multi_structs_three_reg_offset_imm(0, 2, 3, ArrSpec1::T4H, 0));

        let instr = prod.ld1_multi_structs_three_reg_offset_reg(1, 2, 3, ArrSpec1::T8B, 0, 3);
        assert_eq!(instr, "ld1 {v1.8b, v2.8b, v3.8b}, [x0], x3");

        assert_panic!("Should panic: vectors not sequential"; prod.ld1_multi_structs_three_reg_offset_reg(0, 2, 3, ArrSpec1::T4H, 0, 3));
    }

    #[test]
    fn test_ld1_four_reg() {
        let mut prod = TestProducer::new();

        let instr = prod.ld1_multi_structs_four_reg(1, 2, 3, 4, ArrSpec1::T4H, 0);
        assert_eq!(instr, "ld1 {v1.4h, v2.4h, v3.4h, v4.4h}, [x0]");

        assert_panic!("Should panic: vectors not sequential"; prod.ld1_multi_structs_four_reg(0, 2, 3, 4, ArrSpec1::T4H, 0));

        let instr = prod.ld1_multi_structs_four_reg_offset_imm(1, 2, 3, 4, ArrSpec1::T2D, 0);
        assert_eq!(instr, "ld1 {v1.2d, v2.2d, v3.2d, v4.2d}, [x0], #0x40");

        assert_panic!("Should panic: vectors not sequential"; prod.ld1_multi_structs_four_reg_offset_imm(0, 2, 3, 4, ArrSpec1::T4H, 0));

        let instr = prod.ld1_multi_structs_four_reg_offset_reg(1, 2, 3, 4, ArrSpec1::T8B, 0, 3);
        assert_eq!(instr, "ld1 {v1.8b, v2.8b, v3.8b, v4.8b}, [x0], x3");

        assert_panic!("Should panic: vectors not sequential"; prod.ld1_multi_structs_four_reg_offset_reg(0, 2, 3, 4, ArrSpec1::T4H, 0, 3));
    }

    #[test]
    fn test_ld3() {
        let mut prod = TestProducer::new();

        let instr = prod.ld3_multi_structs(1, 2, 3, ArrSpecX::T4S, 0);
        assert_eq!(instr, "ld3 {v1.4s, v2.4s, v3.4s}, [x0]");

        assert_panic!("Should panic: vectors not sequential"; prod.ld3_multi_structs(0, 2, 3, ArrSpecX::T4H, 0));

        let instr = prod.ld3_multi_structs_offset_imm(1, 2, 3, ArrSpecX::T16B, 0);
        assert_eq!(instr, "ld3 {v1.16b, v2.16b, v3.16b}, [x0], #0x30");

        assert_panic!("Should panic: vectors not sequential"; prod.ld3_multi_structs_offset_imm(0, 2, 3, ArrSpecX::T4H, 0));

        let instr = prod.ld3_multi_structs_offset_reg(1, 2, 3, ArrSpecX::T8B, 0, 3);
        assert_eq!(instr, "ld3 {v1.8b, v2.8b, v3.8b}, [x0], x3");

        assert_panic!("Should panic: vectors not sequential"; prod.ld3_multi_structs_offset_reg(0, 2, 3, ArrSpecX::T4H, 0, 3));
    }

    #[test]
    fn test_ld2() {
        let mut prod = TestProducer::new();

        let instr = prod.ld2_multi_structs(1, 2, ArrSpecX::T8B, 0);
        assert_eq!(instr, "ld2 {v1.8b, v2.8b}, [x0]");

        assert_panic!("Should panic: vectors not sequential"; prod.ld2_multi_structs(0, 2, ArrSpecX::T4H, 0));

        let instr = prod.ld2_multi_structs_offset_imm(1, 2, ArrSpecX::T4H, 0);
        assert_eq!(instr, "ld2 {v1.4h, v2.4h}, [x0], #0x10");

        assert_panic!("Should panic: vectors not sequential"; prod.ld2_multi_structs_offset_imm(0, 2, ArrSpecX::T4H, 0));

        let instr = prod.ld2_multi_structs_offset_reg(1, 2, ArrSpecX::T2S, 0, 3);
        assert_eq!(instr, "ld2 {v1.2s, v2.2s}, [x0], x3");

        assert_panic!("Should panic: vectors not sequential"; prod.ld2_multi_structs_offset_reg(0, 2, ArrSpecX::T4H, 0, 3));
    }
}