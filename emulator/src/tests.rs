
#![allow(non_snake_case)]

use crate::interpreter::Interpreter;
use ntest::timeout;


#[test]
#[timeout(5000)]
fn Zca_c_j_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.j-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_lwsp_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.lwsp-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sub_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sub-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_lb_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lb-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicntr_csrrs_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicntr-csrrs-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_A_off_all_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_A_off_all-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_global_pte_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_global_pte_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_mprv_S_Mmode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_mprv_S_Mmode.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPS_napot_legal_lxwr_02_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPS_napot_legal_lxwr-02-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zihpm_csrrs_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zihpm-csrrs-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_srli_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.srli-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sstvala_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sstvala-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_mv_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.mv-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_rem_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-rem-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_walk_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_walk-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPS_na4_legal_lxwr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPS_na4_legal_lxwr-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZca_misaligned_na4_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZca_misaligned_na4-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_walk_05_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_walk-05.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_invalid_pte_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_invalid_pte_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sw-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicsr_csrrsi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrsi-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Zalrsc_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zalrsc_Smode.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZca_aligned_na4_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZca_aligned_na4-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_priority_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_priority-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_na4_all_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_na4_all-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_add_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.add-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_napot_legal_lxwr_01_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_napot_legal_lxwr-01-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_fence_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-fence-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_all_entries_check_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_all_entries_check-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicsr_csrrs_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrs-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZca_aligned_off_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZca_aligned_off-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn U_ucsr_02_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/U_ucsr-02.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_ori_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-ori-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_pmpcfg_walk_04_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_pmpcfg_walk_04-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_spage_mstatus_sum_set_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_spage_mstatus_sum_set_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_xor_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-xor-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_mulh_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-mulh-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPS_tor_legal_lxwr_01_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPS_tor_legal_lxwr-01-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amomaxu_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amomaxu.w-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_mstatus_mprv_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_mstatus_mprv_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_bge_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-bge-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_addi4spn_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.addi4spn-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPU_cfg_XWR_unlocked_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPU_cfg_XWR_unlocked-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_bnez_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.bnez-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_A_all_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_A_all-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn S_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/S-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_invalid_pte_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_invalid_pte_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_walk_09_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_walk-09.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicsr_csrrci_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrci-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_spage_access_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_spage_access_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_walk_02_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_walk-02.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_L_modify_tor_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_L_modify_tor-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ZicntrSm_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ZicntrSm-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_beq_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-beq-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn U_ucsr_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/U_ucsr-01.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_XWR_all_03_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_XWR_all-03-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPU_tor_legal_lxwr_01_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPU_tor_legal_lxwr-01-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_L_modify_napot_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_L_modify_napot-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Zaamo_Mmode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zaamo_Mmode.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_VA_all_ones_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_VA_all_ones_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_lbu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lbu-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn S_scsr_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/S_scsr-01.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sra_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sra-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_slt_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-slt-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_csr_ro_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_csr_ro-01.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_sw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.sw-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_upage_mprv_set_sum_unset_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_upage_mprv_set_sum_unset_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_pmpcfg_walk_01_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_pmpcfg_walk_01-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn U_ucsr_ro_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/U_ucsr_ro-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SvPMP_sv32_pmp_on_pte_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SvPMP_sv32_pmp_on_pte_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_nleaf_pte_level0_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_nleaf_pte_level0_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amoand_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoand.w-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_mulhu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-mulhu-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_L_modify_off_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_L_modify_off-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_bltu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-bltu-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_addr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_addr-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsSm_medeleg_s_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsSm_medeleg_s-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_xori_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-xori-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amoswap_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoswap.w-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Zalrsc_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zalrsc_Umode.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsSm_medeleg_u_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsSm_medeleg_u-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zmmul_mulhu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zmmul-mulhu-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn U_ucsr_05_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/U_ucsr-05.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicsr_csrrw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrw-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_upage_mstatus_sum_set_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_upage_mstatus_sum_set_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_tor_all_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_tor_all-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_mstatus_mxr_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_mstatus_mxr_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Svbare_Svbare_mstatus_mprv_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Svbare_Svbare_mstatus_mprv-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_div_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-div-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_lhu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lhu-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_upage_mprv_set_sum_set_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_upage_mprv_set_sum_set_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_tor_check_02_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_tor_check-02-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZca_misaligned_tor_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZca_misaligned_tor-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sh_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sh-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_nleaf_pte_DAU_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_nleaf_pte_DAU_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_or_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-or-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_jalr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.jalr-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_tor_legal_lxwr_01_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_tor_legal_lxwr-01-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZaamo_cfg_wr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZaamo_cfg_wr-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_shadow_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_shadow-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsS_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsS-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_nop_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-nop-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zmmul_mulhsu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zmmul-mulhsu-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_walk_11_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_walk-11.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPU_tor_legal_lxwr_02_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPU_tor_legal_lxwr-02-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn S_scsr_insufficient_priv_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/S_scsr_insufficient_priv-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_mprv_U_Mmode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_mprv_U_Mmode.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_inst_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_inst-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZca_aligned_napot_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZca_aligned_napot-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_xret_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_xret-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_priority_off_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_priority_off-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ZicntrU_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ZicntrU-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amoxor_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoxor.w-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_nop_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.nop-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_scsr_from_m_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_scsr_from_m-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZca_misaligned_off_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZca_misaligned_off-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPS_cfg_XWR_unlocked_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPS_cfg_XWR_unlocked-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_jr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.jr-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_VA_all_zeros_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_VA_all_zeros_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPS_mprv_check_01_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPS_mprv_check-01-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zmmul_mulh_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zmmul-mulh-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_cntr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_cntr-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sltiu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sltiu-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicsr_csrrwi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrwi-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_addi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.addi-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_jal_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.jal-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_XWR_all_01_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_XWR_all-01-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_walk_04_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_walk-04.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_walk_03_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_walk-03.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_satp_from_m_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_satp_from_m-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZca_cret_tor_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZca_cret_tor-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sscounterenw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sscounterenw-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn U_ucsr_04_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/U_ucsr-04.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SvPMP_sv32_pmp_on_pa_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SvPMP_sv32_pmp_on_pa_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_walk_07_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_walk-07.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zifencei_fence_i_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zifencei-fence.i-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_swsp_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.swsp-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_pte_rsw_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_pte_rsw_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_jal_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-jal-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_csr_insufficient_priv_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_csr_insufficient_priv-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_andi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-andi-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_slli_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.slli-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_mulhsu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-mulhsu-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_and_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.and-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_A_tor_zero_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_A_tor_zero-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn U_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/U-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_pmpcfg_walk_02_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_pmpcfg_walk_02-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPS_cfg_XWR_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPS_cfg_XWR-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_tor_legal_lxwr_02_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_tor_legal_lxwr-02-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_misa_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_misa-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_srli_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-srli-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_XWR_all_04_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_XWR_all-04-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_srai_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.srai-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_pmpcfg_walk_05_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_pmpcfg_walk_05-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sstvecd_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sstvecd-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_nleaf_pte_level0_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_nleaf_pte_level0_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn S_scsr_ro_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/S_scsr_ro-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_upage_mstatus_sum_unset_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_upage_mstatus_sum_unset_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_satp_access_test_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_satp_access_test-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_lui_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.lui-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Svbare_Svbare_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Svbare_Svbare_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcause_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcause-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn U_ucsr_03_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/U_ucsr-03.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsU_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsU-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_A_tor_bot_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_A_tor_bot-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_mstatus_mxr_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_mstatus_mxr_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_addi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-addi-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amomin_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amomin.w-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsZaamo_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsZaamo-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsSm_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsSm-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_srai_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-srai-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_global_pte_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_global_pte_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mstatus_sd_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mstatus_sd-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_mul_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-mul-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_misaligned_page_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_misaligned_page_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_grain_check_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_grain_check-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_walk_06_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_walk-06.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZca_aligned_tor_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZca_aligned_tor-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Svbare_Svbare_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Svbare_Svbare_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn S_scsr_ro_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/S_scsr_ro-01.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_access_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_access-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amominu_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amominu.w-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_bgeu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-bgeu-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_slli_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-slli-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPU_cfg_A_off_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPU_cfg_A_off-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_napot_all_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_napot_all-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sll_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sll-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPU_cfg_XWR_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPU_cfg_XWR-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_add_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-add-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_lh_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lh-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_slti_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-slti-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPS_mprv_check_02_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPS_mprv_check-02-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Smode.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sltu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sltu-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPU_mprv_check_02_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPU_mprv_check-02-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsSm_medeleg_m_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsSm_medeleg_m-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsZc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsZc-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPS_tor_legal_lxwr_02_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPS_tor_legal_lxwr-02-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPS_cfg_A_off_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPS_cfg_A_off-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_walk_08_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_walk-08.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_satp_from_s_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_satp_from_s-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_jalr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-jalr-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_li_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.li-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Zaamo_Smode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zaamo_Smode.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_xor_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.xor-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_andi_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.andi-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_addi16sp_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.addi16sp-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_pte_reserved_rwx_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_pte_reserved_rwx_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_pte_rsw_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_pte_rsw_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_and_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-and-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_spage_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_spage_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_nleaf_pte_DAU_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_nleaf_pte_DAU_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_misaligned_page_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_misaligned_page_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZca_legal_lwrx_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZca_legal_lwrx-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_upage_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_upage_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn S_scsr_addr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/S_scsr_addr-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ZicntrS_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ZicntrS-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn ExceptionsZalrsc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/ExceptionsZalrsc-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zalrsc_lr_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zalrsc-lr.w-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_blt_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-blt-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_csr_ro_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_csr_ro-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn S_scsr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/S_scsr-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Zalrsc_Mmode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zalrsc_Mmode.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_lui_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lui-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_na4_legal_lxwr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_na4_legal_lxwr-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_bne_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-bne-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SvPMP_sv32_pmp_on_pte_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SvPMP_sv32_pmp_on_pte_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amoor_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoor.w-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_mstatus_mprv_Umode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_mstatus_mprv_Umode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_divu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-divu-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn SvPMP_sv32_pmp_on_pa_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/SvPMP_sv32_pmp_on_pa_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZca_cret_na4_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZca_cret_na4-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_walk_10_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_walk-10.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZca_cret_napot_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZca_cret_napot-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPU_mprv_check_01_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPU_mprv_check-01-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amomax_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amomax.w-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicntr_csrrc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicntr-csrrc-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_lw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.lw-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPU_napot_legal_lxwr_02_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPU_napot_legal_lxwr-02-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zihpm_csrrc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zihpm-csrrc-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPU_napot_legal_lxwr_01_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPU_napot_legal_lxwr-01-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_L_access_all_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_L_access_all-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPS_csr_access_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPS_csr_access-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPU_csr_access_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPU_csr_access-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zaamo_amoadd_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zaamo-amoadd.w-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_tor_check_01_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_tor_check-01-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_lw_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-lw-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Umode.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_XWR_all_02_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_XWR_all-02-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_grain_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_grain-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_beqz_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.beqz-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZalrsc_cfg_wr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZalrsc_cfg_wr-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn sv32_exceptions_Zaamo_Umode_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/sv32_exceptions_Zaamo_Umode.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn S_scsr_insufficient_priv_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/S_scsr_insufficient_priv-01.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_napot_legal_lxwr_02_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_napot_legal_lxwr-02-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_pmpcfg_walk_03_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_pmpcfg_walk_03-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPSm_cfg_tor_check_03_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPSm_cfg_tor_check-03-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv32_pte_reserved_rwx_Smode_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv32_pte_reserved_rwx_Smode-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_sub_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.sub-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zicsr_csrrc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zicsr-csrrc-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_sb_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-sb-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_auipc_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-auipc-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn I_srl_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/I-srl-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zmmul_mul_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zmmul-mul-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPZca_misaligned_napot_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPZca_misaligned_napot-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPU_na4_legal_lxwr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPU_na4_legal_lxwr-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zalrsc_sc_w_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zalrsc-sc.w-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Zca_c_or_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Zca-c.or-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sm_mcsr_walk_01_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sm_mcsr_walk-01.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn M_remu_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/M-remu-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn U_ucsr_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/U_ucsr-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn Sv_sv_mstatus_tvm_test_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/Sv_sv_mstatus_tvm_test-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}


#[test]
#[timeout(5000)]
fn PMPS_napot_legal_lxwr_01_00_elf() {
    let mut interpreter = Interpreter::new_test_elf("../semihosting_elf_tests/PMPS_napot_legal_lxwr-01-00.elf");
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}

