//! # Loads and Stores
//!
//! Contains the following instruction types:
//! - [Compare and swap pair](compare_and_swap_pair)
//! - [Load register (literal)](load_register_literal)
//! - [Load/store register (unsigned immediate)](load_store_reg_pre_post_indexed)


use crate::instruction_encoding::loads_and_stores::compare_and_swap::CompareAndSwap;
use crate::instruction_encoding::loads_and_stores::compare_and_swap_pair::CompareAndSwapPair;
use crate::instruction_encoding::loads_and_stores::ldapr_stlr_unscale_imm::LdaprStlrUnscaleImmediate;
use crate::instruction_encoding::loads_and_stores::load_register_literal::{LoadRegisterLiteral, LoadRegisterLiteralWithAddress};
use crate::instruction_encoding::loads_and_stores::load_store_exclusive_pair::LoadStoreExclusivePair;
use crate::instruction_encoding::loads_and_stores::load_store_exclusive_register::LoadStoreExclusiveRegister;
use crate::instruction_encoding::loads_and_stores::load_store_memory_tags::LoadStoreMemoryTags;
use crate::instruction_encoding::loads_and_stores::load_store_no_allocate_pair_offset::LoadStoreNoAllocatePairOffset;
use crate::instruction_encoding::loads_and_stores::load_store_ordered::LoadStoreOrdered;
use crate::instruction_encoding::loads_and_stores::load_store_reg_pair_offset::LoadStoreRegisterPairOffset;
use crate::instruction_encoding::loads_and_stores::load_store_reg_pair_post_indexed::LoadStoreRegisterPairPostIndexed;
use crate::instruction_encoding::loads_and_stores::load_store_reg_pair_pre_indexed::LoadStoreRegisterPairPreIndexed;
use crate::instruction_encoding::loads_and_stores::load_store_reg_pre_post_indexed::LoadStoreRegisterPrePostIndexed;
use crate::instruction_encoding::loads_and_stores::load_store_reg_unprivileged::LoadStoreRegisterUnprivileged;
use crate::instruction_encoding::loads_and_stores::load_store_reg_unscaled_imm::LoadStoreRegisterUnscaledImmediate;
use crate::instruction_encoding::loads_and_stores::load_store_register_unsigned_imm::LoadStoreRegisterUnsignedImmediate;
use crate::instruction_encoding::loads_and_stores::memory_copy_and_memory_set::MemoryCopyAndMemorySet;

pub mod compare_and_swap_pair;
pub mod load_store_reg_pre_post_indexed;
pub mod load_register_literal;
pub mod load_store_memory_tags;
pub mod load_store_exclusive_pair;
pub mod load_store_exclusive_register;
pub mod load_store_ordered;
pub mod compare_and_swap;
pub mod ldapr_stlr_unscale_imm;
pub mod memory_copy_and_memory_set;
pub mod load_store_no_allocate_pair_offset;
pub mod load_store_reg_pair_post_indexed;
pub mod load_store_reg_pair_offset;
pub mod load_store_reg_pair_pre_indexed;
pub mod load_store_reg_unscaled_imm;
pub mod load_store_register_unsigned_imm;
pub mod load_store_reg_unprivileged;


pub trait LoadsAndStores<T>: CompareAndSwapPair<T>
+ LoadRegisterLiteral<T>
+ LoadStoreMemoryTags<T>
+ LoadStoreExclusivePair<T>
+ LoadStoreExclusiveRegister<T>
+ LoadStoreOrdered<T>
+ CompareAndSwap<T>
+ LdaprStlrUnscaleImmediate<T>
+ MemoryCopyAndMemorySet<T>
+ LoadStoreNoAllocatePairOffset<T>
+ LoadStoreRegisterPairPostIndexed<T>
+ LoadStoreRegisterPairOffset<T>
+ LoadStoreRegisterPairPreIndexed<T>
+ LoadStoreRegisterUnscaledImmediate<T>
+ LoadStoreRegisterPrePostIndexed<T>
+ LoadStoreRegisterUnprivileged<T>
+ LoadStoreRegisterUnsignedImmediate<T>
{}

pub trait LoadsAndStoresWithAddress<T>: LoadsAndStores<T>
+ LoadRegisterLiteralWithAddress<T>
{}