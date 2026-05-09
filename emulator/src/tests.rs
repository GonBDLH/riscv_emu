
#![allow(non_snake_case)]

use crate::interpreter::Interpreter;
use ntest::timeout;


#[test]
#[timeout(2000)]
fn Zca_c_j_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.j-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_lwsp_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.lwsp-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_sub_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sub-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_lb_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lb-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zicntr_csrrs_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicntr-csrrs-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_exceptions_mprv_S_Mmode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_mprv_S_Mmode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zihpm_csrrs_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zihpm-csrrs-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_srli_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.srli-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_mv_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.mv-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn M_rem_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-rem-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_sw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sw-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zicsr_csrrsi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrsi-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_exceptions_Zalrsc_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zalrsc_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_add_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.add-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_fence_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-fence-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zicsr_csrrs_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrs-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_ori_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-ori-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_xor_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-xor-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn M_mulh_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-mulh-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zaamo_amomaxu_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amomaxu.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_bge_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-bge-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_addi4spn_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.addi4spn-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_bnez_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.bnez-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zicsr_csrrci_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrci-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_beq_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-beq-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_exceptions_Zaamo_Mmode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zaamo_Mmode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_lbu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lbu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_sra_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sra-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_slt_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-slt-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_sw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.sw-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zaamo_amoand_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoand.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn M_mulhu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-mulhu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_bltu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-bltu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_xori_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-xori-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zaamo_amoswap_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoswap.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_exceptions_Zalrsc_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zalrsc_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zicsr_csrrw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrw-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn ExceptionsZicboU_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsZicboU-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn M_div_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-div-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_lhu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lhu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Svbare_mstatus_mprv_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Svbare_mstatus_mprv.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_sh_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sh-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_or_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-or-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_jalr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.jalr-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn ExceptionsS_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsS-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_nop_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-nop-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_Svade_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_Svade_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_exceptions_mprv_U_Mmode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_mprv_U_Mmode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn ZicntrU_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ZicntrU-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zaamo_amoxor_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoxor.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_nop_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.nop-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_Svadu_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_Svadu_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_jr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.jr-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_sltiu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sltiu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zicsr_csrrwi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrwi-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_addi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.addi-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_jal_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.jal-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_Svadu_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_Svadu_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Sscounterenw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sscounterenw-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zifencei_fence_i_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zifencei-fence.i-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_swsp_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.swsp-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_jal_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-jal-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_andi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-andi-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_slli_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.slli-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn M_mulhsu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-mulhsu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_and_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.and-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn U_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/U-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_srli_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-srli-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_srai_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.srai-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Svbare_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Svbare_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Sstvecd_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sstvecd-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_lui_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.lui-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn ExceptionsU_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsU-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_addi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-addi-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zaamo_amomin_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amomin.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn ExceptionsZaamo_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsZaamo-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn ExceptionsSm_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsSm-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_srai_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-srai-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn M_mul_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-mul-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zaamo_amominu_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amominu.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_bgeu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-bgeu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_slli_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-slli-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_sll_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sll-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_add_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-add-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_lh_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lh-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_slti_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-slti-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_exceptions_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_sltu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sltu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn ExceptionsZc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsZc-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_jalr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-jalr-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_li_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.li-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_exceptions_Zaamo_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zaamo_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_xor_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.xor-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_andi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.andi-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_addi16sp_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.addi16sp-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_and_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-and-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn ZicntrS_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ZicntrS-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn ExceptionsZalrsc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsZalrsc-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zalrsc_lr_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zalrsc-lr.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_blt_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-blt-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_exceptions_Zalrsc_Mmode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zalrsc_Mmode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_lui_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lui-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Svbare_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Svbare_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_bne_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-bne-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zaamo_amoor_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoor.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn M_divu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-divu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zaamo_amomax_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amomax.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zicntr_csrrc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicntr-csrrc-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_lw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.lw-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zihpm_csrrc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zihpm-csrrc-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zaamo_amoadd_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoadd.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_lw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lw-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_exceptions_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_Svade_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_Svade_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn ExceptionsZicboS_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsZicboS-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_beqz_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.beqz-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn sv32_exceptions_Zaamo_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zaamo_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_sub_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.sub-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zicsr_csrrc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrc-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_sb_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sb-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_auipc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-auipc-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn I_srl_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-srl-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zalrsc_sc_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zalrsc-sc.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn Zca_c_or_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.or-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(2000)]
fn M_remu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-remu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}

