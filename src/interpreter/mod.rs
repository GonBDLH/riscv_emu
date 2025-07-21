use std::{fs::File, io::Read};

use ihex::{Reader, Record};

use crate::interpreter::{
    mmu::Mmu,
    riscv_core::{Exception, InstructionType, RVCore},
};

mod csr;
mod extensions;
mod mmu;
mod riscv_core;

const NUM_HARTS: usize = 1;

pub struct Interpreter {
    pub mmu: Mmu,
    core: RVCore,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            mmu: Mmu::default(),
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

            println!("{record:?}");
            if let Record::Data { offset, value } = record {
                value.iter().enumerate().for_each(|(add, val)| {
                    let _ = self.mmu.write_byte(0x80000000 + offset as usize + add, *val);
                });
            }
        }

        println!("DONE");
    }

    pub fn fetch(&mut self) -> Result<u32, Exception> {
        let val = self
            .mmu
            .read_word(self.core.pc as usize)
            .map_err(|_| Exception::InstructionAccessFault)?;

        Ok(val)
    }

    pub fn decode(&mut self, instr: u32) -> Option<InstructionType> {
        self.core.decode(instr)
    }

    pub fn run(&mut self) {
        loop {
            let instr_bytes = self.fetch();
            
            let mut exception = None;

            if self.core.pc == 0x80000004 {
                println!("TEST");
            }

            match instr_bytes {
                Ok(fetched) => {
                    let instr = self.decode(fetched);

                    if let Some(mut instruction) = instr {
                        let result = instruction
                            .execute(&mut self.mmu, &mut self.core)
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
