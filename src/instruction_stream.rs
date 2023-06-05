pub mod data_proc_imm;
pub mod ret;

use std::mem;
use bit_seq::{bseq, bseq_32};
use crate::instruction_emitter::InstrEmitter;
use crate::mc_memory::McMemory;
use crate::types::{Instruction, InstructionPointer, Register};

pub type PatchFn = fn(&mut InstrStream) -> ();

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

    pub fn patch_at(&mut self, intr_ptr: InstructionPointer, patch: PatchFn) {
        // save instruction pointer
        let iptr = self.emitter.instr_ptr();
        self.emitter.set_instr_ptr(intr_ptr);
        patch(self);
        // restore instruction pointer
        self.emitter.set_instr_ptr(iptr);
    }

    #[inline(always)]
    pub fn nullary_fn_ptr(&mut self) -> unsafe extern "C" fn() -> u64 {
        unsafe { mem::transmute(self.base_ptr() as usize) }
    }

    #[inline(always)]
    pub fn base_ptr(&self) -> InstructionPointer {
        self.emitter.base_ptr()
    }

    #[inline(always)]
    fn emit(&mut self, instr: Instruction) {
        debug_assert!(!self.mem.is_executable(), "Cannot emit instruction while memory is in execution mode");
        self.emitter.emit(instr);
    }
}
