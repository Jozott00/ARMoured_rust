#[macro_export]
macro_rules! stream_mock {
    ($stream:ident, $($body:tt)*) => {{
        let mut mem = MockMemory::new();
        let mut emitter = MockEmitter::new();

        // mem expect
        mem.expect_is_executable().returning(|| false);

        // emitter expect
        emitter.expect_instr_ptr().returning(|| 0 as InstructionPointer);
        emitter.expect_emit().returning(|_| ());

        let mut $stream = InstrStream {
            mem: &mut mem,
            emitter,
        };

        $($body)*
    }};
}
