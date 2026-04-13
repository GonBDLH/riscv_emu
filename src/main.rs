use riscv_emu::interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::default();

    // let elf_file = interpreter.load_elf("elf/rv32uc-p-rvc");

    // interpreter.load_hex("rv_tests/rv32ui-p-ld_st.hex");
    interpreter.load_elf("elf_tests/rv32ui-v-and");
    // interpreter.load_elf("elf_tests/rv32ui-p-add");
    // interpreter.load_hex("bin/xv6_32/kernel.hex");
    // interpreter.load_bin("fw/fw_jump.bin");

    let exit_code = interpreter.run();
    println!("{:08X}", exit_code);
}
