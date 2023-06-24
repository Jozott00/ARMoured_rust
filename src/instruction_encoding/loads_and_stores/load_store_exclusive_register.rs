//! # [Load/store exclusive register](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldstexclr)
//!
//! Implements the following instructions:
//!  - [STXRB - Store Exclusive Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXRB--Store-Exclusive-Register-Byte-?lang=en)
//!  - [STLXRB - Store Release Exclusive Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXRB--Store-Release-Exclusive-Register-Byte-?lang=en)
//!  - [LDXRB - Load Exclusive Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXRB--Load-Exclusive-Register-Byte-?lang=en)
//!  - [LDAXRB - Load Acquire Exclusive Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXRB--Load-Acquire-Exclusive-Register-Byte-?lang=en)
//!  - [STXRH - Store Exclusive Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXRH--Store-Exclusive-Register-Halfword-?lang=en)
//!  - [STLXRH - Store Release Exclusive Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXRH--Store-Release-Exclusive-Register-Halfword-?lang=en)
//!  - [LDXRH - Load Exclusive Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXRH--Load-Exclusive-Register-Halfword-?lang=en)
//!  - [LDAXRH - Load Acquire Exclusive Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXRH--Load-Acquire-Exclusive-Register-Halfword-?lang=en)
//!  - [STXR - Store Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXR--Store-Exclusive-Register-?lang=en)
//!  - [STLXR - Store Release Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXR--Store-Release-Exclusive-Register-?lang=en)
//!  - [LDXR - Load Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXR--Load-Exclusive-Register-?lang=en)
//!  - [LDAXR - Load Acquire Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXR--Load-Acquire-Exclusive-Register-?lang=en)

#[inline(always)]
fn emit_ldr_str_excl_reg<P: InstructionProcessor<T>, T>(proc: &mut P, size: u8, l: u8, rs: Register, o0: u8, rt2: Register, rn: Register, rt: Register) -> T {
    let r = bseq_32!(size:2 0010000 l:1 0 rs:5 o0:1 rt2:5 rn:5 rt:5);
    proc.process(r)
}


use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::Register;

/// # [Load/store exclusive register](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldstexclr)
///
/// Implements the following instructions:
///  - [STXRB - Store Exclusive Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXRB--Store-Exclusive-Register-Byte-?lang=en)
///  - [STLXRB - Store Release Exclusive Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXRB--Store-Release-Exclusive-Register-Byte-?lang=en)
///  - [LDXRB - Load Exclusive Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXRB--Load-Exclusive-Register-Byte-?lang=en)
///  - [LDAXRB - Load Acquire Exclusive Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXRB--Load-Acquire-Exclusive-Register-Byte-?lang=en)
///  - [STXRH - Store Exclusive Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXRH--Store-Exclusive-Register-Halfword-?lang=en)
///  - [STLXRH - Store Release Exclusive Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXRH--Store-Release-Exclusive-Register-Halfword-?lang=en)
///  - [LDXRH - Load Exclusive Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXRH--Load-Exclusive-Register-Halfword-?lang=en)
///  - [LDAXRH - Load Acquire Exclusive Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXRH--Load-Acquire-Exclusive-Register-Halfword-?lang=en)
///  - [STXR - Store Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXR--Store-Exclusive-Register-?lang=en)
///  - [STLXR - Store Release Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXR--Store-Release-Exclusive-Register-?lang=en)
///  - [LDXR - Load Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXR--Load-Exclusive-Register-?lang=en)
///  - [LDAXR - Load Acquire Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXR--Load-Acquire-Exclusive-Register-?lang=en)
pub trait LoadStoreExclusiveRegister<T>: InstructionProcessor<T> {
    /// [STXRB - Store Exclusive Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXRB--Store-Exclusive-Register-Byte-?lang=en)
    ///
    /// Store Exclusive Register Byte stores a byte from a register to memory if the PE has exclusive access to the memory address, and returns a status value of 0 if the store was successful, or of 1 if no store was performed. See Synchronization and semaphores. The memory access is atomic.
    ///
    /// For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// STXRB <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stxrb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b00, 0, ws, 0, 0b11111, xn_sp, wt)
    }


    /// [STLXRB - Store Release Exclusive Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXRB--Store-Release-Exclusive-Register-Byte-?lang=en)
    ///
    /// Store-Release Exclusive Register Byte stores a byte from a 32-bit register to memory if the PE has exclusive access to the memory address, and returns a status value of 0 if the store was successful, or of 1 if no store was performed. See Synchronization and semaphores. The memory access is atomic. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLXRB <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stlxrb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b00, 0, ws, 1, 0b11111, xn_sp, wt)
    }


    /// [LDXRB - Load Exclusive Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXRB--Load-Exclusive-Register-Byte-?lang=en)
    ///
    /// Load Exclusive Register Byte derives an address from a base register value, loads a byte from memory, zero-extends it and writes it to a register. The memory access is atomic. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See Synchronization and semaphores. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDXRB <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldxrb(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b00, 1, 0b11111, 0, 0b11111, xn_sp, wt)
    }


    /// [LDAXRB - Load Acquire Exclusive Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXRB--Load-Acquire-Exclusive-Register-Byte-?lang=en)
    ///
    /// Load-Acquire Exclusive Register Byte derives an address from a base register value, loads a byte from memory, zero-extends it and writes it to a register. The memory access is atomic. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See Synchronization and semaphores. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDAXRB <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldaxrb(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b00, 1, 0b11111, 1, 0b11111, xn_sp, wt)
    }


    /// [STXRH - Store Exclusive Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXRH--Store-Exclusive-Register-Halfword-?lang=en)
    ///
    /// Store Exclusive Register Halfword stores a halfword from a register to memory if the PE has exclusive access to the memory address, and returns a status value of 0 if the store was successful, or of 1 if no store was performed. See Synchronization and semaphores. The memory access is atomic.
    ///
    /// For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// STXRH <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stxrh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b01, 0, ws, 0, 0b11111, xn_sp, wt)
    }


    /// [STLXRH - Store Release Exclusive Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXRH--Store-Release-Exclusive-Register-Halfword-?lang=en)
    ///
    /// Store-Release Exclusive Register Halfword stores a halfword from a 32-bit register to memory if the PE has exclusive access to the memory address, and returns a status value of 0 if the store was successful, or of 1 if no store was performed. See Synchronization and semaphores. The memory access is atomic. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLXRH <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stlxrh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b01, 0, ws, 1, 0b11111, xn_sp, wt)
    }


    /// [LDXRH - Load Exclusive Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXRH--Load-Exclusive-Register-Halfword-?lang=en)
    ///
    /// Load Exclusive Register Halfword derives an address from a base register value, loads a halfword from memory, zero-extends it and writes it to a register. The memory access is atomic. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See Synchronization and semaphores. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDXRH <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldxrh(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b01, 1, 0b11111, 0, 0b11111, xn_sp, wt)
    }


    /// [LDAXRH - Load Acquire Exclusive Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXRH--Load-Acquire-Exclusive-Register-Halfword-?lang=en)
    ///
    /// Load-Acquire Exclusive Register Halfword derives an address from a base register value, loads a halfword from memory, zero-extends it and writes it to a register. The memory access is atomic. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See Synchronization and semaphores. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDAXRH <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldaxrh(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b01, 1, 0b11111, 1, 0b11111, xn_sp, wt)
    }


    /// [STXR - Store Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXR--Store-Exclusive-Register-?lang=en)
    ///
    /// Store Exclusive Register stores a 32-bit word or a 64-bit doubleword from a register to memory if the PE has exclusive access to the memory address, and returns a status value of 0 if the store was successful, or of 1 if no store was performed. See Synchronization and semaphores. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// STXR <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stxr_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b10, 0, ws, 0, 0b11111, xn_sp, wt)
    }


    /// [STXR - Store Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXR--Store-Exclusive-Register-?lang=en)
    ///
    /// Store Exclusive Register stores a 32-bit word or a 64-bit doubleword from a register to memory if the PE has exclusive access to the memory address, and returns a status value of 0 if the store was successful, or of 1 if no store was performed. See Synchronization and semaphores. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// STXR <Ws>, <Xt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stxr_64(&mut self, ws: Register, xt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b11, 0, ws, 0, 0b11111, xn_sp, xt)
    }


    /// [STLXR - Store Release Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXR--Store-Release-Exclusive-Register-?lang=en)
    ///
    /// Store-Release Exclusive Register stores a 32-bit word or a 64-bit doubleword to memory if the PE has exclusive access to the memory address, from two registers, and returns a status value of 0 if the store was successful, or of 1 if no store was performed. See Synchronization and semaphores. The memory access is atomic. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLXR <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stlxr_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b10, 0, ws, 1, 0b11111, xn_sp, wt)
    }


    /// [STLXR - Store Release Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXR--Store-Release-Exclusive-Register-?lang=en)
    ///
    /// Store-Release Exclusive Register stores a 32-bit word or a 64-bit doubleword to memory if the PE has exclusive access to the memory address, from two registers, and returns a status value of 0 if the store was successful, or of 1 if no store was performed. See Synchronization and semaphores. The memory access is atomic. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLXR <Ws>, <Xt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stlxr_64(&mut self, ws: Register, xt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b11, 0, ws, 1, 0b11111, xn_sp, xt)
    }


    /// [LDXR - Load Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXR--Load-Exclusive-Register-?lang=en)
    ///
    /// Load Exclusive Register derives an address from a base register value, loads a 32-bit word or a 64-bit doubleword from memory, and writes it to a register. The memory access is atomic. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See Synchronization and semaphores. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDXR <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldxr_32(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b10, 1, 0b11111, 0, 0b11111, xn_sp, wt)
    }


    /// [LDXR - Load Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXR--Load-Exclusive-Register-?lang=en)
    ///
    /// Load Exclusive Register derives an address from a base register value, loads a 32-bit word or a 64-bit doubleword from memory, and writes it to a register. The memory access is atomic. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See Synchronization and semaphores. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDXR <Xt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldxr_64(&mut self, xt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b11, 1, 0b11111, 0, 0b11111, xn_sp, xt)
    }


    /// [LDAXR - Load Acquire Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXR--Load-Acquire-Exclusive-Register-?lang=en)
    ///
    /// Load-Acquire Exclusive Register derives an address from a base register value, loads a 32-bit word or 64-bit doubleword from memory, and writes it to a register. The memory access is atomic. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See Synchronization and semaphores. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDAXR <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldaxr_32(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b10, 1, 0b11111, 1, 0b11111, xn_sp, wt)
    }


    /// [LDAXR - Load Acquire Exclusive Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXR--Load-Acquire-Exclusive-Register-?lang=en)
    ///
    /// Load-Acquire Exclusive Register derives an address from a base register value, loads a 32-bit word or 64-bit doubleword from memory, and writes it to a register. The memory access is atomic. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See Synchronization and semaphores. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about memory accesses see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDAXR <Xt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldaxr_64(&mut self, xt: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_reg(self, 0b11, 1, 0b11111, 1, 0b11111, xn_sp, xt)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_stxrb() {
        let mut prod = TestProducer::new();

        let instr = prod.stxrb(3, 4, 5);
        assert_eq!(instr, "stxrb w3, w4, [x5]");
    }

    #[test]
    fn test_stlxrb() {
        let mut prod = TestProducer::new();

        let instr = prod.stlxrb(3, 4, 5);
        assert_eq!(instr, "stlxrb w3, w4, [x5]");
    }

    #[test]
    fn test_ldxrb() {
        let mut prod = TestProducer::new();

        let instr = prod.ldxrb(3, 4);
        assert_eq!(instr, "ldxrb w3, [x4]");
    }

    #[test]
    fn test_ldaxrb() {
        let mut prod = TestProducer::new();

        let instr = prod.ldaxrb(3, 4);
        assert_eq!(instr, "ldaxrb w3, [x4]");
    }

    #[test]
    fn test_stxrh() {
        let mut prod = TestProducer::new();

        let instr = prod.stxrh(3, 4, 5);
        assert_eq!(instr, "stxrh w3, w4, [x5]");
    }

    #[test]
    fn test_stlxrh() {
        let mut prod = TestProducer::new();

        let instr = prod.stlxrh(3, 4, 5);
        assert_eq!(instr, "stlxrh w3, w4, [x5]");
    }

    #[test]
    fn test_ldxrh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldxrh(3, 4);
        assert_eq!(instr, "ldxrh w3, [x4]");
    }

    #[test]
    fn test_ldaxrh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldaxrh(3, 4);
        assert_eq!(instr, "ldaxrh w3, [x4]");
    }

    #[test]
    fn test_stxr() {
        let mut prod = TestProducer::new();

        let instr = prod.stxr_32(3, 4, 5);
        assert_eq!(instr, "stxr w3, w4, [x5]");

        let instr = prod.stxr_64(3, 4, 5);
        assert_eq!(instr, "stxr w3, x4, [x5]");
    }


    #[test]
    fn test_stlxr() {
        let mut prod = TestProducer::new();

        let instr = prod.stlxr_32(3, 4, 5);
        assert_eq!(instr, "stlxr w3, w4, [x5]");

        let instr = prod.stlxr_64(3, 4, 5);
        assert_eq!(instr, "stlxr w3, x4, [x5]");
    }

    #[test]
    fn test_ldxr() {
        let mut prod = TestProducer::new();

        let instr = prod.ldxr_32(3, 4);
        assert_eq!(instr, "ldxr w3, [x4]");

        let instr = prod.ldxr_64(3, 4);
        assert_eq!(instr, "ldxr x3, [x4]");
    }

    #[test]
    fn test_ldaxr() {
        let mut prod = TestProducer::new();

        let instr = prod.ldaxr_32(3, 4);
        assert_eq!(instr, "ldaxr w3, [x4]");

        let instr = prod.ldaxr_64(3, 4);
        assert_eq!(instr, "ldaxr x3, [x4]");
    }
}