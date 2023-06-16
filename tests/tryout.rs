use armoured_rust::instruction_encoding::branch_exception_system::unconditional_branch_immediate::UnconditionalBranchImmediate;
use armoured_rust::instruction_encoding::branch_exception_system::unconditional_branch_register::UnconditionalBranchRegister;
use armoured_rust::instruction_encoding::data_proc_imm::add_substract_imm::AddSubtractImmediate;
use armoured_rust::instruction_encoding::data_proc_imm::bitfield::BitfieldInstructions;
use armoured_rust::instruction_encoding::data_proc_imm::logical_imm::LogicalImmediate;
use armoured_rust::instruction_encoding::data_proc_imm::mov_wide_imm::MovWideImmediate;
use armoured_rust::instruction_encoding::data_proc_imm::pc_rel_addr::{PcRelAddressing, PcRelAddressingWithAddress};
use armoured_rust::instruction_encoding::data_proc_reg::data_proc_one_src::DataProcessingOneSource;
use armoured_rust::instruction_encoding::data_proc_reg::data_proc_three_src::DataProcessingThreeSource;
use armoured_rust::instruction_encoding::loads_and_stores::compare_and_swap_pair::CompareAndSwapPair;
use armoured_rust::instruction_encoding::loads_and_stores::load_store_reg_pre_post_indexed::LoadStoreRegisterPrePostIndexed;
use armoured_rust::instruction_encoding::loads_and_stores::load_store_reg_unscaled_imm::LoadStoreRegisterUnscaledImmediate;
use armoured_rust::instruction_encoding::loads_and_stores::load_store_register_unsigned_imm::LoadStoreRegisterUnsignedImmediate;
use armoured_rust::instruction_stream::InstrStream;
use armoured_rust::mc_memory::{McMemory, Memory};
use armoured_rust::types::Imm64;
use armoured_rust::types::prefetch_memory::{PrfOp, PrfPolicy, PrfTarget, PrfType};

#[cfg(target_arch = "aarch64")]
#[test]
fn main_tryout() {
    let mut mem = McMemory::new_pagesize();
    let mut stream = InstrStream::new(&mut mem);
    stream.movz_64_imm(1, 0x23);
    stream.add_64_imm(0, 1, 4);
    stream.adr_from_addr(0, stream.base_ptr() as usize);
    stream.adrp_from_byte_offset(0, 0x1000);
    stream.add_32_imm(0, 1, 0x123);
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
    stream.prfum_prfop()
    stream.b_from_byte_offset(0);
    stream.sbfm_64(1, 2, 0x8, 0x3);
    stream.movk_64_imm(2, 0xffff);
    stream.xpaci(3);
    stream.madd_64(3, 4, 2, 6);


    stream.patch_at(stream.base_ptr(), |s| {
        s.movn_64_imm(1, 4);
    });
    stream.print_disasm();

    let func = stream.nullary_fn_ptr();
    mem.make_executable();

    let res = unsafe { func() };
    println!("Called function with result: {res:#x}");
}
