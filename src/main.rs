use riscv_emu::interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::default();

    // interpreter.load_hex("rv_tests/rv32ui-p-ld_st.hex");
    interpreter.load_hex("rv_tests/rv32ui-v-or.hex");
    // interpreter.load_hex("bin/xv6_32/kernel.hex");
    // interpreter.load_bin("fw/fw_jump.bin");

    interpreter.run();
}
