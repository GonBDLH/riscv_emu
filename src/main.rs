use clap::Parser;
use emulator::interpreter::{Interpreter, bus::DRAM_BASE};

use crate::gui::GuiState;

mod gui;

fn main_headless(args: Args) {
    let mut interpreter = Interpreter::default();

    // let elf_file = interpreter.load_elf("elf/rv32uc-p-rvc");

    // interpreter.load_hex("rv_tests/rv32ui-p-ld_st.hex");
    // interpreter.load_elf("I-add-00.elf");
    // interpreter.load_elf("elf_tests/rv32ui-p-add");
    // interpreter.load_hex("bin/xv6_32/kernel.hex");
    // interpreter.load_bin("fw/fw_jump.bin");
    if let Some(fw) = args.firmware {
        interpreter.load_elf(&fw, DRAM_BASE);
    };

    if let Some(kernel) = args.kernel {
        // TODO Sacar direccion a un define o algo
        interpreter.load_bin(&kernel, DRAM_BASE + 0x01000000);
    }

    if let Some(exit_code) = interpreter.run() {
        println!("{:08X}", exit_code);
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    headless: bool,
    #[arg(long, short)]
    firmware: Option<String>,
    #[arg(long, short)]
    kernel: Option<String>
}

fn main() {
    let args = Args::parse();

    if args.headless {
        main_headless(args);
    } else {
        let _res = main_gui();
    }
}

fn main_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "RV EMU",
        options,
        Box::new(|cc| Ok(Box::new(GuiState::new(cc)))),
    )
}
