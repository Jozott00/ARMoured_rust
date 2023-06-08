use crate::instruction_emitter::Emitter;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::Offset32;

impl<'mem, M: Memory, E: Emitter> InstrStream<'mem, M, E> {
    pub fn intr_ptr_offset_to(&self, addr: usize) -> Offset32 {
        let pc = self.emitter.instr_ptr() as usize;
        let offset_abs = pc.checked_sub(addr)
            .unwrap_or_else(|| addr.checked_sub(pc).unwrap());

        debug_assert!(offset_abs <= i32::MAX as usize, "Offset to address is to large (exceeds maximum of {:x})", i32::MAX);

        if addr >= pc { offset_abs as i32 } else { -(offset_abs as i32) }
    }
}
