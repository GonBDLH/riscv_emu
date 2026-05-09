use std::time::Duration;

pub mod timer;
pub mod uart_16550;

pub trait Peripheral {
    fn new() -> Self;
    fn update(&mut self, duration: Duration);
    fn has_interrupt(&mut self) -> bool;
    fn read_byte(&self, address: usize) -> u8;
    fn write_byte(&mut self, address: usize, val: u8);
}
