
#![allow(non_snake_case)]

use crate::interpreter::Interpreter;
use ntest::timeout;


#[test]
#[timeout(5000)]
fn Zca_c_j_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.j-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_lwsp_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.lwsp-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sub_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sub-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_lb_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lb-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicntr_csrrs_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicntr-csrrs-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpzca_cret_tor_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpzca_cret_tor.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_L_modify_off_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_L_modify_off.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_mprv_S_Mmode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_mprv_S_Mmode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpu_csr_access_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpu_csr_access.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpu_mprv_check_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpu_mprv_check-01.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_tor_check_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_tor_check-01.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zihpm_csrrs_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zihpm-csrrs-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_srli_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.srli-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sstvala_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sstvala-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_mv_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.mv-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_rem_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-rem-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpu_cfg_XWR_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpu_cfg_XWR.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_grain_check_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_grain_check.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_tor_check_03_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_tor_check-03.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictSm_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictSm-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sw-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicsr_csrrsi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrsi-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Zalrsc_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zalrsc_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_csr_walk_4_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_csr_walk-4.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictSm_09_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictSm-09.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmps_napot_legal_lxwr_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmps_napot_legal_lxwr-01.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_add_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.add-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpzca_legal_lwrx_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpzca_legal_lwrx.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_VA_all_zeros_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_VA_all_zeros_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_fence_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-fence-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicsr_csrrs_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrs-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_ori_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-ori-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpu_cfg_A_off_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpu_cfg_A_off.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_csr_walk_3_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_csr_walk-3.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_xor_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-xor-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_mulh_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-mulh-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amomaxu_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amomaxu.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_bge_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-bge-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_addi4spn_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.addi4spn-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_bnez_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.bnez-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn S_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/S-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicsr_csrrci_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrci-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictSm_10_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictSm-10.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_beq_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-beq-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_nleaf_pte_DAU_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_nleaf_pte_DAU_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictSm_06_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictSm-06.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Zaamo_Mmode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zaamo_Mmode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_lbu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lbu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_nleaf_pte_DAU_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_nleaf_pte_DAU_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_XWR_all_03_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_XWR_all-03.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sra_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sra-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_slt_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-slt-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_sw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.sw-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpu_napot_legal_lxwr_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpu_napot_legal_lxwr-01.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictSm_08_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictSm-08.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amoand_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoand.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_upage_mstatus_sum_set_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_upage_mstatus_sum_set_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_mulhu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-mulhu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpzalrsc_cfg_wr_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpzalrsc_cfg_wr.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_bltu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-bltu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_xori_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-xori-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amoswap_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoswap.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_mstatus_mxr_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_mstatus_mxr_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Zalrsc_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zalrsc_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_invalid_pte_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_invalid_pte_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictU_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictU-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_XWR_all_02_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_XWR_all-02.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicsr_csrrw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrw-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictS_06_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictS-06.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_upage_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_upage_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmps_mprv_check_02_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmps_mprv_check-02.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictSm_02_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictSm-02.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsZicboU_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsZicboU-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_div_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-div-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_lhu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lhu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Svbare_mstatus_mprv_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Svbare_mstatus_mprv.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_napot_all_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_napot_all.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sh_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sh-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_csr_walk_2_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_csr_walk-2.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_or_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-or-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_jalr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.jalr-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictS_04_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictS-04.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_all_entries_check_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_all_entries_check.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpzca_aligned_napot_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpzca_aligned_napot.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_priority_off_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_priority_off.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_invalid_pte_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_invalid_pte_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_upage_mstatus_sum_unset_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_upage_mstatus_sum_unset_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_csr_walk_9_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_csr_walk-9.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsS_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsS-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_A_all_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_A_all.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_nop_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-nop-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_A_tor_bot_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_A_tor_bot.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmps_napot_legal_lxwr_02_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmps_napot_legal_lxwr-02.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_mstatus_mxr_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_mstatus_mxr_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictS_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictS-01.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_napot_legal_lwxr_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_napot_legal_lwxr.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_mprv_U_Mmode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_mprv_U_Mmode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ZicntrU_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ZicntrU-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpzca_cret_napot_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpzca_cret_napot.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_pmp_on_pte_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_pmp_on_pte_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amoxor_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoxor.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_nop_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.nop-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_pte_rsw_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_pte_rsw_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_mstatus_mprv_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_mstatus_mprv_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpzaamo_cfg_wr_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpzaamo_cfg_wr.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictSm_07_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictSm-07.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_jr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.jr-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_pte_reserved_rwx_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_pte_reserved_rwx_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictSm_05_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictSm-05.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sltiu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sltiu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmps_cfg_XWR_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmps_cfg_XWR.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicsr_csrrwi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrwi-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_addi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.addi-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictU_05_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictU-05.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn InterruptsS_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/InterruptsS-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_L_modify_napot_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_L_modify_napot.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_jal_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.jal-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_tor_all_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_tor_all.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_mstatus_mprv_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_mstatus_mprv_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sscounterenw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sscounterenw-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictS_07_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictS-07.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_pmp_on_pte_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_pmp_on_pte_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zifencei_fence_i_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zifencei-fence.i-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_swsp_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.swsp-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictU_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictU-01.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_jal_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-jal-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_andi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-andi-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_slli_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.slli-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_priority_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_priority.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpu_mprv_check_02_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpu_mprv_check-02.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpu_napot_legal_lxwr_02_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpu_napot_legal_lxwr-02.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_mulhsu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-mulhsu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_and_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.and-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictS_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictS-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_XWR_all_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_XWR_all-01.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn U_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/U-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_srli_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-srli-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_srai_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.srai-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv_mstatus_tvm_test_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv_mstatus_tvm_test.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Svbare_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Svbare_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_tor_check_02_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_tor_check-02.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_A_off_all_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_A_off_all.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_nleaf_pte_level0_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_nleaf_pte_level0_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sstvecd_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sstvecd-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_csr_walk_8_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_csr_walk-8.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_lui_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.lui-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_csr_walk_6_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_csr_walk-6.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_csr_walk_10_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_csr_walk-10.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsU_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsU-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_VA_all_ones_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_VA_all_ones_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_addi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-addi-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amomin_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amomin.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsZaamo_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsZaamo-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsSm_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsSm-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_srai_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-srai-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_satp_access_test_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_satp_access_test.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_mul_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-mul-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_misaligned_page_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_misaligned_page_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpzca_misaligned_off_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpzca_misaligned_off.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_spage_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_spage_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_csr_walk_7_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_csr_walk-7.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpzca_aligned_off_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpzca_aligned_off.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictSm_04_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictSm-04.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amominu_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amominu.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_bgeu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-bgeu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_slli_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-slli-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpu_tor_legal_lxwr_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpu_tor_legal_lxwr.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_pmp_on_pa_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_pmp_on_pa_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sll_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sll-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_csr_walk_1_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_csr_walk-1.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_add_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-add-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_lh_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lh-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_slti_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-slti-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_spage_mstatus_sum_set_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_spage_mstatus_sum_set_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_spage_access_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_spage_access_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sltu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sltu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_A_tor_zero_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_A_tor_zero.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_tor_legal_lwxr_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_tor_legal_lwxr.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsZc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsZc-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_jalr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-jalr-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_li_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.li-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_nleaf_pte_level0_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_nleaf_pte_level0_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Zaamo_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zaamo_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_xor_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.xor-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_andi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.andi-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_misaligned_page_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_misaligned_page_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_addi16sp_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.addi16sp-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmps_cfg_A_off_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmps_cfg_A_off.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmps_tor_legal_lxwr_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmps_tor_legal_lxwr.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_and_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-and-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_XWR_all_04_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_XWR_all-04.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ZicntrS_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ZicntrS-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictS_03_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictS-03.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsZalrsc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsZalrsc-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zalrsc_lr_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zalrsc-lr.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_blt_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-blt-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpzca_misaligned_tor_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpzca_misaligned_tor.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_upage_mprv_set_sum_set_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_upage_mprv_set_sum_set_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpzca_aligned_tor_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpzca_aligned_tor.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Zalrsc_Mmode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zalrsc_Mmode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_global_pte_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_global_pte_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_lui_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lui-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictSm_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictSm-01.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Svbare_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Svbare_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictSm_03_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictSm-03.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_bne_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-bne-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amoor_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoor.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_divu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-divu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_csr_walk_5_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_csr_walk-5.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_L_access_all_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_L_access_all.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_global_pte_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_global_pte_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amomax_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amomax.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmps_csr_access_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmps_csr_access.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_pte_reserved_rwx_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_pte_reserved_rwx_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicntr_csrrc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicntr-csrrc-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_lw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.lw-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zihpm_csrrc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zihpm-csrrc-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn InterruptsU_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/InterruptsU-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amoadd_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoadd.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_pmp_on_pa_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_pmp_on_pa_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_lw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lw-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_upage_mprv_set_sum_unset_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_upage_mprv_set_sum_unset_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsZicboS_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsZicboS-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictU_03_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictU-03.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn InterruptsSm_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/InterruptsSm-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_beqz_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.beqz-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Zaamo_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zaamo_Umode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_grain_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_grain.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_pte_rsw_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_pte_rsw_Smode.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictU_04_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictU-04.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictU_02_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictU-02.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_sub_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.sub-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpsm_cfg_L_modify_tor_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpsm_cfg_L_modify_tor.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicsr_csrrc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrc-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sb_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sb-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_auipc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-auipc-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_srl_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-srl-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zalrsc_sc_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zalrsc-sc.w-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictS_05_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictS-05.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_or_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.or-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmpzca_misaligned_napot_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmpzca_misaligned_napot.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn pmps_mprv_check_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/pmps_mprv_check-01.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_remu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-remu-00.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictS_08_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictS-08.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SsstrictS_02_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SsstrictS-02.elf", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}

