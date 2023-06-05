mod mov;

use std::mem;
use bit_seq::bseq;
use crate::instruction_emitter::InstrEmitter;
use crate::mc_memory::McMemory;
use crate::types::{Instruction, InstructionPointer, Register};

pub struct InstrStream<'mem> {
    mem: &'mem mut McMemory,
    emitter: InstrEmitter,
}


impl<'mem> InstrStream<'mem> {
    pub fn new(mem: &'mem mut McMemory) -> Self {
        let emitter = InstrEmitter::new(&mem);
        Self {
            mem,
            emitter,
        }
    }

    pub fn ret(&mut self) {
        let x30 = 30;
        let r: u32 = bseq!(1 101011001011111 0:6 x30:5 0:5);
        self.emit(r);
    }

    #[inline(always)]
    pub fn nullary_fn_ptr(&mut self) -> unsafe extern "C" fn() -> u64 {
        unsafe { mem::transmute(self.base_ptr() as usize) }
    }

    #[inline(always)]
    fn base_ptr(&self) -> InstructionPointer {
        self.emitter.base_ptr()
    }

    #[inline(always)]
    fn emit(&mut self, instr: Instruction) {
        debug_assert!(!self.mem.is_executable(), "Cannot emit instruction while memory is in execution mode");
        self.emitter.emit(instr);
    }
}
