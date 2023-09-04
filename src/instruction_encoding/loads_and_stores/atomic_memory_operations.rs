//! # [Atomic memory operations](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#memop)
//!
//! Implements the following instructions:
//!  - [LDADDB - LDADDAB - LDADDALB - LDADDLB - Atomic add on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADDB--LDADDAB--LDADDALB--LDADDLB--Atomic-add-on-byte-in-memory-?lang=en)
//!  - [LDCLRB - LDCLRAB - LDCLRALB - LDCLRLB - Atomic bit clear on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLRB--LDCLRAB--LDCLRALB--LDCLRLB--Atomic-bit-clear-on-byte-in-memory-?lang=en)
//!  - [LDEORB - LDEORAB - LDEORALB - LDEORLB - Atomic exclusive OR on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEORB--LDEORAB--LDEORALB--LDEORLB--Atomic-exclusive-OR-on-byte-in-memory-?lang=en)
//!  - [LDSETB - LDSETAB - LDSETALB - LDSETLB - Atomic bit set on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSETB--LDSETAB--LDSETALB--LDSETLB--Atomic-bit-set-on-byte-in-memory-?lang=en)
//!  - [LDSMAXB - LDSMAXAB - LDSMAXALB - LDSMAXLB - Atomic signed maximum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAXB--LDSMAXAB--LDSMAXALB--LDSMAXLB--Atomic-signed-maximum-on-byte-in-memory-?lang=en)
//!  - [LDSMINB - LDSMINAB - LDSMINALB - LDSMINLB - Atomic signed minimum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMINB--LDSMINAB--LDSMINALB--LDSMINLB--Atomic-signed-minimum-on-byte-in-memory-?lang=en)
//!  - [LDUMAXB - LDUMAXAB - LDUMAXALB - LDUMAXLB - Atomic unsigned maximum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAXB--LDUMAXAB--LDUMAXALB--LDUMAXLB--Atomic-unsigned-maximum-on-byte-in-memory-?lang=en)
//!  - [LDUMINB - LDUMINAB - LDUMINALB - LDUMINLB - Atomic unsigned minimum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMINB--LDUMINAB--LDUMINALB--LDUMINLB--Atomic-unsigned-minimum-on-byte-in-memory-?lang=en)
//!  - [SWPB - SWPAB - SWPALB - SWPLB - Swap byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWPB--SWPAB--SWPALB--SWPLB--Swap-byte-in-memory-?lang=en)
//!  - [LDAPRB - Load Acquire RCpc Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPRB--Load-Acquire-RCpc-Register-Byte-?lang=en)
//!  - [LDADDH - LDADDAH - LDADDALH - LDADDLH - Atomic add on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADDH--LDADDAH--LDADDALH--LDADDLH--Atomic-add-on-halfword-in-memory-?lang=en)
//!  - [LDCLRH - LDCLRAH - LDCLRALH - LDCLRLH - Atomic bit clear on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLRH--LDCLRAH--LDCLRALH--LDCLRLH--Atomic-bit-clear-on-halfword-in-memory-?lang=en)
//!  - [LDEORH - LDEORAH - LDEORALH - LDEORLH - Atomic exclusive OR on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEORH--LDEORAH--LDEORALH--LDEORLH--Atomic-exclusive-OR-on-halfword-in-memory-?lang=en)
//!  - [LDSETH - LDSETAH - LDSETALH - LDSETLH - Atomic bit set on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSETH--LDSETAH--LDSETALH--LDSETLH--Atomic-bit-set-on-halfword-in-memory-?lang=en)
//!  - [LDSMAXH - LDSMAXAH - LDSMAXALH - LDSMAXLH - Atomic signed maximum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAXH--LDSMAXAH--LDSMAXALH--LDSMAXLH--Atomic-signed-maximum-on-halfword-in-memory-?lang=en)
//!  - [LDSMINH - LDSMINAH - LDSMINALH - LDSMINLH - Atomic signed minimum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMINH--LDSMINAH--LDSMINALH--LDSMINLH--Atomic-signed-minimum-on-halfword-in-memory-?lang=en)
//!  - [LDUMAXH - LDUMAXAH - LDUMAXALH - LDUMAXLH - Atomic unsigned maximum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAXH--LDUMAXAH--LDUMAXALH--LDUMAXLH--Atomic-unsigned-maximum-on-halfword-in-memory-?lang=en)
//!  - [LDUMINH - LDUMINAH - LDUMINALH - LDUMINLH - Atomic unsigned minimum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMINH--LDUMINAH--LDUMINALH--LDUMINLH--Atomic-unsigned-minimum-on-halfword-in-memory-?lang=en)
//!  - [SWPH - SWPAH - SWPALH - SWPLH - Swap halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWPH--SWPAH--SWPALH--SWPLH--Swap-halfword-in-memory-?lang=en)
//!  - [LDAPRH - Load Acquire RCpc Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPRH--Load-Acquire-RCpc-Register-Halfword-?lang=en)
//!  - [LDADD - LDADDA - LDADDAL - LDADDL - Atomic add on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADD--LDADDA--LDADDAL--LDADDL--Atomic-add-on-word-or-doubleword-in-memory-?lang=en)
//!  - [LDCLR - LDCLRA - LDCLRAL - LDCLRL - Atomic bit clear on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLR--LDCLRA--LDCLRAL--LDCLRL--Atomic-bit-clear-on-word-or-doubleword-in-memory-?lang=en)
//!  - [LDEOR - LDEORA - LDEORAL - LDEORL - Atomic exclusive OR on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEOR--LDEORA--LDEORAL--LDEORL--Atomic-exclusive-OR-on-word-or-doubleword-in-memory-?lang=en)
//!  - [LDSET - LDSETA - LDSETAL - LDSETL - Atomic bit set on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSET--LDSETA--LDSETAL--LDSETL--Atomic-bit-set-on-word-or-doubleword-in-memory-?lang=en)
//!  - [LDSMAX - LDSMAXA - LDSMAXAL - LDSMAXL - Atomic signed maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAX--LDSMAXA--LDSMAXAL--LDSMAXL--Atomic-signed-maximum-on-word-or-doubleword-in-memory-?lang=en)
//!  - [LDSMIN - LDSMINA - LDSMINAL - LDSMINL - Atomic signed minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMIN--LDSMINA--LDSMINAL--LDSMINL--Atomic-signed-minimum-on-word-or-doubleword-in-memory-?lang=en)
//!  - [LDUMAX - LDUMAXA - LDUMAXAL - LDUMAXL - Atomic unsigned maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAX--LDUMAXA--LDUMAXAL--LDUMAXL--Atomic-unsigned-maximum-on-word-or-doubleword-in-memory-?lang=en)
//!  - [LDUMIN - LDUMINA - LDUMINAL - LDUMINL - Atomic unsigned minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMIN--LDUMINA--LDUMINAL--LDUMINL--Atomic-unsigned-minimum-on-word-or-doubleword-in-memory-?lang=en)
//!  - [SWP - SWPA - SWPAL - SWPL - Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWP--SWPA--SWPAL--SWPL--Swap-word-or-doubleword-in-memory-?lang=en)
//!  - [LDAPR - Load Acquire RCpc Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPR--Load-Acquire-RCpc-Register-?lang=en)
//!  - [ST64BV0 - Single copy Atomic 64 byte EL0 Store with Return](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST64BV0--Single-copy-Atomic-64-byte-EL0-Store-with-Return-?lang=en)
//!  - [ST64BV - Single copy Atomic 64 byte Store with Return](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST64BV--Single-copy-Atomic-64-byte-Store-with-Return-?lang=en)
//!  - [ST64B - Single copy Atomic 64 byte Store without Return](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST64B--Single-copy-Atomic-64-byte-Store-without-Return-?lang=en)
//!  - [LD64B - Single copy Atomic 64 byte Load](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LD64B--Single-copy-Atomic-64-byte-Load-?lang=en)

use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::Register;

#[inline(always)]
#[cfg(feature = "arm_feat_lse")]
fn emit_atomic_mem_op<P: InstructionProcessor<T>, T>(proc: &mut P, size: u8, v: u8, a: u8, r: u8, rs: Register, o3: u8, opc: u8, rn: Register, rt: Register) -> T {
    let r = bseq_32!(size:2 111 v:1 00 a:1 r:1 1 rs:5 o3:1 opc:3 00 rn:5 rt:5);
    proc.process(r)
}


/// # [Atomic memory operations](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#memop)
///
/// Implements the following instructions:
///  - [LDADDB - LDADDAB - LDADDALB - LDADDLB - Atomic add on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADDB--LDADDAB--LDADDALB--LDADDLB--Atomic-add-on-byte-in-memory-?lang=en)
///  - [LDCLRB - LDCLRAB - LDCLRALB - LDCLRLB - Atomic bit clear on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLRB--LDCLRAB--LDCLRALB--LDCLRLB--Atomic-bit-clear-on-byte-in-memory-?lang=en)
///  - [LDEORB - LDEORAB - LDEORALB - LDEORLB - Atomic exclusive OR on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEORB--LDEORAB--LDEORALB--LDEORLB--Atomic-exclusive-OR-on-byte-in-memory-?lang=en)
///  - [LDSETB - LDSETAB - LDSETALB - LDSETLB - Atomic bit set on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSETB--LDSETAB--LDSETALB--LDSETLB--Atomic-bit-set-on-byte-in-memory-?lang=en)
///  - [LDSMAXB - LDSMAXAB - LDSMAXALB - LDSMAXLB - Atomic signed maximum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAXB--LDSMAXAB--LDSMAXALB--LDSMAXLB--Atomic-signed-maximum-on-byte-in-memory-?lang=en)
///  - [LDSMINB - LDSMINAB - LDSMINALB - LDSMINLB - Atomic signed minimum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMINB--LDSMINAB--LDSMINALB--LDSMINLB--Atomic-signed-minimum-on-byte-in-memory-?lang=en)
///  - [LDUMAXB - LDUMAXAB - LDUMAXALB - LDUMAXLB - Atomic unsigned maximum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAXB--LDUMAXAB--LDUMAXALB--LDUMAXLB--Atomic-unsigned-maximum-on-byte-in-memory-?lang=en)
///  - [LDUMINB - LDUMINAB - LDUMINALB - LDUMINLB - Atomic unsigned minimum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMINB--LDUMINAB--LDUMINALB--LDUMINLB--Atomic-unsigned-minimum-on-byte-in-memory-?lang=en)
///  - [SWPB - SWPAB - SWPALB - SWPLB - Swap byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWPB--SWPAB--SWPALB--SWPLB--Swap-byte-in-memory-?lang=en)
///  - [LDAPRB - Load Acquire RCpc Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPRB--Load-Acquire-RCpc-Register-Byte-?lang=en)
///  - [LDADDH - LDADDAH - LDADDALH - LDADDLH - Atomic add on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADDH--LDADDAH--LDADDALH--LDADDLH--Atomic-add-on-halfword-in-memory-?lang=en)
///  - [LDCLRH - LDCLRAH - LDCLRALH - LDCLRLH - Atomic bit clear on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLRH--LDCLRAH--LDCLRALH--LDCLRLH--Atomic-bit-clear-on-halfword-in-memory-?lang=en)
///  - [LDEORH - LDEORAH - LDEORALH - LDEORLH - Atomic exclusive OR on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEORH--LDEORAH--LDEORALH--LDEORLH--Atomic-exclusive-OR-on-halfword-in-memory-?lang=en)
///  - [LDSETH - LDSETAH - LDSETALH - LDSETLH - Atomic bit set on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSETH--LDSETAH--LDSETALH--LDSETLH--Atomic-bit-set-on-halfword-in-memory-?lang=en)
///  - [LDSMAXH - LDSMAXAH - LDSMAXALH - LDSMAXLH - Atomic signed maximum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAXH--LDSMAXAH--LDSMAXALH--LDSMAXLH--Atomic-signed-maximum-on-halfword-in-memory-?lang=en)
///  - [LDSMINH - LDSMINAH - LDSMINALH - LDSMINLH - Atomic signed minimum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMINH--LDSMINAH--LDSMINALH--LDSMINLH--Atomic-signed-minimum-on-halfword-in-memory-?lang=en)
///  - [LDUMAXH - LDUMAXAH - LDUMAXALH - LDUMAXLH - Atomic unsigned maximum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAXH--LDUMAXAH--LDUMAXALH--LDUMAXLH--Atomic-unsigned-maximum-on-halfword-in-memory-?lang=en)
///  - [LDUMINH - LDUMINAH - LDUMINALH - LDUMINLH - Atomic unsigned minimum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMINH--LDUMINAH--LDUMINALH--LDUMINLH--Atomic-unsigned-minimum-on-halfword-in-memory-?lang=en)
///  - [SWPH - SWPAH - SWPALH - SWPLH - Swap halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWPH--SWPAH--SWPALH--SWPLH--Swap-halfword-in-memory-?lang=en)
///  - [LDAPRH - Load Acquire RCpc Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPRH--Load-Acquire-RCpc-Register-Halfword-?lang=en)
///  - [LDADD - LDADDA - LDADDAL - LDADDL - Atomic add on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADD--LDADDA--LDADDAL--LDADDL--Atomic-add-on-word-or-doubleword-in-memory-?lang=en)
///  - [LDCLR - LDCLRA - LDCLRAL - LDCLRL - Atomic bit clear on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLR--LDCLRA--LDCLRAL--LDCLRL--Atomic-bit-clear-on-word-or-doubleword-in-memory-?lang=en)
///  - [LDEOR - LDEORA - LDEORAL - LDEORL - Atomic exclusive OR on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEOR--LDEORA--LDEORAL--LDEORL--Atomic-exclusive-OR-on-word-or-doubleword-in-memory-?lang=en)
///  - [LDSET - LDSETA - LDSETAL - LDSETL - Atomic bit set on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSET--LDSETA--LDSETAL--LDSETL--Atomic-bit-set-on-word-or-doubleword-in-memory-?lang=en)
///  - [LDSMAX - LDSMAXA - LDSMAXAL - LDSMAXL - Atomic signed maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAX--LDSMAXA--LDSMAXAL--LDSMAXL--Atomic-signed-maximum-on-word-or-doubleword-in-memory-?lang=en)
///  - [LDSMIN - LDSMINA - LDSMINAL - LDSMINL - Atomic signed minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMIN--LDSMINA--LDSMINAL--LDSMINL--Atomic-signed-minimum-on-word-or-doubleword-in-memory-?lang=en)
///  - [LDUMAX - LDUMAXA - LDUMAXAL - LDUMAXL - Atomic unsigned maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAX--LDUMAXA--LDUMAXAL--LDUMAXL--Atomic-unsigned-maximum-on-word-or-doubleword-in-memory-?lang=en)
///  - [LDUMIN - LDUMINA - LDUMINAL - LDUMINL - Atomic unsigned minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMIN--LDUMINA--LDUMINAL--LDUMINL--Atomic-unsigned-minimum-on-word-or-doubleword-in-memory-?lang=en)
///  - [SWP - SWPA - SWPAL - SWPL - Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWP--SWPA--SWPAL--SWPL--Swap-word-or-doubleword-in-memory-?lang=en)
///  - [LDAPR - Load Acquire RCpc Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPR--Load-Acquire-RCpc-Register-?lang=en)
///  - [ST64BV0 - Single copy Atomic 64 byte EL0 Store with Return](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST64BV0--Single-copy-Atomic-64-byte-EL0-Store-with-Return-?lang=en)
///  - [ST64BV - Single copy Atomic 64 byte Store with Return](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST64BV--Single-copy-Atomic-64-byte-Store-with-Return-?lang=en)
///  - [ST64B - Single copy Atomic 64 byte Store without Return](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST64B--Single-copy-Atomic-64-byte-Store-without-Return-?lang=en)
///  - [LD64B - Single copy Atomic 64 byte Load](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LD64B--Single-copy-Atomic-64-byte-Load-?lang=en)
pub trait AtomicMemoryOperatinos<T>: InstructionProcessor<T> {
    /// [LDADDAB - Atomic add on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADDB--LDADDAB--LDADDALB--LDADDLB--Atomic-add-on-byte-in-memory-?lang=en)
    ///
    /// Atomic add on byte in memory atomically loads an 8-bit byte from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDAB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldaddab(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 0, ws, 0, 0, xn_sp, wt)
    }


    /// [LDADDALB - Atomic add on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADDB--LDADDAB--LDADDALB--LDADDLB--Atomic-add-on-byte-in-memory-?lang=en)
    ///
    /// Atomic add on byte in memory atomically loads an 8-bit byte from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDALB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldaddalb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 1, ws, 0, 0, xn_sp, wt)
    }


    /// [LDADDB - Atomic add on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADDB--LDADDAB--LDADDALB--LDADDLB--Atomic-add-on-byte-in-memory-?lang=en)
    ///
    /// Atomic add on byte in memory atomically loads an 8-bit byte from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldaddb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 0, ws, 0, 0, xn_sp, wt)
    }


    /// [LDADDLB - Atomic add on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADDB--LDADDAB--LDADDALB--LDADDLB--Atomic-add-on-byte-in-memory-?lang=en)
    ///
    /// Atomic add on byte in memory atomically loads an 8-bit byte from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDLB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldaddlb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 1, ws, 0, 0, xn_sp, wt)
    }


    /// [LDCLRAB - Atomic bit clear on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLRB--LDCLRAB--LDCLRALB--LDCLRLB--Atomic-bit-clear-on-byte-in-memory-?lang=en)
    ///
    /// Atomic bit clear on byte in memory atomically loads an 8-bit byte from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRAB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclrab(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 0, ws, 0, 1, xn_sp, wt)
    }


    /// [LDCLRALB - Atomic bit clear on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLRB--LDCLRAB--LDCLRALB--LDCLRLB--Atomic-bit-clear-on-byte-in-memory-?lang=en)
    ///
    /// Atomic bit clear on byte in memory atomically loads an 8-bit byte from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRALB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclralb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 1, ws, 0, 1, xn_sp, wt)
    }


    /// [LDCLRB - Atomic bit clear on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLRB--LDCLRAB--LDCLRALB--LDCLRLB--Atomic-bit-clear-on-byte-in-memory-?lang=en)
    ///
    /// Atomic bit clear on byte in memory atomically loads an 8-bit byte from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclrb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 0, ws, 0, 1, xn_sp, wt)
    }


    /// [LDCLRLB - Atomic bit clear on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLRB--LDCLRAB--LDCLRALB--LDCLRLB--Atomic-bit-clear-on-byte-in-memory-?lang=en)
    ///
    /// Atomic bit clear on byte in memory atomically loads an 8-bit byte from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRLB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclrlb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 1, ws, 0, 1, xn_sp, wt)
    }


    /// [LDEORAB - Atomic exclusive OR on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEORB--LDEORAB--LDEORALB--LDEORLB--Atomic-exclusive-OR-on-byte-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on byte in memory atomically loads an 8-bit byte from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORAB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeorab(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 0, ws, 0, 0b10, xn_sp, wt)
    }


    /// [LDEORALB - Atomic exclusive OR on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEORB--LDEORAB--LDEORALB--LDEORLB--Atomic-exclusive-OR-on-byte-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on byte in memory atomically loads an 8-bit byte from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORALB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeoralb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 1, ws, 0, 0b10, xn_sp, wt)
    }


    /// [LDEORB - Atomic exclusive OR on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEORB--LDEORAB--LDEORALB--LDEORLB--Atomic-exclusive-OR-on-byte-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on byte in memory atomically loads an 8-bit byte from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeorb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 0, ws, 0, 0b10, xn_sp, wt)
    }


    /// [LDEORLB - Atomic exclusive OR on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEORB--LDEORAB--LDEORALB--LDEORLB--Atomic-exclusive-OR-on-byte-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on byte in memory atomically loads an 8-bit byte from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORLB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeorlb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 1, ws, 0, 0b10, xn_sp, wt)
    }


    /// [LDSETAB - Atomic bit set on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSETB--LDSETAB--LDSETALB--LDSETLB--Atomic-bit-set-on-byte-in-memory-?lang=en)
    ///
    /// Atomic bit set on byte in memory atomically loads an 8-bit byte from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETAB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsetab(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 0, ws, 0, 0b11, xn_sp, wt)
    }


    /// [LDSETALB - Atomic bit set on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSETB--LDSETAB--LDSETALB--LDSETLB--Atomic-bit-set-on-byte-in-memory-?lang=en)
    ///
    /// Atomic bit set on byte in memory atomically loads an 8-bit byte from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETALB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsetalb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 1, ws, 0, 0b11, xn_sp, wt)
    }


    /// [LDSETB - Atomic bit set on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSETB--LDSETAB--LDSETALB--LDSETLB--Atomic-bit-set-on-byte-in-memory-?lang=en)
    ///
    /// Atomic bit set on byte in memory atomically loads an 8-bit byte from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsetb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 0, ws, 0, 0b11, xn_sp, wt)
    }


    /// [LDSETLB - Atomic bit set on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSETB--LDSETAB--LDSETALB--LDSETLB--Atomic-bit-set-on-byte-in-memory-?lang=en)
    ///
    /// Atomic bit set on byte in memory atomically loads an 8-bit byte from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETLB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsetlb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 1, ws, 0, 0b11, xn_sp, wt)
    }


    /// [LDSMAXAB - Atomic signed maximum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAXB--LDSMAXAB--LDSMAXALB--LDSMAXLB--Atomic-signed-maximum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXAB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxab(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 0, ws, 0, 0b100, xn_sp, wt)
    }


    /// [LDSMAXALB - Atomic signed maximum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAXB--LDSMAXAB--LDSMAXALB--LDSMAXLB--Atomic-signed-maximum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXALB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxalb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 1, ws, 0, 0b100, xn_sp, wt)
    }


    /// [LDSMAXB - Atomic signed maximum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAXB--LDSMAXAB--LDSMAXALB--LDSMAXLB--Atomic-signed-maximum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 0, ws, 0, 0b100, xn_sp, wt)
    }


    /// [LDSMAXLB - Atomic signed maximum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAXB--LDSMAXAB--LDSMAXALB--LDSMAXLB--Atomic-signed-maximum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXLB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxlb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 1, ws, 0, 0b100, xn_sp, wt)
    }


    /// [LDSMINAB - Atomic signed minimum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMINB--LDSMINAB--LDSMINALB--LDSMINLB--Atomic-signed-minimum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINAB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsminab(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 0, ws, 0, 0b101, xn_sp, wt)
    }


    /// [LDSMINALB - Atomic signed minimum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMINB--LDSMINAB--LDSMINALB--LDSMINLB--Atomic-signed-minimum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINALB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsminalb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 1, ws, 0, 0b101, xn_sp, wt)
    }


    /// [LDSMINB - Atomic signed minimum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMINB--LDSMINAB--LDSMINALB--LDSMINLB--Atomic-signed-minimum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsminb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 0, ws, 0, 0b101, xn_sp, wt)
    }


    /// [LDSMINLB - Atomic signed minimum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMINB--LDSMINAB--LDSMINALB--LDSMINLB--Atomic-signed-minimum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINLB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsminlb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 1, ws, 0, 0b101, xn_sp, wt)
    }


    /// [LDUMAXAB - Atomic unsigned maximum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAXB--LDUMAXAB--LDUMAXALB--LDUMAXLB--Atomic-unsigned-maximum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXAB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxab(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 0, ws, 0, 0b110, xn_sp, wt)
    }


    /// [LDUMAXALB - Atomic unsigned maximum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAXB--LDUMAXAB--LDUMAXALB--LDUMAXLB--Atomic-unsigned-maximum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXALB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxalb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 1, ws, 0, 0b110, xn_sp, wt)
    }


    /// [LDUMAXB - Atomic unsigned maximum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAXB--LDUMAXAB--LDUMAXALB--LDUMAXLB--Atomic-unsigned-maximum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 0, ws, 0, 0b110, xn_sp, wt)
    }


    /// [LDUMAXLB - Atomic unsigned maximum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAXB--LDUMAXAB--LDUMAXALB--LDUMAXLB--Atomic-unsigned-maximum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXLB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxlb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 1, ws, 0, 0b110, xn_sp, wt)
    }


    /// [LDUMINAB - Atomic unsigned minimum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMINB--LDUMINAB--LDUMINALB--LDUMINLB--Atomic-unsigned-minimum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINAB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn lduminab(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 0, ws, 0, 0b111, xn_sp, wt)
    }


    /// [LDUMINALB - Atomic unsigned minimum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMINB--LDUMINAB--LDUMINALB--LDUMINLB--Atomic-unsigned-minimum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINALB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn lduminalb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 1, ws, 0, 0b111, xn_sp, wt)
    }


    /// [LDUMINB - Atomic unsigned minimum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMINB--LDUMINAB--LDUMINALB--LDUMINLB--Atomic-unsigned-minimum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn lduminb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 0, ws, 0, 0b111, xn_sp, wt)
    }


    /// [LDUMINLB - Atomic unsigned minimum on byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMINB--LDUMINAB--LDUMINALB--LDUMINLB--Atomic-unsigned-minimum-on-byte-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on byte in memory atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINLB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn lduminlb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 1, ws, 0, 0b111, xn_sp, wt)
    }


    /// [SWPAB - Swap byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWPB--SWPAB--SWPALB--SWPLB--Swap-byte-in-memory-?lang=en)
    ///
    /// Swap byte in memory atomically loads an 8-bit byte from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPAB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swpab(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 0, ws, 1, 0b000, xn_sp, wt)
    }


    /// [SWPALB - Swap byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWPB--SWPAB--SWPALB--SWPLB--Swap-byte-in-memory-?lang=en)
    ///
    /// Swap byte in memory atomically loads an 8-bit byte from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPALB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swpalb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 1, ws, 1, 0b000, xn_sp, wt)
    }


    /// [SWPB - Swap byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWPB--SWPAB--SWPALB--SWPLB--Swap-byte-in-memory-?lang=en)
    ///
    /// Swap byte in memory atomically loads an 8-bit byte from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swpb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 0, ws, 1, 0b000, xn_sp, wt)
    }


    /// [SWPLB - Swap byte in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWPB--SWPAB--SWPALB--SWPLB--Swap-byte-in-memory-?lang=en)
    ///
    /// Swap byte in memory atomically loads an 8-bit byte from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPLB <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swplb(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 0, 1, ws, 1, 0b000, xn_sp, wt)
    }


    /// [LDAPRB - Load Acquire RCpc Register Byte](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPRB--Load-Acquire-RCpc-Register-Byte-?lang=en)
    ///
    /// Load-Acquire RCpc Register Byte derives an address from a base register value, loads a byte from the derived address in memory, zero-extends it and writes it to a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release, except that:
    ///
    /// ```asm
    /// LDAPRB <Wt>, [<Xn|SP> {,#0}]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lrcpc")]
    fn ldaprb(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0, 0, 1, 0, 0b11111, 1, 0b100, xn_sp, wt)
    }


    /// [LDADDAH - Atomic add on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADDH--LDADDAH--LDADDALH--LDADDLH--Atomic-add-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic add on halfword in memory atomically loads a 16-bit halfword from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDAH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldaddah(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 0, ws, 0, 0b000, xn_sp, wt)
    }


    /// [LDADDALH - Atomic add on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADDH--LDADDAH--LDADDALH--LDADDLH--Atomic-add-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic add on halfword in memory atomically loads a 16-bit halfword from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDALH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldaddalh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 1, ws, 0, 0b000, xn_sp, wt)
    }


    /// [LDADDH - Atomic add on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADDH--LDADDAH--LDADDALH--LDADDLH--Atomic-add-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic add on halfword in memory atomically loads a 16-bit halfword from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldaddh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 0, ws, 0, 0b000, xn_sp, wt)
    }


    /// [LDADDLH - Atomic add on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADDH--LDADDAH--LDADDALH--LDADDLH--Atomic-add-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic add on halfword in memory atomically loads a 16-bit halfword from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDLH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldaddlh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 1, ws, 0, 0b000, xn_sp, wt)
    }


    /// [LDCLRAH - Atomic bit clear on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLRH--LDCLRAH--LDCLRALH--LDCLRLH--Atomic-bit-clear-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic bit clear on halfword in memory atomically loads a 16-bit halfword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRAH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclrah(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 0, ws, 0, 0b001, xn_sp, wt)
    }


    /// [LDCLRALH - Atomic bit clear on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLRH--LDCLRAH--LDCLRALH--LDCLRLH--Atomic-bit-clear-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic bit clear on halfword in memory atomically loads a 16-bit halfword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRALH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclralh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 1, ws, 0, 0b001, xn_sp, wt)
    }


    /// [LDCLRH - Atomic bit clear on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLRH--LDCLRAH--LDCLRALH--LDCLRLH--Atomic-bit-clear-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic bit clear on halfword in memory atomically loads a 16-bit halfword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclrh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 0, ws, 0, 0b001, xn_sp, wt)
    }


    /// [LDCLRLH - Atomic bit clear on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLRH--LDCLRAH--LDCLRALH--LDCLRLH--Atomic-bit-clear-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic bit clear on halfword in memory atomically loads a 16-bit halfword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRLH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclrlh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 1, ws, 0, 0b001, xn_sp, wt)
    }


    /// [LDEORAH - Atomic exclusive OR on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEORH--LDEORAH--LDEORALH--LDEORLH--Atomic-exclusive-OR-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on halfword in memory atomically loads a 16-bit halfword from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORAH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeorah(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 0, ws, 0, 0b010, xn_sp, wt)
    }


    /// [LDEORALH - Atomic exclusive OR on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEORH--LDEORAH--LDEORALH--LDEORLH--Atomic-exclusive-OR-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on halfword in memory atomically loads a 16-bit halfword from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORALH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeoralh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 1, ws, 0, 0b010, xn_sp, wt)
    }


    /// [LDEORH - Atomic exclusive OR on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEORH--LDEORAH--LDEORALH--LDEORLH--Atomic-exclusive-OR-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on halfword in memory atomically loads a 16-bit halfword from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeorh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 0, ws, 0, 0b010, xn_sp, wt)
    }


    /// [LDEORLH - Atomic exclusive OR on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEORH--LDEORAH--LDEORALH--LDEORLH--Atomic-exclusive-OR-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on halfword in memory atomically loads a 16-bit halfword from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORLH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeorlh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 1, ws, 0, 0b010, xn_sp, wt)
    }


    /// [LDSETAH - Atomic bit set on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSETH--LDSETAH--LDSETALH--LDSETLH--Atomic-bit-set-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic bit set on halfword in memory atomically loads a 16-bit halfword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETAH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsetah(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 0, ws, 0, 0b011, xn_sp, wt)
    }


    /// [LDSETALH - Atomic bit set on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSETH--LDSETAH--LDSETALH--LDSETLH--Atomic-bit-set-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic bit set on halfword in memory atomically loads a 16-bit halfword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETALH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsetalh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 1, ws, 0, 0b011, xn_sp, wt)
    }


    /// [LDSETH - Atomic bit set on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSETH--LDSETAH--LDSETALH--LDSETLH--Atomic-bit-set-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic bit set on halfword in memory atomically loads a 16-bit halfword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldseth(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 0, ws, 0, 0b011, xn_sp, wt)
    }


    /// [LDSETLH - Atomic bit set on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSETH--LDSETAH--LDSETALH--LDSETLH--Atomic-bit-set-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic bit set on halfword in memory atomically loads a 16-bit halfword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETLH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsetlh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 1, ws, 0, 0b011, xn_sp, wt)
    }


    /// [LDSMAXAH - Atomic signed maximum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAXH--LDSMAXAH--LDSMAXALH--LDSMAXLH--Atomic-signed-maximum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXAH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxah(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 0, ws, 0, 0b100, xn_sp, wt)
    }


    /// [LDSMAXALH - Atomic signed maximum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAXH--LDSMAXAH--LDSMAXALH--LDSMAXLH--Atomic-signed-maximum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXALH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxalh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 1, ws, 0, 0b100, xn_sp, wt)
    }


    /// [LDSMAXH - Atomic signed maximum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAXH--LDSMAXAH--LDSMAXALH--LDSMAXLH--Atomic-signed-maximum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 0, ws, 0, 0b100, xn_sp, wt)
    }


    /// [LDSMAXLH - Atomic signed maximum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAXH--LDSMAXAH--LDSMAXALH--LDSMAXLH--Atomic-signed-maximum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXLH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxlh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 1, ws, 0, 0b100, xn_sp, wt)
    }


    /// [LDSMINAH - Atomic signed minimum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMINH--LDSMINAH--LDSMINALH--LDSMINLH--Atomic-signed-minimum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINAH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsminah(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 0, ws, 0, 0b101, xn_sp, wt)
    }


    /// [LDSMINALH - Atomic signed minimum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMINH--LDSMINAH--LDSMINALH--LDSMINLH--Atomic-signed-minimum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINALH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsminalh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 1, ws, 0, 0b101, xn_sp, wt)
    }


    /// [LDSMINH - Atomic signed minimum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMINH--LDSMINAH--LDSMINALH--LDSMINLH--Atomic-signed-minimum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsminh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 0, ws, 0, 0b101, xn_sp, wt)
    }


    /// [LDSMINLH - Atomic signed minimum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMINH--LDSMINAH--LDSMINALH--LDSMINLH--Atomic-signed-minimum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINLH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsminlh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 1, ws, 0, 0b101, xn_sp, wt)
    }


    /// [LDUMAXAH - Atomic unsigned maximum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAXH--LDUMAXAH--LDUMAXALH--LDUMAXLH--Atomic-unsigned-maximum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXAH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxah(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 0, ws, 0, 0b110, xn_sp, wt)
    }


    /// [LDUMAXALH - Atomic unsigned maximum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAXH--LDUMAXAH--LDUMAXALH--LDUMAXLH--Atomic-unsigned-maximum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXALH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxalh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 1, ws, 0, 0b110, xn_sp, wt)
    }


    /// [LDUMAXH - Atomic unsigned maximum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAXH--LDUMAXAH--LDUMAXALH--LDUMAXLH--Atomic-unsigned-maximum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 0, ws, 0, 0b110, xn_sp, wt)
    }


    /// [LDUMAXLH - Atomic unsigned maximum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAXH--LDUMAXAH--LDUMAXALH--LDUMAXLH--Atomic-unsigned-maximum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXLH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxlh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 1, ws, 0, 0b110, xn_sp, wt)
    }


    /// [LDUMINAH - Atomic unsigned minimum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMINH--LDUMINAH--LDUMINALH--LDUMINLH--Atomic-unsigned-minimum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINAH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn lduminah(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 0, ws, 0, 0b111, xn_sp, wt)
    }


    /// [LDUMINALH - Atomic unsigned minimum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMINH--LDUMINAH--LDUMINALH--LDUMINLH--Atomic-unsigned-minimum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINALH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn lduminalh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 1, ws, 0, 0b111, xn_sp, wt)
    }


    /// [LDUMINH - Atomic unsigned minimum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMINH--LDUMINAH--LDUMINALH--LDUMINLH--Atomic-unsigned-minimum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn lduminh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 0, ws, 0, 0b111, xn_sp, wt)
    }


    /// [LDUMINLH - Atomic unsigned minimum on halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMINH--LDUMINAH--LDUMINALH--LDUMINLH--Atomic-unsigned-minimum-on-halfword-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on halfword in memory atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINLH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn lduminlh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 1, ws, 0, 0b111, xn_sp, wt)
    }


    /// [SWPAH - Swap halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWPH--SWPAH--SWPALH--SWPLH--Swap-halfword-in-memory-?lang=en)
    ///
    /// Swap halfword in memory atomically loads a 16-bit halfword from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPAH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swpah(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 0, ws, 1, 0b000, xn_sp, wt)
    }


    /// [SWPALH - Swap halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWPH--SWPAH--SWPALH--SWPLH--Swap-halfword-in-memory-?lang=en)
    ///
    /// Swap halfword in memory atomically loads a 16-bit halfword from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPALH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swpalh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 1, ws, 1, 0b000, xn_sp, wt)
    }


    /// [SWPH - Swap halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWPH--SWPAH--SWPALH--SWPLH--Swap-halfword-in-memory-?lang=en)
    ///
    /// Swap halfword in memory atomically loads a 16-bit halfword from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swph(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 0, ws, 1, 0b000, xn_sp, wt)
    }


    /// [SWPLH - Swap halfword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWPH--SWPAH--SWPALH--SWPLH--Swap-halfword-in-memory-?lang=en)
    ///
    /// Swap halfword in memory atomically loads a 16-bit halfword from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPLH <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swplh(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 0, 1, ws, 1, 0b000, xn_sp, wt)
    }


    /// [LDAPRH - Load Acquire RCpc Register Halfword](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPRH--Load-Acquire-RCpc-Register-Halfword-?lang=en)
    ///
    /// Load-Acquire RCpc Register Halfword derives an address from a base register value, loads a halfword from the derived address in memory, zero-extends it and writes it to a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release, except that:
    ///
    /// ```asm
    /// LDAPRH <Wt>, [<Xn|SP> {,#0}]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lrcpc")]
    #[cfg(feature = "arm_feat_lse")]
    fn ldaprh(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b01, 0, 1, 0, 0b11111, 1, 0b100, xn_sp, wt)
    }


    /// [LDADD - Atomic add on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADD--LDADDA--LDADDAL--LDADDL--Atomic-add-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic add on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADD <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldadd_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 0, ws, 0, 0b000, xn_sp, wt)
    }


    /// [LDADDA - Atomic add on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADD--LDADDA--LDADDAL--LDADDL--Atomic-add-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic add on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDA <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldadda_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 0, ws, 0, 0b000, xn_sp, wt)
    }


    /// [LDADDAL - Atomic add on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADD--LDADDA--LDADDAL--LDADDL--Atomic-add-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic add on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDAL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldaddal_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 1, ws, 0, 0b000, xn_sp, wt)
    }


    /// [LDADDL - Atomic add on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADD--LDADDA--LDADDAL--LDADDL--Atomic-add-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic add on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldaddl_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 1, ws, 0, 0b000, xn_sp, wt)
    }


    /// [LDADD - Atomic add on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADD--LDADDA--LDADDAL--LDADDL--Atomic-add-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic add on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADD <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldadd_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 0, xs, 0, 0b000, xn_sp, xt)
    }


    /// [LDADDA - Atomic add on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADD--LDADDA--LDADDAL--LDADDL--Atomic-add-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic add on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDA <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldadda_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 0, xs, 0, 0b000, xn_sp, xt)
    }


    /// [LDADDAL - Atomic add on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADD--LDADDA--LDADDAL--LDADDL--Atomic-add-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic add on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDAL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldaddal_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 1, xs, 0, 0b000, xn_sp, xt)
    }


    /// [LDADDL - Atomic add on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDADD--LDADDA--LDADDAL--LDADDL--Atomic-add-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic add on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, adds the value held in a register to it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDADDL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldaddl_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 1, xs, 0, 0b000, xn_sp, xt)
    }


    /// [LDCLR - Atomic bit clear on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLR--LDCLRA--LDCLRAL--LDCLRL--Atomic-bit-clear-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit clear on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLR <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclr_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 0, ws, 0, 0b001, xn_sp, wt)
    }


    /// [LDCLRA - Atomic bit clear on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLR--LDCLRA--LDCLRAL--LDCLRL--Atomic-bit-clear-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit clear on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRA <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclra_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 0, ws, 0, 0b001, xn_sp, wt)
    }


    /// [LDCLRAL - Atomic bit clear on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLR--LDCLRA--LDCLRAL--LDCLRL--Atomic-bit-clear-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit clear on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRAL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclral_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 1, ws, 0, 0b001, xn_sp, wt)
    }


    /// [LDCLRL - Atomic bit clear on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLR--LDCLRA--LDCLRAL--LDCLRL--Atomic-bit-clear-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit clear on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclrl_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 1, ws, 0, 0b001, xn_sp, wt)
    }


    /// [LDCLR - Atomic bit clear on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLR--LDCLRA--LDCLRAL--LDCLRL--Atomic-bit-clear-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit clear on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLR <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclr_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 0, xs, 0, 0b001, xn_sp, xt)
    }


    /// [LDCLRA - Atomic bit clear on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLR--LDCLRA--LDCLRAL--LDCLRL--Atomic-bit-clear-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit clear on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRA <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclra_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 0, xs, 0, 0b001, xn_sp, xt)
    }


    /// [LDCLRAL - Atomic bit clear on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLR--LDCLRA--LDCLRAL--LDCLRL--Atomic-bit-clear-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit clear on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRAL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclral_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 1, xs, 0, 0b001, xn_sp, xt)
    }


    /// [LDCLRL - Atomic bit clear on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDCLR--LDCLRA--LDCLRAL--LDCLRL--Atomic-bit-clear-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit clear on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDCLRL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldclrl_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 1, xs, 0, 0b001, xn_sp, xt)
    }


    /// [LDEOR - Atomic exclusive OR on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEOR--LDEORA--LDEORAL--LDEORL--Atomic-exclusive-OR-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEOR <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeor_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 0, ws, 0, 0b010, xn_sp, wt)
    }


    /// [LDEORA - Atomic exclusive OR on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEOR--LDEORA--LDEORAL--LDEORL--Atomic-exclusive-OR-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORA <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeora_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 0, ws, 0, 0b010, xn_sp, wt)
    }


    /// [LDEORAL - Atomic exclusive OR on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEOR--LDEORA--LDEORAL--LDEORL--Atomic-exclusive-OR-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORAL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeoral_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 1, ws, 0, 0b010, xn_sp, wt)
    }


    /// [LDEORL - Atomic exclusive OR on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEOR--LDEORA--LDEORAL--LDEORL--Atomic-exclusive-OR-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeorl_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 1, ws, 0, 0b010, xn_sp, wt)
    }


    /// [LDEOR - Atomic exclusive OR on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEOR--LDEORA--LDEORAL--LDEORL--Atomic-exclusive-OR-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEOR <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeor_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 0, xs, 0, 0b010, xn_sp, xt)
    }


    /// [LDEORA - Atomic exclusive OR on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEOR--LDEORA--LDEORAL--LDEORL--Atomic-exclusive-OR-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORA <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeora_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 0, xs, 0, 0b010, xn_sp, xt)
    }


    /// [LDEORAL - Atomic exclusive OR on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEOR--LDEORA--LDEORAL--LDEORL--Atomic-exclusive-OR-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORAL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeoral_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 1, xs, 0, 0b010, xn_sp, xt)
    }


    /// [LDEORL - Atomic exclusive OR on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDEOR--LDEORA--LDEORAL--LDEORL--Atomic-exclusive-OR-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic exclusive OR on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs an exclusive OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDEORL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldeorl_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 1, xs, 0, 0b010, xn_sp, xt)
    }


    /// [LDSET - Atomic bit set on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSET--LDSETA--LDSETAL--LDSETL--Atomic-bit-set-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit set on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSET <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldset_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 0, ws, 0, 0b011, xn_sp, wt)
    }


    /// [LDSETA - Atomic bit set on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSET--LDSETA--LDSETAL--LDSETL--Atomic-bit-set-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit set on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETA <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldseta_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 0, ws, 0, 0b011, xn_sp, wt)
    }


    /// [LDSETAL - Atomic bit set on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSET--LDSETA--LDSETAL--LDSETL--Atomic-bit-set-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit set on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETAL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsetal_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 1, ws, 0, 0b011, xn_sp, wt)
    }


    /// [LDSETL - Atomic bit set on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSET--LDSETA--LDSETAL--LDSETL--Atomic-bit-set-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit set on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsetl_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 1, ws, 0, 0b011, xn_sp, wt)
    }


    /// [LDSET - Atomic bit set on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSET--LDSETA--LDSETAL--LDSETL--Atomic-bit-set-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit set on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSET <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldset_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 0, xs, 0, 0b011, xn_sp, xt)
    }


    /// [LDSETA - Atomic bit set on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSET--LDSETA--LDSETAL--LDSETL--Atomic-bit-set-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit set on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETA <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldseta_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 0, xs, 0, 0b011, xn_sp, xt)
    }


    /// [LDSETAL - Atomic bit set on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSET--LDSETA--LDSETAL--LDSETL--Atomic-bit-set-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit set on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETAL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsetal_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 1, xs, 0, 0b011, xn_sp, xt)
    }


    /// [LDSETL - Atomic bit set on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSET--LDSETA--LDSETAL--LDSETL--Atomic-bit-set-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic bit set on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSETL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsetl_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 1, xs, 0, 0b011, xn_sp, xt)
    }


    /// [LDSMAX - Atomic signed maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAX--LDSMAXA--LDSMAXAL--LDSMAXL--Atomic-signed-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAX <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmax_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 0, ws, 0, 0b100, xn_sp, wt)
    }


    /// [LDSMAXA - Atomic signed maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAX--LDSMAXA--LDSMAXAL--LDSMAXL--Atomic-signed-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXA <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxa_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 0, ws, 0, 0b100, xn_sp, wt)
    }


    /// [LDSMAXAL - Atomic signed maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAX--LDSMAXA--LDSMAXAL--LDSMAXL--Atomic-signed-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXAL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxal_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 1, ws, 0, 0b100, xn_sp, wt)
    }


    /// [LDSMAXL - Atomic signed maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAX--LDSMAXA--LDSMAXAL--LDSMAXL--Atomic-signed-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxl_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 1, ws, 0, 0b100, xn_sp, wt)
    }


    /// [LDSMAX - Atomic signed maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAX--LDSMAXA--LDSMAXAL--LDSMAXL--Atomic-signed-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAX <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmax_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 0, xs, 0, 0b100, xn_sp, xt)
    }


    /// [LDSMAXA - Atomic signed maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAX--LDSMAXA--LDSMAXAL--LDSMAXL--Atomic-signed-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXA <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxa_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 0, xs, 0, 0b100, xn_sp, xt)
    }


    /// [LDSMAXAL - Atomic signed maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAX--LDSMAXA--LDSMAXAL--LDSMAXL--Atomic-signed-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXAL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxal_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 1, xs, 0, 0b100, xn_sp, xt)
    }


    /// [LDSMAXL - Atomic signed maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMAX--LDSMAXA--LDSMAXAL--LDSMAXL--Atomic-signed-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMAXL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmaxl_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 1, xs, 0, 0b100, xn_sp, xt)
    }


    /// [LDSMIN - Atomic signed minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMIN--LDSMINA--LDSMINAL--LDSMINL--Atomic-signed-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMIN <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmin_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 0, ws, 0, 0b101, xn_sp, wt)
    }


    /// [LDSMINA - Atomic signed minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMIN--LDSMINA--LDSMINAL--LDSMINL--Atomic-signed-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINA <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmina_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 0, ws, 0, 0b101, xn_sp, wt)
    }


    /// [LDSMINAL - Atomic signed minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMIN--LDSMINA--LDSMINAL--LDSMINL--Atomic-signed-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINAL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsminal_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 1, ws, 0, 0b101, xn_sp, wt)
    }


    /// [LDSMINL - Atomic signed minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMIN--LDSMINA--LDSMINAL--LDSMINL--Atomic-signed-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsminl_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 1, ws, 0, 0b101, xn_sp, wt)
    }


    /// [LDSMIN - Atomic signed minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMIN--LDSMINA--LDSMINAL--LDSMINL--Atomic-signed-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMIN <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmin_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 0, xs, 0, 0b101, xn_sp, xt)
    }


    /// [LDSMINA - Atomic signed minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMIN--LDSMINA--LDSMINAL--LDSMINL--Atomic-signed-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINA <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsmina_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 0, xs, 0, 0b101, xn_sp, xt)
    }


    /// [LDSMINAL - Atomic signed minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMIN--LDSMINA--LDSMINAL--LDSMINL--Atomic-signed-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINAL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsminal_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 1, xs, 0, 0b101, xn_sp, xt)
    }


    /// [LDSMINL - Atomic signed minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDSMIN--LDSMINA--LDSMINAL--LDSMINL--Atomic-signed-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic signed minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDSMINL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldsminl_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 1, xs, 0, 0b101, xn_sp, xt)
    }


    /// [LDUMAX - Atomic unsigned maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAX--LDUMAXA--LDUMAXAL--LDUMAXL--Atomic-unsigned-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAX <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumax_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 0, ws, 0, 0b110, xn_sp, wt)
    }


    /// [LDUMAXA - Atomic unsigned maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAX--LDUMAXA--LDUMAXAL--LDUMAXL--Atomic-unsigned-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXA <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxa_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 0, ws, 0, 0b110, xn_sp, wt)
    }


    /// [LDUMAXAL - Atomic unsigned maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAX--LDUMAXA--LDUMAXAL--LDUMAXL--Atomic-unsigned-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXAL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxal_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 1, ws, 0, 0b110, xn_sp, wt)
    }


    /// [LDUMAXL - Atomic unsigned maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAX--LDUMAXA--LDUMAXAL--LDUMAXL--Atomic-unsigned-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxl_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 1, ws, 0, 0b110, xn_sp, wt)
    }


    /// [LDUMAX - Atomic unsigned maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAX--LDUMAXA--LDUMAXAL--LDUMAXL--Atomic-unsigned-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAX <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumax_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 0, xs, 0, 0b110, xn_sp, xt)
    }


    /// [LDUMAXA - Atomic unsigned maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAX--LDUMAXA--LDUMAXAL--LDUMAXL--Atomic-unsigned-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXA <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxa_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 0, xs, 0, 0b110, xn_sp, xt)
    }


    /// [LDUMAXAL - Atomic unsigned maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAX--LDUMAXA--LDUMAXAL--LDUMAXL--Atomic-unsigned-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXAL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxal_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 1, xs, 0, 0b110, xn_sp, xt)
    }


    /// [LDUMAXL - Atomic unsigned maximum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMAX--LDUMAXA--LDUMAXAL--LDUMAXL--Atomic-unsigned-maximum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned maximum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMAXL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumaxl_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 1, xs, 0, 0b110, xn_sp, xt)
    }


    /// [LDUMIN - Atomic unsigned minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMIN--LDUMINA--LDUMINAL--LDUMINL--Atomic-unsigned-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMIN <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumin_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 0, ws, 0, 0b111, xn_sp, wt)
    }


    /// [LDUMINA - Atomic unsigned minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMIN--LDUMINA--LDUMINAL--LDUMINL--Atomic-unsigned-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINA <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumina_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 0, ws, 0, 0b111, xn_sp, wt)
    }


    /// [LDUMINAL - Atomic unsigned minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMIN--LDUMINA--LDUMINAL--LDUMINL--Atomic-unsigned-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINAL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn lduminal_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 1, ws, 0, 0b111, xn_sp, wt)
    }


    /// [LDUMINL - Atomic unsigned minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMIN--LDUMINA--LDUMINAL--LDUMINL--Atomic-unsigned-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn lduminl_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 1, ws, 0, 0b111, xn_sp, wt)
    }


    /// [LDUMIN - Atomic unsigned minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMIN--LDUMINA--LDUMINAL--LDUMINL--Atomic-unsigned-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMIN <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumin_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 0, xs, 0, 0b111, xn_sp, xt)
    }


    /// [LDUMINA - Atomic unsigned minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMIN--LDUMINA--LDUMINAL--LDUMINL--Atomic-unsigned-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINA <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn ldumina_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 0, xs, 0, 0b111, xn_sp, xt)
    }


    /// [LDUMINAL - Atomic unsigned minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMIN--LDUMINA--LDUMINAL--LDUMINL--Atomic-unsigned-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINAL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn lduminal_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 1, xs, 0, 0b111, xn_sp, xt)
    }


    /// [LDUMINL - Atomic unsigned minimum on word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDUMIN--LDUMINA--LDUMINAL--LDUMINL--Atomic-unsigned-minimum-on-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Atomic unsigned minimum on word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as unsigned numbers. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// LDUMINL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn lduminl_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 1, xs, 0, 0b111, xn_sp, xt)
    }


    /// [SWP - Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWP--SWPA--SWPAL--SWPL--Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Swap word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWP <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swp_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 0, ws, 1, 0b000, xn_sp, wt)
    }


    /// [SWPA - Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWP--SWPA--SWPAL--SWPL--Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Swap word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPA <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swpa_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 0, ws, 1, 0b000, xn_sp, wt)
    }


    /// [SWPAL- Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWP--SWPA--SWPAL--SWPL--Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Swap word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPAL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swpal_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 1, ws, 1, 0b000, xn_sp, wt)
    }


    /// [SWPL - Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWP--SWPA--SWPAL--SWPL--Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Swap word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPL <Ws>, <Wt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swpl_32(&mut self, ws: Register, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 0, 1, ws, 1, 0b000, xn_sp, wt)
    }


    /// [SWP - Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWP--SWPA--SWPAL--SWPL--Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Swap word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWP <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swp_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 0, xs, 1, 0b000, xn_sp, xt)
    }


    /// [SWPA - Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWP--SWPA--SWPAL--SWPL--Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Swap word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPA <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swpa_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 0, xs, 1, 0b000, xn_sp, xt)
    }


    /// [SWPAL - Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWP--SWPA--SWPAL--SWPL--Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Swap word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPAL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swpal_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 1, xs, 1, 0b000, xn_sp, xt)
    }


    /// [SWPL - Swap word or doubleword in memory](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SWP--SWPA--SWPAL--SWPL--Swap-word-or-doubleword-in-memory-?lang=en)
    ///
    /// Swap word or doubleword in memory atomically loads a 32-bit word or 64-bit doubleword from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register.
    ///
    /// For more information about memory ordering semantics see Load-Acquire, Store-Release.
    ///
    /// ```asm
    /// SWPL <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lse")]
    fn swpl_64(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 1, xs, 1, 0b000, xn_sp, xt)
    }


    /// [LDAPR - Load Acquire RCpc Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPR--Load-Acquire-RCpc-Register-?lang=en)
    ///
    /// Load-Acquire RCpc Register derives an address from a base register value, loads a 32-bit word or 64-bit doubleword from the derived address in memory, and writes it to a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release, except that:
    ///
    /// ```asm
    /// LDAPR <Wt>, [<Xn|SP> {,#0}]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lrcpc")]
    fn ldapr_32(&mut self, wt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b10, 0, 1, 0, 0b11111, 1, 0b100, xn_sp, wt)
    }


    /// [LDAPR - Load Acquire RCpc Register](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LDAPR--Load-Acquire-RCpc-Register-?lang=en)
    ///
    /// Load-Acquire RCpc Register derives an address from a base register value, loads a 32-bit word or 64-bit doubleword from the derived address in memory, and writes it to a register.
    ///
    /// The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release, except that:
    ///
    /// ```asm
    /// LDAPR <Xt>, [<Xn|SP> {,#0}]
    /// ```
    #[inline(always)]
    #[cfg(feature = "arm_feat_lrcpc")]
    fn ldapr_64(&mut self, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 1, 0, 0b11111, 1, 0b100, xn_sp, xt)
    }


    /// [ST64BV0 - Single copy Atomic 64 byte EL0 Store with Return](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST64BV0--Single-copy-Atomic-64-byte-EL0-Store-with-Return-?lang=en)
    ///
    /// Single-copy Atomic 64-byte EL0 Store with Return stores eight 64-bit doublewords from consecutive registers, Xt to X(t+7), to a memory location, with the bottom 32 bits taken from ACCDATA_EL1, and writes the status result of the store to a register. The data that is stored is atomic and is required to be 64-byte aligned.
    ///
    /// ```asm
    /// ST64BV0 <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    ///
    /// *Note:* not tested
    #[inline(always)]
    #[cfg(feature = "arm_feat_ls64_accdata")]
    fn st64bv0(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 0, xs, 1, 0b010, xn_sp, xt)
    }


    /// [ST64BV - Single copy Atomic 64 byte Store with Return](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST64BV--Single-copy-Atomic-64-byte-Store-with-Return-?lang=en)
    ///
    /// Single-copy Atomic 64-byte Store with Return stores eight 64-bit doublewords from consecutive registers, Xt to X(t+7), to a memory location, and writes the status result of the store to a register. The data that is stored is atomic and is required to be 64-byte aligned.
    ///
    /// ```asm
    /// ST64BV <Xs>, <Xt>, [<Xn|SP>]
    /// ```
    ///
    /// *Note:* not tested
    #[inline(always)]
    #[cfg(feature = "arm_feat_ls64_v")]
    fn st64bv(&mut self, xs: Register, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 0, xs, 1, 0b011, xn_sp, xt)
    }


    /// [ST64B - Single copy Atomic 64 byte Store without Return](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ST64B--Single-copy-Atomic-64-byte-Store-without-Return-?lang=en)
    ///
    /// Single-copy Atomic 64-byte Store without Return stores eight 64-bit doublewords from consecutive registers, Xt to X(t+7), to a memory location. The data that is stored is atomic and is required to be 64-byte-aligned.
    ///
    /// ```asm
    /// ST64B <Xt>, [<Xn|SP> {,#0}]
    /// ```
    ///
    /// *Note:* not tested
    #[inline(always)]
    #[cfg(feature = "arm_feat_ls64")]
    fn st64b(&mut self, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 0, 0b11111, 1, 0b001, xn_sp, xt)
    }


    /// [LD64B - Single copy Atomic 64 byte Load](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/LD64B--Single-copy-Atomic-64-byte-Load-?lang=en)
    ///
    /// Single-copy Atomic 64-byte Load derives an address from a base register value, loads eight 64-bit doublewords from a memory location, and writes them to consecutive registers, Xt to X(t+7). The data that is loaded is atomic and is required to be 64-byte aligned.
    ///
    /// ```asm
    /// LD64B <Xt>, [<Xn|SP> {,#0}]
    /// ```
    ///
    /// *Note:* not tested
    #[inline(always)]
    #[cfg(feature = "arm_feat_ls64")]
    fn ld64b(&mut self, xt: Register, xn_sp: Register) -> T {
        emit_atomic_mem_op(self, 0b11, 0, 0, 0, 0b11111, 1, 0b101, xn_sp, xt)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_producer::TestProducer;
    use super::*;

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldadd_x() {
        let mut prod = TestProducer::new();

        let instr = prod.ldaddab(1, 5, 2);
        assert_eq!(instr, "ldaddab w1, w5, [x2]");

        let instr = prod.ldaddalb(1, 5, 2);
        assert_eq!(instr, "ldaddalb w1, w5, [x2]");

        let instr = prod.ldaddb(1, 5, 2);
        assert_eq!(instr, "ldaddb w1, w5, [x2]");

        let instr = prod.ldaddlb(1, 5, 2);
        assert_eq!(instr, "ldaddlb w1, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldclr_x() {
        let mut prod = TestProducer::new();

        let instr = prod.ldclrab(1, 5, 2);
        assert_eq!(instr, "ldclrab w1, w5, [x2]");

        let instr = prod.ldclralb(1, 5, 2);
        assert_eq!(instr, "ldclralb w1, w5, [x2]");

        let instr = prod.ldclrb(1, 5, 2);
        assert_eq!(instr, "ldclrb w1, w5, [x2]");

        let instr = prod.ldclrlb(1, 5, 2);
        assert_eq!(instr, "ldclrlb w1, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldeor_x() {
        let mut prod = TestProducer::new();

        let instr = prod.ldeorab(1, 5, 2);
        assert_eq!(instr, "ldeorab w1, w5, [x2]");

        let instr = prod.ldeoralb(1, 5, 2);
        assert_eq!(instr, "ldeoralb w1, w5, [x2]");

        let instr = prod.ldeorb(1, 5, 2);
        assert_eq!(instr, "ldeorb w1, w5, [x2]");

        let instr = prod.ldeorlb(1, 5, 2);
        assert_eq!(instr, "ldeorlb w1, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldset_x() {
        let mut prod = TestProducer::new();

        let instr = prod.ldsetab(1, 5, 2);
        assert_eq!(instr, "ldsetab w1, w5, [x2]");

        let instr = prod.ldsetalb(1, 5, 2);
        assert_eq!(instr, "ldsetalb w1, w5, [x2]");

        let instr = prod.ldsetb(1, 5, 2);
        assert_eq!(instr, "ldsetb w1, w5, [x2]");

        let instr = prod.ldsetlb(1, 5, 2);
        assert_eq!(instr, "ldsetlb w1, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldsmax_x() {
        let mut prod = TestProducer::new();

        let instr = prod.ldsmaxab(1, 5, 2);
        assert_eq!(instr, "ldsmaxab w1, w5, [x2]");

        let instr = prod.ldsmaxalb(1, 5, 2);
        assert_eq!(instr, "ldsmaxalb w1, w5, [x2]");

        let instr = prod.ldsmaxb(1, 5, 2);
        assert_eq!(instr, "ldsmaxb w1, w5, [x2]");

        let instr = prod.ldsmaxlb(1, 5, 2);
        assert_eq!(instr, "ldsmaxlb w1, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldsmin_x() {
        let mut prod = TestProducer::new();

        let instr = prod.ldsminab(1, 5, 2);
        assert_eq!(instr, "ldsminab w1, w5, [x2]");

        let instr = prod.ldsminalb(1, 5, 2);
        assert_eq!(instr, "ldsminalb w1, w5, [x2]");

        let instr = prod.ldsminb(1, 5, 2);
        assert_eq!(instr, "ldsminb w1, w5, [x2]");

        let instr = prod.ldsminlb(1, 5, 2);
        assert_eq!(instr, "ldsminlb w1, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldumax_x() {
        let mut prod = TestProducer::new();

        let instr = prod.ldumaxab(1, 5, 2);
        assert_eq!(instr, "ldumaxab w1, w5, [x2]");

        let instr = prod.ldumaxalb(1, 5, 2);
        assert_eq!(instr, "ldumaxalb w1, w5, [x2]");

        let instr = prod.ldumaxb(1, 5, 2);
        assert_eq!(instr, "ldumaxb w1, w5, [x2]");

        let instr = prod.ldumaxlb(1, 5, 2);
        assert_eq!(instr, "ldumaxlb w1, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldumin_x() {
        let mut prod = TestProducer::new();

        let instr = prod.lduminab(1, 5, 2);
        assert_eq!(instr, "lduminab w1, w5, [x2]");

        let instr = prod.lduminalb(1, 5, 2);
        assert_eq!(instr, "lduminalb w1, w5, [x2]");

        let instr = prod.lduminb(1, 5, 2);
        assert_eq!(instr, "lduminb w1, w5, [x2]");

        let instr = prod.lduminlb(1, 5, 2);
        assert_eq!(instr, "lduminlb w1, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_swp_x() {
        let mut prod = TestProducer::new();

        let instr = prod.swpab(1, 5, 2);
        assert_eq!(instr, "swpab w1, w5, [x2]");

        let instr = prod.swpalb(1, 5, 2);
        assert_eq!(instr, "swpalb w1, w5, [x2]");

        let instr = prod.swpb(3, 8, 2);
        assert_eq!(instr, "swpb w3, w8, [x2]");

        let instr = prod.swplb(1, 5, 2);
        assert_eq!(instr, "swplb w1, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lrcpc")]
    #[test]
    fn test_ldaprb_x() {
        let mut prod = TestProducer::new();

        let instr = prod.ldaprb(5, 2);
        assert_eq!(instr, "ldaprb w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldadd_xh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldaddah(1, 12, 2);
        assert_eq!(instr, "ldaddah w1, w12, [x2]");

        let instr = prod.ldaddalh(1, 5, 0b11111);
        assert_eq!(instr, "ldaddalh w1, w5, [sp]");

        let instr = prod.ldaddh(3, 30, 2);
        assert_eq!(instr, "ldaddh w3, w30, [x2]");

        let instr = prod.ldaddlh(30, 5, 2);
        assert_eq!(instr, "ldaddlh w30, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldclr_xh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldclrah(1, 12, 2);
        assert_eq!(instr, "ldclrah w1, w12, [x2]");

        let instr = prod.ldclralh(1, 5, 0b11111);
        assert_eq!(instr, "ldclralh w1, w5, [sp]");

        let instr = prod.ldclrh(3, 30, 2);
        assert_eq!(instr, "ldclrh w3, w30, [x2]");

        let instr = prod.ldclrlh(30, 5, 2);
        assert_eq!(instr, "ldclrlh w30, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldeorh_xh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldeorah(1, 12, 2);
        assert_eq!(instr, "ldeorah w1, w12, [x2]");

        let instr = prod.ldeoralh(1, 5, 0b11111);
        assert_eq!(instr, "ldeoralh w1, w5, [sp]");

        let instr = prod.ldeorh(3, 30, 2);
        assert_eq!(instr, "ldeorh w3, w30, [x2]");

        let instr = prod.ldeorlh(30, 5, 2);
        assert_eq!(instr, "ldeorlh w30, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldset_xh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldsetah(1, 12, 2);
        assert_eq!(instr, "ldsetah w1, w12, [x2]");

        let instr = prod.ldsetalh(1, 5, 0b11111);
        assert_eq!(instr, "ldsetalh w1, w5, [sp]");

        let instr = prod.ldseth(3, 30, 2);
        assert_eq!(instr, "ldseth w3, w30, [x2]");

        let instr = prod.ldsetlh(30, 5, 2);
        assert_eq!(instr, "ldsetlh w30, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldsmax_xh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldsmaxah(1, 12, 2);
        assert_eq!(instr, "ldsmaxah w1, w12, [x2]");

        let instr = prod.ldsmaxalh(1, 5, 0b11111);
        assert_eq!(instr, "ldsmaxalh w1, w5, [sp]");

        let instr = prod.ldsmaxh(3, 30, 2);
        assert_eq!(instr, "ldsmaxh w3, w30, [x2]");

        let instr = prod.ldsmaxlh(30, 5, 2);
        assert_eq!(instr, "ldsmaxlh w30, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldsmin_xh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldsminah(1, 12, 2);
        assert_eq!(instr, "ldsminah w1, w12, [x2]");

        let instr = prod.ldsminalh(1, 5, 0b11111);
        assert_eq!(instr, "ldsminalh w1, w5, [sp]");

        let instr = prod.ldsminh(3, 30, 2);
        assert_eq!(instr, "ldsminh w3, w30, [x2]");

        let instr = prod.ldsminlh(30, 5, 2);
        assert_eq!(instr, "ldsminlh w30, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldumax_xh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldumaxah(1, 12, 2);
        assert_eq!(instr, "ldumaxah w1, w12, [x2]");

        let instr = prod.ldumaxalh(1, 5, 0b11111);
        assert_eq!(instr, "ldumaxalh w1, w5, [sp]");

        let instr = prod.ldumaxh(3, 30, 2);
        assert_eq!(instr, "ldumaxh w3, w30, [x2]");

        let instr = prod.ldumaxlh(30, 5, 2);
        assert_eq!(instr, "ldumaxlh w30, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldumin_xh() {
        let mut prod = TestProducer::new();

        let instr = prod.lduminah(1, 12, 2);
        assert_eq!(instr, "lduminah w1, w12, [x2]");

        let instr = prod.lduminalh(1, 5, 0b11111);
        assert_eq!(instr, "lduminalh w1, w5, [sp]");

        let instr = prod.lduminh(3, 30, 2);
        assert_eq!(instr, "lduminh w3, w30, [x2]");

        let instr = prod.lduminlh(30, 5, 2);
        assert_eq!(instr, "lduminlh w30, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_swp_xh() {
        let mut prod = TestProducer::new();

        let instr = prod.swpah(1, 12, 2);
        assert_eq!(instr, "swpah w1, w12, [x2]");

        let instr = prod.swpalh(1, 5, 0b11111);
        assert_eq!(instr, "swpalh w1, w5, [sp]");

        let instr = prod.swph(3, 30, 2);
        assert_eq!(instr, "swph w3, w30, [x2]");

        let instr = prod.swplh(30, 5, 2);
        assert_eq!(instr, "swplh w30, w5, [x2]");
    }

    #[cfg(feature = "arm_feat_lrcpc")]
    #[test]
    fn test_ldaprh() {
        let mut prod = TestProducer::new();

        let instr = prod.ldaprh(30, 30);
        assert_eq!(instr, "ldaprh w30, [x30]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldadd_32_64() {
        let mut prod = TestProducer::new();

        let instr = prod.ldadd_32(1, 12, 2);
        assert_eq!(instr, "ldadd w1, w12, [x2]");

        let instr = prod.ldadda_32(1, 5, 0b11111);
        assert_eq!(instr, "ldadda w1, w5, [sp]");

        let instr = prod.ldaddal_32(3, 30, 2);
        assert_eq!(instr, "ldaddal w3, w30, [x2]");

        let instr = prod.ldaddl_32(30, 5, 2);
        assert_eq!(instr, "ldaddl w30, w5, [x2]");

        let instr = prod.ldadd_64(1, 12, 2);
        assert_eq!(instr, "ldadd x1, x12, [x2]");

        let instr = prod.ldadda_64(1, 5, 0b11111);
        assert_eq!(instr, "ldadda x1, x5, [sp]");

        let instr = prod.ldaddal_64(3, 30, 2);
        assert_eq!(instr, "ldaddal x3, x30, [x2]");

        let instr = prod.ldaddl_64(30, 5, 2);
        assert_eq!(instr, "ldaddl x30, x5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldclr_32_64() {
        let mut prod = TestProducer::new();

        let instr = prod.ldclr_32(1, 12, 2);
        assert_eq!(instr, "ldclr w1, w12, [x2]");

        let instr = prod.ldclra_32(1, 5, 0b11111);
        assert_eq!(instr, "ldclra w1, w5, [sp]");

        let instr = prod.ldclral_32(3, 30, 2);
        assert_eq!(instr, "ldclral w3, w30, [x2]");

        let instr = prod.ldclrl_32(30, 5, 2);
        assert_eq!(instr, "ldclrl w30, w5, [x2]");

        let instr = prod.ldclr_64(1, 12, 2);
        assert_eq!(instr, "ldclr x1, x12, [x2]");

        let instr = prod.ldclra_64(1, 5, 0b11111);
        assert_eq!(instr, "ldclra x1, x5, [sp]");

        let instr = prod.ldclral_64(3, 30, 2);
        assert_eq!(instr, "ldclral x3, x30, [x2]");

        let instr = prod.ldclrl_64(30, 5, 2);
        assert_eq!(instr, "ldclrl x30, x5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldeor_32_64() {
        let mut prod = TestProducer::new();

        let instr = prod.ldeor_32(1, 12, 2);
        assert_eq!(instr, "ldeor w1, w12, [x2]");

        let instr = prod.ldeora_32(1, 5, 0b11111);
        assert_eq!(instr, "ldeora w1, w5, [sp]");

        let instr = prod.ldeoral_32(3, 30, 2);
        assert_eq!(instr, "ldeoral w3, w30, [x2]");

        let instr = prod.ldeorl_32(30, 5, 2);
        assert_eq!(instr, "ldeorl w30, w5, [x2]");

        let instr = prod.ldeor_64(1, 12, 2);
        assert_eq!(instr, "ldeor x1, x12, [x2]");

        let instr = prod.ldeora_64(1, 5, 0b11111);
        assert_eq!(instr, "ldeora x1, x5, [sp]");

        let instr = prod.ldeoral_64(3, 30, 2);
        assert_eq!(instr, "ldeoral x3, x30, [x2]");

        let instr = prod.ldeorl_64(30, 5, 2);
        assert_eq!(instr, "ldeorl x30, x5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldset_32_64() {
        let mut prod = TestProducer::new();

        let instr = prod.ldset_32(1, 12, 2);
        assert_eq!(instr, "ldset w1, w12, [x2]");

        let instr = prod.ldseta_32(1, 5, 0b11111);
        assert_eq!(instr, "ldseta w1, w5, [sp]");

        let instr = prod.ldsetal_32(3, 30, 2);
        assert_eq!(instr, "ldsetal w3, w30, [x2]");

        let instr = prod.ldsetl_32(30, 5, 2);
        assert_eq!(instr, "ldsetl w30, w5, [x2]");

        let instr = prod.ldset_64(1, 12, 2);
        assert_eq!(instr, "ldset x1, x12, [x2]");

        let instr = prod.ldseta_64(1, 5, 0b11111);
        assert_eq!(instr, "ldseta x1, x5, [sp]");

        let instr = prod.ldsetal_64(3, 30, 2);
        assert_eq!(instr, "ldsetal x3, x30, [x2]");

        let instr = prod.ldsetl_64(30, 5, 2);
        assert_eq!(instr, "ldsetl x30, x5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldsmax_32_64() {
        let mut prod = TestProducer::new();

        let instr = prod.ldsmax_32(1, 12, 2);
        assert_eq!(instr, "ldsmax w1, w12, [x2]");

        let instr = prod.ldsmaxa_32(1, 5, 0b11111);
        assert_eq!(instr, "ldsmaxa w1, w5, [sp]");

        let instr = prod.ldsmaxal_32(3, 30, 2);
        assert_eq!(instr, "ldsmaxal w3, w30, [x2]");

        let instr = prod.ldsmaxl_32(30, 5, 2);
        assert_eq!(instr, "ldsmaxl w30, w5, [x2]");

        let instr = prod.ldsmax_64(1, 12, 2);
        assert_eq!(instr, "ldsmax x1, x12, [x2]");

        let instr = prod.ldsmaxa_64(1, 5, 0b11111);
        assert_eq!(instr, "ldsmaxa x1, x5, [sp]");

        let instr = prod.ldsmaxal_64(3, 30, 2);
        assert_eq!(instr, "ldsmaxal x3, x30, [x2]");

        let instr = prod.ldsmaxl_64(30, 5, 2);
        assert_eq!(instr, "ldsmaxl x30, x5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldsmin_32_64() {
        let mut prod = TestProducer::new();

        let instr = prod.ldsmin_32(1, 12, 2);
        assert_eq!(instr, "ldsmin w1, w12, [x2]");

        let instr = prod.ldsmina_32(1, 5, 0b11111);
        assert_eq!(instr, "ldsmina w1, w5, [sp]");

        let instr = prod.ldsminal_32(3, 30, 2);
        assert_eq!(instr, "ldsminal w3, w30, [x2]");

        let instr = prod.ldsminl_32(30, 5, 2);
        assert_eq!(instr, "ldsminl w30, w5, [x2]");

        let instr = prod.ldsmin_64(1, 12, 2);
        assert_eq!(instr, "ldsmin x1, x12, [x2]");

        let instr = prod.ldsmina_64(1, 5, 0b11111);
        assert_eq!(instr, "ldsmina x1, x5, [sp]");

        let instr = prod.ldsminal_64(3, 30, 2);
        assert_eq!(instr, "ldsminal x3, x30, [x2]");

        let instr = prod.ldsminl_64(30, 5, 2);
        assert_eq!(instr, "ldsminl x30, x5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldumax_32_64() {
        let mut prod = TestProducer::new();

        let instr = prod.ldumax_32(1, 12, 2);
        assert_eq!(instr, "ldumax w1, w12, [x2]");

        let instr = prod.ldumaxa_32(1, 5, 0b11111);
        assert_eq!(instr, "ldumaxa w1, w5, [sp]");

        let instr = prod.ldumaxal_32(3, 30, 2);
        assert_eq!(instr, "ldumaxal w3, w30, [x2]");

        let instr = prod.ldumaxl_32(30, 5, 2);
        assert_eq!(instr, "ldumaxl w30, w5, [x2]");

        let instr = prod.ldumax_64(1, 12, 2);
        assert_eq!(instr, "ldumax x1, x12, [x2]");

        let instr = prod.ldumaxa_64(1, 5, 0b11111);
        assert_eq!(instr, "ldumaxa x1, x5, [sp]");

        let instr = prod.ldumaxal_64(3, 30, 2);
        assert_eq!(instr, "ldumaxal x3, x30, [x2]");

        let instr = prod.ldumaxl_64(30, 5, 2);
        assert_eq!(instr, "ldumaxl x30, x5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_ldumin_32_64() {
        let mut prod = TestProducer::new();

        let instr = prod.ldumin_32(1, 12, 2);
        assert_eq!(instr, "ldumin w1, w12, [x2]");

        let instr = prod.ldumina_32(1, 5, 0b11111);
        assert_eq!(instr, "ldumina w1, w5, [sp]");

        let instr = prod.lduminal_32(3, 30, 2);
        assert_eq!(instr, "lduminal w3, w30, [x2]");

        let instr = prod.lduminl_32(30, 5, 2);
        assert_eq!(instr, "lduminl w30, w5, [x2]");

        let instr = prod.ldumin_64(1, 12, 2);
        assert_eq!(instr, "ldumin x1, x12, [x2]");

        let instr = prod.ldumina_64(1, 5, 0b11111);
        assert_eq!(instr, "ldumina x1, x5, [sp]");

        let instr = prod.lduminal_64(3, 30, 2);
        assert_eq!(instr, "lduminal x3, x30, [x2]");

        let instr = prod.lduminl_64(30, 5, 2);
        assert_eq!(instr, "lduminl x30, x5, [x2]");
    }

    #[cfg(feature = "arm_feat_lse")]
    #[test]
    fn test_swp_32_64() {
        let mut prod = TestProducer::new();

        let instr = prod.swp_32(1, 12, 2);
        assert_eq!(instr, "swp w1, w12, [x2]");

        let instr = prod.swpa_32(1, 5, 0b11111);
        assert_eq!(instr, "swpa w1, w5, [sp]");

        let instr = prod.swpal_32(3, 30, 2);
        assert_eq!(instr, "swpal w3, w30, [x2]");

        let instr = prod.swpl_32(30, 5, 2);
        assert_eq!(instr, "swpl w30, w5, [x2]");

        let instr = prod.swp_64(1, 12, 2);
        assert_eq!(instr, "swp x1, x12, [x2]");

        let instr = prod.swpa_64(1, 5, 0b11111);
        assert_eq!(instr, "swpa x1, x5, [sp]");

        let instr = prod.swpal_64(3, 30, 2);
        assert_eq!(instr, "swpal x3, x30, [x2]");

        let instr = prod.swpl_64(30, 5, 2);
        assert_eq!(instr, "swpl x30, x5, [x2]");
    }

    #[cfg(feature = "arm_feat_lrcpc")]
    #[test]
    fn test_ldapr_32_64() {
        let mut prod = TestProducer::new();

        let instr = prod.ldapr_32(30, 30);
        assert_eq!(instr, "ldapr w30, [x30]");

        let instr = prod.ldapr_64(30, 30);
        assert_eq!(instr, "ldapr x30, [x30]");
    }
}