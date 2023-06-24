//! # [Memory Copy and Memory Set](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#memcms)
//!
//! Implements the following instructions:
//!  - [CPYFP - CPYFM - CPYFE - Memory Copy Forward only](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFP--CPYFM--CPYFE--Memory-Copy-Forward-only-?lang=en)
//!  - [CPYFPWT - CPYFMWT - CPYFEWT - Memory Copy Forward only - writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWT--CPYFMWT--CPYFEWT--Memory-Copy-Forward-only--writes-unprivileged-?lang=en)
//!  - [CPYFPRT - CPYFMRT - CPYFERT - Memory Copy Forward only - reads unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRT--CPYFMRT--CPYFERT--Memory-Copy-Forward-only--reads-unprivileged-?lang=en)
//!  - [CPYFPT - CPYFMT - CPYFET - Memory Copy Forward only - reads and writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPT--CPYFMT--CPYFET--Memory-Copy-Forward-only--reads-and-writes-unprivileged-?lang=en)
//!  - [CPYFPWN - CPYFMWN - CPYFEWN - Memory Copy Forward only - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWN--CPYFMWN--CPYFEWN--Memory-Copy-Forward-only--writes-non-temporal-?lang=en)
//!  - [CPYFPWTWN - CPYFMWTWN - CPYFEWTWN - Memory Copy Forward only - writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTWN--CPYFMWTWN--CPYFEWTWN--Memory-Copy-Forward-only--writes-unprivileged-and-non-temporal-?lang=en)
//!  - [CPYFPRTWN - CPYFMRTWN - CPYFERTWN - Memory Copy Forward only - reads unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTWN--CPYFMRTWN--CPYFERTWN--Memory-Copy-Forward-only--reads-unprivileged--writes-non-temporal-?lang=en)
//!  - [CPYFPTWN - CPYFMTWN - CPYFETWN - Memory Copy Forward only - reads and writes unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTWN--CPYFMTWN--CPYFETWN--Memory-Copy-Forward-only--reads-and-writes-unprivileged--writes-non-temporal-?lang=en)
//!  - [CPYFPRN - CPYFMRN - CPYFERN - Memory Copy Forward only - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRN--CPYFMRN--CPYFERN--Memory-Copy-Forward-only--reads-non-temporal-?lang=en)
//!  - [CPYFPWTRN - CPYFMWTRN - CPYFEWTRN - Memory Copy Forward only - writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTRN--CPYFMWTRN--CPYFEWTRN--Memory-Copy-Forward-only--writes-unprivileged--reads-non-temporal-?lang=en)
//!  - [CPYFPRTRN - CPYFMRTRN - CPYFERTRN - Memory Copy Forward only - reads unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTRN--CPYFMRTRN--CPYFERTRN--Memory-Copy-Forward-only--reads-unprivileged-and-non-temporal-?lang=en)
//!  - [CPYFPTRN - CPYFMTRN - CPYFETRN - Memory Copy Forward only - reads and writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTRN--CPYFMTRN--CPYFETRN--Memory-Copy-Forward-only--reads-and-writes-unprivileged--reads-non-temporal-?lang=en)
//!  - [CPYFPN - CPYFMN - CPYFEN - Memory Copy Forward only - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPN--CPYFMN--CPYFEN--Memory-Copy-Forward-only--reads-and-writes-non-temporal-?lang=en)
//!  - [CPYFPWTN - CPYFMWTN - CPYFEWTN - Memory Copy Forward only - writes unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTN--CPYFMWTN--CPYFEWTN--Memory-Copy-Forward-only--writes-unprivileged--reads-and-writes-non-temporal-?lang=en)
//!  - [CPYFPRTN - CPYFMRTN - CPYFERTN - Memory Copy Forward only - reads unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTN--CPYFMRTN--CPYFERTN--Memory-Copy-Forward-only--reads-unprivileged--reads-and-writes-non-temporal-?lang=en)
//!  - [CPYFPTN - CPYFMTN - CPYFETN - Memory Copy Forward only - reads and writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTN--CPYFMTN--CPYFETN--Memory-Copy-Forward-only--reads-and-writes-unprivileged-and-non-temporal-?lang=en)
//!  - [SETP - SETM - SETE - Memory Set](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETP--SETM--SETE--Memory-Set-?lang=en)
//!  - [SETPT - SETMT - SETET - Memory Set - unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPT--SETMT--SETET--Memory-Set--unprivileged-?lang=en)
//!  - [SETPN - SETMN - SETEN - Memory Set - non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPN--SETMN--SETEN--Memory-Set--non-temporal-?lang=en)
//!  - [SETPTN - SETMTN - SETETN - Memory Set - unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPTN--SETMTN--SETETN--Memory-Set--unprivileged-and-non-temporal-?lang=en)
//!  - [CPYP - CPYM - CPYE - Memory Copy](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYP--CPYM--CPYE--Memory-Copy-?lang=en)
//!  - [CPYPWT - CPYMWT - CPYEWT - Memory Copy - writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWT--CPYMWT--CPYEWT--Memory-Copy--writes-unprivileged-?lang=en)
//!  - [CPYPRT - CPYMRT - CPYERT - Memory Copy - reads unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRT--CPYMRT--CPYERT--Memory-Copy--reads-unprivileged-?lang=en)
//!  - [CPYPT - CPYMT - CPYET - Memory Copy - reads and writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPT--CPYMT--CPYET--Memory-Copy--reads-and-writes-unprivileged-?lang=en)
//!  - [CPYPWN - CPYMWN - CPYEWN - Memory Copy - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWN--CPYMWN--CPYEWN--Memory-Copy--writes-non-temporal-?lang=en)
//!  - [CPYPWTWN - CPYMWTWN - CPYEWTWN - Memory Copy - writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTWN--CPYMWTWN--CPYEWTWN--Memory-Copy--writes-unprivileged-and-non-temporal-?lang=en)
//!  - [CPYPRTWN - CPYMRTWN - CPYERTWN - Memory Copy - reads unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTWN--CPYMRTWN--CPYERTWN--Memory-Copy--reads-unprivileged--writes-non-temporal-?lang=en)
//!  - [CPYPTWN - CPYMTWN - CPYETWN - Memory Copy - reads and writes unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTWN--CPYMTWN--CPYETWN--Memory-Copy--reads-and-writes-unprivileged--writes-non-temporal-?lang=en)
//!  - [CPYPRN - CPYMRN - CPYERN - Memory Copy - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRN--CPYMRN--CPYERN--Memory-Copy--reads-non-temporal-?lang=en)
//!  - [CPYPWTRN - CPYMWTRN - CPYEWTRN - Memory Copy - writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTRN--CPYMWTRN--CPYEWTRN--Memory-Copy--writes-unprivileged--reads-non-temporal-?lang=en)
//!  - [CPYPRTRN - CPYMRTRN - CPYERTRN - Memory Copy - reads unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTRN--CPYMRTRN--CPYERTRN--Memory-Copy--reads-unprivileged-and-non-temporal-?lang=en)
//!  - [CPYPTRN - CPYMTRN - CPYETRN - Memory Copy - reads and writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTRN--CPYMTRN--CPYETRN--Memory-Copy--reads-and-writes-unprivileged--reads-non-temporal-?lang=en)
//!  - [CPYPN - CPYMN - CPYEN - Memory Copy - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPN--CPYMN--CPYEN--Memory-Copy--reads-and-writes-non-temporal-?lang=en)
//!  - [CPYPWTN - CPYMWTN - CPYEWTN - Memory Copy - writes unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTN--CPYMWTN--CPYEWTN--Memory-Copy--writes-unprivileged--reads-and-writes-non-temporal-?lang=en)
//!  - [CPYPRTN - CPYMRTN - CPYERTN - Memory Copy - reads unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTN--CPYMRTN--CPYERTN--Memory-Copy--reads-unprivileged--reads-and-writes-non-temporal-?lang=en)
//!  - [CPYPTN - CPYMTN - CPYETN - Memory Copy - reads and writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTN--CPYMTN--CPYETN--Memory-Copy--reads-and-writes-unprivileged-and-non-temporal-?lang=en)
//!  - [SETGP - SETGM - SETGE - Memory Set with tag setting](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGP--SETGM--SETGE--Memory-Set-with-tag-setting-?lang=en)
//!  - [SETGPT - SETGMT - SETGET - Memory Set with tag setting - unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPT--SETGMT--SETGET--Memory-Set-with-tag-setting--unprivileged-?lang=en)
//!  - [SETGPN - SETGMN - SETGEN - Memory Set with tag setting - non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPN--SETGMN--SETGEN--Memory-Set-with-tag-setting--non-temporal-?lang=en)
//!  - [SETGPTN - SETGMTN - SETGETN - Memory Set with tag setting - unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPTN--SETGMTN--SETGETN--Memory-Set-with-tag-setting--unprivileged-and-non-temporal-?lang=en)



use bit_seq::bseq_32;
use crate::instruction_encoding::InstructionProcessor;
use crate::types::Register;

#[inline(always)]
fn emit_mem_cpy_mem_set<P: InstructionProcessor<T>, T>(proc: &mut P, size: u8, o0: u8, op1: u8, rs: Register, op2: u8, rn: Register, rd: Register) -> T {
    let r = bseq_32!(size:2 011 o0:1 01 op1:2 0 rs:5 op2:4 01 rn:5 rd:5);
    proc.process(r)
}


/// # [Memory Copy and Memory Set](https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#memcms)
///
/// Implements the following instructions:
///  - [CPYFP - CPYFM - CPYFE - Memory Copy Forward only](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFP--CPYFM--CPYFE--Memory-Copy-Forward-only-?lang=en)
///  - [CPYFPWT - CPYFMWT - CPYFEWT - Memory Copy Forward only - writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWT--CPYFMWT--CPYFEWT--Memory-Copy-Forward-only--writes-unprivileged-?lang=en)
///  - [CPYFPRT - CPYFMRT - CPYFERT - Memory Copy Forward only - reads unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRT--CPYFMRT--CPYFERT--Memory-Copy-Forward-only--reads-unprivileged-?lang=en)
///  - [CPYFPT - CPYFMT - CPYFET - Memory Copy Forward only - reads and writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPT--CPYFMT--CPYFET--Memory-Copy-Forward-only--reads-and-writes-unprivileged-?lang=en)
///  - [CPYFPWN - CPYFMWN - CPYFEWN - Memory Copy Forward only - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWN--CPYFMWN--CPYFEWN--Memory-Copy-Forward-only--writes-non-temporal-?lang=en)
///  - [CPYFPWTWN - CPYFMWTWN - CPYFEWTWN - Memory Copy Forward only - writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTWN--CPYFMWTWN--CPYFEWTWN--Memory-Copy-Forward-only--writes-unprivileged-and-non-temporal-?lang=en)
///  - [CPYFPRTWN - CPYFMRTWN - CPYFERTWN - Memory Copy Forward only - reads unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTWN--CPYFMRTWN--CPYFERTWN--Memory-Copy-Forward-only--reads-unprivileged--writes-non-temporal-?lang=en)
///  - [CPYFPTWN - CPYFMTWN - CPYFETWN - Memory Copy Forward only - reads and writes unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTWN--CPYFMTWN--CPYFETWN--Memory-Copy-Forward-only--reads-and-writes-unprivileged--writes-non-temporal-?lang=en)
///  - [CPYFPRN - CPYFMRN - CPYFERN - Memory Copy Forward only - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRN--CPYFMRN--CPYFERN--Memory-Copy-Forward-only--reads-non-temporal-?lang=en)
///  - [CPYFPWTRN - CPYFMWTRN - CPYFEWTRN - Memory Copy Forward only - writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTRN--CPYFMWTRN--CPYFEWTRN--Memory-Copy-Forward-only--writes-unprivileged--reads-non-temporal-?lang=en)
///  - [CPYFPRTRN - CPYFMRTRN - CPYFERTRN - Memory Copy Forward only - reads unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTRN--CPYFMRTRN--CPYFERTRN--Memory-Copy-Forward-only--reads-unprivileged-and-non-temporal-?lang=en)
///  - [CPYFPTRN - CPYFMTRN - CPYFETRN - Memory Copy Forward only - reads and writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTRN--CPYFMTRN--CPYFETRN--Memory-Copy-Forward-only--reads-and-writes-unprivileged--reads-non-temporal-?lang=en)
///  - [CPYFPN - CPYFMN - CPYFEN - Memory Copy Forward only - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPN--CPYFMN--CPYFEN--Memory-Copy-Forward-only--reads-and-writes-non-temporal-?lang=en)
///  - [CPYFPWTN - CPYFMWTN - CPYFEWTN - Memory Copy Forward only - writes unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTN--CPYFMWTN--CPYFEWTN--Memory-Copy-Forward-only--writes-unprivileged--reads-and-writes-non-temporal-?lang=en)
///  - [CPYFPRTN - CPYFMRTN - CPYFERTN - Memory Copy Forward only - reads unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTN--CPYFMRTN--CPYFERTN--Memory-Copy-Forward-only--reads-unprivileged--reads-and-writes-non-temporal-?lang=en)
///  - [CPYFPTN - CPYFMTN - CPYFETN - Memory Copy Forward only - reads and writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTN--CPYFMTN--CPYFETN--Memory-Copy-Forward-only--reads-and-writes-unprivileged-and-non-temporal-?lang=en)
///  - [SETP - SETM - SETE - Memory Set](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETP--SETM--SETE--Memory-Set-?lang=en)
///  - [SETPT - SETMT - SETET - Memory Set - unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPT--SETMT--SETET--Memory-Set--unprivileged-?lang=en)
///  - [SETPN - SETMN - SETEN - Memory Set - non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPN--SETMN--SETEN--Memory-Set--non-temporal-?lang=en)
///  - [SETPTN - SETMTN - SETETN - Memory Set - unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPTN--SETMTN--SETETN--Memory-Set--unprivileged-and-non-temporal-?lang=en)
///  - [CPYP - CPYM - CPYE - Memory Copy](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYP--CPYM--CPYE--Memory-Copy-?lang=en)
///  - [CPYPWT - CPYMWT - CPYEWT - Memory Copy - writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWT--CPYMWT--CPYEWT--Memory-Copy--writes-unprivileged-?lang=en)
///  - [CPYPRT - CPYMRT - CPYERT - Memory Copy - reads unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRT--CPYMRT--CPYERT--Memory-Copy--reads-unprivileged-?lang=en)
///  - [CPYPT - CPYMT - CPYET - Memory Copy - reads and writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPT--CPYMT--CPYET--Memory-Copy--reads-and-writes-unprivileged-?lang=en)
///  - [CPYPWN - CPYMWN - CPYEWN - Memory Copy - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWN--CPYMWN--CPYEWN--Memory-Copy--writes-non-temporal-?lang=en)
///  - [CPYPWTWN - CPYMWTWN - CPYEWTWN - Memory Copy - writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTWN--CPYMWTWN--CPYEWTWN--Memory-Copy--writes-unprivileged-and-non-temporal-?lang=en)
///  - [CPYPRTWN - CPYMRTWN - CPYERTWN - Memory Copy - reads unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTWN--CPYMRTWN--CPYERTWN--Memory-Copy--reads-unprivileged--writes-non-temporal-?lang=en)
///  - [CPYPTWN - CPYMTWN - CPYETWN - Memory Copy - reads and writes unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTWN--CPYMTWN--CPYETWN--Memory-Copy--reads-and-writes-unprivileged--writes-non-temporal-?lang=en)
///  - [CPYPRN - CPYMRN - CPYERN - Memory Copy - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRN--CPYMRN--CPYERN--Memory-Copy--reads-non-temporal-?lang=en)
///  - [CPYPWTRN - CPYMWTRN - CPYEWTRN - Memory Copy - writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTRN--CPYMWTRN--CPYEWTRN--Memory-Copy--writes-unprivileged--reads-non-temporal-?lang=en)
///  - [CPYPRTRN - CPYMRTRN - CPYERTRN - Memory Copy - reads unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTRN--CPYMRTRN--CPYERTRN--Memory-Copy--reads-unprivileged-and-non-temporal-?lang=en)
///  - [CPYPTRN - CPYMTRN - CPYETRN - Memory Copy - reads and writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTRN--CPYMTRN--CPYETRN--Memory-Copy--reads-and-writes-unprivileged--reads-non-temporal-?lang=en)
///  - [CPYPN - CPYMN - CPYEN - Memory Copy - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPN--CPYMN--CPYEN--Memory-Copy--reads-and-writes-non-temporal-?lang=en)
///  - [CPYPWTN - CPYMWTN - CPYEWTN - Memory Copy - writes unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTN--CPYMWTN--CPYEWTN--Memory-Copy--writes-unprivileged--reads-and-writes-non-temporal-?lang=en)
///  - [CPYPRTN - CPYMRTN - CPYERTN - Memory Copy - reads unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTN--CPYMRTN--CPYERTN--Memory-Copy--reads-unprivileged--reads-and-writes-non-temporal-?lang=en)
///  - [CPYPTN - CPYMTN - CPYETN - Memory Copy - reads and writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTN--CPYMTN--CPYETN--Memory-Copy--reads-and-writes-unprivileged-and-non-temporal-?lang=en)
///  - [SETGP - SETGM - SETGE - Memory Set with tag setting](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGP--SETGM--SETGE--Memory-Set-with-tag-setting-?lang=en)
///  - [SETGPT - SETGMT - SETGET - Memory Set with tag setting - unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPT--SETGMT--SETGET--Memory-Set-with-tag-setting--unprivileged-?lang=en)
///  - [SETGPN - SETGMN - SETGEN - Memory Set with tag setting - non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPN--SETGMN--SETGEN--Memory-Set-with-tag-setting--non-temporal-?lang=en)
///  - [SETGPTN - SETGMTN - SETGETN - Memory Set with tag setting - unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPTN--SETGMTN--SETGETN--Memory-Set-with-tag-setting--unprivileged-and-non-temporal-?lang=en)
pub trait MemoryCopyAndMemorySet<T>: InstructionProcessor<T> {
    /// [CPYFP - CPYFM - CPYFE - Memory Copy Forward only](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFP--CPYFM--CPYFE--Memory-Copy-Forward-only-?lang=en)
    ///
    /// Memory Copy Forward-only. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFP, then CPYFM, and then CPYFE.
    ///
    /// CPYFP performs some preconditioning of the arguments suitable for using the CPYFM instruction, and performs an implementation defined amount of the memory copy. CPYFM performs an implementation defined amount of the memory copy. CPYFE performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFE [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfe(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0, 0, 0b10, xs, 0, xn, xd)
    }


    /// [CPYFP - CPYFM - CPYFE - Memory Copy Forward only](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFP--CPYFM--CPYFE--Memory-Copy-Forward-only-?lang=en)
    ///
    /// Memory Copy Forward-only. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFP, then CPYFM, and then CPYFE.
    ///
    /// CPYFP performs some preconditioning of the arguments suitable for using the CPYFM instruction, and performs an implementation defined amount of the memory copy. CPYFM performs an implementation defined amount of the memory copy. CPYFE performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFM [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfm(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b01, xs, 0, xn, xd)
    }


    /// [CPYFP - CPYFM - CPYFE - Memory Copy Forward only](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFP--CPYFM--CPYFE--Memory-Copy-Forward-only-?lang=en)
    ///
    /// Memory Copy Forward-only. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFP, then CPYFM, and then CPYFE.
    ///
    /// CPYFP performs some preconditioning of the arguments suitable for using the CPYFM instruction, and performs an implementation defined amount of the memory copy. CPYFM performs an implementation defined amount of the memory copy. CPYFE performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFP [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfp(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b00, xs, 0, xn, xd)
    }


    /// [CPYFPWT - CPYFMWT - CPYFEWT - Memory Copy Forward only - writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWT--CPYFMWT--CPYFEWT--Memory-Copy-Forward-only--writes-unprivileged-?lang=en)
    ///
    /// Memory Copy Forward-only, writes unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWT, then CPYFMWT, and then CPYFEWT.
    ///
    /// CPYFPWT performs some preconditioning of the arguments suitable for using the CPYFMWT instruction, and performs an implementation defined amount of the memory copy. CPYFMWT performs an implementation defined amount of the memory copy. CPYFEWT performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFEWT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfewt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b0001, xn, xd)
    }


    /// [CPYFPWT - CPYFMWT - CPYFEWT - Memory Copy Forward only - writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWT--CPYFMWT--CPYFEWT--Memory-Copy-Forward-only--writes-unprivileged-?lang=en)
    ///
    /// Memory Copy Forward-only, writes unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWT, then CPYFMWT, and then CPYFEWT.
    ///
    /// CPYFPWT performs some preconditioning of the arguments suitable for using the CPYFMWT instruction, and performs an implementation defined amount of the memory copy. CPYFMWT performs an implementation defined amount of the memory copy. CPYFEWT performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMWT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmwt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b01, xs, 0b0001, xn, xd)
    }


    /// [CPYFPWT - CPYFMWT - CPYFEWT - Memory Copy Forward only - writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWT--CPYFMWT--CPYFEWT--Memory-Copy-Forward-only--writes-unprivileged-?lang=en)
    ///
    /// Memory Copy Forward-only, writes unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWT, then CPYFMWT, and then CPYFEWT.
    ///
    /// CPYFPWT performs some preconditioning of the arguments suitable for using the CPYFMWT instruction, and performs an implementation defined amount of the memory copy. CPYFMWT performs an implementation defined amount of the memory copy. CPYFEWT performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPWT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfpwt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b00, xs, 0b0001, xn, xd)
    }


    /// [CPYFPRT - CPYFMRT - CPYFERT - Memory Copy Forward only - reads unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRT--CPYFMRT--CPYFERT--Memory-Copy-Forward-only--reads-unprivileged-?lang=en)
    ///
    /// Memory Copy Forward-only, reads unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRT, then CPYFMRT, and then CPYFERT.
    ///
    /// CPYFPRT performs some preconditioning of the arguments suitable for using the CPYFMRT instruction, and performs an implementation defined amount of the memory copy. CPYFMRT performs an implementation defined amount of the memory copy. CPYFERT performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFERT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfert(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b0010, xn, xd)
    }


    /// [CPYFPRT - CPYFMRT - CPYFERT - Memory Copy Forward only - reads unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRT--CPYFMRT--CPYFERT--Memory-Copy-Forward-only--reads-unprivileged-?lang=en)
    ///
    /// Memory Copy Forward-only, reads unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRT, then CPYFMRT, and then CPYFERT.
    ///
    /// CPYFPRT performs some preconditioning of the arguments suitable for using the CPYFMRT instruction, and performs an implementation defined amount of the memory copy. CPYFMRT performs an implementation defined amount of the memory copy. CPYFERT performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMRT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmrt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b01, xs, 0b0010, xn, xd)
    }


    /// [CPYFPRT - CPYFMRT - CPYFERT - Memory Copy Forward only - reads unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRT--CPYFMRT--CPYFERT--Memory-Copy-Forward-only--reads-unprivileged-?lang=en)
    ///
    /// Memory Copy Forward-only, reads unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRT, then CPYFMRT, and then CPYFERT.
    ///
    /// CPYFPRT performs some preconditioning of the arguments suitable for using the CPYFMRT instruction, and performs an implementation defined amount of the memory copy. CPYFMRT performs an implementation defined amount of the memory copy. CPYFERT performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPRT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfprt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b00, xs, 0b0010, xn, xd)
    }


    /// [CPYFPT - CPYFMT - CPYFET - Memory Copy Forward only - reads and writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPT--CPYFMT--CPYFET--Memory-Copy-Forward-only--reads-and-writes-unprivileged-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPT, then CPYFMT, and then CPYFET.
    ///
    /// CPYFPT performs some preconditioning of the arguments suitable for using the CPYFMT instruction, and performs an implementation defined amount of the memory copy. CPYFMT performs an implementation defined amount of the memory copy. CPYFET performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFET [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfet(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b0011, xn, xd)
    }


    /// [CPYFPT - CPYFMT - CPYFET - Memory Copy Forward only - reads and writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPT--CPYFMT--CPYFET--Memory-Copy-Forward-only--reads-and-writes-unprivileged-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPT, then CPYFMT, and then CPYFET.
    ///
    /// CPYFPT performs some preconditioning of the arguments suitable for using the CPYFMT instruction, and performs an implementation defined amount of the memory copy. CPYFMT performs an implementation defined amount of the memory copy. CPYFET performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b01, xs, 0b0011, xn, xd)
    }


    /// [CPYFPT - CPYFMT - CPYFET - Memory Copy Forward only - reads and writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPT--CPYFMT--CPYFET--Memory-Copy-Forward-only--reads-and-writes-unprivileged-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPT, then CPYFMT, and then CPYFET.
    ///
    /// CPYFPT performs some preconditioning of the arguments suitable for using the CPYFMT instruction, and performs an implementation defined amount of the memory copy. CPYFMT performs an implementation defined amount of the memory copy. CPYFET performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfpt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b00, xs, 0b0011, xn, xd)
    }


    /// [CPYFPWN - CPYFMWN - CPYFEWN - Memory Copy Forward only - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWN--CPYFMWN--CPYFEWN--Memory-Copy-Forward-only--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWN, then CPYFMWN, and then CPYFEWN.
    ///
    /// CPYFPWN performs some preconditioning of the arguments suitable for using the CPYFMWN instruction, and performs an implementation defined amount of the memory copy. CPYFMWN performs an implementation defined amount of the memory copy. CPYFEWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFEWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfewn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b0100, xn, xd)
    }


    /// [CPYFPWN - CPYFMWN - CPYFEWN - Memory Copy Forward only - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWN--CPYFMWN--CPYFEWN--Memory-Copy-Forward-only--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWN, then CPYFMWN, and then CPYFEWN.
    ///
    /// CPYFPWN performs some preconditioning of the arguments suitable for using the CPYFMWN instruction, and performs an implementation defined amount of the memory copy. CPYFMWN performs an implementation defined amount of the memory copy. CPYFEWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b01, xs, 0b0100, xn, xd)
    }


    /// [CPYFPWN - CPYFMWN - CPYFEWN - Memory Copy Forward only - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWN--CPYFMWN--CPYFEWN--Memory-Copy-Forward-only--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWN, then CPYFMWN, and then CPYFEWN.
    ///
    /// CPYFPWN performs some preconditioning of the arguments suitable for using the CPYFMWN instruction, and performs an implementation defined amount of the memory copy. CPYFMWN performs an implementation defined amount of the memory copy. CPYFEWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfpwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b00, xs, 0b0100, xn, xd)
    }


    /// [CPYFPWTWN - CPYFMWTWN - CPYFEWTWN - Memory Copy Forward only - writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTWN--CPYFMWTWN--CPYFEWTWN--Memory-Copy-Forward-only--writes-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, writes unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWTWN, then CPYFMWTWN, and then CPYFEWTWN.
    ///
    /// CPYFPWTWN performs some preconditioning of the arguments suitable for using the CPYFMWTWN instruction, and performs an implementation defined amount of the memory copy. CPYFMWTWN performs an implementation defined amount of the memory copy. CPYFEWTWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFEWTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfewtwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b0101, xn, xd)
    }


    /// [CPYFPWTWN - CPYFMWTWN - CPYFEWTWN - Memory Copy Forward only - writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTWN--CPYFMWTWN--CPYFEWTWN--Memory-Copy-Forward-only--writes-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, writes unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWTWN, then CPYFMWTWN, and then CPYFEWTWN.
    ///
    /// CPYFPWTWN performs some preconditioning of the arguments suitable for using the CPYFMWTWN instruction, and performs an implementation defined amount of the memory copy. CPYFMWTWN performs an implementation defined amount of the memory copy. CPYFEWTWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMWTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmwtwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b01, xs, 0b0101, xn, xd)
    }


    /// [CPYFPWTWN - CPYFMWTWN - CPYFEWTWN - Memory Copy Forward only - writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTWN--CPYFMWTWN--CPYFEWTWN--Memory-Copy-Forward-only--writes-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, writes unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWTWN, then CPYFMWTWN, and then CPYFEWTWN.
    ///
    /// CPYFPWTWN performs some preconditioning of the arguments suitable for using the CPYFMWTWN instruction, and performs an implementation defined amount of the memory copy. CPYFMWTWN performs an implementation defined amount of the memory copy. CPYFEWTWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPWTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfpwtwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b00, xs, 0b0101, xn, xd)
    }


    /// [CPYFPRTWN - CPYFMRTWN - CPYFERTWN - Memory Copy Forward only - reads unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTWN--CPYFMRTWN--CPYFERTWN--Memory-Copy-Forward-only--reads-unprivileged--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads unprivileged, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRTWN, then CPYFMRTWN, and then CPYFERTWN.
    ///
    /// CPYFPRTWN performs some preconditioning of the arguments suitable for using the CPYFMRTWN instruction, and performs an implementation defined amount of the memory copy. CPYFMRTWN performs an implementation defined amount of the memory copy. CPYFERTWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFERTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfertwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b0110, xn, xd)
    }


    /// [CPYFPRTWN - CPYFMRTWN - CPYFERTWN - Memory Copy Forward only - reads unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTWN--CPYFMRTWN--CPYFERTWN--Memory-Copy-Forward-only--reads-unprivileged--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads unprivileged, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRTWN, then CPYFMRTWN, and then CPYFERTWN.
    ///
    /// CPYFPRTWN performs some preconditioning of the arguments suitable for using the CPYFMRTWN instruction, and performs an implementation defined amount of the memory copy. CPYFMRTWN performs an implementation defined amount of the memory copy. CPYFERTWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMRTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmrtwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b01, xs, 0b0110, xn, xd)
    }


    /// [CPYFPRTWN - CPYFMRTWN - CPYFERTWN - Memory Copy Forward only - reads unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTWN--CPYFMRTWN--CPYFERTWN--Memory-Copy-Forward-only--reads-unprivileged--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads unprivileged, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRTWN, then CPYFMRTWN, and then CPYFERTWN.
    ///
    /// CPYFPRTWN performs some preconditioning of the arguments suitable for using the CPYFMRTWN instruction, and performs an implementation defined amount of the memory copy. CPYFMRTWN performs an implementation defined amount of the memory copy. CPYFERTWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPRTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfprtwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b00, xs, 0b0110, xn, xd)
    }


    /// [CPYFPTWN - CPYFMTWN - CPYFETWN - Memory Copy Forward only - reads and writes unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTWN--CPYFMTWN--CPYFETWN--Memory-Copy-Forward-only--reads-and-writes-unprivileged--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes unprivileged, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPTWN, then CPYFMTWN, and then CPYFETWN.
    ///
    /// CPYFPTWN performs some preconditioning of the arguments suitable for using the CPYFMTWN instruction, and performs an implementation defined amount of the memory copy. CPYFMTWN performs an implementation defined amount of the memory copy. CPYFETWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFETWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfetwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b0111, xn, xd)
    }


    /// [CPYFPTWN - CPYFMTWN - CPYFETWN - Memory Copy Forward only - reads and writes unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTWN--CPYFMTWN--CPYFETWN--Memory-Copy-Forward-only--reads-and-writes-unprivileged--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes unprivileged, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPTWN, then CPYFMTWN, and then CPYFETWN.
    ///
    /// CPYFPTWN performs some preconditioning of the arguments suitable for using the CPYFMTWN instruction, and performs an implementation defined amount of the memory copy. CPYFMTWN performs an implementation defined amount of the memory copy. CPYFETWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmtwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b01, xs, 0b0111, xn, xd)
    }


    /// [CPYFPTWN - CPYFMTWN - CPYFETWN - Memory Copy Forward only - reads and writes unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTWN--CPYFMTWN--CPYFETWN--Memory-Copy-Forward-only--reads-and-writes-unprivileged--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes unprivileged, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPTWN, then CPYFMTWN, and then CPYFETWN.
    ///
    /// CPYFPTWN performs some preconditioning of the arguments suitable for using the CPYFMTWN instruction, and performs an implementation defined amount of the memory copy. CPYFMTWN performs an implementation defined amount of the memory copy. CPYFETWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfptwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b0, xs, 0b0111, xn, xd)
    }


    /// [CPYFPRN - CPYFMRN - CPYFERN - Memory Copy Forward only - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRN--CPYFMRN--CPYFERN--Memory-Copy-Forward-only--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRN, then CPYFMRN, and then CPYFERN.
    ///
    /// CPYFPRN performs some preconditioning of the arguments suitable for using the CPYFMRN instruction, and performs an implementation defined amount of the memory copy. CPYFMRN performs an implementation defined amount of the memory copy. CPYFERN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFERN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfern(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b1000, xn, xd)
    }


    /// [CPYFPRN - CPYFMRN - CPYFERN - Memory Copy Forward only - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRN--CPYFMRN--CPYFERN--Memory-Copy-Forward-only--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRN, then CPYFMRN, and then CPYFERN.
    ///
    /// CPYFPRN performs some preconditioning of the arguments suitable for using the CPYFMRN instruction, and performs an implementation defined amount of the memory copy. CPYFMRN performs an implementation defined amount of the memory copy. CPYFERN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b01, xs, 0b1000, xn, xd)
    }


    /// [CPYFPRN - CPYFMRN - CPYFERN - Memory Copy Forward only - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRN--CPYFMRN--CPYFERN--Memory-Copy-Forward-only--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRN, then CPYFMRN, and then CPYFERN.
    ///
    /// CPYFPRN performs some preconditioning of the arguments suitable for using the CPYFMRN instruction, and performs an implementation defined amount of the memory copy. CPYFMRN performs an implementation defined amount of the memory copy. CPYFERN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfprn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b0, xs, 0b1000, xn, xd)
    }


    /// [CPYFPWTRN - CPYFMWTRN - CPYFEWTRN - Memory Copy Forward only - writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTRN--CPYFMWTRN--CPYFEWTRN--Memory-Copy-Forward-only--writes-unprivileged--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, writes unprivileged, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWTRN, then CPYFMWTRN, and then CPYFEWTRN.
    ///
    /// CPYFPWTRN performs some preconditioning of the arguments suitable for using the CPYFMWTRN instruction, and performs an implementation defined amount of the memory copy. CPYFMWTRN performs an implementation defined amount of the memory copy. CPYFEWTRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFEWTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfewtrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b1001, xn, xd)
    }


    /// [CPYFPWTRN - CPYFMWTRN - CPYFEWTRN - Memory Copy Forward only - writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTRN--CPYFMWTRN--CPYFEWTRN--Memory-Copy-Forward-only--writes-unprivileged--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, writes unprivileged, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWTRN, then CPYFMWTRN, and then CPYFEWTRN.
    ///
    /// CPYFPWTRN performs some preconditioning of the arguments suitable for using the CPYFMWTRN instruction, and performs an implementation defined amount of the memory copy. CPYFMWTRN performs an implementation defined amount of the memory copy. CPYFEWTRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMWTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmwtrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b01, xs, 0b1001, xn, xd)
    }


    /// [CPYFPWTRN - CPYFMWTRN - CPYFEWTRN - Memory Copy Forward only - writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTRN--CPYFMWTRN--CPYFEWTRN--Memory-Copy-Forward-only--writes-unprivileged--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, writes unprivileged, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWTRN, then CPYFMWTRN, and then CPYFEWTRN.
    ///
    /// CPYFPWTRN performs some preconditioning of the arguments suitable for using the CPYFMWTRN instruction, and performs an implementation defined amount of the memory copy. CPYFMWTRN performs an implementation defined amount of the memory copy. CPYFEWTRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPWTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfpwtrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b0, xs, 0b1001, xn, xd)
    }


    /// [CPYFPRTRN - CPYFMRTRN - CPYFERTRN - Memory Copy Forward only - reads unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTRN--CPYFMRTRN--CPYFERTRN--Memory-Copy-Forward-only--reads-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRTRN, then CPYFMRTRN, and then CPYFERTRN.
    ///
    /// CPYFPRTRN performs some preconditioning of the arguments suitable for using the CPYFMRTRN instruction, and performs an implementation defined amount of the memory copy. CPYFMRTRN performs an implementation defined amount of the memory copy. CPYFERTRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFERTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfertrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b1010, xn, xd)
    }


    /// [CPYFPRTRN - CPYFMRTRN - CPYFERTRN - Memory Copy Forward only - reads unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTRN--CPYFMRTRN--CPYFERTRN--Memory-Copy-Forward-only--reads-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRTRN, then CPYFMRTRN, and then CPYFERTRN.
    ///
    /// CPYFPRTRN performs some preconditioning of the arguments suitable for using the CPYFMRTRN instruction, and performs an implementation defined amount of the memory copy. CPYFMRTRN performs an implementation defined amount of the memory copy. CPYFERTRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMRTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmrtrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b01, xs, 0b1010, xn, xd)
    }


    /// [CPYFPRTRN - CPYFMRTRN - CPYFERTRN - Memory Copy Forward only - reads unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTRN--CPYFMRTRN--CPYFERTRN--Memory-Copy-Forward-only--reads-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRTRN, then CPYFMRTRN, and then CPYFERTRN.
    ///
    /// CPYFPRTRN performs some preconditioning of the arguments suitable for using the CPYFMRTRN instruction, and performs an implementation defined amount of the memory copy. CPYFMRTRN performs an implementation defined amount of the memory copy. CPYFERTRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPRTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfprtrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b00, xs, 0b1010, xn, xd)
    }


    /// [CPYFPTRN - CPYFMTRN - CPYFETRN - Memory Copy Forward only - reads and writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTRN--CPYFMTRN--CPYFETRN--Memory-Copy-Forward-only--reads-and-writes-unprivileged--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes unprivileged, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPTRN, then CPYFMTRN, and then CPYFETRN.
    ///
    /// CPYFPTRN performs some preconditioning of the arguments suitable for using the CPYFMTRN instruction, and performs an implementation defined amount of the memory copy. CPYFMTRN performs an implementation defined amount of the memory copy. CPYFETRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFETRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfetrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b1011, xn, xd)
    }


    /// [CPYFPTRN - CPYFMTRN - CPYFETRN - Memory Copy Forward only - reads and writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTRN--CPYFMTRN--CPYFETRN--Memory-Copy-Forward-only--reads-and-writes-unprivileged--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes unprivileged, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPTRN, then CPYFMTRN, and then CPYFETRN.
    ///
    /// CPYFPTRN performs some preconditioning of the arguments suitable for using the CPYFMTRN instruction, and performs an implementation defined amount of the memory copy. CPYFMTRN performs an implementation defined amount of the memory copy. CPYFETRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmtrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b01, xs, 0b1011, xn, xd)
    }


    /// [CPYFPTRN - CPYFMTRN - CPYFETRN - Memory Copy Forward only - reads and writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTRN--CPYFMTRN--CPYFETRN--Memory-Copy-Forward-only--reads-and-writes-unprivileged--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes unprivileged, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPTRN, then CPYFMTRN, and then CPYFETRN.
    ///
    /// CPYFPTRN performs some preconditioning of the arguments suitable for using the CPYFMTRN instruction, and performs an implementation defined amount of the memory copy. CPYFMTRN performs an implementation defined amount of the memory copy. CPYFETRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfptrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b0, xs, 0b1011, xn, xd)
    }


    /// [CPYFPN - CPYFMN - CPYFEN - Memory Copy Forward only - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPN--CPYFMN--CPYFEN--Memory-Copy-Forward-only--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPN, then CPYFMN, and then CPYFEN.
    ///
    /// CPYFPN performs some preconditioning of the arguments suitable for using the CPYFMN instruction, and performs an implementation defined amount of the memory copy. CPYFMN performs an implementation defined amount of the memory copy. CPYFEN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFEN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfen(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b1100, xn, xd)
    }


    /// [CPYFPN - CPYFMN - CPYFEN - Memory Copy Forward only - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPN--CPYFMN--CPYFEN--Memory-Copy-Forward-only--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPN, then CPYFMN, and then CPYFEN.
    ///
    /// CPYFPN performs some preconditioning of the arguments suitable for using the CPYFMN instruction, and performs an implementation defined amount of the memory copy. CPYFMN performs an implementation defined amount of the memory copy. CPYFEN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b1, xs, 0b1100, xn, xd)
    }


    /// [CPYFPN - CPYFMN - CPYFEN - Memory Copy Forward only - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPN--CPYFMN--CPYFEN--Memory-Copy-Forward-only--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPN, then CPYFMN, and then CPYFEN.
    ///
    /// CPYFPN performs some preconditioning of the arguments suitable for using the CPYFMN instruction, and performs an implementation defined amount of the memory copy. CPYFMN performs an implementation defined amount of the memory copy. CPYFEN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfpn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b0, xs, 0b1100, xn, xd)
    }


    /// [CPYFPWTN - CPYFMWTN - CPYFEWTN - Memory Copy Forward only - writes unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTN--CPYFMWTN--CPYFEWTN--Memory-Copy-Forward-only--writes-unprivileged--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, writes unprivileged, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWTN, then CPYFMWTN, and then CPYFEWTN.
    ///
    /// CPYFPWTN performs some preconditioning of the arguments suitable for using the CPYFMWTN instruction, and performs an implementation defined amount of the memory copy. CPYFMWTN performs an implementation defined amount of the memory copy. CPYFEWTN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFEWTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfewtn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b1101, xn, xd)
    }


    /// [CPYFPWTN - CPYFMWTN - CPYFEWTN - Memory Copy Forward only - writes unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTN--CPYFMWTN--CPYFEWTN--Memory-Copy-Forward-only--writes-unprivileged--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, writes unprivileged, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWTN, then CPYFMWTN, and then CPYFEWTN.
    ///
    /// CPYFPWTN performs some preconditioning of the arguments suitable for using the CPYFMWTN instruction, and performs an implementation defined amount of the memory copy. CPYFMWTN performs an implementation defined amount of the memory copy. CPYFEWTN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMWTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmwtn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b1, xs, 0b1101, xn, xd)
    }


    /// [CPYFPWTN - CPYFMWTN - CPYFEWTN - Memory Copy Forward only - writes unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPWTN--CPYFMWTN--CPYFEWTN--Memory-Copy-Forward-only--writes-unprivileged--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, writes unprivileged, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPWTN, then CPYFMWTN, and then CPYFEWTN.
    ///
    /// CPYFPWTN performs some preconditioning of the arguments suitable for using the CPYFMWTN instruction, and performs an implementation defined amount of the memory copy. CPYFMWTN performs an implementation defined amount of the memory copy. CPYFEWTN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPWTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfpwtn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b1, xs, 0b1101, xn, xd)
    }


    /// [CPYFPRTN - CPYFMRTN - CPYFERTN - Memory Copy Forward only - reads unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTN--CPYFMRTN--CPYFERTN--Memory-Copy-Forward-only--reads-unprivileged--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads unprivileged, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRTN, then CPYFMRTN, and then CPYFERTN.
    ///
    /// CPYFPRTN performs some preconditioning of the arguments suitable for using the CPYFMRTN instruction, and performs an implementation defined amount of the memory copy. CPYFMRTN performs an implementation defined amount of the memory copy. CPYFERTN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFERTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfertn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b1110, xn, xd)
    }


    /// [CPYFPRTN - CPYFMRTN - CPYFERTN - Memory Copy Forward only - reads unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTN--CPYFMRTN--CPYFERTN--Memory-Copy-Forward-only--reads-unprivileged--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads unprivileged, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRTN, then CPYFMRTN, and then CPYFERTN.
    ///
    /// CPYFPRTN performs some preconditioning of the arguments suitable for using the CPYFMRTN instruction, and performs an implementation defined amount of the memory copy. CPYFMRTN performs an implementation defined amount of the memory copy. CPYFERTN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMRTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmrtn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b1, xs, 0b1110, xn, xd)
    }


    /// [CPYFPRTN - CPYFMRTN - CPYFERTN - Memory Copy Forward only - reads unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPRTN--CPYFMRTN--CPYFERTN--Memory-Copy-Forward-only--reads-unprivileged--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads unprivileged, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPRTN, then CPYFMRTN, and then CPYFERTN.
    ///
    /// CPYFPRTN performs some preconditioning of the arguments suitable for using the CPYFMRTN instruction, and performs an implementation defined amount of the memory copy. CPYFMRTN performs an implementation defined amount of the memory copy. CPYFERTN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPRTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfprtn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b0, xs, 0b1110, xn, xd)
    }


    /// [CPYFPTN - CPYFMTN - CPYFETN - Memory Copy Forward only - reads and writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTN--CPYFMTN--CPYFETN--Memory-Copy-Forward-only--reads-and-writes-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPTN, then CPYFMTN, and then CPYFETN.
    ///
    /// CPYFPTN performs some preconditioning of the arguments suitable for using the CPYFMTN instruction, and performs an implementation defined amount of the memory copy. CPYFMTN performs an implementation defined amount of the memory copy. CPYFETN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFETN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfetn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b10, xs, 0b1111, xn, xd)
    }


    /// [CPYFPTN - CPYFMTN - CPYFETN - Memory Copy Forward only - reads and writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTN--CPYFMTN--CPYFETN--Memory-Copy-Forward-only--reads-and-writes-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPTN, then CPYFMTN, and then CPYFETN.
    ///
    /// CPYFPTN performs some preconditioning of the arguments suitable for using the CPYFMTN instruction, and performs an implementation defined amount of the memory copy. CPYFMTN performs an implementation defined amount of the memory copy. CPYFETN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFMTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfmtn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b1, xs, 0b1111, xn, xd)
    }


    /// [CPYFPTN - CPYFMTN - CPYFETN - Memory Copy Forward only - reads and writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYFPTN--CPYFMTN--CPYFETN--Memory-Copy-Forward-only--reads-and-writes-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy Forward-only, reads and writes unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYFPTN, then CPYFMTN, and then CPYFETN.
    ///
    /// CPYFPTN performs some preconditioning of the arguments suitable for using the CPYFMTN instruction, and performs an implementation defined amount of the memory copy. CPYFMTN performs an implementation defined amount of the memory copy. CPYFETN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYFPTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyfptn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b0, xs, 0b1111, xn, xd)
    }


    /// [SETP - SETM - SETE - Memory Set](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETP--SETM--SETE--Memory-Set-?lang=en)
    ///
    /// Memory Set. These instructions perform a memory set using the value in the bottom byte of the source register. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETP, then SETM, and then SETE.
    ///
    /// SETP performs some preconditioning of the arguments suitable for using the SETM instruction, and performs an implementation defined amount of the memory set. SETM performs an implementation defined amount of the memory set. SETE performs the last part of the memory set.
    ///
    /// ```asm
    /// SETE [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn sete(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b11, xs, 0b1000, xn, xd)
    }


    /// [SETP - SETM - SETE - Memory Set](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETP--SETM--SETE--Memory-Set-?lang=en)
    ///
    /// Memory Set. These instructions perform a memory set using the value in the bottom byte of the source register. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETP, then SETM, and then SETE.
    ///
    /// SETP performs some preconditioning of the arguments suitable for using the SETM instruction, and performs an implementation defined amount of the memory set. SETM performs an implementation defined amount of the memory set. SETE performs the last part of the memory set.
    ///
    /// ```asm
    /// SETM [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setm(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b11, xs, 0b0100, xn, xd)
    }


    /// [SETP - SETM - SETE - Memory Set](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETP--SETM--SETE--Memory-Set-?lang=en)
    ///
    /// Memory Set. These instructions perform a memory set using the value in the bottom byte of the source register. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETP, then SETM, and then SETE.
    ///
    /// SETP performs some preconditioning of the arguments suitable for using the SETM instruction, and performs an implementation defined amount of the memory set. SETM performs an implementation defined amount of the memory set. SETE performs the last part of the memory set.
    ///
    /// ```asm
    /// SETP [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setp(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b11, xs, 0b0000, xn, xd)
    }


    /// [SETPT - SETMT - SETET - Memory Set - unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPT--SETMT--SETET--Memory-Set--unprivileged-?lang=en)
    ///
    /// Memory Set, unprivileged. These instructions perform a memory set using the value in the bottom byte of the source register. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETPT, then SETMT, and then SETET.
    ///
    /// SETPT performs some preconditioning of the arguments suitable for using the SETMT instruction, and performs an implementation defined amount of the memory set. SETMT performs an implementation defined amount of the memory set. SETET performs the last part of the memory set.
    ///
    /// ```asm
    /// SETET [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setet(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b11, xs, 0b1001, xn, xd)
    }


    /// [SETPT - SETMT - SETET - Memory Set - unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPT--SETMT--SETET--Memory-Set--unprivileged-?lang=en)
    ///
    /// Memory Set, unprivileged. These instructions perform a memory set using the value in the bottom byte of the source register. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETPT, then SETMT, and then SETET.
    ///
    /// SETPT performs some preconditioning of the arguments suitable for using the SETMT instruction, and performs an implementation defined amount of the memory set. SETMT performs an implementation defined amount of the memory set. SETET performs the last part of the memory set.
    ///
    /// ```asm
    /// SETMT [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setmt(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b11, xs, 0b0101, xn, xd)
    }


    /// [SETPT - SETMT - SETET - Memory Set - unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPT--SETMT--SETET--Memory-Set--unprivileged-?lang=en)
    ///
    /// Memory Set, unprivileged. These instructions perform a memory set using the value in the bottom byte of the source register. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETPT, then SETMT, and then SETET.
    ///
    /// SETPT performs some preconditioning of the arguments suitable for using the SETMT instruction, and performs an implementation defined amount of the memory set. SETMT performs an implementation defined amount of the memory set. SETET performs the last part of the memory set.
    ///
    /// ```asm
    /// SETPT [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setpt(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b11, xs, 0b0001, xn, xd)
    }


    /// [SETPN - SETMN - SETEN - Memory Set - non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPN--SETMN--SETEN--Memory-Set--non-temporal-?lang=en)
    ///
    /// Memory Set, non-temporal. These instructions perform a memory set using the value in the bottom byte of the source register. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETPN, then SETMN, and then SETEN.
    ///
    /// SETPN performs some preconditioning of the arguments suitable for using the SETMN instruction, and performs an implementation defined amount of the memory set. SETMN performs an implementation defined amount of the memory set. SETEN performs the last part of the memory set.
    ///
    /// ```asm
    /// SETEN [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn seten(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b11, xs, 0b1010, xn, xd)
    }


    /// [SETPN - SETMN - SETEN - Memory Set - non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPN--SETMN--SETEN--Memory-Set--non-temporal-?lang=en)
    ///
    /// Memory Set, non-temporal. These instructions perform a memory set using the value in the bottom byte of the source register. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETPN, then SETMN, and then SETEN.
    ///
    /// SETPN performs some preconditioning of the arguments suitable for using the SETMN instruction, and performs an implementation defined amount of the memory set. SETMN performs an implementation defined amount of the memory set. SETEN performs the last part of the memory set.
    ///
    /// ```asm
    /// SETMN [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setmn(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b11, xs, 0b0110, xn, xd)
    }


    /// [SETPN - SETMN - SETEN - Memory Set - non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPN--SETMN--SETEN--Memory-Set--non-temporal-?lang=en)
    ///
    /// Memory Set, non-temporal. These instructions perform a memory set using the value in the bottom byte of the source register. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETPN, then SETMN, and then SETEN.
    ///
    /// SETPN performs some preconditioning of the arguments suitable for using the SETMN instruction, and performs an implementation defined amount of the memory set. SETMN performs an implementation defined amount of the memory set. SETEN performs the last part of the memory set.
    ///
    /// ```asm
    /// SETPN [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setpn(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b11, xs, 0b0010, xn, xd)
    }


    /// [SETPTN - SETMTN - SETETN - Memory Set - unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPTN--SETMTN--SETETN--Memory-Set--unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Set, unprivileged and non-temporal. These instructions perform a memory set using the value in the bottom byte of the source register. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETPTN, then SETMTN, and then SETETN.
    ///
    /// SETPTN performs some preconditioning of the arguments suitable for using the SETMTN instruction, and performs an implementation defined amount of the memory set. SETMTN performs an implementation defined amount of the memory set. SETETN performs the last part of the memory set.
    ///
    /// ```asm
    /// SETETN [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setetn(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b11, xs, 0b1011, xn, xd)
    }


    /// [SETPTN - SETMTN - SETETN - Memory Set - unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPTN--SETMTN--SETETN--Memory-Set--unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Set, unprivileged and non-temporal. These instructions perform a memory set using the value in the bottom byte of the source register. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETPTN, then SETMTN, and then SETETN.
    ///
    /// SETPTN performs some preconditioning of the arguments suitable for using the SETMTN instruction, and performs an implementation defined amount of the memory set. SETMTN performs an implementation defined amount of the memory set. SETETN performs the last part of the memory set.
    ///
    /// ```asm
    /// SETMTN [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setmtn(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b11, xs, 0b0111, xn, xd)
    }


    /// [SETPTN - SETMTN - SETETN - Memory Set - unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETPTN--SETMTN--SETETN--Memory-Set--unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Set, unprivileged and non-temporal. These instructions perform a memory set using the value in the bottom byte of the source register. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETPTN, then SETMTN, and then SETETN.
    ///
    /// SETPTN performs some preconditioning of the arguments suitable for using the SETMTN instruction, and performs an implementation defined amount of the memory set. SETMTN performs an implementation defined amount of the memory set. SETETN performs the last part of the memory set.
    ///
    /// ```asm
    /// SETPTN [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setptn(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 0, 0b11, xs, 0b0011, xn, xd)
    }


    /// [CPYP - CPYM - CPYE - Memory Copy](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYP--CPYM--CPYE--Memory-Copy-?lang=en)
    ///
    /// Memory Copy. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYP, then CPYM, and then CPYE.
    ///
    /// CPYP performs some preconditioning of the arguments suitable for using the CPYM instruction, and performs an implementation defined amount of the memory copy. CPYM performs an implementation defined amount of the memory copy. CPYE performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYE [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpye(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b0000, xn, xd)
    }


    /// [CPYP - CPYM - CPYE - Memory Copy](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYP--CPYM--CPYE--Memory-Copy-?lang=en)
    ///
    /// Memory Copy. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYP, then CPYM, and then CPYE.
    ///
    /// CPYP performs some preconditioning of the arguments suitable for using the CPYM instruction, and performs an implementation defined amount of the memory copy. CPYM performs an implementation defined amount of the memory copy. CPYE performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYM [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpym(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b0000, xn, xd)
    }


    /// [CPYP - CPYM - CPYE - Memory Copy](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYP--CPYM--CPYE--Memory-Copy-?lang=en)
    ///
    /// Memory Copy. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYP, then CPYM, and then CPYE.
    ///
    /// CPYP performs some preconditioning of the arguments suitable for using the CPYM instruction, and performs an implementation defined amount of the memory copy. CPYM performs an implementation defined amount of the memory copy. CPYE performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYP [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyp(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b0000, xn, xd)
    }


    /// [CPYPWT - CPYMWT - CPYEWT - Memory Copy - writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWT--CPYMWT--CPYEWT--Memory-Copy--writes-unprivileged-?lang=en)
    ///
    /// Memory Copy, writes unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWT, then CPYMWT, and then CPYEWT.
    ///
    /// CPYPWT performs some preconditioning of the arguments suitable for using the CPYMWT instruction, and performs an implementation defined amount of the memory copy. CPYMWT performs an implementation defined amount of the memory copy. CPYEWT performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYEWT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyewt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b0001, xn, xd)
    }


    /// [CPYPWT - CPYMWT - CPYEWT - Memory Copy - writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWT--CPYMWT--CPYEWT--Memory-Copy--writes-unprivileged-?lang=en)
    ///
    /// Memory Copy, writes unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWT, then CPYMWT, and then CPYEWT.
    ///
    /// CPYPWT performs some preconditioning of the arguments suitable for using the CPYMWT instruction, and performs an implementation defined amount of the memory copy. CPYMWT performs an implementation defined amount of the memory copy. CPYEWT performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMWT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymwt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b0001, xn, xd)
    }


    /// [CPYPWT - CPYMWT - CPYEWT - Memory Copy - writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWT--CPYMWT--CPYEWT--Memory-Copy--writes-unprivileged-?lang=en)
    ///
    /// Memory Copy, writes unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWT, then CPYMWT, and then CPYEWT.
    ///
    /// CPYPWT performs some preconditioning of the arguments suitable for using the CPYMWT instruction, and performs an implementation defined amount of the memory copy. CPYMWT performs an implementation defined amount of the memory copy. CPYEWT performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPWT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpypwt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b0001, xn, xd)
    }


    /// [CPYPRT - CPYMRT - CPYERT - Memory Copy - reads unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRT--CPYMRT--CPYERT--Memory-Copy--reads-unprivileged-?lang=en)
    ///
    /// Memory Copy, reads unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRT, then CPYMRT, and then CPYERT.
    ///
    /// CPYPRT performs some preconditioning of the arguments suitable for using the CPYMRT instruction, and performs an implementation defined amount of the memory copy. CPYMRT performs an implementation defined amount of the memory copy. CPYERT performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYERT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyert(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b0010, xn, xd)
    }


    /// [CPYPRT - CPYMRT - CPYERT - Memory Copy - reads unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRT--CPYMRT--CPYERT--Memory-Copy--reads-unprivileged-?lang=en)
    ///
    /// Memory Copy, reads unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRT, then CPYMRT, and then CPYERT.
    ///
    /// CPYPRT performs some preconditioning of the arguments suitable for using the CPYMRT instruction, and performs an implementation defined amount of the memory copy. CPYMRT performs an implementation defined amount of the memory copy. CPYERT performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMRT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymrt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b0010, xn, xd)
    }


    /// [CPYPRT - CPYMRT - CPYERT - Memory Copy - reads unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRT--CPYMRT--CPYERT--Memory-Copy--reads-unprivileged-?lang=en)
    ///
    /// Memory Copy, reads unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRT, then CPYMRT, and then CPYERT.
    ///
    /// CPYPRT performs some preconditioning of the arguments suitable for using the CPYMRT instruction, and performs an implementation defined amount of the memory copy. CPYMRT performs an implementation defined amount of the memory copy. CPYERT performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPRT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyprt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b0010, xn, xd)
    }


    /// [CPYPT - CPYMT - CPYET - Memory Copy - reads and writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPT--CPYMT--CPYET--Memory-Copy--reads-and-writes-unprivileged-?lang=en)
    ///
    /// Memory Copy, reads and writes unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPT, then CPYMT, and then CPYET.
    ///
    /// CPYPT performs some preconditioning of the arguments suitable for using the CPYMT instruction, and performs an implementation defined amount of the memory copy. CPYMT performs an implementation defined amount of the memory copy. CPYET performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYET [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyet(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b0011, xn, xd)
    }


    /// [CPYPT - CPYMT - CPYET - Memory Copy - reads and writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPT--CPYMT--CPYET--Memory-Copy--reads-and-writes-unprivileged-?lang=en)
    ///
    /// Memory Copy, reads and writes unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPT, then CPYMT, and then CPYET.
    ///
    /// CPYPT performs some preconditioning of the arguments suitable for using the CPYMT instruction, and performs an implementation defined amount of the memory copy. CPYMT performs an implementation defined amount of the memory copy. CPYET performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b0011, xn, xd)
    }


    /// [CPYPT - CPYMT - CPYET - Memory Copy - reads and writes unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPT--CPYMT--CPYET--Memory-Copy--reads-and-writes-unprivileged-?lang=en)
    ///
    /// Memory Copy, reads and writes unprivileged. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPT, then CPYMT, and then CPYET.
    ///
    /// CPYPT performs some preconditioning of the arguments suitable for using the CPYMT instruction, and performs an implementation defined amount of the memory copy. CPYMT performs an implementation defined amount of the memory copy. CPYET performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPT [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpypt(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b0011, xn, xd)
    }


    /// [CPYPWN - CPYMWN - CPYEWN - Memory Copy - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWN--CPYMWN--CPYEWN--Memory-Copy--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWN, then CPYMWN, and then CPYEWN.
    ///
    /// CPYPWN performs some preconditioning of the arguments suitable for using the CPYMWN instruction, and performs an implementation defined amount of the memory copy. CPYMWN performs an implementation defined amount of the memory copy. CPYEWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYEWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyewn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b0100, xn, xd)
    }


    /// [CPYPWN - CPYMWN - CPYEWN - Memory Copy - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWN--CPYMWN--CPYEWN--Memory-Copy--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWN, then CPYMWN, and then CPYEWN.
    ///
    /// CPYPWN performs some preconditioning of the arguments suitable for using the CPYMWN instruction, and performs an implementation defined amount of the memory copy. CPYMWN performs an implementation defined amount of the memory copy. CPYEWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b0100, xn, xd)
    }


    /// [CPYPWN - CPYMWN - CPYEWN - Memory Copy - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWN--CPYMWN--CPYEWN--Memory-Copy--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWN, then CPYMWN, and then CPYEWN.
    ///
    /// CPYPWN performs some preconditioning of the arguments suitable for using the CPYMWN instruction, and performs an implementation defined amount of the memory copy. CPYMWN performs an implementation defined amount of the memory copy. CPYEWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpypwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b0100, xn, xd)
    }


    /// [CPYPWTWN - CPYMWTWN - CPYEWTWN - Memory Copy - writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTWN--CPYMWTWN--CPYEWTWN--Memory-Copy--writes-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy, writes unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWTWN, then CPYMWTWN, and then CPYEWTWN.
    ///
    /// CPYPWTWN performs some preconditioning of the arguments suitable for using the CPYMWTWN instruction, and performs an implementation defined amount of the memory copy. CPYMWTWN performs an implementation defined amount of the memory copy. CPYEWTWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYEWTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyewtwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b0101, xn, xd)
    }


    /// [CPYPWTWN - CPYMWTWN - CPYEWTWN - Memory Copy - writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTWN--CPYMWTWN--CPYEWTWN--Memory-Copy--writes-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy, writes unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWTWN, then CPYMWTWN, and then CPYEWTWN.
    ///
    /// CPYPWTWN performs some preconditioning of the arguments suitable for using the CPYMWTWN instruction, and performs an implementation defined amount of the memory copy. CPYMWTWN performs an implementation defined amount of the memory copy. CPYEWTWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMWTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymwtwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b0101, xn, xd)
    }


    /// [CPYPWTWN - CPYMWTWN - CPYEWTWN - Memory Copy - writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTWN--CPYMWTWN--CPYEWTWN--Memory-Copy--writes-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy, writes unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWTWN, then CPYMWTWN, and then CPYEWTWN.
    ///
    /// CPYPWTWN performs some preconditioning of the arguments suitable for using the CPYMWTWN instruction, and performs an implementation defined amount of the memory copy. CPYMWTWN performs an implementation defined amount of the memory copy. CPYEWTWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPWTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpypwtwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b0101, xn, xd)
    }


    /// [CPYPRTWN - CPYMRTWN - CPYERTWN - Memory Copy - reads unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTWN--CPYMRTWN--CPYERTWN--Memory-Copy--reads-unprivileged--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads unprivileged, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRTWN, then CPYMRTWN, and then CPYERTWN.
    ///
    /// CPYPRTWN performs some preconditioning of the arguments suitable for using the CPYMRTWN instruction, and performs an implementation defined amount of the memory copy. CPYMRTWN performs an implementation defined amount of the memory copy. CPYERTWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYERTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyertwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b0110, xn, xd)
    }


    /// [CPYPRTWN - CPYMRTWN - CPYERTWN - Memory Copy - reads unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTWN--CPYMRTWN--CPYERTWN--Memory-Copy--reads-unprivileged--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads unprivileged, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRTWN, then CPYMRTWN, and then CPYERTWN.
    ///
    /// CPYPRTWN performs some preconditioning of the arguments suitable for using the CPYMRTWN instruction, and performs an implementation defined amount of the memory copy. CPYMRTWN performs an implementation defined amount of the memory copy. CPYERTWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMRTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymrtwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b0110, xn, xd)
    }


    /// [CPYPRTWN - CPYMRTWN - CPYERTWN - Memory Copy - reads unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTWN--CPYMRTWN--CPYERTWN--Memory-Copy--reads-unprivileged--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads unprivileged, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRTWN, then CPYMRTWN, and then CPYERTWN.
    ///
    /// CPYPRTWN performs some preconditioning of the arguments suitable for using the CPYMRTWN instruction, and performs an implementation defined amount of the memory copy. CPYMRTWN performs an implementation defined amount of the memory copy. CPYERTWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPRTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyprtwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b0110, xn, xd)
    }


    /// [CPYPTWN - CPYMTWN - CPYETWN - Memory Copy - reads and writes unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTWN--CPYMTWN--CPYETWN--Memory-Copy--reads-and-writes-unprivileged--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads and writes unprivileged, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPTWN, then CPYMTWN, and then CPYETWN.
    ///
    /// CPYPTWN performs some preconditioning of the arguments suitable for using the CPYMTWN instruction, and performs an implementation defined amount of the memory copy. CPYMTWN performs an implementation defined amount of the memory copy. CPYETWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYETWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyetwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b0111, xn, xd)
    }

    /// [CPYPTWN - CPYMTWN - CPYETWN - Memory Copy - reads and writes unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTWN--CPYMTWN--CPYETWN--Memory-Copy--reads-and-writes-unprivileged--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads and writes unprivileged, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPTWN, then CPYMTWN, and then CPYETWN.
    ///
    /// CPYPTWN performs some preconditioning of the arguments suitable for using the CPYMTWN instruction, and performs an implementation defined amount of the memory copy. CPYMTWN performs an implementation defined amount of the memory copy. CPYETWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymtwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b0111, xn, xd)
    }


    /// [CPYPTWN - CPYMTWN - CPYETWN - Memory Copy - reads and writes unprivileged - writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTWN--CPYMTWN--CPYETWN--Memory-Copy--reads-and-writes-unprivileged--writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads and writes unprivileged, writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPTWN, then CPYMTWN, and then CPYETWN.
    ///
    /// CPYPTWN performs some preconditioning of the arguments suitable for using the CPYMTWN instruction, and performs an implementation defined amount of the memory copy. CPYMTWN performs an implementation defined amount of the memory copy. CPYETWN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPTWN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyptwn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b0111, xn, xd)
    }


    /// [CPYPRN - CPYMRN - CPYERN - Memory Copy - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRN--CPYMRN--CPYERN--Memory-Copy--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRN, then CPYMRN, and then CPYERN.
    ///
    /// CPYPRN performs some preconditioning of the arguments suitable for using the CPYMRN instruction, and performs an implementation defined amount of the memory copy. CPYMRN performs an implementation defined amount of the memory copy. CPYERN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYERN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyern(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b1000, xn, xd)
    }


    /// [CPYPRN - CPYMRN - CPYERN - Memory Copy - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRN--CPYMRN--CPYERN--Memory-Copy--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRN, then CPYMRN, and then CPYERN.
    ///
    /// CPYPRN performs some preconditioning of the arguments suitable for using the CPYMRN instruction, and performs an implementation defined amount of the memory copy. CPYMRN performs an implementation defined amount of the memory copy. CPYERN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b1000, xn, xd)
    }


    /// [CPYPRN - CPYMRN - CPYERN - Memory Copy - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRN--CPYMRN--CPYERN--Memory-Copy--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRN, then CPYMRN, and then CPYERN.
    ///
    /// CPYPRN performs some preconditioning of the arguments suitable for using the CPYMRN instruction, and performs an implementation defined amount of the memory copy. CPYMRN performs an implementation defined amount of the memory copy. CPYERN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyprn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b1000, xn, xd)
    }


    /// [CPYPWTRN - CPYMWTRN - CPYEWTRN - Memory Copy - writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTRN--CPYMWTRN--CPYEWTRN--Memory-Copy--writes-unprivileged--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy, writes unprivileged, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWTRN, then CPYMWTRN, and then CPYEWTRN.
    ///
    /// CPYPWTRN performs some preconditioning of the arguments suitable for using the CPYMWTRN instruction, and performs an implementation defined amount of the memory copy. CPYMWTRN performs an implementation defined amount of the memory copy. CPYEWTRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYEWTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyewtrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b1001, xn, xd)
    }


    /// [CPYPWTRN - CPYMWTRN - CPYEWTRN - Memory Copy - writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTRN--CPYMWTRN--CPYEWTRN--Memory-Copy--writes-unprivileged--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy, writes unprivileged, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWTRN, then CPYMWTRN, and then CPYEWTRN.
    ///
    /// CPYPWTRN performs some preconditioning of the arguments suitable for using the CPYMWTRN instruction, and performs an implementation defined amount of the memory copy. CPYMWTRN performs an implementation defined amount of the memory copy. CPYEWTRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMWTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymwtrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b1001, xn, xd)
    }


    /// [CPYPWTRN - CPYMWTRN - CPYEWTRN - Memory Copy - writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTRN--CPYMWTRN--CPYEWTRN--Memory-Copy--writes-unprivileged--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy, writes unprivileged, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWTRN, then CPYMWTRN, and then CPYEWTRN.
    ///
    /// CPYPWTRN performs some preconditioning of the arguments suitable for using the CPYMWTRN instruction, and performs an implementation defined amount of the memory copy. CPYMWTRN performs an implementation defined amount of the memory copy. CPYEWTRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPWTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpypwtrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b1001, xn, xd)
    }


    /// [CPYPRTRN - CPYMRTRN - CPYERTRN - Memory Copy - reads unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTRN--CPYMRTRN--CPYERTRN--Memory-Copy--reads-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRTRN, then CPYMRTRN, and then CPYERTRN.
    ///
    /// CPYPRTRN performs some preconditioning of the arguments suitable for using the CPYMRTRN instruction, and performs an implementation defined amount of the memory copy. CPYMRTRN performs an implementation defined amount of the memory copy. CPYERTRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYERTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyertrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b1010, xn, xd)
    }


    /// [CPYPRTRN - CPYMRTRN - CPYERTRN - Memory Copy - reads unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTRN--CPYMRTRN--CPYERTRN--Memory-Copy--reads-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRTRN, then CPYMRTRN, and then CPYERTRN.
    ///
    /// CPYPRTRN performs some preconditioning of the arguments suitable for using the CPYMRTRN instruction, and performs an implementation defined amount of the memory copy. CPYMRTRN performs an implementation defined amount of the memory copy. CPYERTRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMRTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyemtrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b1010, xn, xd)
    }


    /// [CPYPRTRN - CPYMRTRN - CPYERTRN - Memory Copy - reads unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTRN--CPYMRTRN--CPYERTRN--Memory-Copy--reads-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRTRN, then CPYMRTRN, and then CPYERTRN.
    ///
    /// CPYPRTRN performs some preconditioning of the arguments suitable for using the CPYMRTRN instruction, and performs an implementation defined amount of the memory copy. CPYMRTRN performs an implementation defined amount of the memory copy. CPYERTRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPRTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyprtrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b1010, xn, xd)
    }


    /// [CPYPTRN - CPYMTRN - CPYETRN - Memory Copy - reads and writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTRN--CPYMTRN--CPYETRN--Memory-Copy--reads-and-writes-unprivileged--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads and writes unprivileged, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPTRN, then CPYMTRN, and then CPYETRN.
    ///
    /// CPYPTRN performs some preconditioning of the arguments suitable for using the CPYMTRN instruction, and performs an implementation defined amount of the memory copy. CPYMTRN performs an implementation defined amount of the memory copy. CPYETRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYETRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyetrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b1011, xn, xd)
    }


    /// [CPYPTRN - CPYMTRN - CPYETRN - Memory Copy - reads and writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTRN--CPYMTRN--CPYETRN--Memory-Copy--reads-and-writes-unprivileged--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads and writes unprivileged, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPTRN, then CPYMTRN, and then CPYETRN.
    ///
    /// CPYPTRN performs some preconditioning of the arguments suitable for using the CPYMTRN instruction, and performs an implementation defined amount of the memory copy. CPYMTRN performs an implementation defined amount of the memory copy. CPYETRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymtrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b1011, xn, xd)
    }


    /// [CPYPTRN - CPYMTRN - CPYETRN - Memory Copy - reads and writes unprivileged - reads non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTRN--CPYMTRN--CPYETRN--Memory-Copy--reads-and-writes-unprivileged--reads-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads and writes unprivileged, reads non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPTRN, then CPYMTRN, and then CPYETRN.
    ///
    /// CPYPTRN performs some preconditioning of the arguments suitable for using the CPYMTRN instruction, and performs an implementation defined amount of the memory copy. CPYMTRN performs an implementation defined amount of the memory copy. CPYETRN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPTRN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyptrn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b1011, xn, xd)
    }


    /// [CPYPN - CPYMN - CPYEN - Memory Copy - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPN--CPYMN--CPYEN--Memory-Copy--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPN, then CPYMN, and then CPYEN.
    ///
    /// CPYPN performs some preconditioning of the arguments suitable for using the CPYMN instruction, and performs an implementation defined amount of the memory copy. CPYMN performs an implementation defined amount of the memory copy. CPYEN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYEN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyen(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b1100, xn, xd)
    }


    /// [CPYPN - CPYMN - CPYEN - Memory Copy - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPN--CPYMN--CPYEN--Memory-Copy--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPN, then CPYMN, and then CPYEN.
    ///
    /// CPYPN performs some preconditioning of the arguments suitable for using the CPYMN instruction, and performs an implementation defined amount of the memory copy. CPYMN performs an implementation defined amount of the memory copy. CPYEN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b1100, xn, xd)
    }


    /// [CPYPN - CPYMN - CPYEN - Memory Copy - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPN--CPYMN--CPYEN--Memory-Copy--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPN, then CPYMN, and then CPYEN.
    ///
    /// CPYPN performs some preconditioning of the arguments suitable for using the CPYMN instruction, and performs an implementation defined amount of the memory copy. CPYMN performs an implementation defined amount of the memory copy. CPYEN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpypn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b1100, xn, xd)
    }


    /// [CPYPWTN - CPYMWTN - CPYEWTN - Memory Copy - writes unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTN--CPYMWTN--CPYEWTN--Memory-Copy--writes-unprivileged--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, writes unprivileged, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWTN, then CPYMWTN, and then CPYEWTN.
    ///
    /// CPYPWTN performs some preconditioning of the arguments suitable for using the CPYMWTN instruction, and performs an implementation defined amount of the memory copy. CPYMWTN performs an implementation defined amount of the memory copy. CPYEWTN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYEWTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyewtn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b1101, xn, xd)
    }


    /// [CPYPWTN - CPYMWTN - CPYEWTN - Memory Copy - writes unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTN--CPYMWTN--CPYEWTN--Memory-Copy--writes-unprivileged--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, writes unprivileged, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWTN, then CPYMWTN, and then CPYEWTN.
    ///
    /// CPYPWTN performs some preconditioning of the arguments suitable for using the CPYMWTN instruction, and performs an implementation defined amount of the memory copy. CPYMWTN performs an implementation defined amount of the memory copy. CPYEWTN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMWTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymwtn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b1101, xn, xd)
    }


    /// [CPYPWTN - CPYMWTN - CPYEWTN - Memory Copy - writes unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPWTN--CPYMWTN--CPYEWTN--Memory-Copy--writes-unprivileged--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, writes unprivileged, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPWTN, then CPYMWTN, and then CPYEWTN.
    ///
    /// CPYPWTN performs some preconditioning of the arguments suitable for using the CPYMWTN instruction, and performs an implementation defined amount of the memory copy. CPYMWTN performs an implementation defined amount of the memory copy. CPYEWTN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPWTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpypwtn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b1101, xn, xd)
    }


    /// [CPYPRTN - CPYMRTN - CPYERTN - Memory Copy - reads unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTN--CPYMRTN--CPYERTN--Memory-Copy--reads-unprivileged--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads unprivileged, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRTN, then CPYMRTN, and then CPYERTN.
    ///
    /// CPYPRTN performs some preconditioning of the arguments suitable for using the CPYMRTN instruction, and performs an implementation defined amount of the memory copy. CPYMRTN performs an implementation defined amount of the memory copy. CPYERTN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYERTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyertn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b1110, xn, xd)
    }


    /// [CPYPRTN - CPYMRTN - CPYERTN - Memory Copy - reads unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTN--CPYMRTN--CPYERTN--Memory-Copy--reads-unprivileged--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads unprivileged, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRTN, then CPYMRTN, and then CPYERTN.
    ///
    /// CPYPRTN performs some preconditioning of the arguments suitable for using the CPYMRTN instruction, and performs an implementation defined amount of the memory copy. CPYMRTN performs an implementation defined amount of the memory copy. CPYERTN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMRTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymrtn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b1110, xn, xd)
    }


    /// [CPYPRTN - CPYMRTN - CPYERTN - Memory Copy - reads unprivileged - reads and writes non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPRTN--CPYMRTN--CPYERTN--Memory-Copy--reads-unprivileged--reads-and-writes-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads unprivileged, reads and writes non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPRTN, then CPYMRTN, and then CPYERTN.
    ///
    /// CPYPRTN performs some preconditioning of the arguments suitable for using the CPYMRTN instruction, and performs an implementation defined amount of the memory copy. CPYMRTN performs an implementation defined amount of the memory copy. CPYERTN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPRTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyprtn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b1110, xn, xd)
    }


    /// [CPYPTN - CPYMTN - CPYETN - Memory Copy - reads and writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTN--CPYMTN--CPYETN--Memory-Copy--reads-and-writes-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads and writes unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPTN, then CPYMTN, and then CPYETN.
    ///
    /// CPYPTN performs some preconditioning of the arguments suitable for using the CPYMTN instruction, and performs an implementation defined amount of the memory copy. CPYMTN performs an implementation defined amount of the memory copy. CPYETN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYETN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyetn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b10, xs, 0b1111, xn, xd)
    }


    /// [CPYPTN - CPYMTN - CPYETN - Memory Copy - reads and writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTN--CPYMTN--CPYETN--Memory-Copy--reads-and-writes-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads and writes unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPTN, then CPYMTN, and then CPYETN.
    ///
    /// CPYPTN performs some preconditioning of the arguments suitable for using the CPYMTN instruction, and performs an implementation defined amount of the memory copy. CPYMTN performs an implementation defined amount of the memory copy. CPYETN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYMTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpymtn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b1, xs, 0b1111, xn, xd)
    }


    /// [CPYPTN - CPYMTN - CPYETN - Memory Copy - reads and writes unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/CPYPTN--CPYMTN--CPYETN--Memory-Copy--reads-and-writes-unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Copy, reads and writes unprivileged and non-temporal. These instructions perform a memory copy. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: CPYPTN, then CPYMTN, and then CPYETN.
    ///
    /// CPYPTN performs some preconditioning of the arguments suitable for using the CPYMTN instruction, and performs an implementation defined amount of the memory copy. CPYMTN performs an implementation defined amount of the memory copy. CPYETN performs the last part of the memory copy.
    ///
    /// ```asm
    /// CPYPTN [<Xd>]!, [<Xs>]!, <Xn>!
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn cpyptn(&mut self, xd: Register, xs: Register, xn: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b0, xs, 0b1111, xn, xd)
    }


    /// [SETGP - SETGM - SETGE - Memory Set with tag setting](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGP--SETGM--SETGE--Memory-Set-with-tag-setting-?lang=en)
    ///
    /// Memory Set with tag setting. These instructions perform a memory set using the value in the bottom byte of the source register and store an Allocation Tag to memory for each Tag Granule written. The Allocation Tag is calculated from the Logical Address Tag in the register which holds the first address that the set is made to. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETGP, then SETGM, and then SETGE.
    ///
    /// SETGP performs some preconditioning of the arguments suitable for using the SETGM instruction, and performs an implementation defined amount of the memory set. SETGM performs an implementation defined amount of the memory set. SETGE performs the last part of the memory set.
    ///
    /// ```asm
    /// SETGE [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setge(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b11, xs, 0b1000, xn, xd)
    }


    /// [SETGP - SETGM - SETGE - Memory Set with tag setting](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGP--SETGM--SETGE--Memory-Set-with-tag-setting-?lang=en)
    ///
    /// Memory Set with tag setting. These instructions perform a memory set using the value in the bottom byte of the source register and store an Allocation Tag to memory for each Tag Granule written. The Allocation Tag is calculated from the Logical Address Tag in the register which holds the first address that the set is made to. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETGP, then SETGM, and then SETGE.
    ///
    /// SETGP performs some preconditioning of the arguments suitable for using the SETGM instruction, and performs an implementation defined amount of the memory set. SETGM performs an implementation defined amount of the memory set. SETGE performs the last part of the memory set.
    ///
    /// ```asm
    /// SETGM [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setgm(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b11, xs, 0b0100, xn, xd)
    }


    /// [SETGP - SETGM - SETGE - Memory Set with tag setting](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGP--SETGM--SETGE--Memory-Set-with-tag-setting-?lang=en)
    ///
    /// Memory Set with tag setting. These instructions perform a memory set using the value in the bottom byte of the source register and store an Allocation Tag to memory for each Tag Granule written. The Allocation Tag is calculated from the Logical Address Tag in the register which holds the first address that the set is made to. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETGP, then SETGM, and then SETGE.
    ///
    /// SETGP performs some preconditioning of the arguments suitable for using the SETGM instruction, and performs an implementation defined amount of the memory set. SETGM performs an implementation defined amount of the memory set. SETGE performs the last part of the memory set.
    ///
    /// ```asm
    /// SETGP [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setgp(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b11, xs, 0b0000, xn, xd)
    }


    /// [SETGPT - SETGMT - SETGET - Memory Set with tag setting - unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPT--SETGMT--SETGET--Memory-Set-with-tag-setting--unprivileged-?lang=en)
    ///
    /// Memory Set with tag setting, unprivileged. These instructions perform a memory set using the value in the bottom byte of the source register and store an Allocation Tag to memory for each Tag Granule written. The Allocation Tag is calculated from the Logical Address Tag in the register which holds the first address that the set is made to. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETGPT, then SETGMT, and then SETGET.
    ///
    /// SETGPT performs some preconditioning of the arguments suitable for using the SETGMT instruction, and performs an implementation defined amount of the memory set. SETGMT performs an implementation defined amount of the memory set. SETGET performs the last part of the memory set.
    ///
    /// ```asm
    /// SETGET [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setget(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b11, xs, 0b1001, xn, xd)
    }


    /// [SETGPT - SETGMT - SETGET - Memory Set with tag setting - unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPT--SETGMT--SETGET--Memory-Set-with-tag-setting--unprivileged-?lang=en)
    ///
    /// Memory Set with tag setting, unprivileged. These instructions perform a memory set using the value in the bottom byte of the source register and store an Allocation Tag to memory for each Tag Granule written. The Allocation Tag is calculated from the Logical Address Tag in the register which holds the first address that the set is made to. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETGPT, then SETGMT, and then SETGET.
    ///
    /// SETGPT performs some preconditioning of the arguments suitable for using the SETGMT instruction, and performs an implementation defined amount of the memory set. SETGMT performs an implementation defined amount of the memory set. SETGET performs the last part of the memory set.
    ///
    /// ```asm
    /// SETGMT [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setgmt(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b11, xs, 0b0101, xn, xd)
    }


    /// [SETGPT - SETGMT - SETGET - Memory Set with tag setting - unprivileged](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPT--SETGMT--SETGET--Memory-Set-with-tag-setting--unprivileged-?lang=en)
    ///
    /// Memory Set with tag setting, unprivileged. These instructions perform a memory set using the value in the bottom byte of the source register and store an Allocation Tag to memory for each Tag Granule written. The Allocation Tag is calculated from the Logical Address Tag in the register which holds the first address that the set is made to. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETGPT, then SETGMT, and then SETGET.
    ///
    /// SETGPT performs some preconditioning of the arguments suitable for using the SETGMT instruction, and performs an implementation defined amount of the memory set. SETGMT performs an implementation defined amount of the memory set. SETGET performs the last part of the memory set.
    ///
    /// ```asm
    /// SETGPT [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setgpt(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b11, xs, 0b0001, xn, xd)
    }


    /// [SETGPN - SETGMN - SETGEN - Memory Set with tag setting - non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPN--SETGMN--SETGEN--Memory-Set-with-tag-setting--non-temporal-?lang=en)
    ///
    /// Memory Set with tag setting, non-temporal. These instructions perform a memory set using the value in the bottom byte of the source register and store an Allocation Tag to memory for each Tag Granule written. The Allocation Tag is calculated from the Logical Address Tag in the register which holds the first address that the set is made to. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETGPN, then SETGMN, and then SETGEN.
    ///
    /// SETGPN performs some preconditioning of the arguments suitable for using the SETGMN instruction, and performs an implementation defined amount of the memory set. SETGMN performs an implementation defined amount of the memory set. SETGEN performs the last part of the memory set.
    ///
    /// ```asm
    /// SETGEN [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setgen(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b11, xs, 0b1010, xn, xd)
    }


    /// [SETGPN - SETGMN - SETGEN - Memory Set with tag setting - non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPN--SETGMN--SETGEN--Memory-Set-with-tag-setting--non-temporal-?lang=en)
    ///
    /// Memory Set with tag setting, non-temporal. These instructions perform a memory set using the value in the bottom byte of the source register and store an Allocation Tag to memory for each Tag Granule written. The Allocation Tag is calculated from the Logical Address Tag in the register which holds the first address that the set is made to. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETGPN, then SETGMN, and then SETGEN.
    ///
    /// SETGPN performs some preconditioning of the arguments suitable for using the SETGMN instruction, and performs an implementation defined amount of the memory set. SETGMN performs an implementation defined amount of the memory set. SETGEN performs the last part of the memory set.
    ///
    /// ```asm
    /// SETGMN [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setgmn(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b11, xs, 0b0110, xn, xd)
    }


    /// [SETGPN - SETGMN - SETGEN - Memory Set with tag setting - non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPN--SETGMN--SETGEN--Memory-Set-with-tag-setting--non-temporal-?lang=en)
    ///
    /// Memory Set with tag setting, non-temporal. These instructions perform a memory set using the value in the bottom byte of the source register and store an Allocation Tag to memory for each Tag Granule written. The Allocation Tag is calculated from the Logical Address Tag in the register which holds the first address that the set is made to. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETGPN, then SETGMN, and then SETGEN.
    ///
    /// SETGPN performs some preconditioning of the arguments suitable for using the SETGMN instruction, and performs an implementation defined amount of the memory set. SETGMN performs an implementation defined amount of the memory set. SETGEN performs the last part of the memory set.
    ///
    /// ```asm
    /// SETGPN [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setgpn(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b11, xs, 0b0010, xn, xd)
    }


    /// [SETGPTN - SETGMTN - SETGETN - Memory Set with tag setting - unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPTN--SETGMTN--SETGETN--Memory-Set-with-tag-setting--unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Set with tag setting, unprivileged and non-temporal. These instructions perform a memory set using the value in the bottom byte of the source register and store an Allocation Tag to memory for each Tag Granule written. The Allocation Tag is calculated from the Logical Address Tag in the register which holds the first address that the set is made to. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETGPTN, then SETGMTN, and then SETGETN.
    ///
    /// SETGPTN performs some preconditioning of the arguments suitable for using the SETGMTN instruction, and performs an implementation defined amount of the memory set. SETGMTN performs an implementation defined amount of the memory set. SETGETN performs the last part of the memory set.
    ///
    /// ```asm
    /// SETGETN [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setgetn(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b11, xs, 0b1011, xn, xd)
    }


    /// [SETGPTN - SETGMTN - SETGETN - Memory Set with tag setting - unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPTN--SETGMTN--SETGETN--Memory-Set-with-tag-setting--unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Set with tag setting, unprivileged and non-temporal. These instructions perform a memory set using the value in the bottom byte of the source register and store an Allocation Tag to memory for each Tag Granule written. The Allocation Tag is calculated from the Logical Address Tag in the register which holds the first address that the set is made to. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETGPTN, then SETGMTN, and then SETGETN.
    ///
    /// SETGPTN performs some preconditioning of the arguments suitable for using the SETGMTN instruction, and performs an implementation defined amount of the memory set. SETGMTN performs an implementation defined amount of the memory set. SETGETN performs the last part of the memory set.
    ///
    /// ```asm
    /// SETGMTN [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setgmtn(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b11, xs, 0b0111, xn, xd)
    }


    /// [SETGPTN - SETGMTN - SETGETN - Memory Set with tag setting - unprivileged and non temporal](https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/SETGPTN--SETGMTN--SETGETN--Memory-Set-with-tag-setting--unprivileged-and-non-temporal-?lang=en)
    ///
    /// Memory Set with tag setting, unprivileged and non-temporal. These instructions perform a memory set using the value in the bottom byte of the source register and store an Allocation Tag to memory for each Tag Granule written. The Allocation Tag is calculated from the Logical Address Tag in the register which holds the first address that the set is made to. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETGPTN, then SETGMTN, and then SETGETN.
    ///
    /// SETGPTN performs some preconditioning of the arguments suitable for using the SETGMTN instruction, and performs an implementation defined amount of the memory set. SETGMTN performs an implementation defined amount of the memory set. SETGETN performs the last part of the memory set.
    ///
    /// ```asm
    /// SETGPTN [<Xd>]!, <Xn>!, <Xs>
    /// ```
    ///
    /// **Warning**: Not tested
    #[inline(always)]
    fn setgptn(&mut self, xd: Register, xn: Register, xs: Register) -> T {
        emit_mem_cpy_mem_set(self, 0b00, 1, 0b11, xs, 0b0011, xn, xd)
    }
}
