use bit_seq::bseq_16;
use crate::types::encodable::Encodable;
use PStateField::*;

/// PSTATE field operand.
///
/// For more information check the [arm64 docs](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MSR--immediate---Move-immediate-value-to-Special-Register-?lang=en#sa_pstatefield).
///
/// **Warning**: `ALLINT` is not tested
#[derive(Debug, PartialEq)]
pub enum PStateField {
    UAO,
    PAN,
    SPSel,
    /// **Warning**: not tested
    ALLINT,
    SSBS,
    DIT,
    TCO,
    DAIFSet,
    DAIFClr,
}

impl Encodable<(u8, u8)> for PStateField {
    /// Encoded along [arm64 docs](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MSR--immediate---Move-immediate-value-to-Special-Register-?lang=en#sa_pstatefield)
    ///
    /// Returns (op1, op2)
    fn encode(&self) -> (u8, u8) {
        match self {
            UAO => (0b000, 0b011),
            PAN => (0b000, 0b100),
            SPSel => (0b000, 0b101),
            ALLINT => (0b001, 0b000),
            SSBS => (0b011, 0b001),
            DIT => (0b011, 0b010),
            TCO => (0b011, 0b100),
            DAIFSet => (0b011, 0b110),
            DAIFClr => (0b011, 0b111),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mc_memory::MockMemory;
    use crate::instruction_emitter::MockEmitter;
    use crate::{stream_mock};
    use crate::types::InstructionPointer;
    use crate::instruction_stream::InstrStream;
    use crate::instruction_encoding::branch_exception_system::pstate::PStateInstructions;

    #[test]
    fn test_condition_encodings() {
        stream_mock!(stream, {
            let instr = stream.msr_imm(UAO, 0x0);
            assert_eq!(instr.to_string(), "msr uao, #0x0");

            let instr = stream.msr_imm(PAN, 0x0);
            assert_eq!(instr.to_string(), "msr pan, #0x0");

            let instr = stream.msr_imm(SPSel, 0x0);
            assert_eq!(instr.to_string(), "msr spsel, #0x0");

            let instr = stream.msr_imm(SSBS, 0x0);
            assert_eq!(instr.to_string(), "msr ssbs, #0x0");

            let instr = stream.msr_imm(DIT, 0x0);
            assert_eq!(instr.to_string(), "msr dit, #0x0");

            let instr = stream.msr_imm(TCO, 0x0);
            assert_eq!(instr.to_string(), "msr tco, #0x0");

            let instr = stream.msr_imm(DAIFSet, 0x0);
            assert_eq!(instr.to_string(), "msr daifset, #0x0");

            let instr = stream.msr_imm(DAIFClr, 0x0);
            assert_eq!(instr.to_string(), "msr daifclr, #0x0");
        })
    }
}