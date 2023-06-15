//! # Loads and Stores
//!
//! Contains the following instruction types:
//! - [Compare and swap pair](compare_and_swap_pair)
//! - [Load register (literal)](load_register_literal)
//! - [Load/store register (unsigned immediate)](load_store_reg_uimm)


use crate::instruction_encoding::loads_and_stores::compare_and_swap::CompareAndSwap;
use crate::instruction_encoding::loads_and_stores::compare_and_swap_pair::CompareAndSwapPair;
use crate::instruction_encoding::loads_and_stores::ldapr_stlr_unscale_imm::LdaprStlrUnscaleImmediate;
use crate::instruction_encoding::loads_and_stores::load_register_literal::{LoadRegisterLiteral, LoadRegisterLiteralWithAddress};
use crate::instruction_encoding::loads_and_stores::load_store_exclusive_pair::LoadStoreExclusivePair;
use crate::instruction_encoding::loads_and_stores::load_store_exclusive_register::LoadStoreExclusiveRegister;
use crate::instruction_encoding::loads_and_stores::load_store_memory_tags::LoadStoreMemoryTags;
use crate::instruction_encoding::loads_and_stores::load_store_ordered::LoadStoreOrdered;
use crate::instruction_encoding::loads_and_stores::load_store_reg_uimm::LoadStoreRegUImm;

pub mod compare_and_swap_pair;
pub mod load_store_reg_uimm;
pub mod load_register_literal;
pub mod load_store_memory_tags;
pub mod load_store_exclusive_pair;
pub mod load_store_exclusive_register;
pub mod load_store_ordered;
pub mod compare_and_swap;
pub mod ldapr_stlr_unscale_imm;


pub trait LoadsAndStores<T>: CompareAndSwapPair<T>
+ LoadStoreRegUImm<T>
+ LoadRegisterLiteral<T>
+ LoadStoreMemoryTags<T>
+ LoadStoreExclusivePair<T>
+ LoadStoreExclusiveRegister<T>
+ LoadStoreOrdered<T>
+ CompareAndSwap<T>
+ LdaprStlrUnscaleImmediate<T>
{}

pub trait LoadsAndStoresWithAddress<T>: LoadsAndStores<T>
+ LoadRegisterLiteralWithAddress<T>
{}