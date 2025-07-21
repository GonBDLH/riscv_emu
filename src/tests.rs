
use crate::interpreter::Interpreter;


#[test]
fn rv32ua_p_amoadd_w() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ua-p-amoadd_w.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ua_p_amoand_w() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ua-p-amoand_w.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ua_p_amomaxu_w() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ua-p-amomaxu_w.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ua_p_amomax_w() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ua-p-amomax_w.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ua_p_amominu_w() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ua-p-amominu_w.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ua_p_amomin_w() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ua-p-amomin_w.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ua_p_amoor_w() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ua-p-amoor_w.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ua_p_amoswap_w() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ua-p-amoswap_w.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ua_p_amoxor_w() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ua-p-amoxor_w.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ua_p_lrsc() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ua-p-lrsc.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_add() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-add.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_addi() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-addi.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_and() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-and.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_andi() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-andi.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_auipc() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-auipc.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_beq() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-beq.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_bge() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-bge.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_bgeu() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-bgeu.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_blt() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-blt.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_bltu() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-bltu.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_bne() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-bne.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_fence_i() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-fence_i.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_jal() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-jal.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_jalr() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-jalr.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_lb() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-lb.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_lbu() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-lbu.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_ld_st() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-ld_st.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80002000), 1);
}


#[test]
fn rv32ui_p_lh() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-lh.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_lhu() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-lhu.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_lui() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-lui.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_lw() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-lw.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_ma_data() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-ma_data.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_or() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-or.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_ori() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-ori.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_sb() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-sb.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_sh() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-sh.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_simple() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-simple.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_sll() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-sll.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_slli() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-slli.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_slt() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-slt.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_slti() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-slti.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_sltiu() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-sltiu.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_sltu() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-sltu.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_sra() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-sra.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_srai() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-srai.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_srl() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-srl.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_srli() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-srli.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_st_ld() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-st_ld.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_sub() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-sub.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_sw() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-sw.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_xor() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-xor.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32ui_p_xori() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-xori.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32um_p_div() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32um-p-div.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32um_p_divu() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32um-p-divu.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32um_p_mul() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32um-p-mul.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32um_p_mulh() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32um-p-mulh.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32um_p_mulhsu() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32um-p-mulhsu.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32um_p_mulhu() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32um-p-mulhu.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32um_p_rem() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32um-p-rem.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}


#[test]
fn rv32um_p_remu() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32um-p-remu.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(0x80001000), 1);
}

