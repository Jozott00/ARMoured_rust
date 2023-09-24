//! # [Data Processing - Register](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register)
//!
//! It consists of
//! - [Data-processing (2 source)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#dp_2src)
//! - [Data-processing (1 source)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#dp_1src)
//! - [Logical (shifted register)](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#log_shift)
//! - Add/subtract (shifted register)
//! - Add/subtract (extended register)
//! - Rotate right into flags
//! - Evaluate into flags
//! - Conditional compare (register)
//! - Conditional compare (immediate)
//! - Conditional select
//! - Data-processing (3 source)

use crate::instruction_encoding::data_proc_reg::add_sub_carry::AddSubtractWithCarry;
use crate::instruction_encoding::data_proc_reg::add_sub_ext_reg::AddSubtractExtendedRegister;
use crate::instruction_encoding::data_proc_reg::add_sub_shift_reg::AddSubtractShiftedRegister;
use crate::instruction_encoding::data_proc_reg::cond_compare_imm::ConditionalCompareImmediate;
use crate::instruction_encoding::data_proc_reg::cond_compare_reg::ConditionalCompareRegister;
use crate::instruction_encoding::data_proc_reg::conditional_select::ConditionalSelect;
use crate::instruction_encoding::data_proc_reg::data_proc_one_src::DataProcessingOneSource;
use crate::instruction_encoding::data_proc_reg::data_proc_three_src::DataProcessingThreeSource;
use crate::instruction_encoding::data_proc_reg::data_proc_two_src::DataProcessingTwoSource;
use crate::instruction_encoding::data_proc_reg::evaluate_into_flags::EvaluateIntoFlags;
use crate::instruction_encoding::data_proc_reg::logical_shift_reg::LogicalShiftRegister;
use crate::instruction_encoding::data_proc_reg::rotate_right_into_flags::RotateRightIntoFlags;

pub mod add_sub_carry;
pub mod add_sub_ext_reg;
pub mod add_sub_shift_reg;
pub mod alias_instrs;
pub mod cond_compare_imm;
pub mod cond_compare_reg;
pub mod conditional_select;
pub mod data_proc_one_src;
pub mod data_proc_three_src;
pub mod data_proc_two_src;
pub mod evaluate_into_flags;
pub mod logical_shift_reg;
pub mod rotate_right_into_flags;

pub trait DataProcessingRegister<T>:
    DataProcessingTwoSource<T>
    + DataProcessingOneSource<T>
    + LogicalShiftRegister<T>
    + AddSubtractShiftedRegister<T>
    + AddSubtractExtendedRegister<T>
    + AddSubtractWithCarry<T>
    + RotateRightIntoFlags<T>
    + EvaluateIntoFlags<T>
    + ConditionalCompareRegister<T>
    + ConditionalCompareImmediate<T>
    + ConditionalSelect<T>
    + DataProcessingThreeSource<T>
{
}
