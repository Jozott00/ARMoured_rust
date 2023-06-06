use bad64::{decode, disasm};

use crate::instruction_stream::InstrStream;
use crate::mc_memory::McMemory;
use crate::types::{HW, Imm32, Imm64};

mod mc_memory;
mod instruction_emitter;
mod types;
mod instruction_stream;

fn main() {
    let mut mem = McMemory::new_pagesize();
    let mut stream = InstrStream::new(&mut mem);
    stream.mov_64_imm(1, 0x23);
    stream.add_64_imm(0, 1, 4);
    stream.adr_from_addr(0, stream.base_ptr() as usize);
    stream.adrp_from_byte_offset(0, 0x1000);
    stream.ret();
    stream.and_64_imm(0, 1, Imm64::MAX - 2);
    stream.orr_64_imm(0, 1, Imm64::MAX - 2);
    stream.eor_64_imm(0, 1, Imm64::MAX - 2);
    stream.ands_64_imm(0, 1, Imm64::MAX - 2);
    stream.bfm_32(0, 1, 31, 31);
    stream.adr_from_byte_offset(0, 0x10);
    stream.adr_from_byte_offset(0, -0x10);

    stream.patch_at(stream.base_ptr(), |s| {
        s.movn_64_imm(1, 4);
    });
    stream.print_disasm();

    let func = stream.nullary_fn_ptr();
    mem.make_executable();

    let res = unsafe { func() };
    println!("Called function with result: {res:#x}");
}
