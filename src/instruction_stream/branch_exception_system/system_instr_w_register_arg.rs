//! # Exception Generation
//!
//! Implements the following instructions:
//! - [WFET](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/WFET--Wait-For-Event-with-Timeout-?lang=en)
//! - [WFIT](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/WFIT--Wait-For-Interrupt-with-Timeout-?lang=en)



use bit_seq::bseq_32;
use crate::instruction_emitter::Emitter;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::instruction::Instr;
use crate::types::Register;

impl<'mem, M: Memory, E: Emitter> InstrStream<'mem, M, E> {
    #[inline(always)]
    fn emit_sys_instr_x(&mut self, crm: u8, op2: u8, rt: Register) -> Instr {
        let i = bseq_32!(11010101 0:6 11001 crm:4 op2:4 rt:5);
        self.emit(i)
    }

    /// [WFET](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/WFET--Wait-For-Event-with-Timeout-?lang=en)
    ///
    /// *Note*: FEAT_WFxT required \
    /// **Warning**: Not tested!
    #[inline(always)]
    pub fn wfet(&mut self, xt: Register) -> Instr {
        self.emit_sys_instr_x(0, 0, xt)
    }

    /// [WFIT](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/WFIT--Wait-For-Interrupt-with-Timeout-?lang=en)
    ///
    /// *Note*: FEAT_WFxT required \
    /// **Warning**: Not tested!
    #[inline(always)]
    pub fn wfit(&mut self, xt: Register) -> Instr {
        self.emit_sys_instr_x(0, 1, xt)
    }
}


