//! # Loads and Stores
//!
//! Contains the following instruction types:
//! - [Compare and swap pair](compare_and_swap_pair)
//! - [Load register (literal)](load_register_literal)
//! - [Load/store register (unsigned immediate)](load_store_reg_uimm)


use crate::instruction_stream::loads_and_stores::compare_and_swap_pair::CompareAndSwapPair;
use crate::instruction_stream::loads_and_stores::load_register_literal::{LoadRegisterLiteral, LoadRegisterLiteralWithAddress};
use crate::instruction_stream::loads_and_stores::load_store_reg_uimm::LoadStoreRegUImm;

pub mod compare_and_swap_pair;
pub mod load_store_reg_uimm;
pub mod load_register_literal;


pub trait LoadsAndStores<T>: CompareAndSwapPair<T>
+ LoadStoreRegUImm<T>
+ LoadRegisterLiteral<T>
{}

pub trait LoadsAndStoresWithAddress<T>: LoadsAndStores<T>
+ LoadRegisterLiteralWithAddress<T>
{}