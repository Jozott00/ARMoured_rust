//! # [LDAPR/STLR (unscaled immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldapstl_unscaled)
//!
//! Implements the following instructions:
//!  - [STLURB - Store Release Register Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLURB--Store-Release-Register-Byte--unscaled--?lang=en)
//!  - [LDAPURB - Load Acquire RCpc Register Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURB--Load-Acquire-RCpc-Register-Byte--unscaled--?lang=en)
//!  - [LDAPURSB - Load Acquire RCpc Register Signed Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURSB--Load-Acquire-RCpc-Register-Signed-Byte--unscaled--?lang=en)
//!  - [STLURH - Store Release Register Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLURH--Store-Release-Register-Halfword--unscaled--?lang=en)
//!  - [LDAPURH - Load Acquire RCpc Register Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURH--Load-Acquire-RCpc-Register-Halfword--unscaled--?lang=en)
//!  - [LDAPURSH - Load Acquire RCpc Register Signed Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURSH--Load-Acquire-RCpc-Register-Signed-Halfword--unscaled--?lang=en)
//!  - [STLUR - Store Release Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLUR--Store-Release-Register--unscaled--?lang=en)
//!  - [LDAPUR - Load Acquire RCpc Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPUR--Load-Acquire-RCpc-Register--unscaled--?lang=en)
//!  - [LDAPURSW - Load Acquire RCpc Register Signed Word - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURSW--Load-Acquire-RCpc-Register-Signed-Word--unscaled--?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Imm9, Register};

#[inline(always)]
fn emit_ld_st_instr<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    size: u8,
    opc: u8,
    imm9: u16,
    rn: Register,
    rt: Register,
) -> T {
    let r = bseq_32!(size:2 011001 opc:2 0 imm9:9 00 rn:5 rt:5);
    proc.process(r)
}

/// # [LDAPR/STLR (unscaled immediate)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldapstl_unscaled)
///
/// Implements the following instructions:
///  - [STLURB - Store Release Register Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLURB--Store-Release-Register-Byte--unscaled--?lang=en)
///  - [LDAPURB - Load Acquire RCpc Register Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURB--Load-Acquire-RCpc-Register-Byte--unscaled--?lang=en)
///  - [LDAPURSB - Load Acquire RCpc Register Signed Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURSB--Load-Acquire-RCpc-Register-Signed-Byte--unscaled--?lang=en)
///  - [STLURH - Store Release Register Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLURH--Store-Release-Register-Halfword--unscaled--?lang=en)
///  - [LDAPURH - Load Acquire RCpc Register Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURH--Load-Acquire-RCpc-Register-Halfword--unscaled--?lang=en)
///  - [LDAPURSH - Load Acquire RCpc Register Signed Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURSH--Load-Acquire-RCpc-Register-Signed-Halfword--unscaled--?lang=en)
///  - [STLUR - Store Release Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLUR--Store-Release-Register--unscaled--?lang=en)
///  - [LDAPUR - Load Acquire RCpc Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPUR--Load-Acquire-RCpc-Register--unscaled--?lang=en)
///  - [LDAPURSW - Load Acquire RCpc Register Signed Word - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURSW--Load-Acquire-RCpc-Register-Signed-Word--unscaled--?lang=en)
pub trait LdaprStlrUnscaleImmediate<T>: InstructionProcessor<T> {
    /// [STLURB - Store Release Register Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLURB--Store-Release-Register-Byte--unscaled--?lang=en)
    ///
    /// Store-Release Register Byte (unscaled) calculates an address from a base register value and an immediate offset, and stores a byte to the calculated address, from a 32-bit register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release
    ///
    /// ```asm
    /// STLURB <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stlurb(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_instr(self, 0, 0, simm as u16, xn_sp, wt)
    }

    /// [LDAPURB - Load Acquire RCpc Register Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURB--Load-Acquire-RCpc-Register-Byte--unscaled--?lang=en)
    ///
    /// Load-Acquire RCpc Register Byte (unscaled) calculates an address from a base register and an immediate offset, loads a byte from memory, zero-extends it, and writes it to a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release, except that:
    ///
    /// ```asm
    /// LDAPURB <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldapurb(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_instr(self, 0b00, 0b01, simm as u16, xn_sp, wt)
    }

    /// [LDAPURSB - Load Acquire RCpc Register Signed Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURSB--Load-Acquire-RCpc-Register-Signed-Byte--unscaled--?lang=en)
    ///
    /// Load-Acquire RCpc Register Signed Byte (unscaled) calculates an address from a base register and an immediate offset, loads a signed byte from memory, sign-extends it, and writes it to a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release, except that:
    ///
    /// ```asm
    /// LDAPURSB <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldapursb_32(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_instr(self, 0b00, 0b11, simm as u16, xn_sp, wt)
    }

    /// [LDAPURSB - Load Acquire RCpc Register Signed Byte - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURSB--Load-Acquire-RCpc-Register-Signed-Byte--unscaled--?lang=en)
    ///
    /// Load-Acquire RCpc Register Signed Byte (unscaled) calculates an address from a base register and an immediate offset, loads a signed byte from memory, sign-extends it, and writes it to a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release, except that:
    ///
    /// ```asm
    /// LDAPURSB <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldapursb_64(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_instr(self, 0b00, 0b10, simm as u16, xn_sp, xt)
    }

    /// [STLURH - Store Release Register Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLURH--Store-Release-Register-Halfword--unscaled--?lang=en)
    ///
    /// Store-Release Register Halfword (unscaled) calculates an address from a base register value and an immediate offset, and stores a halfword to the calculated address, from a 32-bit register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release
    ///
    /// ```asm
    /// STLURH <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stlurh(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_instr(self, 0b01, 0b00, simm as u16, xn_sp, wt)
    }

    /// [LDAPURH - Load Acquire RCpc Register Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURH--Load-Acquire-RCpc-Register-Halfword--unscaled--?lang=en)
    ///
    /// Load-Acquire RCpc Register Halfword (unscaled) calculates an address from a base register and an immediate offset, loads a halfword from memory, zero-extends it, and writes it to a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release, except that:
    ///
    /// ```asm
    /// LDAPURH <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldapurh(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_instr(self, 0b01, 0b01, simm as u16, xn_sp, wt)
    }

    /// [LDAPURSH - Load Acquire RCpc Register Signed Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURSH--Load-Acquire-RCpc-Register-Signed-Halfword--unscaled--?lang=en)
    ///
    /// Load-Acquire RCpc Register Signed Halfword (unscaled) calculates an address from a base register and an immediate offset, loads a signed halfword from memory, sign-extends it, and writes it to a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release, except that:
    ///
    /// ```asm
    /// LDAPURSH <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```

    #[inline(always)]
    fn ldapursh_32(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_instr(self, 0b01, 0b11, simm as u16, xn_sp, wt)
    }

    /// [LDAPURSH - Load Acquire RCpc Register Signed Halfword - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURSH--Load-Acquire-RCpc-Register-Signed-Halfword--unscaled--?lang=en)
    ///
    /// Load-Acquire RCpc Register Signed Halfword (unscaled) calculates an address from a base register and an immediate offset, loads a signed halfword from memory, sign-extends it, and writes it to a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release, except that:
    ///
    /// ```asm
    /// LDAPURSH <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldapursh_64(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_instr(self, 0b01, 0b10, simm as u16, xn_sp, xt)
    }

    /// [STLUR - Store Release Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLUR--Store-Release-Register--unscaled--?lang=en)
    ///
    /// Store-Release Register (unscaled) calculates an address from a base register value and an immediate offset, and stores a 32-bit word or a 64-bit doubleword to the calculated address, from a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release
    ///
    /// ```asm
    /// STLUR <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stlur_32(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_instr(self, 0b10, 0b00, simm as u16, xn_sp, wt)
    }

    /// [STLUR - Store Release Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLUR--Store-Release-Register--unscaled--?lang=en)
    ///
    /// Store-Release Register (unscaled) calculates an address from a base register value and an immediate offset, and stores a 32-bit word or a 64-bit doubleword to the calculated address, from a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release
    ///
    /// ```asm
    /// STLUR <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn stlur_64(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_instr(self, 0b11, 0b00, simm as u16, xn_sp, xt)
    }

    /// [LDAPUR - Load Acquire RCpc Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPUR--Load-Acquire-RCpc-Register--unscaled--?lang=en)
    ///
    /// Load-Acquire RCpc Register (unscaled) calculates an address from a base register and an immediate offset, loads a 32-bit word or 64-bit doubleword from memory, zero-extends it, and writes it to a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release, except that:
    ///
    /// ```asm
    /// LDAPUR <Wt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldapur_32(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_instr(self, 0b10, 0b01, simm as u16, xn_sp, wt)
    }

    /// [LDAPUR - Load Acquire RCpc Register - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPUR--Load-Acquire-RCpc-Register--unscaled--?lang=en)
    ///
    /// Load-Acquire RCpc Register (unscaled) calculates an address from a base register and an immediate offset, loads a 32-bit word or 64-bit doubleword from memory, zero-extends it, and writes it to a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release, except that:
    ///
    /// ```asm
    /// LDAPUR <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldapur_64(&mut self, xt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_instr(self, 0b11, 0b01, simm as u16, xn_sp, xt)
    }

    /// [LDAPURSW - Load Acquire RCpc Register Signed Word - unscaled - ](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPURSW--Load-Acquire-RCpc-Register-Signed-Word--unscaled--?lang=en)
    ///
    /// Load-Acquire RCpc Register Signed Word (unscaled) calculates an address from a base register and an immediate offset, loads a signed word from memory, sign-extends it, and writes it to a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release, except that:
    ///
    /// ```asm
    /// LDAPURSW <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn ldapursw(&mut self, wt: Register, xn_sp: Register, simm: Imm9) -> T {
        emit_ld_st_instr(self, 0b10, 0b10, simm as u16, xn_sp, wt)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;

    use super::*;

    #[test]
    fn test_stld_byte() {
        let mut prod = TestProducer::new();

        let instr = prod.stlurb(2, 3, -255);
        assert_eq!(instr, "stlurb w2, [x3, #0xffffffffffffff01]");

        let instr = prod.ldapurb(2, 3, -255);
        assert_eq!(instr, "ldapurb w2, [x3, #0xffffffffffffff01]");

        let instr = prod.ldapursb_32(2, 3, -255);
        assert_eq!(instr, "ldapursb w2, [x3, #0xffffffffffffff01]");

        let instr = prod.ldapursb_64(2, 3, -255);
        assert_eq!(instr, "ldapursb x2, [x3, #0xffffffffffffff01]");
    }

    #[test]
    fn test_stld_halfword() {
        let mut prod = TestProducer::new();

        let instr = prod.stlurh(2, 3, -255);
        assert_eq!(instr, "stlurh w2, [x3, #0xffffffffffffff01]");

        let instr = prod.ldapurh(2, 3, -255);
        assert_eq!(instr, "ldapurh w2, [x3, #0xffffffffffffff01]");

        let instr = prod.ldapursh_32(2, 3, -255);
        assert_eq!(instr, "ldapursh w2, [x3, #0xffffffffffffff01]");

        let instr = prod.ldapursh_64(2, 3, -255);
        assert_eq!(instr, "ldapursh x2, [x3, #0xffffffffffffff01]");
    }

    #[test]
    fn test_stld_32_64() {
        let mut prod = TestProducer::new();

        let instr = prod.stlur_32(2, 3, -255);
        assert_eq!(instr, "stlur w2, [x3, #0xffffffffffffff01]");

        let instr = prod.ldapur_32(2, 3, -255);
        assert_eq!(instr, "ldapur w2, [x3, #0xffffffffffffff01]");

        let instr = prod.ldapur_64(2, 3, -255);
        assert_eq!(instr, "ldapur x2, [x3, #0xffffffffffffff01]");

        let instr = prod.stlur_64(2, 3, -255);
        assert_eq!(instr, "stlur x2, [x3, #0xffffffffffffff01]");

        let instr = prod.ldapursw(2, 3, -255);
        assert_eq!(instr, "ldapursw x2, [x3, #0xffffffffffffff01]");
    }
}
