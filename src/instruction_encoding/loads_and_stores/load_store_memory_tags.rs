//! # [Load/store memory tags](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldsttags)
//!
//! Implements the following instructions:
//!  - [STG - Store Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STG--Store-Allocation-Tag-?lang=en)
//!  - [STZGM - Store Tag and Zero Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZGM--Store-Tag-and-Zero-Multiple-?lang=en)
//!  - [LDG - Load Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDG--Load-Allocation-Tag-?lang=en)
//!  - [STZG - Store Allocation Tag - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZG--Store-Allocation-Tag--Zeroing-?lang=en)
//!  - [ST2G - Store Allocation Tags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST2G--Store-Allocation-Tags-?lang=en)
//!  - [STGM - Store Tag Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STGM--Store-Tag-Multiple-?lang=en)
//!  - [STZ2G - Store Allocation Tags - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZ2G--Store-Allocation-Tags--Zeroing-?lang=en)
//!  - [LDGM - Load Tag Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDGM--Load-Tag-Multiple-?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::constants::LOG2_TAG_GRANULE;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Imm13, Register};

#[inline(always)]
fn emit_ldr_str_enc<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    opc: u8,
    imm9: u16,
    op2: u8,
    rn: Register,
    rt: Register,
) -> T {
    let r = bseq_32!(11011001 opc:2 1 imm9:9 op2:2 rn:5 rt:5);
    proc.process(r)
}

#[inline(always)]
fn emit_ldr_str_mem_tags<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    opc: u8,
    imm13: Imm13,
    op2: u8,
    rn: Register,
    rt: Register,
) -> T {
    debug_assert!(
        imm13 % 16 == 0,
        "imm13 must be a multiply of 16, was {}",
        imm13
    );
    let imm13 = imm13 >> LOG2_TAG_GRANULE;
    emit_ldr_str_enc(proc, opc, imm13 as u16, op2, rn, rt)
}

/// # [Load/store memory tags](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldsttags)
///
/// Implements the following instructions:
///  - [STG - Store Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STG--Store-Allocation-Tag-?lang=en)
///  - [STZGM - Store Tag and Zero Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZGM--Store-Tag-and-Zero-Multiple-?lang=en)
///  - [LDG - Load Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDG--Load-Allocation-Tag-?lang=en)
///  - [STZG - Store Allocation Tag - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZG--Store-Allocation-Tag--Zeroing-?lang=en)
///  - [ST2G - Store Allocation Tags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST2G--Store-Allocation-Tags-?lang=en)
///  - [STGM - Store Tag Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STGM--Store-Tag-Multiple-?lang=en)
///  - [STZ2G - Store Allocation Tags - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZ2G--Store-Allocation-Tags--Zeroing-?lang=en)
///  - [LDGM - Load Tag Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDGM--Load-Tag-Multiple-?lang=en)
pub trait LoadStoreMemoryTags<T>: InstructionProcessor<T> {
    /// [STG - Store Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STG--Store-Allocation-Tag-?lang=en)
    ///
    /// Store Allocation Tag stores an Allocation Tag to memory. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STG <Xt|SP>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn stg_post_index(&mut self, xt_sp: Register, xn_sp: Register, simm: Imm13) -> T {
        emit_ldr_str_mem_tags(self, 0b00, simm, 0b01, xn_sp, xt_sp)
    }

    /// [STG - Store Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STG--Store-Allocation-Tag-?lang=en)
    ///
    /// Store Allocation Tag stores an Allocation Tag to memory. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STG <Xt|SP>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn stg_pre_index(&mut self, xt_sp: Register, xn_sp: Register, simm: Imm13) -> T {
        emit_ldr_str_mem_tags(self, 0b00, simm, 0b11, xn_sp, xt_sp)
    }

    /// [STG - Store Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STG--Store-Allocation-Tag-?lang=en)
    ///
    /// Store Allocation Tag stores an Allocation Tag to memory. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STG <Xt|SP>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stg_signed_offset(&mut self, xt_sp: Register, xn_sp: Register, simm: Imm13) -> T {
        emit_ldr_str_mem_tags(self, 0b00, simm, 0b10, xn_sp, xt_sp)
    }

    /// [STZGM - Store Tag and Zero Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZGM--Store-Tag-and-Zero-Multiple-?lang=en)
    ///
    /// Store Tag and Zero Multiple writes a naturally aligned block of N Allocation Tags and stores zero to the associated data locations, where the size of N is identified in DCZID_EL0.BS, and the Allocation Tag written to address A is taken from the source register bits<3:0>.
    ///
    /// This instruction is undefined at EL0.
    ///
    /// ```asm
    /// STZGM <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn stzgm(&mut self, xt: Register, xn_sp: Register) -> T {
        emit_ldr_str_mem_tags(self, 0b00, 0, 0b00, xn_sp, xt)
    }

    /// [LDG - Load Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDG--Load-Allocation-Tag-?lang=en)
    ///
    /// Load Allocation Tag loads an Allocation Tag from a memory address, generates a Logical Address Tag from the Allocation Tag and merges it into the destination register. The address used for the load is calculated from the base register and an immediate signed offset scaled by the Tag granule.
    ///
    /// ```asm
    /// LDG <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldg(&mut self, xt: Register, xn_sp: Register, simm: Imm13) -> T {
        emit_ldr_str_mem_tags(self, 0b01, simm, 0b00, xn_sp, xt)
    }

    /// [STZG - Store Allocation Tag - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZG--Store-Allocation-Tag--Zeroing-?lang=en)
    ///
    /// Store Allocation Tag, Zeroing stores an Allocation Tag to memory, zeroing the associated data location. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STZG <Xt|SP>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn stzg_post_index(&mut self, xt_sp: Register, xn_sp: Register, simm: Imm13) -> T {
        emit_ldr_str_mem_tags(self, 0b01, simm, 0b01, xn_sp, xt_sp)
    }

    /// [STZG - Store Allocation Tag - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZG--Store-Allocation-Tag--Zeroing-?lang=en)
    ///
    /// Store Allocation Tag, Zeroing stores an Allocation Tag to memory, zeroing the associated data location. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STZG <Xt|SP>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn stzg_pre_index(&mut self, xt_sp: Register, xn_sp: Register, simm: Imm13) -> T {
        emit_ldr_str_mem_tags(self, 0b01, simm, 0b11, xn_sp, xt_sp)
    }

    /// [STZG - Store Allocation Tag - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZG--Store-Allocation-Tag--Zeroing-?lang=en)
    ///
    /// Store Allocation Tag, Zeroing stores an Allocation Tag to memory, zeroing the associated data location. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STZG <Xt|SP>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stzg_signed_offset(&mut self, xt_sp: Register, xn_sp: Register, simm: Imm13) -> T {
        emit_ldr_str_mem_tags(self, 0b01, simm, 0b10, xn_sp, xt_sp)
    }

    /// [ST2G - Store Allocation Tags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST2G--Store-Allocation-Tags-?lang=en)
    ///
    /// Store Allocation Tags stores an Allocation Tag to two Tag granules of memory. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// ST2G <Xt|SP>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn st2g_post_index(&mut self, xt_sp: Register, xn_sp: Register, simm: Imm13) -> T {
        emit_ldr_str_mem_tags(self, 0b10, simm, 0b01, xn_sp, xt_sp)
    }

    /// [ST2G - Store Allocation Tags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST2G--Store-Allocation-Tags-?lang=en)
    ///
    /// Store Allocation Tags stores an Allocation Tag to two Tag granules of memory. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// ST2G <Xt|SP>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn st2g_pre_index(&mut self, xt_sp: Register, xn_sp: Register, simm: Imm13) -> T {
        emit_ldr_str_mem_tags(self, 0b10, simm, 0b11, xn_sp, xt_sp)
    }

    /// [ST2G - Store Allocation Tags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST2G--Store-Allocation-Tags-?lang=en)
    ///
    /// Store Allocation Tags stores an Allocation Tag to two Tag granules of memory. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// ST2G <Xt|SP>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn st2g_signed_offset(&mut self, xt_sp: Register, xn_sp: Register, simm: Imm13) -> T {
        emit_ldr_str_mem_tags(self, 0b10, simm, 0b10, xn_sp, xt_sp)
    }

    /// [STGM - Store Tag Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STGM--Store-Tag-Multiple-?lang=en)
    ///
    /// Store Tag Multiple writes a naturally aligned block of N Allocation Tags, where the size of N is identified in GMID_EL1.BS, and the Allocation Tag written to address A is taken from the source register at 4*A<7:4>+3:4*A<7:4>.
    ///
    /// This instruction is undefined at EL0.
    ///
    /// ```asm
    /// STGM <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn stgm(&mut self, xt: Register, xn_sp: Register) -> T {
        emit_ldr_str_mem_tags(self, 0b10, 0, 0b00, xn_sp, xt)
    }

    /// [STZ2G - Store Allocation Tags - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZ2G--Store-Allocation-Tags--Zeroing-?lang=en)
    ///
    /// Store Allocation Tags, Zeroing stores an Allocation Tag to two Tag granules of memory, zeroing the associated data locations. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STZ2G <Xt|SP>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn stz2g_post_index(&mut self, xt_sp: Register, xn_sp: Register, simm: Imm13) -> T {
        emit_ldr_str_mem_tags(self, 0b11, simm, 0b01, xn_sp, xt_sp)
    }

    /// [STZ2G - Store Allocation Tags - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZ2G--Store-Allocation-Tags--Zeroing-?lang=en)
    ///
    /// Store Allocation Tags, Zeroing stores an Allocation Tag to two Tag granules of memory, zeroing the associated data locations. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STZ2G <Xt|SP>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn stz2g_pre_index(&mut self, xt_sp: Register, xn_sp: Register, simm: Imm13) -> T {
        emit_ldr_str_mem_tags(self, 0b11, simm, 0b11, xn_sp, xt_sp)
    }

    /// [STZ2G - Store Allocation Tags - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZ2G--Store-Allocation-Tags--Zeroing-?lang=en)
    ///
    /// Store Allocation Tags, Zeroing stores an Allocation Tag to two Tag granules of memory, zeroing the associated data locations. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STZ2G <Xt|SP>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stz2g_signed_offset(&mut self, xt_sp: Register, xn_sp: Register, simm: Imm13) -> T {
        emit_ldr_str_mem_tags(self, 0b11, simm, 0b10, xn_sp, xt_sp)
    }

    /// [LDGM - Load Tag Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDGM--Load-Tag-Multiple-?lang=en)
    ///
    /// Load Tag Multiple reads a naturally aligned block of N Allocation Tags, where the size of N is identified in GMID_EL1.BS, and writes the Allocation Tag read from address A to the destination register at 4*A<7:4>+3:4*A<7:4>. Bits of the destination register not written with an Allocation Tag are set to 0.
    ///
    /// This instruction is undefined at EL0.
    ///
    /// ```asm
    /// LDGM <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn ldgm(&mut self, xt: Register, xn_sp: Register) -> T {
        emit_ldr_str_mem_tags(self, 0b11, 0, 0b00, xn_sp, xt)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_stg_post_index() {
        let mut prod = TestProducer::new();

        let instr = prod.stg_post_index(3, 4, -4096);
        assert_eq!(instr, "stg x3, [x4], #0xfffffffffffff000");

        let instr = prod.stg_post_index(3, 4, 4080);
        assert_eq!(instr, "stg x3, [x4], #0xff0");

        assert_panic!("Should panic: not multiply"; prod.stg_post_index(3, 4, 15) )
    }

    #[test]
    fn test_stg_pre_index() {
        let mut prod = TestProducer::new();

        let instr = prod.stg_pre_index(3, 4, -4096);
        assert_eq!(instr, "stg x3, [x4, #0xfffffffffffff000]!");

        let instr = prod.stg_pre_index(3, 4, 4080);
        assert_eq!(instr, "stg x3, [x4, #0xff0]!");

        assert_panic!("Should panic: not multiply"; prod.stg_pre_index(3, 4, 15) )
    }

    #[test]
    fn test_stg_signed_offset() {
        let mut prod = TestProducer::new();

        let instr = prod.stg_signed_offset(3, 4, -4096);
        assert_eq!(instr, "stg x3, [x4, #0xfffffffffffff000]");

        let instr = prod.stg_signed_offset(3, 4, 4080);
        assert_eq!(instr, "stg x3, [x4, #0xff0]");

        assert_panic!("Should panic: not multiply"; prod.stg_signed_offset(3, 4, 15) )
    }

    #[test]
    fn test_stzgm() {
        let mut prod = TestProducer::new();
        let instr = prod.stzgm(3, 4);
        assert_eq!(instr, "stzgm x3, [x4]");
    }

    #[test]
    fn test_ldg() {
        let mut prod = TestProducer::new();

        let instr = prod.ldg(3, 4, -4096);
        assert_eq!(instr, "ldg x3, [x4, #0xfffffffffffff000]");

        let instr = prod.ldg(3, 4, 4080);
        assert_eq!(instr, "ldg x3, [x4, #0xff0]");

        assert_panic!("Should panic: not multiply"; prod.ldg(3, 4, 15) )
    }

    #[test]
    fn test_stzg_post_index() {
        let mut prod = TestProducer::new();

        let instr = prod.stzg_post_index(3, 4, -4096);
        assert_eq!(instr, "stzg x3, [x4], #0xfffffffffffff000");

        let instr = prod.stzg_post_index(3, 4, 4080);
        assert_eq!(instr, "stzg x3, [x4], #0xff0");

        assert_panic!("Should panic: not multiply"; prod.stzg_post_index(3, 4, 15) )
    }

    #[test]
    fn test_stzg_pre_index() {
        let mut prod = TestProducer::new();

        let instr = prod.stzg_pre_index(3, 4, -4096);
        assert_eq!(instr, "stzg x3, [x4, #0xfffffffffffff000]!");

        let instr = prod.stzg_pre_index(3, 4, 4080);
        assert_eq!(instr, "stzg x3, [x4, #0xff0]!");

        assert_panic!("Should panic: not multiply"; prod.stzg_pre_index(3, 4, 15) )
    }

    #[test]
    fn test_stzg_signed_offset() {
        let mut prod = TestProducer::new();

        let instr = prod.stzg_signed_offset(3, 4, -4096);
        assert_eq!(instr, "stzg x3, [x4, #0xfffffffffffff000]");

        let instr = prod.stzg_signed_offset(3, 4, 4080);
        assert_eq!(instr, "stzg x3, [x4, #0xff0]");

        assert_panic!("Should panic: not multiply"; prod.stzg_signed_offset(3, 4, 15) )
    }

    #[test]
    fn test_st2g_post_index() {
        let mut prod = TestProducer::new();

        let instr = prod.st2g_post_index(3, 4, -4096);
        assert_eq!(instr, "st2g x3, [x4], #0xfffffffffffff000");

        let instr = prod.st2g_post_index(3, 4, 4080);
        assert_eq!(instr, "st2g x3, [x4], #0xff0");

        assert_panic!("Should panic: not multiply"; prod.st2g_post_index(3, 4, 15) )
    }

    #[test]
    fn test_st2g_pre_index() {
        let mut prod = TestProducer::new();

        let instr = prod.st2g_pre_index(3, 4, -4096);
        assert_eq!(instr, "st2g x3, [x4, #0xfffffffffffff000]!");

        let instr = prod.st2g_pre_index(3, 4, 4080);
        assert_eq!(instr, "st2g x3, [x4, #0xff0]!");

        assert_panic!("Should panic: not multiply"; prod.st2g_pre_index(3, 4, 15) )
    }

    #[test]
    fn test_st2g_signed_offset() {
        let mut prod = TestProducer::new();

        let instr = prod.st2g_signed_offset(3, 4, -4096);
        assert_eq!(instr, "st2g x3, [x4, #0xfffffffffffff000]");

        let instr = prod.st2g_signed_offset(3, 4, 4080);
        assert_eq!(instr, "st2g x3, [x4, #0xff0]");

        assert_panic!("Should panic: not multiply"; prod.st2g_signed_offset(3, 4, 15) )
    }

    #[test]
    fn test_stgm() {
        let mut prod = TestProducer::new();
        let instr = prod.stgm(3, 4);
        assert_eq!(instr, "stgm x3, [x4]");
    }

    #[test]
    fn test_stz2g_post_index() {
        let mut prod = TestProducer::new();

        let instr = prod.stz2g_post_index(3, 4, -4096);
        assert_eq!(instr, "stz2g x3, [x4], #0xfffffffffffff000");

        let instr = prod.stz2g_post_index(3, 4, 4080);
        assert_eq!(instr, "stz2g x3, [x4], #0xff0");

        assert_panic!("Should panic: not multiply"; prod.stz2g_post_index(3, 4, 15) )
    }

    #[test]
    fn test_stz2g_pre_index() {
        let mut prod = TestProducer::new();

        let instr = prod.stz2g_pre_index(3, 4, -4096);
        assert_eq!(instr, "stz2g x3, [x4, #0xfffffffffffff000]!");

        let instr = prod.stz2g_pre_index(3, 4, 4080);
        assert_eq!(instr, "stz2g x3, [x4, #0xff0]!");

        assert_panic!("Should panic: not multiply"; prod.stz2g_pre_index(3, 4, 15) )
    }

    #[test]
    fn test_stz2g_signed_offset() {
        let mut prod = TestProducer::new();

        let instr = prod.stz2g_signed_offset(3, 4, -4096);
        assert_eq!(instr, "stz2g x3, [x4, #0xfffffffffffff000]");

        let instr = prod.stz2g_signed_offset(3, 4, 4080);
        assert_eq!(instr, "stz2g x3, [x4, #0xff0]");

        assert_panic!("Should panic: not multiply"; prod.stz2g_signed_offset(3, 4, 15) )
    }

    #[test]
    fn test_ldgm() {
        let mut prod = TestProducer::new();
        let instr = prod.ldgm(3, 4);
        assert_eq!(instr, "ldgm x3, [x4]");
    }
}
