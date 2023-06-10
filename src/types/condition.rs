use crate::types::encodable::Encodable;
use crate::types::UImm4;
use Condition::*;

pub enum Condition {
    EQ,
    NE,
    CS,
    HS,
    CC,
    LO,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
    NV,
}

impl Encodable<UImm4> for Condition {
    fn encode(&self) -> UImm4 {
        match self {
            EQ => 0b0000,
            NE => 0b0001,
            CS | HS => 0b0010,
            CC | LO => 0b0011,
            MI => 0b0100,
            PL => 0b0101,
            VS => 0b0110,
            VC => 0b0111,
            HI => 0b1000,
            LS => 0b1001,
            GE => 0b1010,
            LT => 0b1011,
            GT => 0b1100,
            LE => 0b1101,
            AL => 0b1110,
            NV => 0b1111,
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
    use crate::instruction_stream::branch_exception_system::conditional_branch_imm::ConditionalBranchImmediateWithAddress;

    #[test]
    fn test_condition_encodings() {
        stream_mock!(stream, {
            let instr = stream.b_cond_to_addr(EQ, 0x0);
            assert_eq!(instr.to_string(), "b.eq 0x0");

            let instr = stream.b_cond_to_addr(NE, 0x0);
            assert_eq!(instr.to_string(), "b.ne 0x0");

            let instr = stream.b_cond_to_addr(CS, 0x0);
            assert_eq!(instr.to_string(), "b.hs 0x0");

            let instr = stream.b_cond_to_addr(HS, 0x0);
            assert_eq!(instr.to_string(), "b.hs 0x0");

            let instr = stream.b_cond_to_addr(CC, 0x0);
            assert_eq!(instr.to_string(), "b.lo 0x0");

            let instr = stream.b_cond_to_addr(LO, 0x0);
            assert_eq!(instr.to_string(), "b.lo 0x0");

            let instr = stream.b_cond_to_addr(MI, 0x0);
            assert_eq!(instr.to_string(), "b.mi 0x0");

            let instr = stream.b_cond_to_addr(PL, 0x0);
            assert_eq!(instr.to_string(), "b.pl 0x0");

            let instr = stream.b_cond_to_addr(VS, 0x0);
            assert_eq!(instr.to_string(), "b.vs 0x0");

            let instr = stream.b_cond_to_addr(VC, 0x0);
            assert_eq!(instr.to_string(), "b.vc 0x0");

            let instr = stream.b_cond_to_addr(HI, 0x0);
            assert_eq!(instr.to_string(), "b.hi 0x0");

            let instr = stream.b_cond_to_addr(LS, 0x0);
            assert_eq!(instr.to_string(), "b.ls 0x0");

            let instr = stream.b_cond_to_addr(GE, 0x0);
            assert_eq!(instr.to_string(), "b.ge 0x0");

            let instr = stream.b_cond_to_addr(LT, 0x0);
            assert_eq!(instr.to_string(), "b.lt 0x0");

            let instr = stream.b_cond_to_addr(GT, 0x0);
            assert_eq!(instr.to_string(), "b.gt 0x0");

            let instr = stream.b_cond_to_addr(LE, 0x0);
            assert_eq!(instr.to_string(), "b.le 0x0");

            let instr = stream.b_cond_to_addr(AL, 0x0);
            assert_eq!(instr.to_string(), "b.al 0x0");

            let instr = stream.b_cond_to_addr(NV, 0x0);
            assert_eq!(instr.to_string(), "b.nv 0x0");

        })
    }
}

