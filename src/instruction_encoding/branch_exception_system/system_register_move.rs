//! # System register move
//!
//! Implements the following instructions:
//! - [MSR (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MSR--register---Move-general-purpose-register-to-System-Register-?lang=en)
//! - [MRS](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MRS--Move-System-Register-?lang=en)

use bit_seq::bseq_32;

use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Register, UImm2, UImm3, UImm4};

#[inline(always)]
fn emit_system_register_move<P: InstructionProcessor<T>, T>(
    proc: &mut P,
    l: u8,
    o0: u8,
    op1: u8,
    crn: u8,
    crm: u8,
    op2: u8,
    rt: Register,
) -> T {
    let i = bseq_32!(1101010100 l:1 1 o0:1 op1:3 crn:4 crm:4 op2:3 rt:5);
    proc.process(i)
}

pub trait SystemRegisterMove<T>: InstructionProcessor<T> {
    /// [MRS](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MRS--Move-System-Register-?lang=en)
    ///
    /// As we currently do not provide system registers as type, please read the [arm64 docs](https://developer.arm.com/documentation/ddi0595/2021-12/AArch64-Registers)
    /// to find the system register encoding you are looking for.
    ///
    /// ```asm
    /// MSR (<systemreg>|S<op0>_<op1>_<Cn>_<Cm>_<op2>), <Xt>
    /// ```
    #[inline(always)]
    fn msr_register(
        &mut self,
        op0: UImm2,
        op1: UImm3,
        crn: UImm4,
        crm: UImm4,
        op2: UImm3,
        xt: Register,
    ) -> T {
        debug_assert!(
            op0 == 2 || op0 == 3,
            "op0 must be either 2 or 3, was {}",
            op0
        );
        emit_system_register_move(self, 0, op0, op1, crn, crm, op2, xt)
    }

    /// [MRS](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MRS--Move-System-Register-?lang=en)
    ///
    /// As we currently do not provide system registers as type, please read the [arm64 docs](https://developer.arm.com/documentation/ddi0595/2021-12/AArch64-Registers)
    /// to find the system register encoding you are looking for.
    ///
    /// ```asm
    /// MRS <Xt>, (<systemreg>|S<op0>_<op1>_<Cn>_<Cm>_<op2>)
    /// ```
    #[inline(always)]
    fn mrs(
        &mut self,
        xt: Register,
        op0: UImm2,
        op1: UImm3,
        crn: UImm4,
        crm: UImm4,
        op2: UImm3,
    ) -> T {
        debug_assert!(
            op0 == 2 || op0 == 3,
            "op0 must be either 2 or 3, was {}",
            op0
        );
        emit_system_register_move(self, 1, op0, op1, crn, crm, op2, xt)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_panic, stream_mock};
    use crate::instruction_emitter::MockEmitter;
    use crate::instruction_stream::InstrStream;
    use crate::mc_memory::MockMemory;
    use crate::types::InstructionPointer;

    use super::*;

    #[test]
    fn test_msr_register() {
        stream_mock!(stream, {
            let instr = stream.msr_register(0b11, 0b110, 0b0101, 0b0001, 0b000, 1);
            assert_eq!(instr.to_string(), "msr afsr0_el3, x1");

            assert_panic!("Should panic: op0 not 2 or 3"; stream.msr_register(0b01, 0b110, 0b0101, 0b0001, 0b000, 1));
        })
    }

    #[test]
    fn test_mrs() {
        stream_mock!(stream, {
            let instr = stream.mrs(1, 0b11, 0b110, 0b0101, 0b0001, 0b000);
            assert_eq!(instr.to_string(), "mrs x1, afsr0_el3");

            assert_panic!("Should panic: op0 not 2 or 3"; stream.mrs(1, 0b01, 0b110, 0b0101, 0b0001, 0b000));
        })
    }
}
