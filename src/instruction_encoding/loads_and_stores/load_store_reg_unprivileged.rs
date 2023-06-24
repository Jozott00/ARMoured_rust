//! # [Load/store register (unprivileged)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldst_unpriv)
//!
//! Implements the following instructions:
//!  - [STTRB - Store Register Byte - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STTRB--Store-Register-Byte--unprivileged--?lang=en)
//!  - [LDTRB - Load Register Byte - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRB--Load-Register-Byte--unprivileged--?lang=en)
//!  - [LDTRSB - Load Register Signed Byte - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRSB--Load-Register-Signed-Byte--unprivileged--?lang=en)
//!  - [STTRH - Store Register Halfword - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STTRH--Store-Register-Halfword--unprivileged--?lang=en)
//!  - [LDTRH - Load Register Halfword - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRH--Load-Register-Halfword--unprivileged--?lang=en)
//!  - [LDTRSH - Load Register Signed Halfword - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRSH--Load-Register-Signed-Halfword--unprivileged--?lang=en)
//!  - [STTR - Store Register - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STTR--Store-Register--unprivileged--?lang=en)
//!  - [LDTR - Load Register - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTR--Load-Register--unprivileged--?lang=en)
//!  - [LDTRSW - Load Register Signed Word - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRSW--Load-Register-Signed-Word--unprivileged--?lang=en)



use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Imm9, Register};

#[inline(always)]
fn emit_ld_st_reg_unp<P: InstructionProcessor<T>, T>(proc: &mut P, size: u8, v: u8, opc: u8, imm9: u16, rn: Register, rt: Register) -> T {
    let r = bseq_32!(size:2 111 v:1 00 opc:2 0 imm9:9 10 rn:5 rt:5);
    proc.process(r)
}

#[inline(always)]
fn emit_ld_st_reg_unp_checked<P: InstructionProcessor<T>, T>(proc: &mut P, size: u8, v: u8, opc: u8, simm: Imm9, rn: Register, rt: Register) -> T {
    emit_ld_st_reg_unp(proc, size, v, opc, simm as u16, rn, rt)
}

/// # [Load/store register (unprivileged)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldst_unpriv)
///
/// Implements the following instructions:
///  - [STTRB - Store Register Byte - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STTRB--Store-Register-Byte--unprivileged--?lang=en)
///  - [LDTRB - Load Register Byte - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRB--Load-Register-Byte--unprivileged--?lang=en)
///  - [LDTRSB - Load Register Signed Byte - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRSB--Load-Register-Signed-Byte--unprivileged--?lang=en)
///  - [STTRH - Store Register Halfword - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STTRH--Store-Register-Halfword--unprivileged--?lang=en)
///  - [LDTRH - Load Register Halfword - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRH--Load-Register-Halfword--unprivileged--?lang=en)
///  - [LDTRSH - Load Register Signed Halfword - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRSH--Load-Register-Signed-Halfword--unprivileged--?lang=en)
///  - [STTR - Store Register - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STTR--Store-Register--unprivileged--?lang=en)
///  - [LDTR - Load Register - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTR--Load-Register--unprivileged--?lang=en)
///  - [LDTRSW - Load Register Signed Word - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRSW--Load-Register-Signed-Word--unprivileged--?lang=en)
pub trait LoadStoreRegisterUnprivileged<T>: InstructionProcessor<T> {
    /// [STTRB - Store Register Byte - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STTRB--Store-Register-Byte--unprivileged--?lang=en)
    ///
    /// Store Register Byte (unprivileged) stores a byte from a 32-bit register to memory. The address that is used for the store is calculated from a base register and an immediate offset.
    ///
    /// Memory accesses made by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either:
    ///
    /// ```asm
    /// STTRB <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn sttrb(&mut self, wt: Register, xn: Register, simm: Imm9) -> T {
        emit_ld_st_reg_unp_checked(self, 0b00, 0, 0b00, simm, xn, wt)
    }


    /// [LDTRB - Load Register Byte - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRB--Load-Register-Byte--unprivileged--?lang=en)
    ///
    /// Load Register Byte (unprivileged) loads a byte from memory, zero-extends it, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset.
    ///
    /// Memory accesses made by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either:
    ///
    /// ```asm
    /// LDTRB <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldtrb(&mut self, wt: Register, xn: Register, simm: Imm9) -> T {
        emit_ld_st_reg_unp_checked(self, 0b00, 0, 0b01, simm, xn, wt)
    }


    /// [LDTRSB - Load Register Signed Byte - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRSB--Load-Register-Signed-Byte--unprivileged--?lang=en)
    ///
    /// Load Register Signed Byte (unprivileged) loads a byte from memory, sign-extends it to 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset.
    ///
    /// Memory accesses made by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either:
    ///
    /// ```asm
    /// LDTRSB <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldtrsb_32(&mut self, wt: Register, xn: Register, simm: Imm9) -> T {
        emit_ld_st_reg_unp_checked(self, 0b00, 0, 0b11, simm, xn, wt)
    }


    /// [LDTRSB - Load Register Signed Byte - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRSB--Load-Register-Signed-Byte--unprivileged--?lang=en)
    ///
    /// Load Register Signed Byte (unprivileged) loads a byte from memory, sign-extends it to 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset.
    ///
    /// Memory accesses made by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either:
    ///
    /// ```asm
    /// LDTRSB <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldtrsb_64(&mut self, xt: Register, xn: Register, simm: Imm9) -> T {
        emit_ld_st_reg_unp_checked(self, 0b00, 0, 0b10, simm, xn, xt)
    }


    /// [STTRH - Store Register Halfword - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STTRH--Store-Register-Halfword--unprivileged--?lang=en)
    ///
    /// Store Register Halfword (unprivileged) stores a halfword from a 32-bit register to memory. The address that is used for the store is calculated from a base register and an immediate offset.
    ///
    /// Memory accesses made by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either:
    ///
    /// ```asm
    /// STTRH <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn sttrh(&mut self, wt: Register, xn: Register, simm: Imm9) -> T {
        emit_ld_st_reg_unp_checked(self, 0b01, 0, 0b00, simm, xn, wt)
    }


    /// [LDTRH - Load Register Halfword - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRH--Load-Register-Halfword--unprivileged--?lang=en)
    ///
    /// Load Register Halfword (unprivileged) loads a halfword from memory, zero-extends it, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset.
    ///
    /// Memory accesses made by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either:
    ///
    /// ```asm
    /// LDTRH <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldtrh(&mut self, wt: Register, xn: Register, simm: Imm9) -> T {
        emit_ld_st_reg_unp_checked(self, 0b01, 0, 0b01, simm, xn, wt)
    }


    /// [LDTRSH - Load Register Signed Halfword - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRSH--Load-Register-Signed-Halfword--unprivileged--?lang=en)
    ///
    /// Load Register Signed Halfword (unprivileged) loads a halfword from memory, sign-extends it to 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset.
    ///
    /// Memory accesses made by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either:
    ///
    /// ```asm
    /// LDTRSH <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldtrsh_32(&mut self, wt: Register, xn: Register, simm: Imm9) -> T {
        emit_ld_st_reg_unp_checked(self, 0b01, 0, 0b11, simm, xn, wt)
    }


    /// [LDTRSH - Load Register Signed Halfword - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRSH--Load-Register-Signed-Halfword--unprivileged--?lang=en)
    ///
    /// Load Register Signed Halfword (unprivileged) loads a halfword from memory, sign-extends it to 32 bits or 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset.
    ///
    /// Memory accesses made by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either:
    ///
    /// ```asm
    /// LDTRSH <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldtrsh_64(&mut self, xt: Register, xn: Register, simm: Imm9) -> T {
        emit_ld_st_reg_unp_checked(self, 0b01, 0, 0b10, simm, xn, xt)
    }


    /// [STTR - Store Register - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STTR--Store-Register--unprivileged--?lang=en)
    ///
    /// Store Register (unprivileged) stores a word or doubleword from a register to memory. The address that is used for the store is calculated from a base register and an immediate offset.
    ///
    /// Memory accesses made by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either:
    ///
    /// ```asm
    /// STTR <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn sttr_32(&mut self, wt: Register, xn: Register, simm: Imm9) -> T {
        emit_ld_st_reg_unp_checked(self, 0b10, 0, 0b00, simm, xn, wt)
    }


    /// [STTR - Store Register - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STTR--Store-Register--unprivileged--?lang=en)
    ///
    /// Store Register (unprivileged) stores a word or doubleword from a register to memory. The address that is used for the store is calculated from a base register and an immediate offset.
    ///
    /// Memory accesses made by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either:
    ///
    /// ```asm
    /// STTR <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn sttr_64(&mut self, xt: Register, xn: Register, simm: Imm9) -> T {
        emit_ld_st_reg_unp_checked(self, 0b11, 0, 0b00, simm, xn, xt)
    }


    /// [LDTR - Load Register - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTR--Load-Register--unprivileged--?lang=en)
    ///
    /// Load Register (unprivileged) loads a word or doubleword from memory, and writes it to a register. The address that is used for the load is calculated from a base register and an immediate offset.
    ///
    /// Memory accesses made by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either:
    ///
    /// ```asm
    /// LDTR <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldtr_32(&mut self, wt: Register, xn: Register, simm: Imm9) -> T {
        emit_ld_st_reg_unp_checked(self, 0b10, 0, 0b01, simm, xn, wt)
    }

    /// [LDTR - Load Register - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTR--Load-Register--unprivileged--?lang=en)
    ///
    /// Load Register (unprivileged) loads a word or doubleword from memory, and writes it to a register. The address that is used for the load is calculated from a base register and an immediate offset.
    ///
    /// Memory accesses made by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either:
    ///
    /// ```asm
    /// LDTR <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldtr_64(&mut self, xt: Register, xn: Register, simm: Imm9) -> T {
        emit_ld_st_reg_unp_checked(self, 0b11, 0, 0b01, simm, xn, xt)
    }


    /// [LDTRSW - Load Register Signed Word - unprivileged - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDTRSW--Load-Register-Signed-Word--unprivileged--?lang=en)
    ///
    /// Load Register Signed Word (unprivileged) loads a word from memory, sign-extends it to 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset.
    ///
    /// Memory accesses made by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either:
    ///
    /// ```asm
    /// LDTRSW <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldtrsw(&mut self, xt: Register, xn: Register, simm: Imm9) -> T {
        emit_ld_st_reg_unp_checked(self, 0b10, 0, 0b10, simm, xn, xt)
    }
}


#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;
    use crate::types::prefetch_memory::{PrfPolicy, PrfTarget, PrfType};
    use super::*;

    #[test]
    fn test_sttrb() {
        let mut prod = TestProducer::new();

        let instr = prod.sttrb(2, 3, -256);
        assert_eq!(instr, "sttrb w2, [x3, #0xffffffffffffff00]");
        let instr = prod.sttrb(2, 3, -257);
        assert_eq!(instr, "sttrb w2, [x3, #0xff]");
    }

    #[test]
    fn test_ldtrb() {
        let mut prod = TestProducer::new();

        let instr = prod.ldtrb(2, 3, -256);
        assert_eq!(instr, "ldtrb w2, [x3, #0xffffffffffffff00]");
        let instr = prod.ldtrb(2, 3, -257);
        assert_eq!(instr, "ldtrb w2, [x3, #0xff]");
    }

    #[test]
    fn test_ldtrsb() {
        let mut prod = TestProducer::new();

        let instr = prod.ldtrsb_32(2, 3, -256);
        assert_eq!(instr, "ldtrsb w2, [x3, #0xffffffffffffff00]");

        let instr = prod.ldtrsb_64(2, 3, -257);
        assert_eq!(instr, "ldtrsb x2, [x3, #0xff]");
    }

    #[test]
    fn test_sttrh() {
        let mut prod = TestProducer::new();

        let instr = prod.sttrh(2, 3, -256);
        assert_eq!(instr, "sttrh w2, [x3, #0xffffffffffffff00]");
        let instr = prod.sttrh(2, 3, -257);
        assert_eq!(instr, "sttrh w2, [x3, #0xff]");
    }

    #[test]
    fn test_ldtrh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldtrh(2, 3, -256);
        assert_eq!(instr, "ldtrh w2, [x3, #0xffffffffffffff00]");
        let instr = prod.ldtrh(2, 3, -257);
        assert_eq!(instr, "ldtrh w2, [x3, #0xff]");
    }

    #[test]
    fn test_ldtrsh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldtrsh_32(2, 3, -256);
        assert_eq!(instr, "ldtrsh w2, [x3, #0xffffffffffffff00]");

        let instr = prod.ldtrsh_64(2, 3, -257);
        assert_eq!(instr, "ldtrsh x2, [x3, #0xff]");
    }

    #[test]
    fn test_sttr() {
        let mut prod = TestProducer::new();

        let instr = prod.sttr_32(2, 3, -256);
        assert_eq!(instr, "sttr w2, [x3, #0xffffffffffffff00]");

        let instr = prod.sttr_64(2, 3, -257);
        assert_eq!(instr, "sttr x2, [x3, #0xff]");
    }

    #[test]
    fn test_ldtr() {
        let mut prod = TestProducer::new();

        let instr = prod.ldtr_32(2, 3, -256);
        assert_eq!(instr, "ldtr w2, [x3, #0xffffffffffffff00]");

        let instr = prod.ldtr_64(2, 3, -257);
        assert_eq!(instr, "ldtr x2, [x3, #0xff]");
    }

    #[test]
    fn test_ldtrsw() {
        let mut prod = TestProducer::new();

        let instr = prod.ldtrsw(2, 3, -256);
        assert_eq!(instr, "ldtrsw x2, [x3, #0xffffffffffffff00]");
        let instr = prod.ldtrsw(2, 3, -257);
        assert_eq!(instr, "ldtrsw x2, [x3, #0xff]");
    }
}