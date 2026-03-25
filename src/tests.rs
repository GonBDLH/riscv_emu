
use crate::interpreter::Interpreter;
use ntest::timeout;


#[test]
#[timeout(2000)]
fn rv32ui_v_sw() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-sw.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_and() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-and.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_pmpaddr() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-pmpaddr.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_sbreak() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-sbreak.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sra() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-sra.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_or() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-or.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32si_p_sbreak() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32si-p-sbreak.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amomaxu_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-v-amomaxu_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_lbu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-lbu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_breakpoint() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-breakpoint.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amominu_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-p-amominu_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_bgeu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-bgeu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_addi() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-addi.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_v_mulh() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-v-mulh.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_slli() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-slli.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_auipc() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-auipc.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amoswap_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-v-amoswap_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_slt() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-slt.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_auipc() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-auipc.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_v_div() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-v-div.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amoadd_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-p-amoadd_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_xori() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-xori.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_srli() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-srli.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_zicntr() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-zicntr.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_bne() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-bne.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_ma_data() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-ma_data.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_ma_fetch() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-ma_fetch.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_lhu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-lhu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_lh() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-lh.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amoand_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-v-amoand_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_slti() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-slti.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_blt() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-blt.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sltiu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-sltiu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sh() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-sh.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_add() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-add.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_add() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-add.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_v_mul() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-v-mul.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amoor_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-p-amoor_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_fence_i() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-fence_i.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_fence_i() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-fence_i.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amomax_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-v-amomax_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sub() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-sub.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_p_mulhsu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-p-mulhsu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_lh() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-lh.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_lui() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-lui.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_slti() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-slti.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_lh_misaligned() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-lh-misaligned.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_ld_st() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80002000);
        interpreter.load_hex("rv_tests/rv32ui-p-ld_st.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_v_remu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-v-remu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_p_divu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-p-divu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_v_rem() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-v-rem.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_scall() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-scall.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_st_ld() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-st_ld.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amoadd_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-v-amoadd_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_beq() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-beq.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_srai() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-srai.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_simple() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-simple.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_v_mulhsu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-v-mulhsu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_csr() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-csr.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_instret_overflow() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-instret_overflow.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amomin_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-p-amomin_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32si_p_ma_fetch() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32si-p-ma_fetch.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amoxor_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-v-amoxor_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_lb() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-lb.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_jalr() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-jalr.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amoxor_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-p-amoxor_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_v_lrsc() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-v-lrsc.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_p_div() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-p-div.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_xor() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-xor.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amoswap_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-p-amoswap_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_v_divu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-v-divu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_ld_st() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-ld_st.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_or() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-or.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_bge() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-bge.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32si_p_csr() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32si-p-csr.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_lb() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-lb.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_bne() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-bne.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_and() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-and.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_p_remu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-p-remu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_jal() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-jal.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sw() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-sw.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_p_mulhu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-p-mulhu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_beq() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-beq.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_lw() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-lw.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amoand_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-p-amoand_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sltu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-sltu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_jalr() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-jalr.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sltiu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-sltiu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_xori() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-xori.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sb() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-sb.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_ori() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-ori.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32si_p_wfi() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32si-p-wfi.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amominu_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-v-amominu_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_andi() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-andi.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_srai() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-srai.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_jal() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-jal.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_lui() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-lui.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_ori() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-ori.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_slli() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-slli.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sra() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-sra.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_v_mulhu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-v-mulhu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amomaxu_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-p-amomaxu_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_srli() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-srli.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_lw_misaligned() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-lw-misaligned.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_addi() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-addi.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sub() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-sub.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sll() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-sll.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_p_lrsc() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-p-lrsc.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_illegal() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-illegal.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sltu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-sltu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32si_p_dirty() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32si-p-dirty.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_bltu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-bltu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_bltu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-bltu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_p_amomax_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-p-amomax_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amomin_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-v-amomin_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_p_mul() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-p-mul.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32si_p_scall() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32si-p-scall.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_bgeu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-bgeu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_slt() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-slt.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_lw() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-lw.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_shamt() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-shamt.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_ma_addr() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-ma_addr.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_st_ld() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-st_ld.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_lbu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-lbu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_lhu() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-lhu.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_ma_data() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-ma_data.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sb() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-sb.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_simple() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-simple.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_bge() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-bge.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_sh_misaligned() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-sh-misaligned.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_p_rem() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-p-rem.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_andi() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-andi.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_srl() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-srl.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ua_v_amoor_w() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ua-v-amoor_w.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_sh() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-sh.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32um_p_mulh() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32um-p-mulh.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_blt() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-blt.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_sw_misaligned() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-sw-misaligned.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_srl() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-srl.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_p_xor() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-p-xor.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32mi_p_mcsr() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32mi-p-mcsr.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}


#[test]
#[timeout(2000)]
fn rv32ui_v_sll() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        let mut interpreter = Interpreter::new_test(0x80001000);
        interpreter.load_hex("rv_tests/rv32ui-v-sll.hex");
        interpreter.run();
    });

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
}

