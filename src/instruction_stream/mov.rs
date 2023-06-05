use bit_seq::bseq;
use crate::instruction_stream::InstrStream;
use crate::types::{Instruction, Register};

impl<'mem> InstrStream<'mem> {
    pub fn mov_reg(&mut self, sf: bool, dest: Register, src: Register) {
        let sf_i = if sf { 1 } else { 0 };
        let d = dest as u32;
        let s = src as u32;
        let r: Instruction = bseq!(sf_i:1 0101010 0:3 s:5 0:6 11111 d:5);
        self.emit(r);
    }

    pub fn mov_imm(&mut self, d: u8, imm: u16) {
        let d_32 = d as u32;
        let imm_32 = imm as u32;
        let r: u32 = bseq!(1 10100101 0:2 imm_32:16 d_32:5);
        self.emit(r);
    }
}