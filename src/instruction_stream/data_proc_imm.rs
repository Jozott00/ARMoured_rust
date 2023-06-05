//! # Data processing - immediate instructions
//!
//! It consists of
//! - Add/subtract (immediate)
//! - Logical (immediate)
//! - Move wide (immediate)
//! - Bitfield
//! - Extract

pub mod add_subtract_imm {
    //! # Add/subtract (immediate)
    //!
    //! - ADD 32bit
    //! - ADDS 32bit
    //! - SUB 32bit
    //! - SUBS 32bit
    //! - ADD 64bit
    //! - ADDS 64bit
    //! - SUB 64bit
    //! - SUBS 64bit

    use bit_seq::bseq_32;

    use crate::instruction_stream::InstrStream;
    use crate::types::{Imm12, Register};
    use crate::types::shifts::Shift1;

    impl<'mem> InstrStream<'mem> {
        /// The `add_sub_imm` function is a helper function to generate ADD/SUB instructions
        /// (with immediate value) according to the ARMv8 encoding rules. This is not intended to be used
        /// directly but rather used internally by other public-facing functions.
        /// The parameters are bits used in the encoding: `sf` for setting the operation size, `op` for
        /// determining add/sub operation, `S` for setting the flags, `shift` for optional left shift
        /// of the immediate, `imm12` is the immediate value, `rn` and `rd` are registers.
        #[inline(always)]
        fn add_sub_imm(&mut self, sf: u8, op: u8, S: u8, shift: u8, imm12: u16, rn: Register, rd: Register) {
            let r = bseq_32!(sf:1 op:1 S:1 10001 0 shift:1 imm12:12 rn:5 rd:5);
            self.emit(r);
        }

        /// Generates a 32-bit ADD instruction with immediate value. The immediate value is provided
        /// as `imm12`, and `rn` and `rd` are the source and destination registers respectively.
        #[inline(always)]
        pub fn add_32_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) {
            self.add_sub_imm(0, 0, 0, 0, imm12, rn, rd);
        }

        /// Similar to `add_32_imm`, but with the ability to left shift the immediate value
        /// by an amount specified in the `lsl` parameter.
        #[inline(always)]
        pub fn add_32_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) {
            self.add_sub_imm(0, 0, 0, lsl.into(), imm12, rn, rd);
        }

        /// Generates a 64-bit ADD instruction with immediate value.
        /// The parameters are same as `add_32_imm` but this operates on 64-bit registers.
        #[inline(always)]
        pub fn add_64_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) {
            self.add_sub_imm(1, 0, 0, 0, imm12, rn, rd);
        }

        /// Similar to `add_64_imm`, but with the ability to left shift the immediate value
        /// by an amount specified in the `lsl` parameter.
        #[inline(always)]
        pub fn add_64_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) {
            self.add_sub_imm(1, 0, 0, lsl.into(), imm12, rn, rd);
        }

        /// Generates a 32-bit SUB instruction with immediate value.
        /// The immediate value is provided as `imm12`, and `rn` and `rd` are the source and destination registers respectively.
        #[inline(always)]
        pub fn sub_32_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) {
            self.add_sub_imm(0, 1, 0, 0, imm12, rn, rd);
        }

        /// Similar to `sub_32_imm`, but with the ability to left shift the immediate value
        /// by an amount specified in the `lsl` parameter.
        #[inline(always)]
        pub fn sub_32_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) {
            self.add_sub_imm(0, 1, 0, lsl.into(), imm12, rn, rd);
        }

        /// Generates a 64-bit SUB instruction with immediate value.
        /// The parameters are the same as `sub_32_imm` but this operates on 64-bit registers.
        #[inline(always)]
        pub fn sub_64_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) {
            self.add_sub_imm(1, 1, 0, 0, imm12, rn, rd);
        }

        /// Similar to `sub_64_imm`, but with the ability to left shift the immediate value
        /// by an amount specified in the `lsl` parameter.
        #[inline(always)]
        pub fn sub_64_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) {
            self.add_sub_imm(1, 1, 1, lsl.into(), imm12, rn, rd);
        }

        // The following functions are analogous to the above, but instead generating the
        // corresponding ADDS/SUBS instructions. ADDS and SUBS are same as ADD and SUB
        // but they also update the condition flags.

        /// Generates a 32-bit ADDS instruction with immediate value.
        /// The ADDS instruction is the same as the ADD instruction but also updates the condition flags.
        /// The immediate value is provided as `imm12`, and `rn` and `rd` are the source and destination registers respectively.
        #[inline(always)]
        pub fn adds_32_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) {
            self.add_sub_imm(0, 0, 1, 0, imm12, rn, rd);
        }

        /// Similar to `adds_32_imm`, but with the ability to left shift the immediate value
        /// by an amount specified in the `lsl` parameter.
        #[inline(always)]
        pub fn adds_32_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) {
            self.add_sub_imm(0, 0, 1, lsl.into(), imm12, rn, rd);
        }

        /// Generates a 64-bit ADDS instruction with immediate value.
        /// The parameters are the same as `adds_32_imm` but this operates on 64-bit registers.
        #[inline(always)]
        pub fn adds_64_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) {
            self.add_sub_imm(1, 0, 1, 0, imm12, rn, rd);
        }

        /// Similar to `adds_64_imm`, but with the ability to left shift the immediate value
        /// by an amount specified in the `lsl` parameter.
        #[inline(always)]
        pub fn adds_64_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) {
            self.add_sub_imm(1, 0, 1, lsl.into(), imm12, rn, rd);
        }


        /// Generates a 32-bit SUBS instruction with immediate value.
        /// The SUBS instruction is same as the SUB instruction but also updates the condition flags.
        /// The immediate value is provided as `imm12`, and `rn` and `rd` are the source and destination registers respectively.
        #[inline(always)]
        pub fn subs_32_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) {
            self.add_sub_imm(0, 1, 1, 0, imm12, rn, rd);
        }

        /// Similar to `subs_32_imm`, but with the ability to left shift the immediate value
        /// by an amount specified in the `lsl` parameter.
        #[inline(always)]
        pub fn subs_32_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) {
            self.add_sub_imm(0, 1, 1, lsl.into(), imm12, rn, rd);
        }

        /// Generates a 64-bit SUBS instruction with immediate value.
        /// The parameters are same as `subs_32_imm` but this operates on 64-bit registers.
        #[inline(always)]
        pub fn subs_64_imm(&mut self, rd: Register, rn: Register, imm12: Imm12) {
            self.add_sub_imm(1, 1, 1, 0, imm12, rn, rd);
        }

        /// Similar to `subs_64_imm`, but with the ability to left shift the immediate value
        /// by an amount specified in the `lsl` parameter.
        #[inline(always)]
        pub fn subs_64_imm_lsl(&mut self, rd: Register, rn: Register, imm12: Imm12, lsl: Shift1) {
            self.add_sub_imm(1, 1, 1, lsl.into(), imm12, rn, rd);
        }
    }
}

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
    use crate::types::{HW, Imm16, Register};

    impl<'mem> InstrStream<'mem> {
        // TODO: move somewhere else
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
        pub fn movn_32_imm(&mut self, d: Register, imm: Imm16) {
            self.movn_imm(false, d, imm, 0.into());
        }

        /// Generates a 32-bit `MOVN` (Move Not) instruction.
        #[inline(always)]
        pub fn movn_32_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movn_imm(false, d, imm, lsl);
        }

        /// Generates a 64-bit `MOVN` (Move Not) instruction.
        #[inline(always)]
        pub fn movn_64_imm(&mut self, d: Register, imm: Imm16) {
            self.movn_imm(true, d, imm, 0.into());
        }

        /// Generates a 64-bit `MOVN` (Move Not) instruction.
        #[inline(always)]
        pub fn movn_64_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) {
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
        pub fn movz_32_imm(&mut self, d: Register, imm: Imm16) {
            self.movz_imm(false, d, imm, 0.into());
        }

        /// Generates a 32-bit `MOVZ` (Move Zero) instruction.
        #[inline(always)]
        pub fn movz_32_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movz_imm(false, d, imm, lsl);
        }

        /// Generates a 64-bit `MOVZ` (Move Zero) instruction.
        #[inline(always)]
        pub fn movz_64_imm(&mut self, d: Register, imm: Imm16) {
            self.movz_imm(true, d, imm, 0.into());
        }

        /// Generates a 64-bit `MOVZ` (Move Zero) instruction.
        #[inline(always)]
        pub fn movz_64_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) {
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
        pub fn movk_32_imm(&mut self, d: Register, imm: Imm16) {
            self.movk_imm(false, d, imm, 0.into());
        }

        /// Generates a 32-bit `MOVK` (Move Keep) instruction.
        #[inline(always)]
        pub fn movk_32_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movk_imm(false, d, imm, lsl);
        }

        /// Generates a 64-bit `MOVK` (Move Keep) instruction.
        #[inline(always)]
        pub fn movk_64_imm(&mut self, d: Register, imm: Imm16) {
            self.movk_imm(true, d, imm, 0.into());
        }

        /// Generates a 64-bit `MOVK` (Move Keep) instruction.
        #[inline(always)]
        pub fn movk_64_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movk_imm(true, d, imm, lsl);
        }


        /// An alias of the `MOVZ` 32 bit instruction
        #[inline(always)]
        pub fn mov_32_imm(&mut self, d: Register, imm: Imm16) {
            self.movz_imm(false, d, imm, 0.into());
        }

        /// An alias of the `MOVZ` 32 bit instruction
        #[inline(always)]
        pub fn mov_32_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movz_imm(false, d, imm, lsl);
        }


        /// An alias of the `MOVZ` 64 bit instruction
        #[inline(always)]
        pub fn mov_64_imm(&mut self, d: Register, imm: Imm16) {
            self.movz_imm(true, d, imm, 0.into());
        }

        /// An alias of the `MOVZ` 64 bit instruction
        #[inline(always)]
        pub fn mov_64_imm_lsl(&mut self, d: Register, imm: Imm16, lsl: HW) {
            self.movz_imm(true, d, imm, lsl);
        }
    }
}
