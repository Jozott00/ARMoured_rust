//! # [Load/store ordered](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldstord)
//!
//! Implements the following instructions:
//!  - [STLLRB - Store LORelease Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLLRB--Store-LORelease-Register-Byte-?lang=en)
//!  - [STLRB - Store Release Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLRB--Store-Release-Register-Byte-?lang=en)
//!  - [LDLARB - Load LOAcquire Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDLARB--Load-LOAcquire-Register-Byte-?lang=en)
//!  - [LDARB - Load Acquire Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDARB--Load-Acquire-Register-Byte-?lang=en)
//!  - [STLLRH - Store LORelease Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLLRH--Store-LORelease-Register-Halfword-?lang=en)
//!  - [STLRH - Store Release Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLRH--Store-Release-Register-Halfword-?lang=en)
//!  - [LDLARH - Load LOAcquire Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDLARH--Load-LOAcquire-Register-Halfword-?lang=en)
//!  - [LDARH - Load Acquire Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDARH--Load-Acquire-Register-Halfword-?lang=en)
//!  - [STLLR - Store LORelease Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLLR--Store-LORelease-Register-?lang=en)
//!  - [STLR - Store Release Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLR--Store-Release-Register-?lang=en)
//!  - [LDLAR - Load LOAcquire Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDLAR--Load-LOAcquire-Register-?lang=en)
//!  - [LDAR - Load Acquire Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAR--Load-Acquire-Register-?lang=en)



use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::Register;

#[inline(always)]
fn emit_ldr_str_ord<P: InstructionProcessor<T>, T>(proc: &mut P, size: u8, l: u8, rs: Register, o0: u8, rt2: Register, rn: Register, rt: Register) -> T {
    let r = bseq_32!(size:2 0010001 l:1 0 rs:5 o0:1 rt2:5 rn:5 rt:5);
    proc.process(r)
}

/// # [Load/store ordered](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldstord)
///
/// Implements the following instructions:
///  - [STLLRB - Store LORelease Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLLRB--Store-LORelease-Register-Byte-?lang=en)
///  - [STLRB - Store Release Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLRB--Store-Release-Register-Byte-?lang=en)
///  - [LDLARB - Load LOAcquire Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDLARB--Load-LOAcquire-Register-Byte-?lang=en)
///  - [LDARB - Load Acquire Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDARB--Load-Acquire-Register-Byte-?lang=en)
///  - [STLLRH - Store LORelease Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLLRH--Store-LORelease-Register-Halfword-?lang=en)
///  - [STLRH - Store Release Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLRH--Store-Release-Register-Halfword-?lang=en)
///  - [LDLARH - Load LOAcquire Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDLARH--Load-LOAcquire-Register-Halfword-?lang=en)
///  - [LDARH - Load Acquire Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDARH--Load-Acquire-Register-Halfword-?lang=en)
///  - [STLLR - Store LORelease Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLLR--Store-LORelease-Register-?lang=en)
///  - [STLR - Store Release Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLR--Store-Release-Register-?lang=en)
///  - [LDLAR - Load LOAcquire Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDLAR--Load-LOAcquire-Register-?lang=en)
///  - [LDAR - Load Acquire Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAR--Load-Acquire-Register-?lang=en)
pub trait LoadStoreOrdered<T>: InstructionProcessor<T> {
    /// [STLLRB - Store LORelease Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLLRB--Store-LORelease-Register-Byte-?lang=en)
    ///
    /// Store LORelease Register Byte stores a byte from a 32-bit register to a memory location. The instruction also has memory ordering semantics as described in Load LOAcquire, Store LORelease. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLLRB <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stllrb(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b00, 0, 0b11111, 0, 0b11111, xn_sp, wt)
    }


    /// [STLRB - Store Release Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLRB--Store-Release-Register-Byte-?lang=en)
    ///
    /// Store-Release Register Byte stores a byte from a 32-bit register to a memory location. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLRB <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stlrb(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b00, 0, 0b11111, 1, 0b11111, xn_sp, wt)
    }


    /// [LDLARB - Load LOAcquire Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDLARB--Load-LOAcquire-Register-Byte-?lang=en)
    ///
    /// Load LOAcquire Register Byte loads a byte from memory, zero-extends it and writes it to a register. The instruction also has memory ordering semantics as described in Load LOAcquire, Store LORelease. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDLARB <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldlarb(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b00, 1, 0b11111, 0, 0b11111, xn_sp, wt)
    }


    /// [LDARB - Load Acquire Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDARB--Load-Acquire-Register-Byte-?lang=en)
    ///
    /// Load-Acquire Register Byte derives an address from a base register value, loads a byte from memory, zero-extends it and writes it to a register. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDARB <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldarb(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b00, 1, 0b11111, 1, 0b11111, xn_sp, wt)
    }


    /// [STLLRH - Store LORelease Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLLRH--Store-LORelease-Register-Halfword-?lang=en)
    ///
    /// Store LORelease Register Halfword stores a halfword from a 32-bit register to a memory location. The instruction also has memory ordering semantics as described in Load LOAcquire, Store LORelease. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLLRH <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stllrh(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b01, 0, 0b11111, 0, 0b11111, xn_sp, wt)
    }


    /// [STLRH - Store Release Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLRH--Store-Release-Register-Halfword-?lang=en)
    ///
    /// Store-Release Register Halfword stores a halfword from a 32-bit register to a memory location. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLRH <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stlrh(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b01, 0, 0b11111, 1, 0b11111, xn_sp, wt)
    }


    /// [LDLARH - Load LOAcquire Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDLARH--Load-LOAcquire-Register-Halfword-?lang=en)
    ///
    /// Load LOAcquire Register Halfword loads a halfword from memory, zero-extends it, and writes it to a register. The instruction also has memory ordering semantics as described in Load LOAcquire, Store LORelease. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDLARH <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldlarh(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b01, 1, 0b11111, 0, 0b11111, xn_sp, wt)
    }


    /// [LDARH - Load Acquire Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDARH--Load-Acquire-Register-Halfword-?lang=en)
    ///
    /// Load-Acquire Register Halfword derives an address from a base register value, loads a halfword from memory, zero-extends it, and writes it to a register. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDARH <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldarh(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b01, 1, 0b11111, 1, 0b11111, xn_sp, wt)
    }


    /// [STLLR - Store LORelease Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLLR--Store-LORelease-Register-?lang=en)
    ///
    /// Store LORelease Register stores a 32-bit word or a 64-bit doubleword to a memory location, from a register. The instruction also has memory ordering semantics as described in Load LOAcquire, Store LORelease. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLLR <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stllr_32(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b10, 0, 0b11111, 0, 0b11111, xn_sp, wt)
    }


    /// [STLLR - Store LORelease Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLLR--Store-LORelease-Register-?lang=en)
    ///
    /// Store LORelease Register stores a 32-bit word or a 64-bit doubleword to a memory location, from a register. The instruction also has memory ordering semantics as described in Load LOAcquire, Store LORelease. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLLR <Xt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stllr_64(&mut self, xt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b11, 0, 0b11111, 0, 0b11111, xn_sp, xt)
    }


    /// [STLR - Store Release Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLR--Store-Release-Register-?lang=en)
    ///
    /// Store-Release Register stores a 32-bit word or a 64-bit doubleword to a memory location, from a register. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLR <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stlr_32(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b10, 0, 0b11111, 1, 0b11111, xn_sp, wt)
    }


    /// [STLR - Store Release Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLR--Store-Release-Register-?lang=en)
    ///
    /// Store-Release Register stores a 32-bit word or a 64-bit doubleword to a memory location, from a register. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLR <Xt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stlr_64(&mut self, xt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b11, 0, 0b11111, 1, 0b11111, xn_sp, xt)
    }


    /// [LDLAR - Load LOAcquire Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDLAR--Load-LOAcquire-Register-?lang=en)
    ///
    /// Load LOAcquire Register loads a 32-bit word or 64-bit doubleword from memory, and writes it to a register. The instruction also has memory ordering semantics as described in Load LOAcquire, Store LORelease. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDLAR <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldlar_32(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b10, 1, 0b11111, 0, 0b11111, xn_sp, wt)
    }


    /// [LDLAR - Load LOAcquire Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDLAR--Load-LOAcquire-Register-?lang=en)
    ///
    /// Load LOAcquire Register loads a 32-bit word or 64-bit doubleword from memory, and writes it to a register. The instruction also has memory ordering semantics as described in Load LOAcquire, Store LORelease. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDLAR <Xt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldlar_64(&mut self, xt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b11, 1, 0b11111, 0, 0b11111, xn_sp, xt)
    }


    /// [LDAR - Load Acquire Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAR--Load-Acquire-Register-?lang=en)
    ///
    /// Load-Acquire Register derives an address from a base register value, loads a 32-bit word or 64-bit doubleword from memory, and writes it to a register. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDAR <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldar_32(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b10, 1, 0b11111, 1, 0b11111, xn_sp, wt)
    }


    /// [LDAR - Load Acquire Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAR--Load-Acquire-Register-?lang=en)
    ///
    /// Load-Acquire Register derives an address from a base register value, loads a 32-bit word or 64-bit doubleword from memory, and writes it to a register. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDAR <Xt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldar_64(&mut self, xt: Register, xn_sp: Register) -> T {
        emit_ldr_str_ord(self, 0b11, 1, 0b11111, 1, 0b11111, xn_sp, xt)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_stllr() {
        let mut prod = TestProducer::new();

        let instr = prod.stllrb(3, 4);
        assert_eq!(instr, "stllrb w3, [x4]");

        let instr = prod.stllrh(3, 4);
        assert_eq!(instr, "stllrh w3, [x4]");

        let instr = prod.stllr_32(3, 4);
        assert_eq!(instr, "stllr w3, [x4]");

        let instr = prod.stllr_64(3, 4);
        assert_eq!(instr, "stllr x3, [x4]");
    }

    #[test]
    fn test_stlr() {
        let mut prod = TestProducer::new();

        let instr = prod.stlrb(3, 4);
        assert_eq!(instr, "stlrb w3, [x4]");

        let instr = prod.stlrh(3, 4);
        assert_eq!(instr, "stlrh w3, [x4]");

        let instr = prod.stlr_32(3, 4);
        assert_eq!(instr, "stlr w3, [x4]");

        let instr = prod.stlr_64(3, 4);
        assert_eq!(instr, "stlr x3, [x4]");
    }

    #[test]
    fn test_ldlar() {
        let mut prod = TestProducer::new();

        let instr = prod.ldlarb(3, 4);
        assert_eq!(instr, "ldlarb w3, [x4]");

        let instr = prod.ldlarh(3, 4);
        assert_eq!(instr, "ldlarh w3, [x4]");

        let instr = prod.ldlar_32(3, 4);
        assert_eq!(instr, "ldlar w3, [x4]");

        let instr = prod.ldlar_64(3, 4);
        assert_eq!(instr, "ldlar x3, [x4]");
    }

    #[test]
    fn test_ldar() {
        let mut prod = TestProducer::new();

        let instr = prod.ldarb(3, 4);
        assert_eq!(instr, "ldarb w3, [x4]");

        let instr = prod.ldarh(3, 4);
        assert_eq!(instr, "ldarh w3, [x4]");

        let instr = prod.ldar_32(3, 4);
        assert_eq!(instr, "ldar w3, [x4]");

        let instr = prod.ldar_64(3, 4);
        assert_eq!(instr, "ldar x3, [x4]");
    }
}