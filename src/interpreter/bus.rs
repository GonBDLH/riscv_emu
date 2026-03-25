#![allow(clippy::items_after_test_module)]

use std::{collections::HashSet, io::{Write, stdout}};

use crate::{
    interpreter::{NUM_HARTS, riscv_core::{Exception, ExceptionType}, virtual_memory::sv32::{PhysicalAddress, translate_address}},
    peripherals::uart_16550::Uart16550,
};

pub const DRAM_BASE: usize = 0x80000000;
pub const DRAM_SIZE: usize = 8 * 1024 * 1024;
pub const DRAM_END: usize = DRAM_BASE + DRAM_SIZE;

pub const ROM_BASE: usize = 0x00001000;
pub const ROM_SIZE: usize = 0x00001000;
pub const ROM_END: usize = ROM_BASE + ROM_SIZE;

pub const UART_BASE: usize = 0x10000000;
pub const UART_SIZE: usize = 0x100;
pub const UART_END: usize = UART_BASE + UART_SIZE;

pub struct Bus {
    #[cfg(test)]
    to_host: usize,

    pub dram: Vec<u8>,

    rom: Vec<u8>,
    pub uart: Uart16550,

    // PARA RV32A
    reserved_addresses: [HashSet<usize>; NUM_HARTS],
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            #[cfg(test)]
            to_host: 0x80001000,

            dram: vec![0x00; DRAM_SIZE],
            rom: vec![0x00; ROM_SIZE],
            uart: Uart16550::new(),
            reserved_addresses: [HashSet::new(); NUM_HARTS],
        }
    }
}

impl Bus {
    #[cfg(test)]
    pub fn new_test(to_host: usize) -> Self {
        let mut bus = Self::default();
        bus.to_host = to_host;

        bus
    }

    pub fn read_byte(&self, phys_address: &PhysicalAddress) -> Result<u8, Exception> {
        let address = phys_address.0 as usize;

        match address {
            DRAM_BASE..DRAM_END => Ok(self.dram[address - DRAM_BASE]),
            ROM_BASE..ROM_END => Ok(self.rom[address - ROM_BASE]),
            UART_BASE..UART_END => Ok(self.uart.read(address - UART_BASE)),
            _ => Err(Exception::new(ExceptionType::LoadAccessFault, 0)),
        }
    }

    pub fn write_byte(&mut self, phys_address: &PhysicalAddress, val: u8) -> Result<(), Exception> {
        let address = phys_address.0 as usize;

        match address {
            DRAM_BASE..DRAM_END => {
                self.dram[address - DRAM_BASE] = val;

                for i in 0..NUM_HARTS {
                    if self.is_address_reserved(i, address) {
                        self.invalidate_reserved_address(i, address);
                    }
                }

                Ok(())
            }
            UART_BASE..UART_END => {
                self.uart.write(address - UART_BASE, val);
                Ok(())
            }

            _ => Err(Exception::new(ExceptionType::StoreAmoAccessFault, 0)),
        }
    }

    pub fn read_aligned_word(&self, phys_address: &PhysicalAddress) -> Result<u32, Exception> {
        if phys_address.0 % 4 != 0 {
            return Err(Exception::new(ExceptionType::LoadAddressMisaligned, 0));
        }

        #[cfg(test)]
        if (self.to_host + 0x40) == phys_address.0 as usize {
            return Ok(1);
        }

        #[cfg(not(test))]
        if (0x80001000 + 0x40) == phys_address.0 as usize {
            return Ok(1);
        }

        let val_0 = self.read_byte(phys_address)?;
        let val_1 = self.read_byte(&phys_address.wrapping_add(1))?;
        let val_2 = self.read_byte(&phys_address.wrapping_add(2))?;
        let val_3 = self.read_byte(&phys_address.wrapping_add(3))?;

        Ok(u32::from_le_bytes([val_0, val_1, val_2, val_3]))
    }

    pub fn read_word(&self, phys_address: &PhysicalAddress) -> Result<u32, Exception> {
        let val_0 = self.read_byte(phys_address)?;
        let val_1 = self.read_byte(&phys_address.wrapping_add(1))?;
        let val_2 = self.read_byte(&phys_address.wrapping_add(2))?;
        let val_3 = self.read_byte(&phys_address.wrapping_add(3))?;

        Ok(u32::from_le_bytes([val_0, val_1, val_2, val_3]))
    }

    pub fn write_aligned_word(&mut self, phys_address: &PhysicalAddress, word: u32) -> Result<(), Exception> {
        if phys_address.0 % 4 != 0 {
            return Err(Exception::new(ExceptionType::StoreAmoAddressMisaligned, 0));
        }

        if phys_address.0 == 0x80001000 {
            println!("{:#016X} {:#08X}", phys_address.0, word);

        }

        #[cfg(test)]
        if phys_address.0 == self.to_host as u64 {
            println!("{word}");
            if word == 1 {
                panic!("PASS");
            } else {
                panic!("FAIL {}", word);
            }
            
        }

        let bytes = word.to_le_bytes();
        self.write_byte(phys_address, bytes[0])?;
        self.write_byte(&phys_address.wrapping_add(1), bytes[1])?;
        self.write_byte(&phys_address.wrapping_add(2), bytes[2])?;
        self.write_byte(&phys_address.wrapping_add(3), bytes[3])
    }

    pub fn reserve_address(&mut self, hart_id: usize, address: usize) {
        self.reserved_addresses[hart_id].insert(address);
    }

    pub fn invalidate_reserved_address(&mut self, hart_id: usize, address: usize) {
        self.reserved_addresses[hart_id].remove(&address);
    }

    pub fn is_address_reserved(&self, hart_id: usize, address: usize) -> bool {
        self.reserved_addresses[hart_id].contains(&address)
    }
}
