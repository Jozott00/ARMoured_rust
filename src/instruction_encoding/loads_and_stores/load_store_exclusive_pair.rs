//! # [Load/store exclusive pair](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldstexclp)
//!
//! Implements the following instructions:
//!  - [STXP - Store Exclusive Pair of registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXP--Store-Exclusive-Pair-of-registers-?lang=en)
//!  - [STLXP - Store Release Exclusive Pair of registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXP--Store-Release-Exclusive-Pair-of-registers-?lang=en)
//!  - [LDXP - Load Exclusive Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXP--Load-Exclusive-Pair-of-Registers-?lang=en)
//!  - [LDAXP - Load Acquire Exclusive Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXP--Load-Acquire-Exclusive-Pair-of-Registers-?lang=en)



use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::Register;

#[inline(always)]
fn emit_ldr_str_excl_pair<P: InstructionProcessor<T>, T>(proc: &mut P, sz: u8, l: u8, rs: Register, o0: u8, rt2: Register, rn: Register, rt: Register) -> T {
    let r = bseq_32!(1 sz:1 0010000 l:1 1 rs:5 o0:1 rt2:5 rn:5 rt:5);
    proc.process(r)
}


/// # [Load/store exclusive pair](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldstexclp)
///
/// Implements the following instructions:
///  - [STXP - Store Exclusive Pair of registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXP--Store-Exclusive-Pair-of-registers-?lang=en)
///  - [STLXP - Store Release Exclusive Pair of registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXP--Store-Release-Exclusive-Pair-of-registers-?lang=en)
///  - [LDXP - Load Exclusive Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXP--Load-Exclusive-Pair-of-Registers-?lang=en)
///  - [LDAXP - Load Acquire Exclusive Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXP--Load-Acquire-Exclusive-Pair-of-Registers-?lang=en)
pub trait LoadStoreExclusivePair<T>: InstructionProcessor<T> {
    /// [STXP - Store Exclusive Pair of registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXP--Store-Exclusive-Pair-of-registers-?lang=en)
    ///
    /// Store Exclusive Pair of registers stores two 32-bit words or two 64-bit doublewords from two registers to a memory location if the PE has exclusive access to the memory address, and returns a status value of 0 if the store was successful, or of 1 if no store was performed. See Synchronization and semaphores. For information on single-copy atomicity and alignment requirements, see Requirements for single-copy atomicity and Alignment of data accesses. If a 64-bit pair Store-Exclusive succeeds, it causes a single-copy atomic update of the 128-bit memory location being updated. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STXP <Ws>, <Wt1>, <Wt2>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stxp_32(&mut self, ws: Register, wt1: Register, wt2: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_pair(self, 0, 0, ws, 0, wt2, xn_sp, wt1)
    }


    /// [STXP - Store Exclusive Pair of registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STXP--Store-Exclusive-Pair-of-registers-?lang=en)
    ///
    /// Store Exclusive Pair of registers stores two 32-bit words or two 64-bit doublewords from two registers to a memory location if the PE has exclusive access to the memory address, and returns a status value of 0 if the store was successful, or of 1 if no store was performed. See Synchronization and semaphores. For information on single-copy atomicity and alignment requirements, see Requirements for single-copy atomicity and Alignment of data accesses. If a 64-bit pair Store-Exclusive succeeds, it causes a single-copy atomic update of the 128-bit memory location being updated. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STXP <Ws>, <Xt1>, <Xt2>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stxp_64(&mut self, ws: Register, xt1: Register, xt2: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_pair(self, 1, 0, ws, 0, xt2, xn_sp, xt1)
    }


    /// [STLXP - Store Release Exclusive Pair of registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXP--Store-Release-Exclusive-Pair-of-registers-?lang=en)
    ///
    /// Store-Release Exclusive Pair of registers stores two 32-bit words or two 64-bit doublewords to a memory location if the PE has exclusive access to the memory address, from two registers, and returns a status value of 0 if the store was successful, or of 1 if no store was performed. See Synchronization and semaphores. For information on single-copy atomicity and alignment requirements, see Requirements for single-copy atomicity and Alignment of data accesses. If a 64-bit pair Store-Exclusive succeeds, it causes a single-copy atomic update of the 128-bit memory location being updated. The instruction also has memory ordering semantics, as described in Load-Acquire, Store-Release. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLXP <Ws>, <Wt1>, <Wt2>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stlxp_32(&mut self, ws: Register, wt1: Register, wt2: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_pair(self, 0, 0, ws, 1, wt2, xn_sp, wt1)
    }


    /// [STLXP - Store Release Exclusive Pair of registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STLXP--Store-Release-Exclusive-Pair-of-registers-?lang=en)
    ///
    /// Store-Release Exclusive Pair of registers stores two 32-bit words or two 64-bit doublewords to a memory location if the PE has exclusive access to the memory address, from two registers, and returns a status value of 0 if the store was successful, or of 1 if no store was performed. See Synchronization and semaphores. For information on single-copy atomicity and alignment requirements, see Requirements for single-copy atomicity and Alignment of data accesses. If a 64-bit pair Store-Exclusive succeeds, it causes a single-copy atomic update of the 128-bit memory location being updated. The instruction also has memory ordering semantics, as described in Load-Acquire, Store-Release. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// STLXP <Ws>, <Xt1>, <Xt2>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn stlxp_64(&mut self, ws: Register, xt1: Register, xt2: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_pair(self, 1, 0, ws, 1, xt2, xn_sp, xt1)
    }


    /// [LDXP - Load Exclusive Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXP--Load-Exclusive-Pair-of-Registers-?lang=en)
    ///
    /// Load Exclusive Pair of Registers derives an address from a base register value, loads two 32-bit words or two 64-bit doublewords from memory, and writes them to two registers. For information on single-copy atomicity and alignment requirements, see Requirements for single-copy atomicity and Alignment of data accesses. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See Synchronization and semaphores. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDXP <Wt1>, <Wt2>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldxp_32(&mut self, wt1: Register, wt2: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_pair(self, 0, 1, 0b11111, 0, wt2, xn_sp, wt1)
    }


    /// [LDXP - Load Exclusive Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDXP--Load-Exclusive-Pair-of-Registers-?lang=en)
    ///
    /// Load Exclusive Pair of Registers derives an address from a base register value, loads two 32-bit words or two 64-bit doublewords from memory, and writes them to two registers. For information on single-copy atomicity and alignment requirements, see Requirements for single-copy atomicity and Alignment of data accesses. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See Synchronization and semaphores. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDXP <Xt1>, <Xt2>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldxp_64(&mut self, xt1: Register, xt2: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_pair(self, 1, 1, 0b11111, 0, xt2, xn_sp, xt1)
    }


    /// [LDAXP - Load Acquire Exclusive Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXP--Load-Acquire-Exclusive-Pair-of-Registers-?lang=en)
    ///
    /// Load-Acquire Exclusive Pair of Registers derives an address from a base register value, loads two 32-bit words or two 64-bit doublewords from memory, and writes them to two registers. For information on single-copy atomicity and alignment requirements, see Requirements for single-copy atomicity and Alignment of data accesses. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See Synchronization and semaphores. The instruction also has memory ordering semantics, as described in Load-Acquire, Store-Release. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDAXP <Wt1>, <Wt2>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldaxp_32(&mut self, wt1: Register, wt2: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_pair(self, 0, 1, 0b11111, 1, wt2, xn_sp, wt1)
    }


    /// [LDAXP - Load Acquire Exclusive Pair of Registers](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAXP--Load-Acquire-Exclusive-Pair-of-Registers-?lang=en)
    ///
    /// Load-Acquire Exclusive Pair of Registers derives an address from a base register value, loads two 32-bit words or two 64-bit doublewords from memory, and writes them to two registers. For information on single-copy atomicity and alignment requirements, see Requirements for single-copy atomicity and Alignment of data accesses. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See Synchronization and semaphores. The instruction also has memory ordering semantics, as described in Load-Acquire, Store-Release. For information about memory accesses, see Load/Store addressing modes.
    ///
    /// ```asm
    /// LDAXP <Xt1>, <Xt2>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn ldaxp_64(&mut self, xt1: Register, xt2: Register, xn_sp: Register) -> T {
        emit_ldr_str_excl_pair(self, 1, 1, 0b11111, 1, xt2, xn_sp, xt1)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_stxp() {
        let mut prod = TestProducer::new();

        let instr = prod.stxp_32(3, 4, 5, 6);
        assert_eq!(instr, "stxp w3, w4, w5, [x6]");

        let instr = prod.stxp_64(3, 4, 5, 6);
        assert_eq!(instr, "stxp w3, x4, x5, [x6]");
    }

    #[test]
    fn test_stlxp() {
        let mut prod = TestProducer::new();

        let instr = prod.stlxp_32(3, 4, 5, 6);
        assert_eq!(instr, "stlxp w3, w4, w5, [x6]");

        let instr = prod.stlxp_64(3, 4, 5, 6);
        assert_eq!(instr, "stlxp w3, x4, x5, [x6]");
    }

    #[test]
    fn test_ldxp() {
        let mut prod = TestProducer::new();

        let instr = prod.ldxp_32(4, 5, 6);
        assert_eq!(instr, "ldxp w4, w5, [x6]");

        let instr = prod.ldxp_64(4, 5, 6);
        assert_eq!(instr, "ldxp x4, x5, [x6]");
    }

    #[test]
    fn test_ldaxp() {
        let mut prod = TestProducer::new();

        let instr = prod.ldaxp_32(4, 5, 6);
        assert_eq!(instr, "ldaxp w4, w5, [x6]");

        let instr = prod.ldaxp_64(4, 5, 6);
        assert_eq!(instr, "ldaxp x4, x5, [x6]");
    }
}