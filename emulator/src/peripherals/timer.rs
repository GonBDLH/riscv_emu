use std::time::{Duration, Instant};

use crate::peripherals::Peripheral;

pub struct RealTimeCounter {
    earlier: Instant,

    mtime: u64,
    mtimecmp: u64,
}

impl Peripheral for RealTimeCounter {
    fn new() -> Self {
        Self {
            earlier: Instant::now(),
            mtime: 0,
            mtimecmp: 0,
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
        if address < 4 {
            ((self.mtime >> (8 * address)) & 0xFF) as u8
        } else if address < 8 {
            ((self.mtimecmp >> (8 * (address - 4))) & 0xFF) as u8
        } else {
            0
        }
    }

    fn write_byte(&mut self, address: usize, val: u8) {
        if address < 4 {
            let old_mtime = self.mtime;
            let mut new_mtime = old_mtime & !(0xFF << (8 * address));
            new_mtime |= (val as u64) << (8 * address);

            self.mtime = new_mtime;
        } else if address < 8 {
            let old_mtimecmp = self.mtimecmp;
            let mut new_mtimecmp = old_mtimecmp & !(0xFF << (8 * address));
            new_mtimecmp |= (val as u64) << (8 * address);

            self.mtimecmp = new_mtimecmp;
        }
    }
}
