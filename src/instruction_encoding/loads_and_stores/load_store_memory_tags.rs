//! # [Load/store memory tags](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldsttags)
//!
//! Implements the following instructions:
//!  - [STG - Store Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STG--Store-Allocation-Tag-?lang=en)
//!  - [STZGM - Store Tag and Zero Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZGM--Store-Tag-and-Zero-Multiple-?lang=en)
//!  - [LDG - Load Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDG--Load-Allocation-Tag-?lang=en)
//!  - [STZG - Store Allocation Tag - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZG--Store-Allocation-Tag--Zeroing-?lang=en)
//!  - [ST2G - Store Allocation Tags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST2G--Store-Allocation-Tags-?lang=en)
//!  - [STGM - Store Tag Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STGM--Store-Tag-Multiple-?lang=en)
//!  - [STZ2G - Store Allocation Tags - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZ2G--Store-Allocation-Tags--Zeroing-?lang=en)
//!  - [LDGM - Load Tag Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDGM--Load-Tag-Multiple-?lang=en)



use crate::instruction_encoding::InstructionProcessor;
use crate::types::{Imm9, Register};

/// # [Load/store memory tags](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldsttags)
///
/// Implements the following instructions:
///  - [STG - Store Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STG--Store-Allocation-Tag-?lang=en)
///  - [STZGM - Store Tag and Zero Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZGM--Store-Tag-and-Zero-Multiple-?lang=en)
///  - [LDG - Load Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDG--Load-Allocation-Tag-?lang=en)
///  - [STZG - Store Allocation Tag - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZG--Store-Allocation-Tag--Zeroing-?lang=en)
///  - [ST2G - Store Allocation Tags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST2G--Store-Allocation-Tags-?lang=en)
///  - [STGM - Store Tag Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STGM--Store-Tag-Multiple-?lang=en)
///  - [STZ2G - Store Allocation Tags - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZ2G--Store-Allocation-Tags--Zeroing-?lang=en)
///  - [LDGM - Load Tag Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDGM--Load-Tag-Multiple-?lang=en)
pub trait LoadStoreMemoryTags<T>: InstructionProcessor<T> {



    /// [STG - Store Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STG--Store-Allocation-Tag-?lang=en)
    ///
    /// Store Allocation Tag stores an Allocation Tag to memory. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STG <Xt|SP>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn stg(&mut self, xt_sp: Register, xn_sp: Register, simm: Imm9) -> T {
        todo!()

    }


    /// [STG - Store Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STG--Store-Allocation-Tag-?lang=en)
    ///
    /// Store Allocation Tag stores an Allocation Tag to memory. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STG <Xt|SP>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn f1(&mut self, ) -> T {
        todo!()

    }


    /// [STG - Store Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STG--Store-Allocation-Tag-?lang=en)
    ///
    /// Store Allocation Tag stores an Allocation Tag to memory. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STG <Xt|SP>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn f2(&mut self, ) -> T {
        todo!()

    }


    /// [STZGM - Store Tag and Zero Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZGM--Store-Tag-and-Zero-Multiple-?lang=en)
    ///
    /// Store Tag and Zero Multiple writes a naturally aligned block of N Allocation Tags and stores zero to the associated data locations, where the size of N is identified in DCZID_EL0.BS, and the Allocation Tag written to address A is taken from the source register bits<3:0>.
    ///
    /// This instruction is undefined at EL0.
    ///
    /// ```asm
    /// STZGM <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn f3(&mut self, ) -> T {
        todo!()

    }


    /// [LDG - Load Allocation Tag](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDG--Load-Allocation-Tag-?lang=en)
    ///
    /// Load Allocation Tag loads an Allocation Tag from a memory address, generates a Logical Address Tag from the Allocation Tag and merges it into the destination register. The address used for the load is calculated from the base register and an immediate signed offset scaled by the Tag granule.
    ///
    /// ```asm
    /// LDG <Xt>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn f4(&mut self, ) -> T {
        todo!()

    }


    /// [STZG - Store Allocation Tag - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZG--Store-Allocation-Tag--Zeroing-?lang=en)
    ///
    /// Store Allocation Tag, Zeroing stores an Allocation Tag to memory, zeroing the associated data location. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STZG <Xt|SP>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn f5(&mut self, ) -> T {
        todo!()

    }


    /// [STZG - Store Allocation Tag - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZG--Store-Allocation-Tag--Zeroing-?lang=en)
    ///
    /// Store Allocation Tag, Zeroing stores an Allocation Tag to memory, zeroing the associated data location. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STZG <Xt|SP>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn f6(&mut self, ) -> T {
        todo!()

    }


    /// [STZG - Store Allocation Tag - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZG--Store-Allocation-Tag--Zeroing-?lang=en)
    ///
    /// Store Allocation Tag, Zeroing stores an Allocation Tag to memory, zeroing the associated data location. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STZG <Xt|SP>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn f7(&mut self, ) -> T {
        todo!()

    }


    /// [ST2G - Store Allocation Tags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST2G--Store-Allocation-Tags-?lang=en)
    ///
    /// Store Allocation Tags stores an Allocation Tag to two Tag granules of memory. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// ST2G <Xt|SP>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn f8(&mut self, ) -> T {
        todo!()

    }


    /// [ST2G - Store Allocation Tags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST2G--Store-Allocation-Tags-?lang=en)
    ///
    /// Store Allocation Tags stores an Allocation Tag to two Tag granules of memory. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// ST2G <Xt|SP>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn f9(&mut self, ) -> T {
        todo!()

    }


    /// [ST2G - Store Allocation Tags](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST2G--Store-Allocation-Tags-?lang=en)
    ///
    /// Store Allocation Tags stores an Allocation Tag to two Tag granules of memory. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// ST2G <Xt|SP>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn f10(&mut self, ) -> T {
        todo!()

    }


    /// [STGM - Store Tag Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STGM--Store-Tag-Multiple-?lang=en)
    ///
    /// Store Tag Multiple writes a naturally aligned block of N Allocation Tags, where the size of N is identified in GMID_EL1.BS, and the Allocation Tag written to address A is taken from the source register at 4*A<7:4>+3:4*A<7:4>.
    ///
    /// This instruction is undefined at EL0.
    ///
    /// ```asm
    /// STGM <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn f11(&mut self, ) -> T {
        todo!()

    }


    /// [STZ2G - Store Allocation Tags - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZ2G--Store-Allocation-Tags--Zeroing-?lang=en)
    ///
    /// Store Allocation Tags, Zeroing stores an Allocation Tag to two Tag granules of memory, zeroing the associated data locations. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STZ2G <Xt|SP>, [<Xn|SP>], #<simm>
    /// ```
    #[inline(always)]
    fn f12(&mut self, ) -> T {
        todo!()

    }


    /// [STZ2G - Store Allocation Tags - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZ2G--Store-Allocation-Tags--Zeroing-?lang=en)
    ///
    /// Store Allocation Tags, Zeroing stores an Allocation Tag to two Tag granules of memory, zeroing the associated data locations. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STZ2G <Xt|SP>, [<Xn|SP>, #<simm>]!
    /// ```
    #[inline(always)]
    fn f13(&mut self, ) -> T {
        todo!()

    }


    /// [STZ2G - Store Allocation Tags - Zeroing](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STZ2G--Store-Allocation-Tags--Zeroing-?lang=en)
    ///
    /// Store Allocation Tags, Zeroing stores an Allocation Tag to two Tag granules of memory, zeroing the associated data locations. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag granule. The Allocation Tag is calculated from the Logical Address Tag in the source register.
    ///
    /// This instruction generates an Unchecked access.
    ///
    /// ```asm
    /// STZ2G <Xt|SP>, [<Xn|SP>{, #<simm>}]
    /// ```
    #[inline(always)]
    fn f14(&mut self, ) -> T {
        todo!()

    }


    /// [LDGM - Load Tag Multiple](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDGM--Load-Tag-Multiple-?lang=en)
    ///
    /// Load Tag Multiple reads a naturally aligned block of N Allocation Tags, where the size of N is identified in GMID_EL1.BS, and writes the Allocation Tag read from address A to the destination register at 4*A<7:4>+3:4*A<7:4>. Bits of the destination register not written with an Allocation Tag are set to 0.
    ///
    /// This instruction is undefined at EL0.
    ///
    /// ```asm
    /// LDGM <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    fn f15(&mut self, ) -> T {
        todo!()

    }


}
