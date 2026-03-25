use std::{fs::File, io::Read};

use ihex::{Reader, Record};

use crate::interpreter::{
    bus::Bus,
    riscv_core::{Exception, ExceptionType, InstructionType, RVCore, Trap}, virtual_memory::sv32::{AccessType, PhysicalAddress, translate_address},
};

mod bus;
mod csr;
mod extensions;
mod virtual_memory;
mod riscv_core;

const NUM_HARTS: usize = 1;

#[derive(Default)]
pub struct Interpreter {
    pub bus: Bus,
    core: RVCore,
}

impl Interpreter {
    #[cfg(test)]
    pub fn new_test(to_host: usize) -> Self {
        Self {
            bus: Bus::new_test(to_host),
            core: RVCore::default(),
        }
    }

    #[cfg(not(test))]
    pub fn new() -> Self {
        Self {
            bus: Bus::default(),
            core: RVCore::default(),
        }
    }

    pub fn load_hex(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let mut buf = String::new();

        file.read_to_string(&mut buf).unwrap();

        let reader = Reader::new(&buf);

        for i in reader {
            let record = i.unwrap();

            if let Record::Data { offset, value } = record {
                value.iter().enumerate().for_each(|(add, val)| {
                    let _ = self
                        .bus
                        .write_byte(&PhysicalAddress(0x80000000u64 + offset as u64 + add as u64), *val);
                });
            }
        }
    }

    pub fn load_bin(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let mut buf: Vec<u8> = Vec::new();


        file.read_to_end(&mut buf).unwrap();

        for (i, val) in buf.iter().enumerate() {
            let _ = self.bus.write_byte(&PhysicalAddress(0x80000000u64 + i as u64), *val);
        }
    }

    pub fn fetch(&mut self) -> Result<u32, Exception> {
        let pc = self.core.pc;
        let phys_pc = translate_address(&mut self.core, &mut self.bus, pc, AccessType::Execute)?;

        // println!("{:#016X}", phys_pc.0);
        if phys_pc.0 == 0x80002948 {
            println!("TEST");
        }

        let val = self
            .bus
            .read_aligned_word(&phys_pc)
            .map_err(|_| Exception::new(ExceptionType::InstructionAccessFault, pc))?;

        Ok(val)
    }

    pub fn decode(&mut self, instr: u32) -> Option<InstructionType> {
        self.core.decode(instr)
    }

    pub fn step(&mut self) -> Result<(), Exception> {
        let fetched = self.fetch()?;
        let mut instr = self.decode(fetched).ok_or(Exception::new(ExceptionType::IllegalInstruction, fetched))?;

        instr.execute(&mut self.bus, &mut self.core)?;

        self.core.control_and_status.increment_minstret();
        self.core.pc = self.core.pc.wrapping_add(4);

        Ok(())
    }

    #[cfg(test)]
    pub fn read_test_result(&self, to_host: usize) -> u32 {
        let val_1 = self.bus.dram[to_host - 0x80000000];
        let val_2 = self.bus.dram[to_host - 0x80000000 + 1];
        let val_3 = self.bus.dram[to_host - 0x80000000 + 2];
        let val_4 = self.bus.dram[to_host - 0x80000000 + 3];

        u32::from_le_bytes([val_1, val_2, val_3, val_4])
    }

    pub fn run(&mut self) {
        loop {
            if let Err(exception) = self.step() {
                Trap::Exception(exception).handle(&mut self.core);
            };
        }
    }
}
