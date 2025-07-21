#![allow(clippy::items_after_test_module)]

use crate::{interpreter::{riscv_core::Exception, NUM_HARTS}, peripherals::uart_16550::Uart16550};

// #[cfg(not(test))]
mod addresses {
    pub const DRAM_BASE: usize = 0x80000000;
    pub const DRAM_SIZE: usize = 8 * 1024 * 1024;
    pub const DRAM_END: usize = DRAM_BASE + DRAM_SIZE;

    pub const ROM_BASE: usize = 0x00001000;
    pub const ROM_SIZE: usize = 0x00001000;
    pub const ROM_END: usize = ROM_BASE + ROM_SIZE;

    pub const UART_BASE: usize = 0x10000000;
    pub const UART_SIZE: usize = 8;
    pub const UART_END: usize = UART_BASE + UART_SIZE;
}

// #[cfg(test)]
// mod addresses {
//     pub const DRAM_BASE: usize = 0x80000000;
//     pub const DRAM_SIZE: usize = 0x1000;
//     pub const DRAM_END: usize = DRAM_BASE + DRAM_SIZE;

//     pub const ROM_BASE: usize = 0x00001000;
//     pub const ROM_SIZE: usize = 0x00001000;
//     pub const ROM_END: usize = ROM_BASE + ROM_SIZE;

//     pub const UART_BASE: usize = 0x80001000;
//     pub const UART_SIZE: usize = 8;
//     pub const UART_END: usize = UART_BASE + UART_SIZE;
// }

use addresses::*;

pub struct Bus {
    #[cfg(test)]
    pub dram: Vec<u8>,

    #[cfg(not(test))]
    dram: Vec<u8>,

    rom: Vec<u8>,
    pub uart: Uart16550,

    // PARA RV32A
    reserved_addresses: [Option<usize>; NUM_HARTS],
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            dram: vec![0x00; DRAM_SIZE],
            rom: vec![0x00; ROM_SIZE],
            uart: Uart16550::new(),
            reserved_addresses: [None; NUM_HARTS],
        }
    }
}

impl Bus {
    pub fn read_byte(&self, address: usize) -> Result<u8, Exception> {
        match address {
            DRAM_BASE..DRAM_END => Ok(self.dram[address - DRAM_BASE]),
            ROM_BASE..ROM_END => Ok(self.rom[address - ROM_BASE]),
            UART_BASE..UART_END => Ok(self.uart.read(address - UART_BASE)),
            _ => Err(Exception::LoadAccessFault)
        }
    }

    pub fn write_byte(&mut self, address: usize, val: u8) -> Result<(), Exception> {
        match address {
            DRAM_BASE..DRAM_END => {
                self.dram[address - DRAM_BASE] = val;

                for i in 0..NUM_HARTS {
                    if self.is_address_reserved(i, address) {
                        self.invalidate_reserved_address(i);
                    }
                }

                Ok(())
            },
            UART_BASE..UART_END => {
                self.uart.write(address - UART_BASE, val);
                Ok(())
            }

            _ => Err(Exception::StoreAmoAccessFault),
        }

    }

    pub fn read_word(&mut self, address: usize) -> Result<u32, Exception> {
        if address % 4 != 0 {
            return Err(Exception::LoadAddressMisaligned);
        }

        let val_0 = self.read_byte(address)?;
        let val_1 = self.read_byte(address.wrapping_add(1))?;
        let val_2 = self.read_byte(address.wrapping_add(2))?;
        let val_3 = self.read_byte(address.wrapping_add(3))?;

        Ok(u32::from_le_bytes([val_0, val_1, val_2, val_3]))
    }

    pub fn write_word(&mut self, address: usize, word: u32) -> Result<(), Exception> {
        if address % 4 != 0 {
            return Err(Exception::StoreAmoAddressMisaligned);
        }

        if address < DRAM_SIZE {
            let bytes = word.to_le_bytes();
            self.write_byte(address, bytes[0])?;
            self.write_byte(address.wrapping_add(1), bytes[1])?;
            self.write_byte(address.wrapping_add(2), bytes[2])?;
            self.write_byte(address.wrapping_add(3), bytes[3])
        } else {
            Err(Exception::StoreAmoAccessFault)
        }
    }

    pub fn reserve_address(&mut self, hart_id: usize, address: usize) {
        self.reserved_addresses[hart_id] = Some(address);
    }

    pub fn invalidate_reserved_address(&mut self, hart_id: usize) {
        self.reserved_addresses[hart_id] = None;
    }

    pub fn is_address_reserved(&self, hart_id: usize, address: usize) -> bool {
        if let Some(v) = self.reserved_addresses[hart_id] {
            v == address
        } else {
            false
        }
    }
}
