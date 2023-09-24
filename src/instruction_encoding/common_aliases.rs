//! # Common Aliases
//!
//! This module implements common aliases such as MOV.
//!
//! Implements the following instructions:
//!  - [MOV (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOV--register---Move--register---an-alias-of-ORR--shifted-register--?lang=en)

use crate::instruction_encoding::data_proc_reg::logical_shift_reg::LogicalShiftRegister;
use crate::types::register::{WZR, XZR};
use crate::types::Register;

/// # Common Aliases
///
/// This trait implements common aliases such as MOV.
///
/// Implements the following instructions:
///  - [MOV (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOV--register---Move--register---an-alias-of-ORR--shifted-register--?lang=en)
pub trait CommonAliases<T>: LogicalShiftRegister<T> {
    /// [MOV (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOV--register---Move--register---an-alias-of-ORR--shifted-register--?lang=en)
    ///
    /// Move (register) copies the value in a source register to the destination register.
    ///
    /// This is an alias of ORR (shifted register). This means:
    /// - The encodings in this description are named to match the encodings of ORR (shifted register).
    /// - The description of ORR (shifted register) gives the operational pseudocode for this instruction.
    ///
    /// ```asm
    /// MOV <Wd>, <Wm>
    /// ```
    fn mov_32_reg(&mut self, wd: Register, wm: Register) -> T {
        self.orr_32(wd, WZR, wm, None)
    }

    /// [MOV (register)](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/MOV--register---Move--register---an-alias-of-ORR--shifted-register--?lang=en)
    ///
    /// Move (register) copies the value in a source register to the destination register.
    ///
    /// This is an alias of ORR (shifted register). This means:
    /// - The encodings in this description are named to match the encodings of ORR (shifted register).
    /// - The description of ORR (shifted register) gives the operational pseudocode for this instruction.
    ///
    /// ```asm
    /// MOV <Xd>, <Xm>
    /// ```
    fn mov_64_reg(&mut self, xd: Register, xm: Register) -> T {
        self.orr_64(xd, XZR, xm, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_producer::TestProducer;

    #[test]
    fn test_mov() {
        let mut prod = TestProducer::new();

        let instr = prod.mov_32_reg(3, 4);
        assert_eq!(instr, "mov w3, w4");

        let instr = prod.mov_64_reg(1, 28);
        assert_eq!(instr, "mov x1, x28");
    }
}
