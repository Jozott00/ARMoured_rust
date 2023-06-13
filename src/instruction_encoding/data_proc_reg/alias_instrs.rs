use crate::instruction_encoding::data_proc_reg::add_sub_shift_reg::AddSubtractShiftedRegister;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::Register;
use crate::types::shifts::Shift3;

pub trait AliasInstructions<T>: AddSubtractShiftedRegister<T> {
    /// [ADD - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--shifted-register---Add--shifted-register--?lang=en)
    ///
    /// Add adds a register value and a register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// ADD <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn add_32_reg(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        self.add_32_shifted_reg(wd, wn, wm, Shift3::LSL(0))
    }


    /// [ADD - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADD--shifted-register---Add--shifted-register--?lang=en)
    ///
    /// Add adds a register value and a register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// ADD <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn add_64_reg(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        self.add_64_shifted_reg(xd, xn, xm, Shift3::LSL(0))
    }


    /// [ADDS - shifted register -  Add - shifted register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--shifted-register---Add--shifted-register---setting-flags-?lang=en)
    ///
    /// Add (shifted register), setting flags, adds a register value and an optionally-shifted register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// This instruction is used by the alias CMN (shifted register).
    ///
    /// ```asm
    /// ADDS <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn adds_32_reg(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        self.adds_32_shifted_reg(wd, wn, wm, Shift3::LSL(0))
    }


    /// [ADDS - register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADDS--shifted-register---Add--shifted-register---setting-flags-?lang=en)
    ///
    /// Add (shifted register), setting flags, adds a register value and a register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// ```asm
    /// ADDS <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn adds_64_reg(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        self.adds_64_shifted_reg(xd, xn, xm, Shift3::LSL(0))
    }


    /// [SUB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--shifted-register---Subtract--shifted-register--?lang=en)
    ///
    /// Subtract (shifted register) subtracts a register value from a register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// SUB <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn sub_32_reg(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        self.sub_32_shifted_reg(wd, wn, wm, Shift3::LSL(0))
    }


    /// [SUB - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--shifted-register---Subtract--shifted-register--?lang=en)
    ///
    /// Subtract (shifted register) subtracts a register value from a register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// SUB <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn sub_64_reg(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        self.sub_64_shifted_reg(xd, xn, xm, Shift3::LSL(0))
    }


    /// [SUBS - register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUB--shifted-register---Subtract--shifted-register--?lang=en)
    ///
    /// Subtract (shifted register) subtracts a register value from a register value, and writes the result to the destination register.
    ///
    /// ```asm
    /// SUBS <Wd>, <Wn>, <Wm>
    /// ```
    #[inline(always)]
    fn subs_32_reg(&mut self, wd: Register, wn: Register, wm: Register) -> T {
        self.subs_32_shifted_reg(wd, wn, wm, Shift3::LSL(0))
    }


    /// [SUBS - register -  setting flags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SUBS--shifted-register---Subtract--shifted-register---setting-flags-?lang=en)
    ///
    /// Subtract (shifted register), setting flags, subtracts a register value from a register value, and writes the result to the destination register. It updates the condition flags based on the result.
    ///
    /// ```asm
    /// SUBS <Xd>, <Xn>, <Xm>
    /// ```
    #[inline(always)]
    fn subs_64_reg(&mut self, xd: Register, xn: Register, xm: Register) -> T {
        self.subs_64_shifted_reg(xd, xn, xm, Shift3::LSL(0))
    }
}