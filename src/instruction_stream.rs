use std::{mem, slice};

use bad64::disasm;
use bit_seq::{bseq, bseq_32};

use crate::instruction_emitter::{Emitter, InstrEmitter};
use crate::mc_memory::{McMemory, Memory};
use crate::types::{Instruction, InstructionPointer, Register};
use crate::types::instruction::Instr;

pub mod data_proc_imm;
pub mod loads_and_stores;
pub mod branch_exception_system;
mod utils;

pub type PatchFn<M: Memory, E: Emitter> = fn(&mut InstrStream<M, E>) -> ();

pub struct InstrStream<'mem, M: Memory, E: Emitter> {
    mem: &'mem mut M,
    emitter: E,
}


impl<'mem> InstrStream<'mem, McMemory, InstrEmitter> {
    pub fn new(mem: &'mem mut McMemory) -> Self {
        let emitter = InstrEmitter::from_mem(mem);
        Self {
            mem,
            emitter,
        }
    }
}

impl<'mem, M: Memory, E: Emitter> InstrStream<'mem, M, E> {
    pub fn patch_at(&mut self, intr_ptr: InstructionPointer, patch: PatchFn<M, E>) {
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
    fn emit(&mut self, instr: Instruction) -> Instr {
        debug_assert!(!self.mem.is_executable(), "Cannot emit instruction while memory is in execution mode");

        let iptr = self.emitter.instr_ptr();
        self.emitter.emit(instr);

        Instr::new(instr, iptr)
    }

    pub fn print_disasm(&self) {
        let decoded_iter = disasm(self.written_memory(), self.base_ptr() as u64);
        for instr in decoded_iter {
            if let Ok(instr) = instr {
                let encoding = instr.opcode().to_le_bytes();
                let enc_str = encoding.map(|e| format!("{e:02x}")).join(" ");
                println!("{:#x}: {enc_str}      {instr}", instr.address());
            } else {
                println!("! Incorrect instruction");
            };
        }
    }

    pub fn written_memory(&self) -> &[u8] {
        let len = (self.emitter.instr_ptr() as usize) - (self.mem.addr() as usize);
        let ptr = self.mem.addr() as *const u8;

        assert_eq!(len % 4, 0, "Len is not a multiple of 4");
        assert_eq!(ptr as usize % mem::align_of::<u32>(), 0, "Memory not u32 aligned");
        assert!(self.mem.len() >= len, "Requested length exceeds memory map!");

        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

#[cfg(test)]
mod mocking_util {
    use crate::mc_memory::MockMemory;
    use crate::instruction_emitter::MockEmitter;
    use crate::instruction_stream::InstrStream;

    impl<'mem> InstrStream<'mem, MockMemory, MockEmitter> {
        pub fn new_mocked(mem: &'mem mut MockMemory, emitter: MockEmitter) -> Self {
            InstrStream {
                mem,
                emitter,
            }
        }
    }
}
