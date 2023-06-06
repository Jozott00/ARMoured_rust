//! # Compare and swap pair
//!
//! Implements the following instructions:
//! - [CASP](InstrStream::casp)

use bit_seq::bseq_32;
use crate::instruction_emitter::Emitter;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::instruction::Instr;
use crate::types::Register;

impl<'mem, M: Memory, E: Emitter> InstrStream<'mem, M, E> {
    #[inline(always)]
    fn emit_casp_x(&mut self, sz: u8, L: u8, rs: Register, o0: u8, rn: Register, rt: Register) -> Instr {
        let r = bseq_32!(0 sz:1 0010000 L:1 1 rs:5 o0:1 !0:5 rn:5 rt:5);
        self.emit(r)
    }


    #[inline(always)]
    pub fn casp_32(&mut self, ws1: Register, ws2: Register, wt1: Register, wt2: Register, xn: Register) -> Instr {
        debug_assert!(ws1 % 2 == 0, "ws1 must be even");
        debug_assert!(ws2 == ws1 + 1, "ws2 must be the consecutive register of ws1");
        debug_assert!(wt1 % 2 == 0, "wt1 must be even");
        debug_assert!(wt2 == wt1 + 1, "wt2 must be the consecutive register of wt1");

        self.emit_casp_x(0, 0, ws1, 0, xn, wt1)
    }

    #[inline(always)]
    pub fn casp_64(&mut self, xs1: Register, xs2: Register, xt1: Register, xt2: Register, xn: Register) -> Instr {
        debug_assert!(xs1 % 2 == 0, "xs1 must be even");
        debug_assert!(xs2 == xs1 + 1, "xs2 must be the consecutive register of xs1");
        debug_assert!(xt1 % 2 == 0, "xt1 must be even");
        debug_assert!(xt2 == xt1 + 1, "xt2 must be the consecutive register of xt1");

        self.emit_casp_x(1, 0, xs1, 0, xn, xt1)
    }

    #[inline(always)]
    pub fn caspa_32(&mut self, ws1: Register, ws2: Register, wt1: Register, wt2: Register, xn: Register) -> Instr {
        debug_assert!(ws1 % 2 == 0, "ws1 must be even");
        debug_assert!(ws2 == ws1 + 1, "ws2 must be the consecutive register of ws1");
        debug_assert!(wt1 % 2 == 0, "wt1 must be even");
        debug_assert!(wt2 == wt1 + 1, "wt2 must be the consecutive register of wt1");

        self.emit_casp_x(0, 1, ws1, 0, xn, wt1)
    }

    #[inline(always)]
    pub fn caspa_64(&mut self, xs1: Register, xs2: Register, xt1: Register, xt2: Register, xn: Register) -> Instr {
        debug_assert!(xs1 % 2 == 0, "xs1 must be even");
        debug_assert!(xs2 == xs1 + 1, "xs2 must be the consecutive register of xs1");
        debug_assert!(xt1 % 2 == 0, "xt1 must be even");
        debug_assert!(xt2 == xt1 + 1, "xt2 must be the consecutive register of xt1");

        self.emit_casp_x(1, 1, xs1, 0, xn, xt1)
    }

    #[inline(always)]
    pub fn caspal_32(&mut self, ws1: Register, ws2: Register, wt1: Register, wt2: Register, xn: Register) -> Instr {
        debug_assert!(ws1 % 2 == 0, "ws1 must be even");
        debug_assert!(ws2 == ws1 + 1, "ws2 must be the consecutive register of ws1");
        debug_assert!(wt1 % 2 == 0, "wt1 must be even");
        debug_assert!(wt2 == wt1 + 1, "wt2 must be the consecutive register of wt1");

        self.emit_casp_x(0, 1, ws1, 1, xn, wt1)
    }

    #[inline(always)]
    pub fn caspal_64(&mut self, xs1: Register, xs2: Register, xt1: Register, xt2: Register, xn: Register) -> Instr {
        debug_assert!(xs1 % 2 == 0, "xs1 must be even");
        debug_assert!(xs2 == xs1 + 1, "xs2 must be the consecutive register of xs1");
        debug_assert!(xt1 % 2 == 0, "xt1 must be even");
        debug_assert!(xt2 == xt1 + 1, "xt2 must be the consecutive register of xt1");

        self.emit_casp_x(1, 1, xs1, 1, xn, xt1)
    }

    #[inline(always)]
    pub fn caspl_32(&mut self, ws1: Register, ws2: Register, wt1: Register, wt2: Register, xn: Register) -> Instr {
        debug_assert!(ws1 % 2 == 0, "ws1 must be even");
        debug_assert!(ws2 == ws1 + 1, "ws2 must be the consecutive register of ws1");
        debug_assert!(wt1 % 2 == 0, "wt1 must be even");
        debug_assert!(wt2 == wt1 + 1, "wt2 must be the consecutive register of wt1");

        self.emit_casp_x(0, 0, ws1, 1, xn, wt1)
    }

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



    #[test]
    fn test_casp_32() {

    }
}