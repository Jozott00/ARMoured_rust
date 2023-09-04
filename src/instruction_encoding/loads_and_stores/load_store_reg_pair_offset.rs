//! # [Load/store register pair (offset)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldstpair_off)
//!
//! Implements the following instructions:
//!  - [STP - Store Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STP--Store-Pair-of-Registers-?lang=en)
//!  - [LDP - Load Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDP--Load-Pair-of-Registers-?lang=en)
//!  - [STP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STP--SIMD-FP---Store-Pair-of-SIMD-FP-registers-?lang=en)
//!  - [LDP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDP--SIMD-FP---Load-Pair-of-SIMD-FP-registers-?lang=en)
//!  - [STGP - Store Allocation Tag and Pair of registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STGP--Store-Allocation-Tag-and-Pair-of-registers-?lang=en)
//!  - [LDPSW - Load Pair of Registers Signed Word](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDPSW--Load-Pair-of-Registers-Signed-Word-?lang=en)



use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Imm10, Imm11, Imm9, Register};

#[inline(always)]
fn emit_ld_st_reg_offset<P: InstructionProcessor<T>, T>(proc: &mut P, opc: u8, v: u8, l: u8, imm7: u8, rt2: Register, rn: Register, rt: Register) -> T {
    let r = bseq_32!(opc:2 101 v:1 010 l:1 imm7:7 rt2:5 rn:5 rt:5);
    proc.process(r)
}

#[inline(always)]
fn emit_ld_st_reg_offset_32<P: InstructionProcessor<T>, T>(proc: &mut P, opc: u8, v: u8, l: u8, imm: Imm9, rt2: Register, rn: Register, rt: Register) -> T {
    debug_assert!(imm % 4 == 0, "imm must be multiply of 4, was {}", imm);
    let imm = imm / 4;
    emit_ld_st_reg_offset(proc, opc, v, l, imm as u8, rt2, rn, rt)
}

#[inline(always)]
fn emit_ld_st_reg_offset_64<P: InstructionProcessor<T>, T>(proc: &mut P, opc: u8, v: u8, l: u8, imm: Imm10, rt2: Register, rn: Register, rt: Register) -> T {
    debug_assert!(imm % 8 == 0, "imm must be multiply of 8, was {}", imm);
    let imm = imm / 8;
    emit_ld_st_reg_offset(proc, opc, v, l, imm as u8, rt2, rn, rt)
}

#[inline(always)]
fn emit_ld_st_reg_offset_128<P: InstructionProcessor<T>, T>(proc: &mut P, opc: u8, v: u8, l: u8, imm: Imm11, rt2: Register, rn: Register, rt: Register) -> T {
    debug_assert!(imm % 16 == 0, "imm must be multiply of 16, was {}", imm);
    let imm = imm / 16;
    emit_ld_st_reg_offset(proc, opc, v, l, imm as u8, rt2, rn, rt)
}


/// # [Load/store register pair (offset)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldstpair_off)
///
/// Implements the following instructions:
///  - [STP - Store Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STP--Store-Pair-of-Registers-?lang=en)
///  - [LDP - Load Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDP--Load-Pair-of-Registers-?lang=en)
///  - [STP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STP--SIMD-FP---Store-Pair-of-SIMD-FP-registers-?lang=en)
///  - [LDP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDP--SIMD-FP---Load-Pair-of-SIMD-FP-registers-?lang=en)
///  - [STGP - Store Allocation Tag and Pair of registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STGP--Store-Allocation-Tag-and-Pair-of-registers-?lang=en)
///  - [LDPSW - Load Pair of Registers Signed Word](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDPSW--Load-Pair-of-Registers-Signed-Word-?lang=en)
pub trait LoadStoreRegisterPairOffset<T>: InstructionProcessor<T> {
    /// [STP - Store Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STP--Store-Pair-of-Registers-?lang=en)
    ///
    /// Store Pair of Registers calculates an address from a base register value and an immediate offset, and stores two 32-bit words or two 64-bit doublewords to the calculated address, from two registers. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Signed offset
    ///
    /// ```asm
    /// STP <Wt1>, <Wt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn stp_32_offset(&mut self, wt1: Register, wt2: Register, xn_sp: Register, imm: Imm9) -> T {
        emit_ld_st_reg_offset_32(self, 0b00, 0, 0, imm, wt2, xn_sp, wt1)
    }


    /// [STP - Store Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STP--Store-Pair-of-Registers-?lang=en)
    ///
    /// Store Pair of Registers calculates an address from a base register value and an immediate offset, and stores two 32-bit words or two 64-bit doublewords to the calculated address, from two registers. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Signed offset
    ///
    /// ```asm
    /// STP <Xt1>, <Xt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn stp_64_offset(&mut self, xt1: Register, xt2: Register, xn_sp: Register, imm: Imm10) -> T {
        emit_ld_st_reg_offset_64(self, 0b10, 0, 0, imm, xt2, xn_sp, xt1)
    }


    /// [LDP - Load Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDP--Load-Pair-of-Registers-?lang=en)
    ///
    /// Load Pair of Registers calculates an address from a base register value and an immediate offset, loads two 32-bit words or two 64-bit doublewords from memory, and writes them to two registers. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Signed offset
    ///
    /// ```asm
    /// LDP <Wt1>, <Wt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn ldp_32_offset(&mut self, wt1: Register, wt2: Register, xn_sp: Register, imm: Imm9) -> T {
        emit_ld_st_reg_offset_32(self, 0b00, 0, 1, imm, wt2, xn_sp, wt1)
    }


    /// [LDP - Load Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDP--Load-Pair-of-Registers-?lang=en)
    ///
    /// Load Pair of Registers calculates an address from a base register value and an immediate offset, loads two 32-bit words or two 64-bit doublewords from memory, and writes them to two registers. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Signed offset
    ///
    /// ```asm
    /// LDP <Xt1>, <Xt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn ldp_64_offset(&mut self, xt1: Register, xt2: Register, xn_sp: Register, imm: Imm10) -> T {
        emit_ld_st_reg_offset_64(self, 0b10, 0, 1, imm, xt2, xn_sp, xt1)
    }


    /// [STP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STP--SIMD-FP---Store-Pair-of-SIMD-FP-registers-?lang=en)
    ///
    /// Store Pair of SIMD&FP registers. This instruction stores a pair of SIMD&FP registers to memory. The address used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STP <St1>, <St2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn stp_32_simd_offset(&mut self, st1: Register, st2: Register, xn_sp: Register, imm: Imm9) -> T {
        emit_ld_st_reg_offset_32(self, 0b00, 1, 0, imm, st2, xn_sp, st1)
    }


    /// [STP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STP--SIMD-FP---Store-Pair-of-SIMD-FP-registers-?lang=en)
    ///
    /// Store Pair of SIMD&FP registers. This instruction stores a pair of SIMD&FP registers to memory. The address used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STP <Dt1>, <Dt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn stp_64_simd_offset(&mut self, dt1: Register, dt2: Register, xn_sp: Register, imm: Imm10) -> T {
        emit_ld_st_reg_offset_64(self, 0b01, 1, 0, imm, dt2, xn_sp, dt1)
    }

    /// [STP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/STP--SIMD-FP---Store-Pair-of-SIMD-FP-registers-?lang=en)
    ///
    /// Store Pair of SIMD&FP registers. This instruction stores a pair of SIMD&FP registers to memory. The address used for the store is calculated from a base register value and an immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// STP <Qt1>, <Qt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn stp_128_simd_offset(&mut self, qt1: Register, qt2: Register, xn_sp: Register, imm: Imm11) -> T {
        emit_ld_st_reg_offset_128(self, 0b10, 1, 0, imm, qt2, xn_sp, qt1)
    }

    /// [LDP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDP--SIMD-FP---Load-Pair-of-SIMD-FP-registers-?lang=en)
    ///
    /// Load Pair of SIMD&FP registers. This instruction loads a pair of SIMD&FP registers from memory. The address that is used for the load is calculated from a base register value and an optional immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDP <St1>, <St2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn ldp_32_simd_offset(&mut self, st1: Register, st2: Register, xn_sp: Register, imm: Imm9) -> T {
        emit_ld_st_reg_offset_32(self, 0b00, 1, 1, imm, st2, xn_sp, st1)
    }


    /// [LDP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDP--SIMD-FP---Load-Pair-of-SIMD-FP-registers-?lang=en)
    ///
    /// Load Pair of SIMD&FP registers. This instruction loads a pair of SIMD&FP registers from memory. The address that is used for the load is calculated from a base register value and an optional immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDP <Dt1>, <Dt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn ldp_64_simd_offset(&mut self, dt1: Register, dt2: Register, xn_sp: Register, imm: Imm10) -> T {
        emit_ld_st_reg_offset_64(self, 0b01, 1, 1, imm, dt2, xn_sp, dt1)
    }


    /// [LDP - SIMD FP](https://developer.arm.com/documentation/ddi0596/2021-12/SIMD-FP-Instructions/LDP--SIMD-FP---Load-Pair-of-SIMD-FP-registers-?lang=en)
    ///
    /// Load Pair of SIMD&FP registers. This instruction loads a pair of SIMD&FP registers from memory. The address that is used for the load is calculated from a base register value and an optional immediate offset.
    ///
    /// Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.
    ///
    /// ```asm
    /// LDP <Qt1>, <Qt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn ldp_128_simd_offset(&mut self, qt1: Register, qt2: Register, xn_sp: Register, imm: Imm11) -> T {
        emit_ld_st_reg_offset_128(self, 0b10, 1, 1, imm, qt2, xn_sp, qt1)
    }

    /// [STGP - Store Allocation Tag and Pair of registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STGP--Store-Allocation-Tag-and-Pair-of-registers-?lang=en)
    ///
    /// Store Allocation Tag and Pair of registers stores an Allocation Tag and two 64-bit doublewords to memory, from two registers. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the base register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STGP <Xt1>, <Xt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn stgp_offset(&mut self, xt1: Register, xt2: Register, xn_sp: Register, imm: Imm11) -> T {
        emit_ld_st_reg_offset_128(self, 0b01, 0, 0, imm, xt2, xn_sp, xt1)
    }


    /// [LDPSW - Load Pair of Registers Signed Word](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDPSW--Load-Pair-of-Registers-Signed-Word-?lang=en)
    ///
    /// Load Pair of Registers Signed Word calculates an address from a base register value and an immediate offset, loads two 32-bit words from memory, sign-extends them, and writes them to two registers. For information about memory accesses, see Load/Store addressing modes.
    ///
    ///  It has encodings from 3 classes: Post-index , Pre-index and Signed offset
    ///
    /// ```asm
    /// LDPSW <Xt1>, <Xt2>, [<Xn|SP>{, #<imm>}]
    /// ```
    #[inline(always)]
    fn ldpsw_offset(&mut self, xt1: Register, xt2: Register, xn_sp: Register, imm: Imm9) -> T {
        emit_ld_st_reg_offset_32(self, 0b01, 0, 1, imm, xt2, xn_sp, xt1)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_stgp() {
        let mut prod = TestProducer::new();

        let instr = prod.stgp_offset(2, 3, 4, -1024);
        assert_eq!(instr, "stgp x2, x3, [x4, #0xfffffffffffffc00]");
        assert_panic!("Should panic: not right multiply"; prod.stgp_offset(2, 3, 4, 8));
    }

    #[test]
    fn test_ldpsw() {
        let mut prod = TestProducer::new();

        let instr = prod.ldpsw_offset(2, 3, 4, -256);
        assert_eq!(instr, "ldpsw x2, x3, [x4, #0xffffffffffffff00]");
        assert_panic!("Should panic: not right multiply"; prod.ldpsw_offset(2, 3, 4, 2));
    }

    #[test]
    fn test_stp() {
        let mut prod = TestProducer::new();

        let instr = prod.stp_32_offset(2, 3, 4, -256);
        assert_eq!(instr, "stp w2, w3, [x4, #0xffffffffffffff00]");
        assert_panic!("Should panic: not right multiply"; prod.stp_32_offset(2, 3, 4, 3));

        let instr = prod.stp_64_offset(2, 3, 4, -512);
        assert_eq!(instr, "stp x2, x3, [x4, #0xfffffffffffffe00]");
        assert_panic!("Should panic: not right multiply"; prod.stp_64_offset(2, 3, 4, 4));
    }

    #[test]
    fn test_ldp() {
        let mut prod = TestProducer::new();

        let instr = prod.ldp_32_offset(2, 3, 4, -256);
        assert_eq!(instr, "ldp w2, w3, [x4, #0xffffffffffffff00]");
        assert_panic!("Should panic: not right multiply"; prod.ldp_32_offset(2, 3, 4, 3));

        let instr = prod.ldp_64_offset(2, 3, 4, -512);
        assert_eq!(instr, "ldp x2, x3, [x4, #0xfffffffffffffe00]");
        assert_panic!("Should panic: not right multiply"; prod.ldp_64_offset(2, 3, 4, 4));
    }

    #[test]
    fn test_stp_simd() {
        let mut prod = TestProducer::new();

        let instr = prod.stp_32_simd_offset(2, 3, 4, -256);
        assert_eq!(instr, "stp s2, s3, [x4, #0xffffffffffffff00]");
        assert_panic!("Should panic: not right multiply"; prod.stp_32_offset(2, 3, 4, 3));

        let instr = prod.stp_64_simd_offset(2, 3, 4, -512);
        assert_eq!(instr, "stp d2, d3, [x4, #0xfffffffffffffe00]");
        assert_panic!("Should panic: not right multiply"; prod.stp_64_offset(2, 3, 4, 4));

        let instr = prod.stp_128_simd_offset(2, 3, 4, -1024);
        assert_eq!(instr, "stp q2, q3, [x4, #0xfffffffffffffc00]");
        assert_panic!("Should panic: not right multiply"; prod.stp_128_simd_offset(2, 3, 4, 8));
    }

    #[test]
    fn test_ldp_simd() {
        let mut prod = TestProducer::new();

        let instr = prod.ldp_32_simd_offset(2, 3, 4, -256);
        assert_eq!(instr, "ldp s2, s3, [x4, #0xffffffffffffff00]");
        assert_panic!("Should panic: not right multiply"; prod.ldp_32_offset(2, 3, 4, 3));

        let instr = prod.ldp_64_simd_offset(2, 3, 4, -512);
        assert_eq!(instr, "ldp d2, d3, [x4, #0xfffffffffffffe00]");
        assert_panic!("Should panic: not right multiply"; prod.ldp_64_offset(2, 3, 4, 4));

        let instr = prod.ldp_128_simd_offset(2, 3, 4, -1024);
        assert_eq!(instr, "ldp q2, q3, [x4, #0xfffffffffffffc00]");
        assert_panic!("Should panic: not right multiply"; prod.ldp_128_simd_offset(2, 3, 4, 8));
    }
}
