use std::time::Duration;

use crate::{interpreter::riscv_core::InterruptType, peripherals::{timer::RealTimeCounter, uart_16550::Uart16550}};

pub mod timer;
pub mod uart_16550;

pub const RTC_BASE: usize = 0x03000000;
pub const RTC_SIZE: usize = 0x1000;
pub const RTC_END: usize = RTC_BASE + RTC_SIZE;

pub const UART_BASE: usize = RTC_END;
pub const UART_SIZE: usize = 0x100;
pub const UART_END: usize = UART_BASE + UART_SIZE;

pub trait Peripheral {
    fn new() -> Self;
    fn update(&mut self, duration: Duration);
    fn has_interrupt(&mut self) -> bool;
    fn read_byte(&self, address: usize) -> u8;
    fn write_byte(&mut self, address: usize, val: u8);
}

pub struct Mmio {
    rtc: RealTimeCounter,
    uart: Uart16550
}

impl Mmio {
    pub fn new() -> Self {
        Self { 
            rtc: RealTimeCounter::new(),
            uart: Uart16550::new()
        }
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        match address {
            UART_BASE..UART_END => self.uart.read_byte(address - UART_BASE),
            RTC_BASE..RTC_END => self.rtc.read_byte(address - RTC_BASE),
            _ => unreachable!("Missing peripheral at 0x{:08X}", address)
        }
    }

    pub fn write_byte(&mut self, address: usize, val: u8) {
        match address {
            UART_BASE..UART_END => self.uart.write_byte(address - UART_BASE, val),
            RTC_BASE..RTC_END => self.rtc.write_byte(address - RTC_BASE, val),
            _ => unreachable!("Missing peripheral at 0x{:08X}", address)
        }
    }

    pub fn update(&mut self, duration: Duration) {
        self.rtc.update(duration);
    }

    pub fn has_interrupt(&mut self) -> Option<InterruptType> {
        if self.rtc.has_interrupt() {
            Some(InterruptType::MachineTimerInt)
        } else if self.uart.has_interrupt() {
            todo!()
        } else {
            None
        }
    }
}