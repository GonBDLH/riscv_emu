
use crate::interpreter::Interpreter;
use ntest::timeout;


#[test]
#[timeout(2000)]
fn rv32ui_p_auipc() {
    let hitf_size = if "rv32ui-p-auipc".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-auipc", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32si_p_scall() {
    let hitf_size = if "rv32si-p-scall".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32si-p-scall", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_xor() {
    let hitf_size = if "rv32ui-p-xor".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-xor", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_bgeu() {
    let hitf_size = if "rv32ui-v-bgeu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-bgeu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_zicntr() {
    let hitf_size = if "rv32mi-p-zicntr".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-zicntr", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_p_mulhu() {
    let hitf_size = if "rv32um-p-mulhu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-p-mulhu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_pmpaddr() {
    let hitf_size = if "rv32mi-p-pmpaddr".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-pmpaddr", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amoor_w() {
    let hitf_size = if "rv32ua-v-amoor_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-v-amoor_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_and() {
    let hitf_size = if "rv32ui-p-and".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-and", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_bne() {
    let hitf_size = if "rv32ui-p-bne".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-bne", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_srl() {
    let hitf_size = if "rv32ui-v-srl".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-srl", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_andi() {
    let hitf_size = if "rv32ui-p-andi".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-andi", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sub() {
    let hitf_size = if "rv32ui-v-sub".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-sub", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32si_p_dirty() {
    let hitf_size = if "rv32si-p-dirty".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32si-p-dirty", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_lh() {
    let hitf_size = if "rv32ui-p-lh".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-lh", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_instret_overflow() {
    let hitf_size = if "rv32mi-p-instret_overflow".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-instret_overflow", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_v_div() {
    let hitf_size = if "rv32um-v-div".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-v-div", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_lh_misaligned() {
    let hitf_size = if "rv32mi-p-lh-misaligned".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-lh-misaligned", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sh() {
    let hitf_size = if "rv32ui-v-sh".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-sh", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_sw_misaligned() {
    let hitf_size = if "rv32mi-p-sw-misaligned".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-sw-misaligned", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_ori() {
    let hitf_size = if "rv32ui-v-ori".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-ori", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_bltu() {
    let hitf_size = if "rv32ui-v-bltu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-bltu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_v_mulhsu() {
    let hitf_size = if "rv32um-v-mulhsu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-v-mulhsu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_ori() {
    let hitf_size = if "rv32ui-p-ori".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-ori", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amomaxu_w() {
    let hitf_size = if "rv32ua-v-amomaxu_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-v-amomaxu_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_add() {
    let hitf_size = if "rv32ui-p-add".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-add", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_lbu() {
    let hitf_size = if "rv32ui-v-lbu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-lbu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_slt() {
    let hitf_size = if "rv32ui-p-slt".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-slt", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_p_mulhsu() {
    let hitf_size = if "rv32um-p-mulhsu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-p-mulhsu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_p_lrsc() {
    let hitf_size = if "rv32ua-p-lrsc".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-p-lrsc", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32si_p_csr() {
    let hitf_size = if "rv32si-p-csr".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32si-p-csr", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_jal() {
    let hitf_size = if "rv32ui-p-jal".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-jal", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_v_mulh() {
    let hitf_size = if "rv32um-v-mulh".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-v-mulh", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_ma_data() {
    let hitf_size = if "rv32ui-v-ma_data".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-ma_data", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amoadd_w() {
    let hitf_size = if "rv32ua-v-amoadd_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-v-amoadd_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_fence_i() {
    let hitf_size = if "rv32ui-v-fence_i".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-fence_i", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_srai() {
    let hitf_size = if "rv32ui-p-srai".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-srai", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_v_remu() {
    let hitf_size = if "rv32um-v-remu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-v-remu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_st_ld() {
    let hitf_size = if "rv32ui-p-st_ld".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-st_ld", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_simple() {
    let hitf_size = if "rv32ui-p-simple".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-simple", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_add() {
    let hitf_size = if "rv32ui-v-add".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-add", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_addi() {
    let hitf_size = if "rv32ui-p-addi".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-addi", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_ld_st() {
    let hitf_size = if "rv32ui-p-ld_st".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-ld_st", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amoswap_w() {
    let hitf_size = if "rv32ua-p-amoswap_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-p-amoswap_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_v_lrsc() {
    let hitf_size = if "rv32ua-v-lrsc".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-v-lrsc", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_lb() {
    let hitf_size = if "rv32ui-p-lb".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-lb", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_mcsr() {
    let hitf_size = if "rv32mi-p-mcsr".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-mcsr", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amomin_w() {
    let hitf_size = if "rv32ua-p-amomin_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-p-amomin_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_lbu() {
    let hitf_size = if "rv32ui-p-lbu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-lbu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_scall() {
    let hitf_size = if "rv32mi-p-scall".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-scall", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_slli() {
    let hitf_size = if "rv32ui-v-slli".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-slli", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_lb() {
    let hitf_size = if "rv32ui-v-lb".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-lb", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_p_mul() {
    let hitf_size = if "rv32um-p-mul".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-p-mul", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amoswap_w() {
    let hitf_size = if "rv32ua-v-amoswap_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-v-amoswap_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_lw() {
    let hitf_size = if "rv32ui-v-lw".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-lw", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amomax_w() {
    let hitf_size = if "rv32ua-v-amomax_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-v-amomax_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sltiu() {
    let hitf_size = if "rv32ui-v-sltiu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-sltiu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sltu() {
    let hitf_size = if "rv32ui-p-sltu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-sltu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32si_p_ma_fetch() {
    let hitf_size = if "rv32si-p-ma_fetch".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32si-p-ma_fetch", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_auipc() {
    let hitf_size = if "rv32ui-v-auipc".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-auipc", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_bge() {
    let hitf_size = if "rv32ui-v-bge".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-bge", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_blt() {
    let hitf_size = if "rv32ui-v-blt".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-blt", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sltiu() {
    let hitf_size = if "rv32ui-p-sltiu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-sltiu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_andi() {
    let hitf_size = if "rv32ui-v-andi".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-andi", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_bltu() {
    let hitf_size = if "rv32ui-p-bltu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-bltu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_addi() {
    let hitf_size = if "rv32ui-v-addi".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-addi", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amomax_w() {
    let hitf_size = if "rv32ua-p-amomax_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-p-amomax_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32si_p_wfi() {
    let hitf_size = if "rv32si-p-wfi".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32si-p-wfi", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_p_remu() {
    let hitf_size = if "rv32um-p-remu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-p-remu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sra() {
    let hitf_size = if "rv32ui-p-sra".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-sra", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_beq() {
    let hitf_size = if "rv32ui-p-beq".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-beq", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sub() {
    let hitf_size = if "rv32ui-p-sub".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-sub", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_ma_data() {
    let hitf_size = if "rv32ui-p-ma_data".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-ma_data", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sra() {
    let hitf_size = if "rv32ui-v-sra".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-sra", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_ld_st() {
    let hitf_size = if "rv32ui-v-ld_st".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-ld_st", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sb() {
    let hitf_size = if "rv32ui-p-sb".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-sb", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sw() {
    let hitf_size = if "rv32ui-p-sw".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-sw", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_blt() {
    let hitf_size = if "rv32ui-p-blt".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-blt", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_jalr() {
    let hitf_size = if "rv32ui-p-jalr".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-jalr", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_sbreak() {
    let hitf_size = if "rv32mi-p-sbreak".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-sbreak", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_st_ld() {
    let hitf_size = if "rv32ui-v-st_ld".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-st_ld", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_slli() {
    let hitf_size = if "rv32ui-p-slli".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-slli", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_and() {
    let hitf_size = if "rv32ui-v-and".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-and", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amominu_w() {
    let hitf_size = if "rv32ua-p-amominu_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-p-amominu_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_p_divu() {
    let hitf_size = if "rv32um-p-divu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-p-divu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_v_mulhu() {
    let hitf_size = if "rv32um-v-mulhu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-v-mulhu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_ma_addr() {
    let hitf_size = if "rv32mi-p-ma_addr".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-ma_addr", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32uc_v_rvc() {
    let hitf_size = if "rv32uc-v-rvc".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32uc-v-rvc", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amominu_w() {
    let hitf_size = if "rv32ua-v-amominu_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-v-amominu_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amomin_w() {
    let hitf_size = if "rv32ua-v-amomin_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-v-amomin_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amoxor_w() {
    let hitf_size = if "rv32ua-p-amoxor_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-p-amoxor_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_p_mulh() {
    let hitf_size = if "rv32um-p-mulh".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-p-mulh", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_v_divu() {
    let hitf_size = if "rv32um-v-divu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-v-divu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_lui() {
    let hitf_size = if "rv32ui-v-lui".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-lui", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_bne() {
    let hitf_size = if "rv32ui-v-bne".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-bne", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_srai() {
    let hitf_size = if "rv32ui-v-srai".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-srai", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_csr() {
    let hitf_size = if "rv32mi-p-csr".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-csr", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_fence_i() {
    let hitf_size = if "rv32ui-p-fence_i".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-fence_i", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_lw_misaligned() {
    let hitf_size = if "rv32mi-p-lw-misaligned".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-lw-misaligned", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_lhu() {
    let hitf_size = if "rv32ui-v-lhu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-lhu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_simple() {
    let hitf_size = if "rv32ui-v-simple".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-simple", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_lui() {
    let hitf_size = if "rv32ui-p-lui".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-lui", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_ma_fetch() {
    let hitf_size = if "rv32mi-p-ma_fetch".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-ma_fetch", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_lw() {
    let hitf_size = if "rv32ui-p-lw".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-lw", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amomaxu_w() {
    let hitf_size = if "rv32ua-p-amomaxu_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-p-amomaxu_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_slti() {
    let hitf_size = if "rv32ui-p-slti".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-slti", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_p_div() {
    let hitf_size = if "rv32um-p-div".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-p-div", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_lhu() {
    let hitf_size = if "rv32ui-p-lhu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-lhu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_xori() {
    let hitf_size = if "rv32ui-v-xori".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-xori", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_or() {
    let hitf_size = if "rv32ui-v-or".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-or", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_breakpoint() {
    let hitf_size = if "rv32mi-p-breakpoint".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-breakpoint", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_bge() {
    let hitf_size = if "rv32ui-p-bge".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-bge", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sb() {
    let hitf_size = if "rv32ui-v-sb".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-sb", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_srli() {
    let hitf_size = if "rv32ui-v-srli".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-srli", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_v_rem() {
    let hitf_size = if "rv32um-v-rem".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-v-rem", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sltu() {
    let hitf_size = if "rv32ui-v-sltu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-sltu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32si_p_sbreak() {
    let hitf_size = if "rv32si-p-sbreak".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32si-p-sbreak", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_xori() {
    let hitf_size = if "rv32ui-p-xori".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-xori", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_lh() {
    let hitf_size = if "rv32ui-v-lh".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-lh", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sll() {
    let hitf_size = if "rv32ui-v-sll".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-sll", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_bgeu() {
    let hitf_size = if "rv32ui-p-bgeu".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-bgeu", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_illegal() {
    let hitf_size = if "rv32mi-p-illegal".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-illegal", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32uc_p_rvc() {
    let hitf_size = if "rv32uc-p-rvc".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32uc-p-rvc", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_srl() {
    let hitf_size = if "rv32ui-p-srl".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-srl", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amoor_w() {
    let hitf_size = if "rv32ua-p-amoor_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-p-amoor_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sw() {
    let hitf_size = if "rv32ui-v-sw".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-sw", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_slt() {
    let hitf_size = if "rv32ui-v-slt".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-slt", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_srli() {
    let hitf_size = if "rv32ui-p-srli".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-srli", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amoand_w() {
    let hitf_size = if "rv32ua-p-amoand_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-p-amoand_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_or() {
    let hitf_size = if "rv32ui-p-or".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-or", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sll() {
    let hitf_size = if "rv32ui-p-sll".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-sll", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_xor() {
    let hitf_size = if "rv32ui-v-xor".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-xor", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_jal() {
    let hitf_size = if "rv32ui-v-jal".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-jal", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sh() {
    let hitf_size = if "rv32ui-p-sh".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-p-sh", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amoxor_w() {
    let hitf_size = if "rv32ua-v-amoxor_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-v-amoxor_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amoand_w() {
    let hitf_size = if "rv32ua-v-amoand_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-v-amoand_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_slti() {
    let hitf_size = if "rv32ui-v-slti".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-slti", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amoadd_w() {
    let hitf_size = if "rv32ua-p-amoadd_w".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ua-p-amoadd_w", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_jalr() {
    let hitf_size = if "rv32ui-v-jalr".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-jalr", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_sh_misaligned() {
    let hitf_size = if "rv32mi-p-sh-misaligned".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-sh-misaligned", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_v_mul() {
    let hitf_size = if "rv32um-v-mul".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-v-mul", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32ui_v_beq() {
    let hitf_size = if "rv32ui-v-beq".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32ui-v-beq", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32mi_p_shamt() {
    let hitf_size = if "rv32mi-p-shamt".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32mi-p-shamt", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}


#[test]
#[timeout(2000)]
fn rv32um_p_rem() {
    let hitf_size = if "rv32um-p-rem".contains("-v-") {
        8
    } else {
        4
    };

    let mut interpreter = Interpreter::new_test_elf("elf_tests/rv32um-p-rem", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}

