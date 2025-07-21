use std::{fs::File, io::Read};

use ihex::{Reader, Record};

use crate::interpreter::{
    bus::Bus,
    riscv_core::{Exception, InstructionType, RVCore},
};

mod bus;
mod csr;
mod extensions;
mod mmu;
mod riscv_core;

const NUM_HARTS: usize = 1;

#[derive(Default)]
pub struct Interpreter {
    pub bus: Bus,
    core: RVCore,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            bus: Bus::default(),
            core: RVCore::default(),
        }
    }

    pub fn load_test(&mut self, path: &str) {
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
                        .write_byte(0x80000000 + offset as usize + add, *val);
                });
            }
        }
    }

    pub fn fetch(&mut self) -> Result<u32, Exception> {
        let val = self
            .bus
            .read_aligned_word(self.core.pc as usize)
            .map_err(|_| Exception::InstructionAccessFault)?;

        Ok(val)
    }

    pub fn decode(&mut self, instr: u32) -> Option<InstructionType> {
        self.core.decode(instr)
    }

    #[cfg(test)]
    pub fn run(&mut self) {
        loop {
            let instr_bytes = self.fetch();

            let mut exception = None;

            if self.core.pc == 0x80000f8c {
                println!("DEBUG");
            }

            if self.core.pc == 0x8000004c {
                break;
            }

            match instr_bytes {
                Ok(fetched) => {
                    let instr = self.decode(fetched);

                    if let Some(mut instruction) = instr {
                        let result = instruction
                            .execute(&mut self.bus, &mut self.core)
                            .map_err(|execution_exception| exception = Some(execution_exception));
                        if result.is_ok() {
                            self.core.pc = self.core.pc.wrapping_add(4);
                        }
                    } else {
                        exception = Some(Exception::IllegalInstruction(fetched))
                    };
                }
                Err(fetch_exception) => exception = Some(fetch_exception),
            }

            if let Some(exc) = exception {
                exc.trap(&mut self.core);
            }

        }
        // println!("TEST");
    }

    #[cfg(test)]
    pub fn read_test_result(&self, to_host: usize) -> u32 {
        let val_1 = self.bus.dram[to_host - 0x80000000];
        let val_2 = self.bus.dram[to_host - 0x80000000 + 1];
        let val_3 = self.bus.dram[to_host - 0x80000000 + 2];
        let val_4 = self.bus.dram[to_host - 0x80000000 + 3];

        u32::from_le_bytes([val_1, val_2, val_3, val_4])
    }

    #[cfg(not(test))]
    pub fn run(&mut self) {
        loop {
            let instr_bytes = self.fetch();

            let mut exception = None;

            match instr_bytes {
                Ok(fetched) => {
                    let instr = self.decode(fetched);

                    if let Some(mut instruction) = instr {
                        let result = instruction
                            .execute(&mut self.bus, &mut self.core)
                            .map_err(|execution_exception| exception = Some(execution_exception));
                        if result.is_ok() {
                            self.core.pc = self.core.pc.wrapping_add(4);
                        }
                    } else {
                        exception = Some(Exception::IllegalInstruction(fetched))
                    };
                }
                Err(fetch_exception) => exception = Some(fetch_exception),
            }

            if let Some(exc) = exception {
                exc.trap(&mut self.core);
            }
        }
    }
}
