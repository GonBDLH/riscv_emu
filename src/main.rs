use riscv_emu::interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new();

    interpreter.load_hex("rv_tests/rv32mi-p-ma_fetch.hex");
    // interpreter.load_hex("bin/xv6_32/kernel.hex");

    interpreter.run();
}
