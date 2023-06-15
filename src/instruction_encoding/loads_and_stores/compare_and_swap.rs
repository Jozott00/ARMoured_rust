//! # [Compare and swap](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#comswap)
//!
//! Implements the following instructions:
//!  - [CASB - CASAB - CASALB - CASLB - Compare and Swap byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CASB--CASAB--CASALB--CASLB--Compare-and-Swap-byte-in-memory-?lang=en)
//!  - [CASH - CASAH - CASALH - CASLH - Compare and Swap halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CASH--CASAH--CASALH--CASLH--Compare-and-Swap-halfword-in-memory-?lang=en)
//!  - [CAS - CASA - CASAL - CASL - Compare and Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CAS--CASA--CASAL--CASL--Compare-and-Swap-word-or-doubleword-in-memory-?lang=en)



use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::Register;

#[inline(always)]
fn emit_cmp_swap<P: InstructionProcessor<T>, T>(proc: &mut P, size: u8, l: u8, rs: Register, o0: u8, rt2: Register, rn: Register, rt: Register) -> T {
    let r = bseq_32!(size:2 0010001 l:1 1 rs:5 o0:1 rt2:5 rn:5 rt:5);
    proc.process(r)
}

/// # [Compare and swap](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#comswap)
///
/// Implements the following instructions:
///  - [CASB - CASAB - CASALB - CASLB - Compare and Swap byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CASB--CASAB--CASALB--CASLB--Compare-and-Swap-byte-in-memory-?lang=en)
///  - [CASH - CASAH - CASALH - CASLH - Compare and Swap halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CASH--CASAH--CASALH--CASLH--Compare-and-Swap-halfword-in-memory-?lang=en)
///  - [CAS - CASA - CASAL - CASL - Compare and Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CAS--CASA--CASAL--CASL--Compare-and-Swap-word-or-doubleword-in-memory-?lang=en)
pub trait CompareAndSwap<T>: InstructionProcessor<T> {
    /// [CASB - CASAB - CASALB - CASLB - Compare and Swap byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CASB--CASAB--CASALB--CASLB--Compare-and-Swap-byte-in-memory-?lang=en)
    ///
    /// Compare and Swap byte in memory reads an 8-bit byte from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASAB <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn casab(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b00, 1, ws, 0, 0b11111, xn_sp, wt)
    }


    /// [CASB - CASAB - CASALB - CASLB - Compare and Swap byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CASB--CASAB--CASALB--CASLB--Compare-and-Swap-byte-in-memory-?lang=en)
    ///
    /// Compare and Swap byte in memory reads an 8-bit byte from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASALB <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn casalb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b00, 1, ws, 1, 0b11111, xn_sp, wt)
    }


    /// [CASB - CASAB - CASALB - CASLB - Compare and Swap byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CASB--CASAB--CASALB--CASLB--Compare-and-Swap-byte-in-memory-?lang=en)
    ///
    /// Compare and Swap byte in memory reads an 8-bit byte from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASB <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn casb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b00, 0, ws, 0, 0b11111, xn_sp, wt)
    }


    /// [CASB - CASAB - CASALB - CASLB - Compare and Swap byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CASB--CASAB--CASALB--CASLB--Compare-and-Swap-byte-in-memory-?lang=en)
    ///
    /// Compare and Swap byte in memory reads an 8-bit byte from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASLB <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn caslb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b00, 0, ws, 1, 0b11111, xn_sp, wt)
    }


    /// [CASH - CASAH - CASALH - CASLH - Compare and Swap halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CASH--CASAH--CASALH--CASLH--Compare-and-Swap-halfword-in-memory-?lang=en)
    ///
    /// Compare and Swap halfword in memory reads a 16-bit halfword from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASAH <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn casah(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b01, 1, ws, 0, 0b11111, xn_sp, wt)
    }


    /// [CASH - CASAH - CASALH - CASLH - Compare and Swap halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CASH--CASAH--CASALH--CASLH--Compare-and-Swap-halfword-in-memory-?lang=en)
    ///
    /// Compare and Swap halfword in memory reads a 16-bit halfword from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASALH <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn casalh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b01, 1, ws, 1, 0b11111, xn_sp, wt)
    }


    /// [CASH - CASAH - CASALH - CASLH - Compare and Swap halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CASH--CASAH--CASALH--CASLH--Compare-and-Swap-halfword-in-memory-?lang=en)
    ///
    /// Compare and Swap halfword in memory reads a 16-bit halfword from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASH <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn cash(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b01, 0, ws, 0, 0b11111, xn_sp, wt)
    }


    /// [CASH - CASAH - CASALH - CASLH - Compare and Swap halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CASH--CASAH--CASALH--CASLH--Compare-and-Swap-halfword-in-memory-?lang=en)
    ///
    /// Compare and Swap halfword in memory reads a 16-bit halfword from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASLH <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn caslh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b01, 0, ws, 1, 0b11111, xn_sp, wt)
    }


    /// [CAS - CASA - CASAL - CASL - Compare and Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CAS--CASA--CASAL--CASL--Compare-and-Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Compare and Swap word or doubleword in memory reads a 32-bit word or 64-bit doubleword from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CAS <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn cas_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b10, 0, ws, 0, 0b11111, xn_sp, wt)
    }


    /// [CAS - CASA - CASAL - CASL - Compare and Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CAS--CASA--CASAL--CASL--Compare-and-Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Compare and Swap word or doubleword in memory reads a 32-bit word or 64-bit doubleword from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASA <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn casa_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b10, 1, ws, 0, 0b11111, xn_sp, wt)
    }


    /// [CAS - CASA - CASAL - CASL - Compare and Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CAS--CASA--CASAL--CASL--Compare-and-Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Compare and Swap word or doubleword in memory reads a 32-bit word or 64-bit doubleword from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASAL <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn casal_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b10, 1, ws, 1, 0b11111, xn_sp, wt)
    }


    /// [CAS - CASA - CASAL - CASL - Compare and Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CAS--CASA--CASAL--CASL--Compare-and-Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Compare and Swap word or doubleword in memory reads a 32-bit word or 64-bit doubleword from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASL <Ws>, <Wt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn casl_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b10, 0, ws, 1, 0b11111, xn_sp, wt)
    }


    /// [CAS - CASA - CASAL - CASL - Compare and Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CAS--CASA--CASAL--CASL--Compare-and-Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Compare and Swap word or doubleword in memory reads a 32-bit word or 64-bit doubleword from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CAS <Xs>, <Xt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn cas_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b11, 0, xs, 0, 0b11111, xn_sp, xt)
    }


    /// [CAS - CASA - CASAL - CASL - Compare and Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CAS--CASA--CASAL--CASL--Compare-and-Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Compare and Swap word or doubleword in memory reads a 32-bit word or 64-bit doubleword from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASA <Xs>, <Xt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn casa_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b11, 1, xs, 0, 0b11111, xn_sp, xt)
    }


    /// [CAS - CASA - CASAL - CASL - Compare and Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CAS--CASA--CASAL--CASL--Compare-and-Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Compare and Swap word or doubleword in memory reads a 32-bit word or 64-bit doubleword from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASAL <Xs>, <Xt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn casal_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b11, 1, xs, 1, 0b11111, xn_sp, xt)
    }


    /// [CAS - CASA - CASAL - CASL - Compare and Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CAS--CASA--CASAL--CASL--Compare-and-Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Compare and Swap word or doubleword in memory reads a 32-bit word or 64-bit doubleword from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// CASL <Xs>, <Xt>, [<Xn|SP>{,#0}]
    /// ```
    #[inline(always)]
    fn casl_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_cmp_swap(self, 0b11, 0, xs, 1, 0b11111, xn_sp, xt)
    }
}


#[cfg(test)]
mod tests {
    use crate::assert_panic;
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[test]
    fn test_casb() {
        let mut prod = TestProducer::new();

        let instr = prod.casb(3, 4, 5);
        assert_eq!(instr, "casb w3, w4, [x5]");

        let instr = prod.caslb(3, 4, 5);
        assert_eq!(instr, "caslb w3, w4, [x5]");

        let instr = prod.casab(3, 4, 5);
        assert_eq!(instr, "casab w3, w4, [x5]");

        let instr = prod.casalb(3, 4, 5);
        assert_eq!(instr, "casalb w3, w4, [x5]");
    }

    #[test]
    fn test_cash() {
        let mut prod = TestProducer::new();

        let instr = prod.cash(3, 4, 5);
        assert_eq!(instr, "cash w3, w4, [x5]");

        let instr = prod.caslh(3, 4, 5);
        assert_eq!(instr, "caslh w3, w4, [x5]");

        let instr = prod.casah(3, 4, 5);
        assert_eq!(instr, "casah w3, w4, [x5]");

        let instr = prod.casalh(3, 4, 5);
        assert_eq!(instr, "casalh w3, w4, [x5]");
    }

    #[test]
    fn test_cas_32() {
        let mut prod = TestProducer::new();

        let instr = prod.cas_32(3, 4, 5);
        assert_eq!(instr, "cas w3, w4, [x5]");

        let instr = prod.casl_32(3, 4, 5);
        assert_eq!(instr, "casl w3, w4, [x5]");

        let instr = prod.casa_32(3, 4, 5);
        assert_eq!(instr, "casa w3, w4, [x5]");

        let instr = prod.casal_32(3, 4, 5);
        assert_eq!(instr, "casal w3, w4, [x5]");
    }

    #[test]
    fn test_cas_64() {
        let mut prod = TestProducer::new();

        let instr = prod.cas_64(3, 4, 5);
        assert_eq!(instr, "cas x3, x4, [x5]");

        let instr = prod.casl_64(3, 4, 5);
        assert_eq!(instr, "casl x3, x4, [x5]");

        let instr = prod.casa_64(3, 4, 5);
        assert_eq!(instr, "casa x3, x4, [x5]");

        let instr = prod.casal_64(3, 4, 5);
        assert_eq!(instr, "casal x3, x4, [x5]");
    }
}
