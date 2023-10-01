//! # [Load/store register (pac)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldst_pac)
//!
//! Implements the following instructions:
//!  - [LDRAA - LDRAB - Load Register - with pointer authentication](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRAA--LDRAB--Load-Register--with-pointer-authentication-?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Imm10, Register};

#[inline(always)]
fn emit_ld_st_reg_pac<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    size: u8,
    v: u8,
    m: u8,
    s: u8,
    imm9: u16,
    w: u8,
    rn: Register,
    rt: Register,
) -> T {
    let r = bseq_32!(size:2 111 v:1 00 m:1 s:1 1 imm9:9 w:1 1 rn:5 rt:5);
    proc.process(r)
}

#[inline(always)]
fn emit_ld_st_reg_pac_checked<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    size: u8,
    v: u8,
    m: u8,
    simm: Imm10,
    w: u8,
    rn: Register,
    rt: Register,
) -> T {
    debug_assert!(simm % 8 == 0, "imm9 must be a multiply of 8");
    let imm9 = (simm / 8) as u16;
    let s = imm9 >> 9;
    emit_ld_st_reg_pac(proc, size, v, m, s as u8, imm9, w, rn, rt)
}

/// # [Load/store register (pac)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldst_pac)
///
/// Implements the following instructions:
///  - [LDRAA - LDRAB - Load Register - with pointer authentication](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRAA--LDRAB--Load-Register--with-pointer-authentication-?lang=en)
pub trait LoadStoreRegisterPac<T>: InstructionProcessor<T> {
    /// [LDRAA - LDRAB - Load Register - with pointer authentication](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRAA--LDRAB--Load-Register--with-pointer-authentication-?lang=en)
    ///
    /// Load Register, with pointer authentication. This instruction authenticates an address from a base register using a modifier of zero and the specified key, adds an immediate offset to the authenticated address, and loads a 64-bit doubleword from memory at this resulting address into a register.
    ///
    /// Key A is used for LDRAA, and key B is used for LDRAB.
    ///
    /// ```asm
    /// LDRAA <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldraa_offset(&mut self, xt: Register, xn_sp: Register, simm: Imm10) -> T {
        emit_ld_st_reg_pac_checked(self, 0b11, 0, 0, simm, 0, xn_sp, xt)
    }

    /// [LDRAA - LDRAB - Load Register - with pointer authentication](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRAA--LDRAB--Load-Register--with-pointer-authentication-?lang=en)
    ///
    /// Load Register, with pointer authentication. This instruction authenticates an address from a base register using a modifier of zero and the specified key, adds an immediate offset to the authenticated address, and loads a 64-bit doubleword from memory at this resulting address into a register.
    ///
    /// Key A is used for LDRAA, and key B is used for LDRAB.
    ///
    /// ```asm
    /// LDRAA <Xt>, [<Xn|SP>{, #<simm>}]!
    /// ```
    #[inline(always)]
    fn ldraa_pre_indexed(&mut self, xt: Register, xn_sp: Register, simm: Imm10) -> T {
        emit_ld_st_reg_pac_checked(self, 0b11, 0, 0, simm, 1, xn_sp, xt)
    }

    /// [LDRAA - LDRAB - Load Register - with pointer authentication](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRAA--LDRAB--Load-Register--with-pointer-authentication-?lang=en)
    ///
    /// Load Register, with pointer authentication. This instruction authenticates an address from a base register using a modifier of zero and the specified key, adds an immediate offset to the authenticated address, and loads a 64-bit doubleword from memory at this resulting address into a register.
    ///
    /// Key A is used for LDRAA, and key B is used for LDRAB.
    ///
    /// ```asm
    /// LDRAB <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldrab_offset(&mut self, xt: Register, xn_sp: Register, simm: Imm10) -> T {
        emit_ld_st_reg_pac_checked(self, 0b11, 0, 1, simm, 0, xn_sp, xt)
    }

    /// [LDRAA - LDRAB - Load Register - with pointer authentication](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDRAA--LDRAB--Load-Register--with-pointer-authentication-?lang=en)
    ///
    /// Load Register, with pointer authentication. This instruction authenticates an address from a base register using a modifier of zero and the specified key, adds an immediate offset to the authenticated address, and loads a 64-bit doubleword from memory at this resulting address into a register.
    ///
    /// Key A is used for LDRAA, and key B is used for LDRAB.
    ///
    /// ```asm
    /// LDRAB <Xt>, [<Xn|SP>{, #<simm>}]!
    /// ```
    #[inline(always)]
    fn ldrab_pre_indexed(&mut self, xt: Register, xn_sp: Register, simm: Imm10) -> T {
        emit_ld_st_reg_pac_checked(self, 0b11, 0, 1, simm, 1, xn_sp, xt)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_ldraa() {
        let mut prod = TestProducer::new();

        let instr = prod.ldraa_pre_indexed(1, 2, -4096);
        assert_eq!(instr, "ldraa x1, [x2, #0xfffffffffffff000]!");

        assert_panic!("Should panic: invalid simm"; prod.ldraa_pre_indexed(1, 2, -4095));

        let instr = prod.ldraa_offset(1, 2, -4096);
        assert_eq!(instr, "ldraa x1, [x2, #0xfffffffffffff000]");

        assert_panic!("Should panic: invalid simm"; prod.ldraa_offset(1, 2, -4095));
    }

    #[test]
    fn test_ldrab() {
        let mut prod = TestProducer::new();

        let instr = prod.ldrab_pre_indexed(1, 2, -4096);
        assert_eq!(instr, "ldrab x1, [x2, #0xfffffffffffff000]!");

        assert_panic!("Should panic: invalid simm"; prod.ldrab_pre_indexed(1, 2, -4095));

        let instr = prod.ldrab_offset(1, 2, -4096);
        assert_eq!(instr, "ldrab x1, [x2, #0xfffffffffffff000]");

        assert_panic!("Should panic: invalid simm"; prod.ldrab_offset(1, 2, -4095));
    }
}
