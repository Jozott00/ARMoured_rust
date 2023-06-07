//! # Compare and swap pair
//!
//! Implements the following instructions:
//! - [CASP 32bit](InstrStream::casp_32)
//! - [CASP 64bit](InstrStream::casp_64)
//! - [CASPA 32bit](InstrStream::caspa_32)
//! - [CASPA 64bit](InstrStream::caspa_64)
//! - [CASPAL 32bit](InstrStream::caspal_32)
//! - [CASPAL 64bit](InstrStream::caspal_64)
//! - [CASPL 32bit](InstrStream::caspl_32)
//! - [CASPL 64bit](InstrStream::caspl_64)

use bit_seq::bseq_32;
use crate::instruction_emitter::Emitter;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::instruction::Instr;
use crate::types::Register;

impl<'mem, M: Memory, E: Emitter> InstrStream<'mem, M, E> {
    /// Helper method that emits the Compare and Swap Pair instruction for the supplied parameters.
    ///
    /// # Arguments
    ///
    /// * `sz` - Size flag. Determines if instruction is 32 or 64 bits wide.
    /// * `L` - Sequential or atomic access flag.
    /// * `rs` - Source register pair.
    /// * `o0` - Load/Store order specifier.
    /// * `rn` - Base register. The memory address to be used for the operation is in this register.
    /// * `rt` - Target register pair.
    #[inline(always)]
    fn emit_casp_x(&mut self, sz: u8, L: u8, rs: Register, o0: u8, rn: Register, rt: Register) -> Instr {
        let r = bseq_32!(0 sz:1 0010000 L:1 1 rs:5 o0:1 !0:5 rn:5 rt:5);
        self.emit(r)
    }


    /// Emits a 32-bit wide Compare and Swap Pair (CASP) instruction. This instruction atomically loads a pair of
    /// words from memory, compares them with a pair of registers (ws1 and ws2), and if they are the same,
    /// stores a new pair of words from another register pair (wt1 and wt2) to memory.
    ///
    /// The method asserts that ws1 and wt1 registers are even and that ws2 and wt2 are the consecutive registers of ws1 and wt1.
    #[inline(always)]
    pub fn casp_32(&mut self, ws1: Register, ws2: Register, wt1: Register, wt2: Register, xn: Register) -> Instr {
        debug_assert!(ws1 % 2 == 0, "ws1 must be even");
        debug_assert!(ws2 == ws1 + 1, "ws2 must be the consecutive register of ws1");
        debug_assert!(wt1 % 2 == 0, "wt1 must be even");
        debug_assert!(wt2 == wt1 + 1, "wt2 must be the consecutive register of wt1");

        self.emit_casp_x(0, 0, ws1, 0, xn, wt1)
    }

    /// Emits a 64-bit wide Compare and Swap Pair (CASP) instruction. This instruction atomically loads a pair of
    /// double words from memory, compares them with a pair of registers (xs1 and xs2), and if they are the same,
    /// stores a new pair of double words from another register pair (xt1 and xt2) to memory.
    ///
    /// The method asserts that xs1 and xt1 registers are even and that xs2 and xt2 are the consecutive registers of xs1 and xt1.
    #[inline(always)]
    pub fn casp_64(&mut self, xs1: Register, xs2: Register, xt1: Register, xt2: Register, xn: Register) -> Instr {
        debug_assert!(xs1 % 2 == 0, "xs1 must be even");
        debug_assert!(xs2 == xs1 + 1, "xs2 must be the consecutive register of xs1");
        debug_assert!(xt1 % 2 == 0, "xt1 must be even");
        debug_assert!(xt2 == xt1 + 1, "xt2 must be the consecutive register of xt1");

        self.emit_casp_x(1, 0, xs1, 0, xn, xt1)
    }

    /// Emits a 32-bit wide Compare and Swap Pair Acquire (CASPA) instruction. The instruction provides a combining load/store operation
    /// that provides the necessary acquire semantics for synchronisation primitives.
    #[inline(always)]
    pub fn caspa_32(&mut self, ws1: Register, ws2: Register, wt1: Register, wt2: Register, xn: Register) -> Instr {
        debug_assert!(ws1 % 2 == 0, "ws1 must be even");
        debug_assert!(ws2 == ws1 + 1, "ws2 must be the consecutive register of ws1");
        debug_assert!(wt1 % 2 == 0, "wt1 must be even");
        debug_assert!(wt2 == wt1 + 1, "wt2 must be the consecutive register of wt1");

        self.emit_casp_x(0, 1, ws1, 0, xn, wt1)
    }

    /// Emits a 64-bit wide Compare and Swap Pair Acquire (CASPA) instruction. The instruction provides a combining load/store operation
    /// that provides the necessary acquire semantics for synchronisation primitives.
    #[inline(always)]
    pub fn caspa_64(&mut self, xs1: Register, xs2: Register, xt1: Register, xt2: Register, xn: Register) -> Instr {
        debug_assert!(xs1 % 2 == 0, "xs1 must be even");
        debug_assert!(xs2 == xs1 + 1, "xs2 must be the consecutive register of xs1");
        debug_assert!(xt1 % 2 == 0, "xt1 must be even");
        debug_assert!(xt2 == xt1 + 1, "xt2 must be the consecutive register of xt1");

        self.emit_casp_x(1, 1, xs1, 0, xn, xt1)
    }

    /// Emits a 32-bit wide Compare and Swap Pair Acquire, release (CASPAL) instruction. The instruction provides a combining load/store operation
    /// that provides the necessary acquire, release semantics for synchronisation primitives.
    #[inline(always)]
    pub fn caspal_32(&mut self, ws1: Register, ws2: Register, wt1: Register, wt2: Register, xn: Register) -> Instr {
        debug_assert!(ws1 % 2 == 0, "ws1 must be even");
        debug_assert!(ws2 == ws1 + 1, "ws2 must be the consecutive register of ws1");
        debug_assert!(wt1 % 2 == 0, "wt1 must be even");
        debug_assert!(wt2 == wt1 + 1, "wt2 must be the consecutive register of wt1");

        self.emit_casp_x(0, 1, ws1, 1, xn, wt1)
    }

    /// Emits a 64-bit wide Compare and Swap Pair Acquire and Release (CASPAL) instruction. The instruction provides a combining load/store operation
    /// that has special semantics for synchronized access to memory.
    ///
    /// CASPAL is intended for use in constructing higher-level synchronization primitives and operations such as semaphore
    /// acquire and release, and monitor enter and exit operations in a multiprocessor system.
    ///
    /// # Parameters
    ///
    /// - xs1: Even-numbered source register, containing the expected old value.
    /// - xs2: Consecutive source register to xs1, containing the expected old value.
    /// - xt1: Even-numbered source register, containing the new value to be stored.
    /// - xt2: Consecutive source register to xt1, containing the new value to be stored.
    /// - xn: Base register containing the memory address where the operation will take place.
    ///
    /// # Preconditions
    ///
    /// - Both source register pairs (xs1 and xs2, xt1 and xt2) should be consecutive and even-numbered.
    /// - The base register (xn) should contain a valid memory address.
    #[inline(always)]
    pub fn caspal_64(&mut self, xs1: Register, xs2: Register, xt1: Register, xt2: Register, xn: Register) -> Instr {
        debug_assert!(xs1 % 2 == 0, "xs1 must be even");
        debug_assert!(xs2 == xs1 + 1, "xs2 must be the consecutive register of xs1");
        debug_assert!(xt1 % 2 == 0, "xt1 must be even");
        debug_assert!(xt2 == xt1 + 1, "xt2 must be the consecutive register of xt1");

        self.emit_casp_x(1, 1, xs1, 1, xn, xt1)
    }

    /// Emits a 32-bit wide Compare and Swap Pair and Link (CASPL) instruction. The instruction provides a combining load/store operation
    /// that has special semantics for synchronized access to memory.
    ///
    /// CASPL is intended for use in constructing higher-level synchronization primitives and operations such as semaphore
    /// acquire and release, and monitor enter and exit operations in a multiprocessor system.
    #[inline(always)]
    pub fn caspl_32(&mut self, ws1: Register, ws2: Register, wt1: Register, wt2: Register, xn: Register) -> Instr {
        debug_assert!(ws1 % 2 == 0, "ws1 must be even");
        debug_assert!(ws2 == ws1 + 1, "ws2 must be the consecutive register of ws1");
        debug_assert!(wt1 % 2 == 0, "wt1 must be even");
        debug_assert!(wt2 == wt1 + 1, "wt2 must be the consecutive register of wt1");

        self.emit_casp_x(0, 0, ws1, 1, xn, wt1)
    }

    /// Emits a 64-bit wide Compare and Swap Pair and Link (CASPL) instruction. The instruction provides a combining load/store operation
    /// that has special semantics for synchronized access to memory.
    ///
    /// CASPL is intended for use in constructing higher-level synchronization primitives and operations such as semaphore
    /// acquire and release, and monitor enter and exit operations in a multiprocessor system.
    #[inline(always)]
    pub fn caspl_64(&mut self, xs1: Register, xs2: Register, xt1: Register, xt2: Register, xn: Register) -> Instr {
        debug_assert!(xs1 % 2 == 0, "xs1 must be even");
        debug_assert!(xs2 == xs1 + 1, "xs2 must be the consecutive register of xs1");
        debug_assert!(xt1 % 2 == 0, "xt1 must be even");
        debug_assert!(xt2 == xt1 + 1, "xt2 must be the consecutive register of xt1");

        self.emit_casp_x(1, 0, xs1, 1, xn, xt1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mc_memory::MockMemory;
    use crate::instruction_emitter::MockEmitter;
    use crate::{stream_mock, assert_panic};
    use crate::types::InstructionPointer;


    #[test]
    fn test_casp() {
        stream_mock!(stream, {
            let instr = stream.casp_32(0, 1, 4, 5, 0);
            assert_eq!(instr.to_string(), "casp w0, w1, w4, w5, [x0]");

            let instr = stream.casp_64(0, 1, 4, 5, 0);
            assert_eq!(instr.to_string(), "casp x0, x1, x4, x5, [x0]");


            // panic 32
            assert_panic!("Should panic because of odd ws1"; {
                stream.casp_32(1, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because ws2 not ws1+1 "; {
                stream.casp_32(0, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because of odd wt1"; {
                stream.casp_32(1, 2, 3, 5, 0)
            });
            assert_panic!("Should panic because wt2 not wt1+1 "; {
                stream.casp_32(0, 2, 4, 6, 0)
            });


            // panic 64
            assert_panic!("Should panic because of odd ws1"; {
                stream.casp_64(1, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because ws2 not ws1+1 "; {
                stream.casp_64(0, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because of odd wt1"; {
                stream.casp_64(1, 2, 3, 5, 0)
            });
            assert_panic!("Should panic because wt2 not wt1+1 "; {
                stream.casp_64(0, 2, 4, 6, 0)
            });

        });
    }

    #[test]
    fn test_caspa() {
        stream_mock!(stream, {
            let instr = stream.caspa_32(0, 1, 4, 5, 0);
            assert_eq!(instr.to_string(), "caspa w0, w1, w4, w5, [x0]");

            let instr = stream.caspa_64(0, 1, 4, 5, 0);
            assert_eq!(instr.to_string(), "caspa x0, x1, x4, x5, [x0]");


            // panic 32
            assert_panic!("Should panic because of odd ws1"; {
                stream.caspa_32(1, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because ws2 not ws1+1 "; {
                stream.caspa_32(0, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because of odd wt1"; {
                stream.caspa_32(1, 2, 3, 5, 0)
            });
            assert_panic!("Should panic because wt2 not wt1+1 "; {
                stream.caspa_32(0, 2, 4, 6, 0)
            });


            // panic 64
            assert_panic!("Should panic because of odd ws1"; {
                stream.caspa_64(1, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because ws2 not ws1+1 "; {
                stream.caspa_64(0, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because of odd wt1"; {
                stream.caspa_64(1, 2, 3, 5, 0)
            });
            assert_panic!("Should panic because wt2 not wt1+1 "; {
                stream.caspa_64(0, 2, 4, 6, 0)
            });
        });
    }

    #[test]
    fn test_caspal() {
        stream_mock!(stream, {
            let instr = stream.caspal_32(0, 1, 4, 5, 0);
            assert_eq!(instr.to_string(), "caspal w0, w1, w4, w5, [x0]");

            let instr = stream.caspal_64(0, 1, 4, 5, 0);
            assert_eq!(instr.to_string(), "caspal x0, x1, x4, x5, [x0]");


            // panic 32
            assert_panic!("Should panic because of odd ws1"; {
                stream.caspal_32(1, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because ws2 not ws1+1 "; {
                stream.caspal_32(0, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because of odd wt1"; {
                stream.caspal_32(1, 2, 3, 5, 0)
            });
            assert_panic!("Should panic because wt2 not wt1+1 "; {
                stream.caspal_32(0, 2, 4, 6, 0)
            });


            // panic 64
            assert_panic!("Should panic because of odd ws1"; {
                stream.caspal_64(1, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because ws2 not ws1+1 "; {
                stream.caspal_64(0, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because of odd wt1"; {
                stream.caspal_64(1, 2, 3, 5, 0)
            });
            assert_panic!("Should panic because wt2 not wt1+1 "; {
                stream.caspal_64(0, 2, 4, 6, 0)
            });
        });
    }

    #[test]
    fn test_caspl() {
        stream_mock!(stream, {
            let instr = stream.caspl_32(0, 1, 4, 5, 0);
            assert_eq!(instr.to_string(), "caspl w0, w1, w4, w5, [x0]");

            let instr = stream.caspl_64(0, 1, 4, 5, 0);
            assert_eq!(instr.to_string(), "caspl x0, x1, x4, x5, [x0]");


            // panic 32
            assert_panic!("Should panic because of odd ws1"; {
                stream.caspl_32(1, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because ws2 not ws1+1 "; {
                stream.caspl_32(0, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because of odd wt1"; {
                stream.caspl_32(1, 2, 3, 5, 0)
            });
            assert_panic!("Should panic because wt2 not wt1+1 "; {
                stream.caspl_32(0, 2, 4, 6, 0)
            });


            // panic 64
            assert_panic!("Should panic because of odd ws1"; {
                stream.caspl_64(1, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because ws2 not ws1+1 "; {
                stream.caspl_64(0, 2, 4, 5, 0)
            });
            assert_panic!("Should panic because of odd wt1"; {
                stream.caspl_64(1, 2, 3, 5, 0)
            });
            assert_panic!("Should panic because wt2 not wt1+1 "; {
                stream.caspl_64(0, 2, 4, 6, 0)
            });
        });
    }
}