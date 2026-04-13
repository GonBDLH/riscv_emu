#![allow(clippy::items_after_test_module)]

use std::collections::HashSet;

#[cfg(feature = "hitf")]
use crate::interpreter::hitf::HitfState;
use crate::{
    interpreter::{NUM_HARTS, riscv_core::{Exception, ExceptionType}, virtual_memory::sv32::{PhysicalAddress}},
    peripherals::{Peripheral, timer::RealTimeCounter, uart_16550::Uart16550},
};

pub const DRAM_BASE: usize = 0x80000000;
pub const DRAM_SIZE: usize = 8 * 1024 * 1024;
pub const DRAM_END: usize = DRAM_BASE + DRAM_SIZE;

pub const ROM_BASE: usize = 0x00001000;
pub const ROM_SIZE: usize = 0x00001000;
pub const ROM_END: usize = ROM_BASE + ROM_SIZE;

pub const RTC_BASE: usize = 0x00101000;
pub const RTC_SIZE: usize = 0x1000;
pub const RTC_END: usize = RTC_BASE + RTC_SIZE;

pub const UART_BASE: usize = 0x10000000;
pub const UART_SIZE: usize = 0x100;
pub const UART_END: usize = UART_BASE + UART_SIZE;

pub struct Bus {
    pub dram: Vec<u8>,

    rom: Vec<u8>,
    pub uart: Uart16550,
    pub timer: RealTimeCounter,

    #[cfg(feature = "hitf")]
    pub hitf: HitfState, 

    // PARA RV32A
    reserved_addresses: [HashSet<usize>; NUM_HARTS],
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            dram: vec![0x00; DRAM_SIZE],
            rom: vec![0x00; ROM_SIZE],
            uart: Uart16550::new(),
            timer: RealTimeCounter::new(),
            reserved_addresses: [HashSet::new(); NUM_HARTS],

            #[cfg(feature = "hitf")]
            hitf: HitfState::default()
        }
    }
}

impl Bus {
    pub fn read_byte(&self, phys_address: &PhysicalAddress) -> Result<u8, Exception> {
        let address = phys_address.0 as usize;

        match address {
            DRAM_BASE..DRAM_END => {
                #[cfg(feature = "hitf")]
                if address >= self.hitf.tohost && address < (self.hitf.tohost + 8) {
                    return Ok(self.hitf.read_tohost_byte(address));
                }
                #[cfg(feature = "hitf")]
                if address >= self.hitf.fromhost && address < (self.hitf.fromhost + 8) {
                    return Ok(self.hitf.read_fromhost_byte(address))
                }


                Ok(self.dram[address - DRAM_BASE])
            } 
            ROM_BASE..ROM_END => Ok(self.rom[address - ROM_BASE]),
            UART_BASE..UART_END => Ok(self.uart.read_byte(address - UART_BASE)),
            RTC_BASE..RTC_END => Ok(self.timer.read_byte(address - RTC_BASE)),
            _ => Err(Exception::new(ExceptionType::LoadAccessFault, 0)),
        }
    }

    pub fn write_byte(&mut self, phys_address: &PhysicalAddress, val: u8) -> Result<(), Exception> {
        let address = phys_address.0 as usize;

        match address {
            DRAM_BASE..DRAM_END => {
                #[cfg(feature = "hitf")]
                if address >= self.hitf.tohost && address < (self.hitf.tohost + 8) {
                    let hitf = self.hitf.write_tohost_byte(address, val);
                    if let Some(hitf) = hitf {
                        return hitf.run();
                    }
                }
                #[cfg(feature = "hitf")]
                if address >= self.hitf.fromhost && address < (self.hitf.fromhost + 8) {
                    self.hitf.write_fromhost_byte(address - self.hitf.fromhost, val);
                }

                self.dram[address - DRAM_BASE] = val;

                for i in 0..NUM_HARTS {
                    if self.is_address_reserved(i, address) {
                        self.invalidate_reserved_address(i, address);
                    }
                }

                Ok(())
            }
            UART_BASE..UART_END => {
                self.uart.write_byte(address - UART_BASE, val);
                Ok(())
            }
            RTC_BASE..RTC_END => {
                self.timer.write_byte(address - RTC_BASE, val);
                Ok(())
            }

            _ => Err(Exception::new(ExceptionType::StoreAmoAccessFault, 0)),
        }
    }

    pub fn read_aligned_word(&self, phys_address: &PhysicalAddress) -> Result<u32, Exception> {
        if phys_address.0 % 4 != 0 {
            return Err(Exception::new(ExceptionType::LoadAddressMisaligned, 0));
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

    pub fn read_aligned_half_word(&self, phys_address: &PhysicalAddress) -> Result<u16, Exception> {
        if phys_address.0 % 2 != 0 {
            return Err(Exception::new(ExceptionType::LoadAddressMisaligned, 0));
        }

        let val_0 = self.read_byte(phys_address)?;
        let val_1 = self.read_byte(&phys_address.wrapping_add(1))?;

        Ok(u16::from_le_bytes([val_0, val_1]))
    }

    pub fn write_aligned_half_word(&mut self, phys_address: &PhysicalAddress, half_word: u16) -> Result<(), Exception> {
        if phys_address.0 % 2 != 0 {
            return Err(Exception::new(ExceptionType::StoreAmoAddressMisaligned, 0));
        }

        let bytes = half_word.to_le_bytes();
        self.write_byte(phys_address, bytes[0])?;
        self.write_byte(&phys_address.wrapping_add(1), bytes[1])?;

        Ok(())
    }

    pub fn write_aligned_word(&mut self, phys_address: &PhysicalAddress, word: u32) -> Result<(), Exception> {
        if phys_address.0 % 4 != 0 {
            return Err(Exception::new(ExceptionType::StoreAmoAddressMisaligned, 0));
        }

        let bytes = word.to_le_bytes();
        self.write_byte(phys_address, bytes[0])?;
        self.write_byte(&phys_address.wrapping_add(1), bytes[1])?;
        self.write_byte(&phys_address.wrapping_add(2), bytes[2])?;
        self.write_byte(&phys_address.wrapping_add(3), bytes[3])?;

        Ok(())
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

impl Bus {
    pub fn load_section(&mut self, data: &[u8], start: usize) {
        assert!(start >= DRAM_BASE, "Invalid address");
        assert!(start + data.len() <= DRAM_END, "Segment too big");
        
        let offset = start - DRAM_BASE;
        let end = offset + data.len();

        self.dram[offset..end].copy_from_slice(data);
    }

    pub fn fill_zeros(&mut self, start: usize, end: usize) {
        assert!(start >= DRAM_BASE, "Invalid address");
        assert!(end <= DRAM_END, "Segment too big");

        let offset_start = start - DRAM_BASE;
        let offset_end = end - DRAM_BASE;
        
        self.dram[offset_start..offset_end].fill(0);
    } 
}