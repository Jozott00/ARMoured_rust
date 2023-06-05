use std::{mem, ptr};
use std::mem::size_of;

use crate::mc_memory::McMemory;
use crate::types::{Instruction, InstructionPointer};

pub struct InstrEmitter {
    base_ptr: InstructionPointer,
    iptr: InstructionPointer,
    bound_ptr: InstructionPointer,
}

impl InstrEmitter {
    pub fn new(mc_mem: &McMemory) -> Self {
        let base_ptr = mc_mem.addr() as InstructionPointer;
        let bound_ptr = mc_mem.bound_ptr() as InstructionPointer;
        InstrEmitter {
            base_ptr,
            iptr: base_ptr,
            bound_ptr,
        }
    }

    #[inline(always)]
    pub fn emit(&mut self, instr: Instruction) {
        debug_assert!(
            self.in_bound(),
            "Memory out of bound! Emitting an instruction would write out of memory bounds.");

        unsafe {
            ptr::write(self.iptr, instr);
            self.iptr = self.iptr.add(1);
        }
    }

    pub fn base_ptr(&self) -> InstructionPointer {
        self.base_ptr
    }

    pub fn instr_ptr(&self) -> InstructionPointer { self.iptr }

    pub fn set_instr_ptr(&mut self, iptr: InstructionPointer) {
        self.iptr = iptr;
    }

    /// Returns true if emitting a new instruction does not override
    /// memory boundaries
    fn in_bound(&self) -> bool {
        (self.iptr as usize) <= (self.bound_ptr as usize - size_of::<Instruction>())
    }
}