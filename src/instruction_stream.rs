pub mod data_proc_imm;
pub mod ret;

use std::{mem, slice};
use bad64::disasm;
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

    fn written_memory(&self) -> &[u8] {
        let len = (self.emitter.instr_ptr() as usize) - (self.mem.addr() as usize);
        let ptr = self.mem.addr() as *const u8;

        assert_eq!(len % 4, 0, "Len is not a multiple of 4");
        assert_eq!(ptr as usize % mem::align_of::<u32>(), 0, "Memory not u32 aligned");
        assert!(self.mem.len() >= len, "Requested length exceeds memory map!");

        unsafe { slice::from_raw_parts(ptr, len) }
    }
}
