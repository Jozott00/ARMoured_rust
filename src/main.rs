use bad64::{decode, disasm};

use crate::mc_memory::{McMemory, Memory};
use crate::types::{HW, Imm32, Imm64};
use std::fs;
use std::io::Result;
use crate::instruction_encoding::branch_exception_system::unconditional_branch_immediate::UnconditionalBranchImmediate;
use crate::instruction_encoding::branch_exception_system::unconditional_branch_register::UnconditionalBranchRegister;
use crate::instruction_encoding::data_proc_imm::add_substract_imm::AddSubtractImmediate;
use crate::instruction_encoding::data_proc_imm::bitfield::BitfieldInstructions;
use crate::instruction_encoding::data_proc_imm::logical_imm::LogicalImmediate;
use crate::instruction_encoding::data_proc_imm::mov_wide_imm::MovWideImmediate;
use crate::instruction_encoding::data_proc_imm::pc_rel_addr::{PcRelAddressing, PcRelAddressingWithAddress};
use crate::instruction_encoding::loads_and_stores::compare_and_swap_pair::CompareAndSwapPair;
use crate::instruction_encoding::loads_and_stores::load_store_reg_uimm::LoadStoreRegUImm;
use crate::instruction_stream::InstrStream;
use crate::types::prefetch_memory::{PrfOp, PrfPolicy, PrfTarget, PrfType};

pub mod mc_memory;
pub mod instruction_emitter;
pub mod types;
pub mod instruction_encoding;
pub mod test_utils;
pub mod instruction_producer;
pub mod instruction_stream;

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
    stream.casp_32(0, 1, 4, 5, 0);
    stream.casp_64(0, 1, 4, 5, 0);
    stream.caspa_32(0, 1, 4, 5, 0);
    stream.caspa_64(0, 1, 4, 5, 0);
    stream.caspal_32(0, 1, 4, 5, 0);
    stream.caspal_64(0, 1, 4, 5, 0);
    stream.caspl_32(0, 1, 4, 5, 0);
    stream.caspl_64(0, 1, 4, 5, 0);
    stream.strb_pre_index(0, 3, -256);
    stream.prfm_imm_prfop(PrfOp(PrfType::PLD, PrfTarget::L1, PrfPolicy::KEEP), 0, 0x0);
    stream.b_from_byte_offset(0);

    stream.patch_at(stream.base_ptr(), |s| {
        s.movn_64_imm(1, 4);
    });
    stream.print_disasm();

    let memory = stream.written_memory();
    write_bytes_to_file("target/test", memory);


    let func = stream.nullary_fn_ptr();
    mem.make_executable();

    let res = unsafe { func() };
    println!("Called function with result: {res:#x}");
}

fn write_bytes_to_file(path: &str, bytes: &[u8]) -> std::io::Result<()> {
    fs::write(path, bytes)?;
    Ok(())
}