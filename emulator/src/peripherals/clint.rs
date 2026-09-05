use std::time::{Duration, Instant};

use crate::peripherals::Peripheral;

pub const MTIME_OFFSET: usize = 0xBFF8;
const MTIME_OFFSET_END: usize = MTIME_OFFSET + 8;
pub const MTIMECMP_OFFSET: usize = 0x4000;
const MTIMECMP_OFFSET_END: usize = MTIMECMP_OFFSET + 8;
pub const MSIP_OFFSET: usize = 0x0;
const MSIP_OFFSET_END: usize = MSIP_OFFSET + 4;

pub struct Clint {
    earlier: Instant,

    mtime: u64,
    mtimecmp: u64,
    msip: u32,
}

impl Peripheral for Clint {
    fn new() -> Self {
        Self {
            earlier: Instant::now(),
            mtime: 0,
            mtimecmp: 0,
            msip: 0
        }
    }

    // TODO Ver si aqui hay que usar Duration o no (Es un RTC, deberia tener en cuenta solo el tiempo que se ejecuta el emulador, o el tiempo total?)
    fn update(&mut self, _duration: Duration) {
        let new_earlier = Instant::now();
        self.mtime += (new_earlier.duration_since(self.earlier).as_nanos() / 100) as u64;
        self.earlier = new_earlier;
    }

    fn has_interrupt(&mut self) -> bool {
        self.mtime >= self.mtimecmp
    }

    fn read_byte(&self, address: usize) -> u8 {
        // if address < 4 {
        //     ((self.mtime >> (8 * address)) & 0xFF) as u8
        // } else if address < 8 {
        //     ((self.mtimecmp >> (8 * (address - 4))) & 0xFF) as u8
        // } else {
        //     0
        // }
        match address {
            MTIME_OFFSET..MTIME_OFFSET_END => {
                let byte = address - MTIME_OFFSET;

                let val = ((self.mtime >> (8 * byte)) & 0xFF) as u8;
                val
            }
            MTIMECMP_OFFSET..MTIMECMP_OFFSET_END => {
                let byte = address - MTIME_OFFSET;

                ((self.mtimecmp >> (8 * byte)) & 0xFF) as u8
            }
            MSIP_OFFSET..MSIP_OFFSET_END => {
                let byte = address - MSIP_OFFSET;

                ((self.msip >> (4 * byte)) & 0xFF) as u8
            }
            _ => unreachable!("Aqui nunca deberia entrar: {:08X}", address)
        }
    }

    fn write_byte(&mut self, address: usize, val: u8) {
        match address {
            MTIME_OFFSET..MTIME_OFFSET_END => {
                let byte = address - MTIME_OFFSET;

                let old_mtime = self.mtime;
                let mut new_mtime = old_mtime & !(0xFF << (8 * byte));
                new_mtime |= (val as u64) << (8 * byte);

                self.mtime = new_mtime;
            }
            MTIMECMP_OFFSET..MTIMECMP_OFFSET_END => {
                let byte = address - MTIMECMP_OFFSET;

                let old_mtimecmp = self.mtimecmp;
                let mut new_mtimecmp = old_mtimecmp & !(0xFF << (8 * byte));
                new_mtimecmp |= (val as u64) << (8 * byte);

                self.mtimecmp = new_mtimecmp;
            }
            MSIP_OFFSET..MSIP_OFFSET_END => {
                let byte = address - MSIP_OFFSET;

                let old_msip = self.msip;
                let mut new_msip = old_msip & !(0xFF << (4 * byte));
                new_msip |= (val as u32) << (4 * byte);

                self.msip = new_msip;
            }
            _ => unreachable!("Aqui nunca deberia entrar: {:08X}", address)
        }
    }
}
