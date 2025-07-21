
use crate::interpreter::Interpreter;


#[test]
fn rv32ui_p_add() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-add.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(), 1);
}


#[test]
fn rv32ui_p_simple() {
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/rv32ui-p-simple.hex");
    interpreter.run();
    assert_eq!(interpreter.read_test_result(), 1);
}

