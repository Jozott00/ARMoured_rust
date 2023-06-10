//! # Exception Generation
//!
//! Implements the following instructions:
//! - [WFET](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/WFET--Wait-For-Event-with-Timeout-?lang=en)
//! - [WFIT](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/WFIT--Wait-For-Interrupt-with-Timeout-?lang=en)



use bit_seq::bseq_32;
use crate::instruction_emitter::Emitter;
use crate::instruction_encoding::InstructionProcessor;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::instruction::Instr;
use crate::types::Register;

#[inline(always)]
fn emit_sys_instr_x<P: InstructionProcessor<T>, T>(proc: &mut P, crm: u8, op2: u8, rt: Register) -> T {
    let i = bseq_32!(11010101 0:6 11001 crm:4 op2:4 rt:5);
    proc.emit(i)
}

pub trait SystemInstructionsWithRegArg<T>: InstructionProcessor<T> {
    /// [WFET](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/WFET--Wait-For-Event-with-Timeout-?lang=en)
    ///
    /// *Note*: FEAT_WFxT required \
    /// **Warning**: Not tested!
    #[inline(always)]
    fn wfet(&mut self, xt: Register) -> T {
        emit_sys_instr_x(self, 0, 0, xt)
    }

    /// [WFIT](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/WFIT--Wait-For-Interrupt-with-Timeout-?lang=en)
    ///
    /// *Note*: FEAT_WFxT required \
    /// **Warning**: Not tested!
    #[inline(always)]
    fn wfit(&mut self, xt: Register) -> T {
        emit_sys_instr_x(self, 0, 1, xt)
    }
}


