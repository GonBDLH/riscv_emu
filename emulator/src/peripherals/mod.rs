use std::time::Duration;

use crate::{interpreter::riscv_core::InterruptType, peripherals::{clint::Clint, uart_16550::Uart16550}};

pub mod clint;
pub mod uart_16550;

pub const CLINT_BASE: usize = 0x02000000;
pub const CLINT_SIZE: usize = 0xC0000;
pub const CLINT_END: usize = CLINT_BASE + CLINT_SIZE;

pub const UART_BASE: usize = 0x03001000;
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
    clint: Clint,
    uart: Uart16550
}

impl Mmio {
    pub fn new() -> Self {
        Self { 
            clint: Clint::new(),
            uart: Uart16550::new()
        }
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        match address {
            UART_BASE..UART_END => self.uart.read_byte(address - UART_BASE),
            CLINT_BASE..CLINT_END => self.clint.read_byte(address - CLINT_BASE),
            _ => unreachable!("Missing peripheral at 0x{:08X}", address)
        }
    }

    pub fn write_byte(&mut self, address: usize, val: u8) {
        match address {
            UART_BASE..UART_END => self.uart.write_byte(address - UART_BASE, val),
            CLINT_BASE..CLINT_END => self.clint.write_byte(address - CLINT_BASE, val),
            _ => unreachable!("Missing peripheral at 0x{:08X}", address)
        }
    }

    pub fn update(&mut self, duration: Duration) {
        self.clint.update(duration);
    }

    pub fn has_interrupt(&mut self) -> Option<InterruptType> {
        if self.clint.has_interrupt() {
            Some(InterruptType::MachineTimerInt)
        } else if self.uart.has_interrupt() {
            todo!()
        } else {
            None
        }
    }
}