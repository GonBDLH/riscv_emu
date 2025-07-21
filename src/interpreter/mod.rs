use std::{fs::File, io::Read};

use ihex::{Reader, Record};

use crate::interpreter::{
    bus::Bus,
    riscv_core::{Exception, InstructionType, RVCore},
};

mod csr;
mod extensions;
mod bus;
mod riscv_core;
mod mmu;

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
                    let _ = self.bus.write_byte(0x80000000 + offset as usize + add, *val);
                });
            }
        }

    }

    pub fn fetch(&mut self) -> Result<u32, Exception> {
        let val = self
            .bus
            .read_word(self.core.pc as usize)
            .map_err(|_| Exception::InstructionAccessFault)?;

        Ok(val)
    }

    pub fn decode(&mut self, instr: u32) -> Option<InstructionType> {
        self.core.decode(instr)
    }

    #[cfg(test)]
    pub fn run(&mut self) {
        let mut i = 0;

        while i < 0x1000 {
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
        
            i += 1;
        }
        // println!("TEST");
    }

    #[cfg(test)]
    pub fn read_test_result(&self) -> u8 {
        self.bus.dram[0x80001000 - 0x80000000]
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
