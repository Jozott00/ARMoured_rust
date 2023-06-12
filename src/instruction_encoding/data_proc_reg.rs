//! # Data Processing - Register
//!
//! It consists of
//! - Data-processing (2 source)
//! - Data-processing (1 source)
//! - Logical (shifted register)
//! - Add/subtract (shifted register)
//! - Add/subtract (extended register)
//! - Rotate right into flags
//! - Evaluate into flags
//! - Conditional compare (register)
//! - Conditional compare (immediate)
//! - Conditional select
//! - Data-processing (3 source)

use crate::instruction_encoding::data_proc_reg::data_proc_two_src::DataProcessingTwoSource;

pub mod data_proc_two_src;

pub trait DataProcessingRegister<T>: DataProcessingTwoSource<T> {}