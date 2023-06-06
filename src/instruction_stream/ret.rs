use bit_seq::bseq_32;
use crate::instruction_emitter::Emitter;

use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::Register;

impl<'mem, M: Memory, E: Emitter> InstrStream<'mem, M, E> {
    #[inline(always)]
    pub fn ret(&mut self) {
        self.ret_reg(30);
    }

    #[inline(always)]
    pub fn ret_reg(&mut self, reg: Register) {
        let r = bseq_32!(1 101011001011111 0:6 reg:5 0:5);
        self.emit(r);
    }
}