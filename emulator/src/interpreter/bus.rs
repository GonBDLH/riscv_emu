#![allow(clippy::items_after_test_module)]

#[cfg(feature = "hitf")]
use crate::interpreter::hitf::HitfState;
use crate::{
    interpreter::{NUM_HARTS, riscv_core::ExceptionType, virtual_memory::sv32::{AccessType, PhysicalAddress}},
    peripherals::{Mmio, RTC_BASE, RTC_END, UART_BASE, UART_END},
};

pub const DRAM_BASE: usize = 0x80000000;
pub const DRAM_SIZE: usize = 8 * 1024 * 1024;
pub const DRAM_END: usize = DRAM_BASE + DRAM_SIZE;

pub const ROM_BASE: usize = 0x00001000;
pub const ROM_SIZE: usize = 0x00001000;
pub const ROM_END: usize = ROM_BASE + ROM_SIZE;

pub const MMIO_BASE: usize = 0x02000000;
pub const MMIO_SIZE: usize = 0x10000000;
pub const MMIO_END: usize = MMIO_BASE + MMIO_SIZE;

pub struct Bus {
    pub dram: Vec<u8>,

    rom: Vec<u8>,
    // pub uart: Uart16550,
    // pub timer: RealTimeCounter,
    pub mmio: Mmio,

    #[cfg(feature = "hitf")]
    pub hitf: HitfState,

    // PARA RV32A
    reserved_addresses: [Option<(usize, usize)>; NUM_HARTS],
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            dram: vec![0x00; DRAM_SIZE],
            rom: vec![0x00; ROM_SIZE],
            // uart: Uart16550::new(),
            // timer: RealTimeCounter::new(),
            mmio: Mmio::new(),
            reserved_addresses: [None],

            #[cfg(feature = "hitf")]
            hitf: HitfState::default(),
        }
    }
}

impl Bus {
    pub fn read_byte(&self, phys_address: &PhysicalAddress) -> Result<u8, ExceptionType> {
        let address = phys_address.0 as usize;

        match address {
            DRAM_BASE..DRAM_END => {
                #[cfg(feature = "hitf")]
                if address >= self.hitf.tohost && address < (self.hitf.tohost + 8) {
                    return Ok(self.hitf.read_tohost_byte(address));
                }
                #[cfg(feature = "hitf")]
                if address >= self.hitf.fromhost && address < (self.hitf.fromhost + 8) {
                    return Ok(self.hitf.read_fromhost_byte(address));
                }

                Ok(self.dram[address - DRAM_BASE])
            }
            ROM_BASE..ROM_END => Ok(self.rom[address - ROM_BASE]),
            MMIO_BASE..MMIO_END => Ok(self.mmio.read_byte(address)),
            _ => Err(ExceptionType::LoadAccessFault),
        }
    }

    pub fn write_byte(
        &mut self,
        phys_address: &PhysicalAddress,
        val: u8,
    ) -> Result<(), ExceptionType> {
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
                    self.hitf
                        .write_fromhost_byte(address - self.hitf.fromhost, val);
                }

                self.dram[address - DRAM_BASE] = val;

                // TODO Si en algun momento meto mas HARTS hay que hacer que se invaliden los de OTROS HARTS
                // for i in 0..NUM_HARTS {
                //     if self.is_address_inside_reservation_set(i, address) {
                //         self.invalidate_reserved_address(i);
                //     }
                // }

                Ok(())
            }
            MMIO_BASE..MMIO_END => {
                self.mmio.write_byte(address, val);
                Ok(())
            }

            _ => Err(ExceptionType::StoreAmoAccessFault),
        }
    }

    pub fn read_aligned_word(&self, phys_address: &PhysicalAddress) -> Result<u32, ExceptionType> {
        if phys_address.0 % 4 != 0 {
            return Err(ExceptionType::LoadAddressMisaligned);
        }

        let val_0 = self.read_byte(phys_address)?;
        let val_1 = self.read_byte(&phys_address.wrapping_add(1))?;
        let val_2 = self.read_byte(&phys_address.wrapping_add(2))?;
        let val_3 = self.read_byte(&phys_address.wrapping_add(3))?;

        Ok(u32::from_le_bytes([val_0, val_1, val_2, val_3]))
    }

    pub fn read_word(&self, phys_address: &PhysicalAddress) -> Result<u32, ExceptionType> {
        let val_0 = self.read_byte(phys_address)?;
        let val_1 = self.read_byte(&phys_address.wrapping_add(1))?;
        let val_2 = self.read_byte(&phys_address.wrapping_add(2))?;
        let val_3 = self.read_byte(&phys_address.wrapping_add(3))?;

        Ok(u32::from_le_bytes([val_0, val_1, val_2, val_3]))
    }

    pub fn read_aligned_half_word(
        &self,
        phys_address: &PhysicalAddress,
    ) -> Result<u16, ExceptionType> {
        if phys_address.0 % 2 != 0 {
            return Err(ExceptionType::LoadAddressMisaligned);
        }

        let val_0 = self.read_byte(phys_address)?;
        let val_1 = self.read_byte(&phys_address.wrapping_add(1))?;

        Ok(u16::from_le_bytes([val_0, val_1]))
    }

    pub fn write_aligned_half_word(
        &mut self,
        phys_address: &PhysicalAddress,
        half_word: u16,
    ) -> Result<(), ExceptionType> {
        if phys_address.0 % 2 != 0 {
            return Err(ExceptionType::StoreAmoAddressMisaligned);
        }

        let bytes = half_word.to_le_bytes();
        self.write_byte(phys_address, bytes[0])?;
        self.write_byte(&phys_address.wrapping_add(1), bytes[1])?;

        Ok(())
    }

    pub fn write_aligned_word(
        &mut self,
        phys_address: &PhysicalAddress,
        word: u32,
    ) -> Result<(), ExceptionType> {
        if phys_address.0 % 4 != 0 {
            return Err(ExceptionType::StoreAmoAddressMisaligned);
        }

        let bytes = word.to_le_bytes();
        self.write_byte(phys_address, bytes[0])?;
        self.write_byte(&phys_address.wrapping_add(1), bytes[1])?;
        self.write_byte(&phys_address.wrapping_add(2), bytes[2])?;
        self.write_byte(&phys_address.wrapping_add(3), bytes[3])?;

        Ok(())
    }

    pub fn check_pma(&self, phys_address: &PhysicalAddress, _acces_type: AccessType) -> bool {
        let address = phys_address.0 as usize;

        // TODO Queda usar access_type
        match address {
            DRAM_BASE..DRAM_END => {
                true
            }
            UART_BASE..UART_END => {
                true
            }
            RTC_BASE..RTC_END => {
                true
            }

            _ => false
        }
    }

    pub fn reserve_address(&mut self, hart_id: usize, address_start: usize, address_end: usize) {
        assert!(hart_id < NUM_HARTS);

        self.reserved_addresses[hart_id] = Some((address_start, address_end))
    }

    pub fn invalidate_reserved_address(&mut self, hart_id: usize) {
        assert!(hart_id < NUM_HARTS);

        self.reserved_addresses[hart_id] = None
    }

    pub fn is_address_reserved(&self, hart_id: usize, address_start: usize, address_end: usize) -> bool {
        assert!(hart_id < NUM_HARTS);

        if let Some((start, end)) = self.reserved_addresses[hart_id] {
            address_start >= start && address_end < end
        } else {
            false
        }
    }

    pub fn is_address_inside_reservation_set(&self, hart_id: usize, address: usize) -> bool {
        assert!(hart_id < NUM_HARTS);

        if let Some((start, end)) = self.reserved_addresses[hart_id] {
            address >= start && address < end
        } else {
            false
        }
    }
}

impl Bus {
    pub fn load_section(&mut self, data: &[u8], start: usize) {
        assert!(start >= DRAM_BASE, "Invalid address {start}");
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
