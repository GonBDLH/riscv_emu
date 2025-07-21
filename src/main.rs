use riscv_emu::interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new();

    // interpreter.load_test("tests/rv32ui-p-add.hex");
    interpreter.load_hex("bin/xv6_32/kernel.hex");

    interpreter.run();
}
