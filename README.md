# ARMoured_rust

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/jozott00/armoured_rust/tests.yml?logo=github)
[![codecov](https://codecov.io/gh/Jozott00/ARMoured_rust/branch/main/graph/badge.svg?token=G3ZKEBKPEV)](https://codecov.io/gh/Jozott00/ARMoured_rust)

Provides a convenient API to produce arm64/Aarch64 instructions.

## Example

Here is an example usage of the `McMemory` and `InstrStream`. The `McMemory` (Machine Code Memory) allocates space for
one page size, while the `InstrStream` generates and emits instruction to this memory.

```rust
fn main() {
    let mut mem = McMemory::new_pagesize();
    let mut stream = InstrStream::new(&mut mem);
    stream.mov_64_imm(1, 0x23);
    stream.add_64_imm(0, 1, 0x4);
    stream.ret();

    // print stream disassembly before patch
    println!("Disasm before patch: ");
    stream.print_disasm();

    // patch `stream.mov_64_imm(1, 0x23);` instruction
    stream.patch_at(stream.base_ptr(), |s| {
        s.movn_64_imm(1, 4);
    });

    // print stream disassembly after patch
    println!("Disasm after patch: ");
    stream.print_disasm();

    // get stream `fn() -> u64` pointer and make memory executable 
    let func = stream.nullary_fn_ptr();
    mem.make_executable();

    // call function and get result
    let res = unsafe { func() };
    println!("Called function with result: {res:#x}");
}
```
