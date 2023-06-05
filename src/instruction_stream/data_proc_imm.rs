//! # Data processing - immediate instructions
//!
//! It consists of
//! - Add/subtract (immediate)
//! - Logical (immediate)
//! - Move wide (immediate)
//! - Bitfield
//! - Extract

use bit_seq::{bseq_32};
use crate::instruction_stream::InstrStream;
use crate::types::{Register};

pub mod mov_wide_imm {
    //! # Move wide (immediate)
    //!
    //! - MOVN 32bit
    //! - MOVZ 32bit
    //! - MOVK 32bit
    //! - MOVN 64bit
    //! - MOVZ 64bit
    //! - MOVK 64bit

    use bit_seq::bseq_32;
    use crate::instruction_stream::InstrStream;
    use crate::types::{Register, HW, Imm16};

    impl<'mem> InstrStream<'mem> {
        // move somewhere else
        #[inline(always)]
        pub fn mov_reg(&mut self, sf: bool, dest: Register, src: Register) {
            let sf_i = if sf { 1 } else { 0 };
            let r = bseq_32!(sf_i:1 0101010 0:3 src:5 0:6 !0:5 dest:5);
            self.emit(r);
        }

        /// Internal function used to encode `MOV` instructions with immediate values.
        /// Parameters:
        /// * `sf`: defines whether the operation is 32-bit or 64-bit.
        /// * `opc`: defines the specific `MOV` operation.
        /// * `hw`: provides the shift amount.
        /// * `d`: specifies the destination register.
        /// * `imm`: specifies the immediate value.
        #[inline(always)]
        fn mov_x_imm(&mut self, sf: u8, opc: u8, hw: u8, d: u8, imm: u16) {
            let r = bseq_32!(sf:1 opc:2 100101 hw:2 imm:16 d:5);
            self.emit(r);
        }

        // MOVN 32 and 64

        /// Generates a `MOVN` (Move Not) instruction. The `sf` parameter indicates whether it's a 32-bit (`false`) or 64-bit (`true`) operation.
        /// * `d`: destination register.
        /// * `imm`: immediate value.
        /// * `lsl`: shift amount as defined by the `HW` enum.
        #[inline(always)]
        pub fn movn_imm(&mut self, sf: bool, d: Register, imm: Imm16, lsl: HW) {
            self.mov_x_imm(sf.into(), 0b00, lsl.into(), d, imm)
        }

        /// Generates a 32-bit `MOVN` (Move Not) instruction.
        #[inline(always)]
        pub fn movn_32_imm(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movn_imm(false, d, imm, lsl);
        }

        /// Generates a 64-bit `MOVN` (Move Not) instruction.
        #[inline(always)]
        pub fn movn_64_imm(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movn_imm(true, d, imm, lsl);
        }

        // MOVZ 32 and 64

        /// Generates a `MOVZ` (Move Zero) instruction. The `sf` parameter indicates whether it's a 32-bit (`false`) or 64-bit (`true`) operation.
        #[inline(always)]
        pub fn movz_imm(&mut self, sf: bool, d: Register, imm: Imm16, lsl: HW) {
            self.mov_x_imm(sf.into(), 0b10, lsl.into(), d, imm)
        }

        /// Generates a 32-bit `MOVZ` (Move Zero) instruction.
        #[inline(always)]
        pub fn movz_32_imm(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movz_imm(false, d, imm, lsl);
        }

        /// Generates a 64-bit `MOVZ` (Move Zero) instruction.
        #[inline(always)]
        pub fn movz_64_imm(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movz_imm(true, d, imm, lsl);
        }

        // MOVK 32 and 64

        /// Generates a `MOVK` (Move Keep) instruction. The `sf` parameter indicates whether it's a 32-bit (`false`) or 64-bit (`true`) operation.
        #[inline(always)]
        pub fn movk_imm(&mut self, sf: bool, d: Register, imm: Imm16, lsl: HW) {
            self.mov_x_imm(sf.into(), 0b11, lsl.into(), d, imm)
        }

        /// Generates a 32-bit `MOVK` (Move Keep) instruction.
        #[inline(always)]
        pub fn movk_32_imm(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movk_imm(false, d, imm, lsl);
        }

        /// Generates a 64-bit `MOVK` (Move Keep) instruction.
        #[inline(always)]
        pub fn movk_64_imm(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movk_imm(true, d, imm, lsl);
        }


        /// An alias of the `MOVZ` 32 bit instruction
        #[inline(always)]
        pub fn mov_32_imm(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movz_imm(false, d, imm, lsl);
        }

        /// An alias of the `MOVZ` 64 bit instruction
        #[inline(always)]
        pub fn mov_64_imm(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movz_imm(true, d, imm, lsl);
        }
    }
}
