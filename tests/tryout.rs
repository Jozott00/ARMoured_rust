use armoured_rust::instruction_encoding::branch_exception_system::unconditional_branch_register::UnconditionalBranchRegister;
use armoured_rust::instruction_encoding::data_proc_imm::mov_wide_imm::MovWideImmediate;
use armoured_rust::instruction_stream::InstrStream;
use armoured_rust::mc_memory::{McMemory, Memory};

#[cfg(target_arch = "aarch64")]
#[test]
fn main_tryout() {
    let mut mem = McMemory::new_pagesize();
    let mut stream = InstrStream::new(&mut mem);

    stream.movz_64_imm(0, 0x123);
    stream.ret();

    stream.patch_at(stream.base_ptr(), |s| {
        s.movz_64_imm(0, 0x456);
    });
    stream.print_disasm();

    let func = stream.nullary_fn_ptr();
    mem.make_executable();

    let res = unsafe { func() };
    println!("Called function with result: {res:#x}");
}
