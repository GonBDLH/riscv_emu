use std::{
    fs::File,
    io::Read,
    time::{Duration, Instant},
};

use elf::{ElfBytes, endian::LittleEndian};
use ihex::{Reader, Record};

#[cfg(feature = "semihosting")]
use crate::interpreter::semihosting::semihosting;

use crate::interpreter::{
    bus::Bus,
    csr::ControlAndStatus,
    riscv_core::{
        Exception, ExceptionType, InstructionType, Interrupt, InterruptType, PrivilegeLevel,
        RVCore, Trap, WithErrVal,
    },
    virtual_memory::sv32::{AccessType, PhysicalAddress, translate_address},
};

mod bus;
mod csr;
mod extensions;
mod pmp;
pub mod riscv_core;
mod virtual_memory;

#[cfg(feature = "semihosting")]
mod semihosting;

const NUM_HARTS: usize = 1;

#[derive(Default)]
pub struct Interpreter {
    pub bus: Bus,
    pub core: RVCore,
}

impl Interpreter {
    #[cfg(test)]
    #[allow(unused_variables)]
    pub fn new_test_elf(path: &str) -> Self {
        let mut interpreter = Self {
            bus: Bus::default(),
            core: RVCore::default(),
        };

        interpreter.load_elf(path);

        interpreter
    }

    #[cfg(not(test))]
    pub fn new() -> Self {
        Self {
            bus: Bus::default(),
            core: RVCore::default()
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
                    let _ = self.bus.write_byte(
                        &PhysicalAddress(0x80000000u64 + offset as u64 + add as u64),
                        *val,
                    );
                });
            }
        }
    }

    pub fn load_bin(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let mut buf: Vec<u8> = Vec::new();

        file.read_to_end(&mut buf).unwrap();

        for (i, val) in buf.iter().enumerate() {
            let _ = self
                .bus
                .write_byte(&PhysicalAddress(0x80000000u64 + i as u64), *val);
        }
    }

    pub fn load_elf(&mut self, path: &str) {
        let path_buf = std::path::PathBuf::from(path);
        let file_data = std::fs::read(path_buf).expect("Could not read file.");
        let slice = file_data.as_slice();
        let file = ElfBytes::<LittleEndian>::minimal_parse(slice).expect("Bad format");

        for phdr in file.segments().unwrap() {
            if phdr.p_type == 1 {
                // PT_LOAD
                let data = file.segment_data(&phdr).unwrap();
                let phys_address = phdr.p_paddr as usize;

                self.bus.load_section(data, phys_address);

                if phdr.p_filesz != phdr.p_memsz {
                    self.bus.fill_zeros(
                        phys_address + phdr.p_filesz as usize,
                        phys_address + phdr.p_memsz as usize,
                    );
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.bus = Bus::default();
        self.core = RVCore::default();
    }

    pub fn fetch(&mut self) -> Result<u32, Exception> {
        let pc = self.core.get_pc();

        // TODO Hay que cambiar esto para cuando se haga un fecth de 16 bits (C instr)
        let phys_pc = translate_address(&mut self.core, &mut self.bus, pc, AccessType::Execute, 4)?;

        if phys_pc.0 == 0x8000004c {
            println!("!");
        }

        if !self.bus.check_pma(&phys_pc, AccessType::Execute) {
            return Err(Exception::new(
                AccessType::Execute.get_access_fault_exception(),
                pc,
            ));
        }

        let val = self.bus.read_word(&phys_pc).with_err_val(pc)?;

        Ok(val)
    }

    pub fn decode(&mut self, instr: u32) -> Result<InstructionType, Exception> {
        let width_bits = instr & 0b11;

        if width_bits == 0b11 {
            self.core
                .decode32(instr)
                .ok_or(Exception::new(ExceptionType::IllegalInstruction, instr))
        } else {
            self.core.decode16(instr as u16).ok_or(Exception::new(
                ExceptionType::IllegalInstruction,
                instr & 0x0000FFFF,
            ))
        }
    }

    pub fn core_step(&mut self) -> Result<(), Exception> {
        #![allow(unreachable_code)]

        if self.core.stalled {
            return Ok(());
        }

        let fetched = self.fetch()?;
        #[cfg(feature = "semihosting")]
        {
            if fetched == 0x01f01013 {
                use crate::interpreter::riscv_core::WithErrVal;

                let address = PhysicalAddress(self.core.get_pc() as u64 + 4);
                let break_instr = self
                    .bus
                    .read_word(&address)
                    .with_err_val(address.0 as u32)?;
                let address = PhysicalAddress(self.core.get_pc() as u64 + 8);
                let exit = self
                    .bus
                    .read_word(&address)
                    .with_err_val(address.0 as u32)?;

                if break_instr == 0x00100073 && exit == 0x40705013 {
                    self.core.new_pc = self.core.get_pc().wrapping_add(12);
                    return semihosting(self.core.read_reg(10), self.core.read_reg(11));
                }
            }
        }

        let instr = self.decode(fetched)?;

        instr.execute(&mut self.bus, &mut self.core)?;

        self.core.control_and_status.increment_minstret();

        Ok(())
    }

    pub fn emulator_step(&mut self, duration: Duration) -> Option<u32> {
        self.update_peripherals(duration);

        // TODO Check interrupts
        if let Some(int) = self.check_interrupt() {
            int.handle(&mut self.core);
        }

        if let Err(exception) = self.core_step() {
            match exception.exc_type {
                #[cfg(feature = "semihosting")]
                ExceptionType::ExitException => return Some(exception.get_val()),
                _ => exception.handle(&mut self.core),
            }
        };

        self.core.control_and_status.inc_cycle();
        self.core.update_pc();

        None
    }

    pub fn update_peripherals(&mut self, duration: Duration) {
        // TODO

        self.bus.mmio.update(duration);

        if let Some(int) = self.bus.mmio.has_interrupt() {
            self.core.control_and_status.set_mip_bit(int as u32);
        }

        // TODO let uart_int = self.bus.uart.has_interrupt();
    }

    pub fn check_interrupt(&mut self) -> Option<Interrupt> {
        let mip: u32 = self
            .core
            .control_and_status
            // .read_csr(bus, ControlAndStatus::MIP, PrivilegeLevel::Machine)
            .read_mip_unchecked();
        let mie = self.core.control_and_status.read_mie_unchecked();
        // .read_csr(bus, ControlAndStatus::MIE, PrivilegeLevel::Machine)
        // .unwrap();

        let pending = mip & mie;

        if pending == 0 {
            return None;
        }

        // Prioridad: external > timer > software
        let candidates = [
            InterruptType::MachineExternalInt,
            InterruptType::MachineSwInt,
            InterruptType::MachineTimerInt,
            InterruptType::SupervisorExternalInt,
            InterruptType::SupervisorSwInt,
            InterruptType::SupervisorTimerInt,
            InterruptType::CounterOverflowInt,
        ];

        for int_type in candidates {
            if self.core.check_int_to_m(int_type) {
                return Some(Interrupt::new(int_type, false));
            } else if self.core.check_int_to_s(int_type) {
                return Some(Interrupt::new(int_type, true));
            }
        }

        None
    }

    #[cfg(test)]
    pub fn read_test_result(&self, to_host: usize) -> u32 {
        let val_1 = self.bus.dram[to_host - 0x80000000];
        let val_2 = self.bus.dram[to_host - 0x80000000 + 1];
        let val_3 = self.bus.dram[to_host - 0x80000000 + 2];
        let val_4 = self.bus.dram[to_host - 0x80000000 + 3];

        u32::from_le_bytes([val_1, val_2, val_3, val_4])
    }

    pub fn run(&mut self) -> Option<u32> {
        let mut start = Instant::now();

        loop {
            let now = Instant::now();
            if let Some(ret) = self.emulator_step(now.duration_since(start)) {
                return Some(ret);
            }
            start = now;
        }
    }

    pub fn run_time(&mut self, duration: Duration) -> Option<u32> {
        let start = Instant::now();

        while Instant::now().duration_since(start) < duration {
            if let Some(ret) = self.emulator_step(duration) {
                return Some(ret);
            }
        }

        None
    }
}
