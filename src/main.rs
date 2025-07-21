
use riscv_emu::interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new();

    interpreter.load_test("tests/rv32ui-p-add.hex");

    interpreter.run();
}
