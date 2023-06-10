//! # Exception Generation
//!
//! Implements the following instructions:
//! - [SVC - Supervisor Call](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SVC--Supervisor-Call-?lang=en)
//! - [HVC - Hypervisor Call](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/HVC--Hypervisor-Call-?lang=en)
//! - [SMC - Secure Monitor Call](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SMC--Secure-Monitor-Call-?lang=en)
//! - [BRK - Breakpoint instruction](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BRK--Breakpoint-instruction-?lang=en)
//! - [HLT - Halt instruction](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/HLT--Halt-instruction-?lang=en)
//! - [DCPS1](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DCPS1--Debug-Change-PE-State-to-EL1--?lang=en)
//! - [DCPS2](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DCPS2--Debug-Change-PE-State-to-EL2--?lang=en)
//! - [DCPS3](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DCPS3--Debug-Change-PE-State-to-EL3-?lang=en)

use bit_seq::bseq_32;
use crate::instruction_emitter::Emitter;
use crate::instruction_encoding::InstructionProcessor;
use crate::instruction_stream::InstrStream;
use crate::mc_memory::Memory;
use crate::types::instruction::Instr;
use crate::types::UImm16;

#[inline(always)]
fn emit_exception_gen_x<P: InstructionProcessor<T>, T>(proc: &mut P, opc: u8, imm16: u16, op2: u8, LL: u8) -> T {
    let i = bseq_32!(11010100 opc:3 imm16:16 op2:3 LL:2);
    proc.emit(i)
}

pub trait ExceptionGeneration<T>: InstructionProcessor<T> {
    /// [SVC - Supervisor Call](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SVC--Supervisor-Call-?lang=en)
    ///
    /// `SVC #<imm>`
    #[inline(always)]
    fn svc(&mut self, imm: UImm16) -> T {
        emit_exception_gen_x(self, 0b000, imm, 0b000, 0b01)
    }

    /// [HVC - Hypervisor Call](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/HVC--Hypervisor-Call-?lang=en)
    #[inline(always)]
    fn hvc(&mut self, imm: UImm16) -> T {
        emit_exception_gen_x(self, 0b000, imm, 0b000, 0b10)
    }

    /// [SMC - Secure Monitor Call](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SMC--Secure-Monitor-Call-?lang=en)
    #[inline(always)]
    fn smc(&mut self, imm: UImm16) -> T {
        emit_exception_gen_x(self, 0b000, imm, 0b000, 0b11)
    }

    /// [BRK - Breakpoint instruction](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/BRK--Breakpoint-instruction-?lang=en)
    #[inline(always)]
    fn brk(&mut self, imm: UImm16) -> T {
        emit_exception_gen_x(self, 0b001, imm, 0b000, 0b00)
    }

    /// [HLT - Halt instruction](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/HLT--Halt-instruction-?lang=en)
    #[inline(always)]
    fn hlt(&mut self, imm: UImm16) -> T {
        emit_exception_gen_x(self, 0b010, imm, 0b000, 0b00)
    }

    /// [DCPS1](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DCPS1--Debug-Change-PE-State-to-EL1--?lang=en)
    #[inline(always)]
    fn dcps1(&mut self, imm: UImm16) -> T {
        emit_exception_gen_x(self, 0b101, imm, 0b000, 0b01)
    }

    /// [DCPS2](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DCPS2--Debug-Change-PE-State-to-EL2--?lang=en)
    #[inline(always)]
    fn dcps2(&mut self, imm: UImm16) -> T {
        emit_exception_gen_x(self, 0b101, imm, 0b000, 0b10)
    }

    /// [DCPS3](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/DCPS3--Debug-Change-PE-State-to-EL3-?lang=en)
    #[inline(always)]
    fn dcps3(&mut self, imm: UImm16) -> T {
        emit_exception_gen_x(self, 0b001, imm, 0b000, 0b11)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::mc_memory::MockMemory;
    use crate::instruction_emitter::MockEmitter;
    use crate::{stream_mock};
    use crate::types::InstructionPointer;

    #[test]
    fn test_exception_gen() {
        stream_mock!(stream, {
            let instr = stream.svc(0x1);
            assert_eq!(instr.to_string(), "svc #0x1");

            let instr = stream.hvc(0x1);
            assert_eq!(instr.to_string(), "hvc #0x1");

            let instr = stream.smc(0x1);
            assert_eq!(instr.to_string(), "smc #0x1");

            let instr = stream.brk(0x1);
            assert_eq!(instr.to_string(), "brk #0x1");

            let instr = stream.hlt(0x1);
            assert_eq!(instr.to_string(), "hlt #0x1");

            let instr = stream.dcps1(0x1);
            assert_eq!(instr.to_string(), "dcps1 #0x1");

            let instr = stream.dcps2(0x1);
            assert_eq!(instr.to_string(), "dcps2 #0x1");

            let instr = stream.dcps2(0x1);
            assert_eq!(instr.to_string(), "dcps2 #0x1");
        })
    }
}