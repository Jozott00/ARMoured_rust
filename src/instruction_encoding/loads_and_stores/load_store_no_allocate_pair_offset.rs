//! # [Load/store no-allocate pair (offset)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldstnapair_offs)
//!
//! Implements the following instructions:
//!  - [STNP - Store Pair of Registers - with non temporal hint](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STNP--Store-Pair-of-Registers--with-non-temporal-hint-?lang=en)
//!  - [LDNP - Load Pair of Registers - with non temporal hint](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDNP--Load-Pair-of-Registers--with-non-temporal-hint-?lang=en)
//!  - [STNP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STNP--SIMD-FP---Store-Pair-of-SIMD-FP-registers--with-Non-temporal-hint-?lang=en)
//!  - [LDNP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDNP--SIMD-FP---Load-Pair-of-SIMD-FP-registers--with-Non-temporal-hint-?lang=en)



use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Imm10, Imm11, Imm9, Register};

#[inline(always)]
fn emit_ld_st_noallo<P: InstructionProcessor<T>, T>(proc: &mut P, opc: u8, v: u8, l: u8, imm7: u8, rt2: Register, rn: Register, rt: Register) -> T {
    let r = bseq_32!(opc:2 101 v:1 000 l:1 imm7:7 rt2:5 rn:5 rt:5);
    proc.process(r)
}

#[inline(always)]
fn emit_ld_st_noallo_32<P: InstructionProcessor<T>, T>(proc: &mut P, opc: u8, v: u8, l: u8, imm: Imm9, rt2: Register, rn: Register, rt: Register) -> T {
    debug_assert!(imm % 4 == 0, "imm must be multiply of 4, was {}", imm);
    let imm = imm / 4;
    emit_ld_st_noallo(proc, opc, v, l, imm as u8, rt2, rn, rt)
}

#[inline(always)]
fn emit_ld_st_noallo_64<P: InstructionProcessor<T>, T>(proc: &mut P, opc: u8, v: u8, l: u8, imm: Imm10, rt2: Register, rn: Register, rt: Register) -> T {
    debug_assert!(imm % 8 == 0, "imm must be multiply of 8, was {}", imm);
    let imm = imm / 8;
    emit_ld_st_noallo(proc, opc, v, l, imm as u8, rt2, rn, rt)
}

#[inline(always)]
fn emit_ld_st_noallo_128<P: InstructionProcessor<T>, T>(proc: &mut P, opc: u8, v: u8, l: u8, imm: Imm11, rt2: Register, rn: Register, rt: Register) -> T {
    debug_assert!(imm % 16 == 0, "imm must be multiply of 16, was {}", imm);
    let imm = imm / 16;
    emit_ld_st_noallo(proc, opc, v, l, imm as u8, rt2, rn, rt)
}


/// # [Load/store no-allocate pair (offset)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldstnapair_offs)
///
/// Implements the following instructions:
///  - [STNP - Store Pair of Registers - with non temporal hint](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STNP--Store-Pair-of-Registers--with-non-temporal-hint-?lang=en)
///  - [LDNP - Load Pair of Registers - with non temporal hint](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDNP--Load-Pair-of-Registers--with-non-temporal-hint-?lang=en)
///  - [STNP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STNP--SIMD-FP---Store-Pair-of-SIMD-FP-registers--with-Non-temporal-hint-?lang=en)
///  - [LDNP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDNP--SIMD-FP---Load-Pair-of-SIMD-FP-registers--with-Non-temporal-hint-?lang=en)
pub trait LoadStoreNoAllocatePairOffset<T>: InstructionProcessor<T> {
    /// [STNP - Store Pair of Registers - with non temporal hint](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STNP--Store-Pair-of-Registers--with-non-temporal-hint-?lang=en)
    ///
    /// Store Pair of Registers, with non-temporal hint, calculates an address from a base register value and an immediate offset, and stores two 32-bit words or two 64-bit doublewords to the calculated address, from two registers. For information about memory accesses, see Load/Store addressing modes. For information about Non-temporal pair instructions, see Load/Store Non-temporal pair.
    ///
    /// ```asm
    /// STNP <Wt1>, <Wt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn stnp_32(&mut self, wt1: Register, wt2: Register, xn_sp: Register, imm: Imm9) -> T {
        emit_ld_st_noallo_32(self, 0, 0, 0, imm, wt2, xn_sp, wt1)
    }


    /// [STNP - Store Pair of Registers - with non temporal hint](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STNP--Store-Pair-of-Registers--with-non-temporal-hint-?lang=en)
    ///
    /// Store Pair of Registers, with non-temporal hint, calculates an address from a base register value and an immediate offset, and stores two 32-bit words or two 64-bit doublewords to the calculated address, from two registers. For information about memory accesses, see Load/Store addressing modes. For information about Non-temporal pair instructions, see Load/Store Non-temporal pair.
    ///
    /// ```asm
    /// STNP <Xt1>, <Xt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn stnp_64(&mut self, xt1: Register, xt2: Register, xn_sp: Register, imm: Imm10) -> T {
        emit_ld_st_noallo_64(self, 0b10, 0, 0, imm, xt2, xn_sp, xt1)
    }


    /// [LDNP - Load Pair of Registers - with non temporal hint](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDNP--Load-Pair-of-Registers--with-non-temporal-hint-?lang=en)
    ///
    /// Load Pair of Registers, with non-temporal hint, calculates an address from a base register value and an immediate offset, loads two 32-bit words or two 64-bit doublewords from memory, and writes them to two registers.
    ///
    /// For information about memory accesses, see Load/Store addressing modes. For information about Non-temporal pair instructions, see Load/Store Non-temporal pair.
    ///
    /// ```asm
    /// LDNP <Wt1>, <Wt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn ldnp_32(&mut self, wt1: Register, wt2: Register, xn_sp: Register, imm: Imm9) -> T {
        emit_ld_st_noallo_32(self, 0, 0, 1, imm, wt2, xn_sp, wt1)
    }


    /// [LDNP - Load Pair of Registers - with non temporal hint](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDNP--Load-Pair-of-Registers--with-non-temporal-hint-?lang=en)
    ///
    /// Load Pair of Registers, with non-temporal hint, calculates an address from a base register value and an immediate offset, loads two 32-bit words or two 64-bit doublewords from memory, and writes them to two registers.
    ///
    /// For information about memory accesses, see Load/Store addressing modes. For information about Non-temporal pair instructions, see Load/Store Non-temporal pair.
    ///
    /// ```asm
    /// LDNP <Xt1>, <Xt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn ldnp_64(&mut self, xt1: Register, xt2: Register, xn_sp: Register, imm: Imm10) -> T {
        emit_ld_st_noallo_64(self, 0b10, 0, 1, imm, xt2, xn_sp, xt1)
    }


    /// [STNP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STNP--SIMD-FP---Store-Pair-of-SIMD-FP-registers--with-Non-temporal-hint-?lang=en)
    ///
    /// Store Pair of SIMD&FP registers, with Non-temporal hint. This instruction stores a pair of SIMD&FP registers to memory, issuing a hint to the memory system that the access is non-temporal. The address used for the store is calculated from an address from a base register value and an immediate offset. For information about non-temporal pair instructions, see Load/Store SIMD and Floating-point Non-temporal pair.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STNP <St1>, <St2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn stnp_32_simd(&mut self, st1: Register, st2: Register, xn_sp: Register, imm: Imm9) -> T {
        emit_ld_st_noallo_32(self, 0, 1, 0, imm, st2, xn_sp, st1)
    }


    /// [STNP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STNP--SIMD-FP---Store-Pair-of-SIMD-FP-registers--with-Non-temporal-hint-?lang=en)
    ///
    /// Store Pair of SIMD&FP registers, with Non-temporal hint. This instruction stores a pair of SIMD&FP registers to memory, issuing a hint to the memory system that the access is non-temporal. The address used for the store is calculated from an address from a base register value and an immediate offset. For information about non-temporal pair instructions, see Load/Store SIMD and Floating-point Non-temporal pair.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STNP <Dt1>, <Dt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn stnp_64_simd(&mut self, dt1: Register, dt2: Register, xn_sp: Register, imm: Imm10) -> T {
        emit_ld_st_noallo_64(self, 0b01, 1, 0, imm, dt2, xn_sp, dt1)
    }


    /// [STNP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STNP--SIMD-FP---Store-Pair-of-SIMD-FP-registers--with-Non-temporal-hint-?lang=en)
    ///
    /// Store Pair of SIMD&FP registers, with Non-temporal hint. This instruction stores a pair of SIMD&FP registers to memory, issuing a hint to the memory system that the access is non-temporal. The address used for the store is calculated from an address from a base register value and an immediate offset. For information about non-temporal pair instructions, see Load/Store SIMD and Floating-point Non-temporal pair.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STNP <Qt1>, <Qt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn stnp_128_simd(&mut self, qt1: Register, qt2: Register, xn_sp: Register, imm: Imm11) -> T {
        emit_ld_st_noallo_128(self, 0b10, 1, 0, imm, qt2, xn_sp, qt1)
    }


    /// [LDNP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDNP--SIMD-FP---Load-Pair-of-SIMD-FP-registers--with-Non-temporal-hint-?lang=en)
    ///
    /// Load Pair of SIMD&FP registers, with Non-temporal hint. This instruction loads a pair of SIMD&FP registers from memory, issuing a hint to the memory system that the access is non-temporal. The address that is used for the load is calculated from a base register value and an optional immediate offset.
    ///
    /// For information about non-temporal pair instructions, see Load/Store SIMD and Floating-point Non-temporal pair.
    ///
    /// ```asm
    /// LDNP <St1>, <St2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn ldnp_32_simd(&mut self, st1: Register, st2: Register, xn_sp: Register, imm: Imm9) -> T {
        emit_ld_st_noallo_32(self, 0, 1, 1, imm, st2, xn_sp, st1)
    }


    /// [LDNP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDNP--SIMD-FP---Load-Pair-of-SIMD-FP-registers--with-Non-temporal-hint-?lang=en)
    ///
    /// Load Pair of SIMD&FP registers, with Non-temporal hint. This instruction loads a pair of SIMD&FP registers from memory, issuing a hint to the memory system that the access is non-temporal. The address that is used for the load is calculated from a base register value and an optional immediate offset.
    ///
    /// For information about non-temporal pair instructions, see Load/Store SIMD and Floating-point Non-temporal pair.
    ///
    /// ```asm
    /// LDNP <Dt1>, <Dt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn ldnp_64_simd(&mut self, dt1: Register, dt2: Register, xn_sp: Register, imm: Imm10) -> T {
        emit_ld_st_noallo_64(self, 0b01, 1, 1, imm, dt2, xn_sp, dt1)
    }


    /// [LDNP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDNP--SIMD-FP---Load-Pair-of-SIMD-FP-registers--with-Non-temporal-hint-?lang=en)
    ///
    /// Load Pair of SIMD&FP registers, with Non-temporal hint. This instruction loads a pair of SIMD&FP registers from memory, issuing a hint to the memory system that the access is non-temporal. The address that is used for the load is calculated from a base register value and an optional immediate offset.
    ///
    /// For information about non-temporal pair instructions, see Load/Store SIMD and Floating-point Non-temporal pair.
    ///
    /// ```asm
    /// LDNP <Qt1>, <Qt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn ldnp_128_simd(&mut self, qt1: Register, qt2: Register, xn_sp: Register, imm: Imm11) -> T {
        emit_ld_st_noallo_128(self, 0b10, 1, 1, imm, qt2, xn_sp, qt1)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_stnp() {
        let mut prod = TestProducer::new();

        let instr = prod.stnp_32(2, 3, 4, -256);
        assert_eq!(instr, "stnp w2, w3, [x4, #0xffffffffffffff00]");

        assert_panic!("Should panic: not right multiply"; prod.stnp_32(2, 3, 4, 3));

        let instr = prod.stnp_64(2, 3, 4, -512);
        assert_eq!(instr, "stnp x2, x3, [x4, #0xfffffffffffffe00]");

        assert_panic!("Should panic: not right multiply"; prod.stnp_64(2, 3, 4, 4));
    }

    #[test]
    fn test_ldnp() {
        let mut prod = TestProducer::new();

        let instr = prod.ldnp_32(2, 3, 4, -256);
        assert_eq!(instr, "ldnp w2, w3, [x4, #0xffffffffffffff00]");

        assert_panic!("Should panic: not right multiply"; prod.ldnp_32(2, 3, 4, 3));

        let instr = prod.ldnp_64(2, 3, 4, -512);
        assert_eq!(instr, "ldnp x2, x3, [x4, #0xfffffffffffffe00]");

        assert_panic!("Should panic: not right multiply"; prod.ldnp_64(2, 3, 4, 4));
    }

    #[test]
    fn test_stnp_simd() {
        let mut prod = TestProducer::new();

        let instr = prod.stnp_32_simd(2, 3, 4, -256);
        assert_eq!(instr, "stnp s2, s3, [x4, #0xffffffffffffff00]");

        assert_panic!("Should panic: not right multiply"; prod.stnp_32_simd(2, 3, 4, 3));

        let instr = prod.stnp_64_simd(2, 3, 4, -512);
        assert_eq!(instr, "stnp d2, d3, [x4, #0xfffffffffffffe00]");

        assert_panic!("Should panic: not right multiply"; prod.stnp_64_simd(2, 3, 4, 4));


        let instr = prod.stnp_128_simd(2, 3, 4, -1024);
        assert_eq!(instr, "stnp q2, q3, [x4, #0xfffffffffffffc00]");

        assert_panic!("Should panic: not right multiply"; prod.stnp_128_simd(2, 3, 4, 8));
    }
}
