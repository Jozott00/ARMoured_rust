mod mc_memory;
mod instruction_emitter;
mod types;
mod instruction_stream;

use bad64::{decode, disasm};
use crate::instruction_stream::InstrStream;
use crate::mc_memory::McMemory;


fn main() {
    let mut mem = McMemory::new_pagesize();
    let mut stream = InstrStream::new(&mut mem);
    stream.mov_64_imm(0, 0x23, 0.into());
    stream.ret();

    stream.patch_at(stream.base_ptr(), |s| {
        s.mov_64_imm(0, 0x78, 0.into());
    });
    stream.print_disasm();

    let func = stream.nullary_fn_ptr();
    mem.make_executable();

    let res = unsafe { func() };
    println!("Called function with result: {res:#x}");
}
