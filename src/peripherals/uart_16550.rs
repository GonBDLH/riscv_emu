#![allow(clippy::new_without_default)]

// SACADO DE: https://github.com/d0iasm/rvemu

use std::{
    io::{self, stdin, Read, Write},
    sync::{
        atomic::{AtomicBool, Ordering}, Arc, Condvar, Mutex
    },
    thread,
};

const UART_RHR_THR: usize = 0; 
const UART_LSR: usize = 5;

const UART_LSR_RX: u8 = 1;
const UART_LSR_TX: u8 = 1 << 5;

pub struct Uart16550 {
    regs: Arc<(Mutex<[u8; 8]>, Condvar)>,

    interrupt: Arc<AtomicBool>,
}

impl Uart16550 {
    pub fn new() -> Self {
        let regs = Arc::new((Mutex::new([0; 8]), Condvar::new()));
        let interrupt = Arc::new(AtomicBool::new(false));

        {
            let (regs, _cvar) = &*regs;
            let mut regs = regs.lock().expect("Fallo al obtener la uart");
            regs[UART_LSR] |= UART_LSR_TX;
        }

        let cloned_regs = regs.clone();
        let cloned_interrupt = interrupt.clone();

        let _stdin_read_thread = thread::spawn(move || {
            loop {
                let mut buf = [0u8];

                if stdin().read_exact(&mut buf).is_ok() {
                    let (regs, cvar) = &*cloned_regs;
                    let mut regs = regs.lock().expect("MUTEX ENVENENDADO");
                    while regs[UART_LSR] & UART_LSR_RX == 1 {
                        regs = cvar.wait(regs).expect("Mutex envenenado");
                    }

                    regs[UART_RHR_THR] = buf[0];
                    cloned_interrupt.store(true, Ordering::Release);

                    regs[UART_LSR] |= UART_LSR_RX;
                }
            }
        });

        Self { regs, interrupt }
    }

    pub fn has_interrupt(&self) -> bool {
        self.interrupt.swap(false, Ordering::Acquire)
    }

    pub fn read(&self, address: usize) -> u8 {
        let (regs, cvar) = &*self.regs;
        let mut regs = regs.lock().expect("Mutex envenendado");
        match address {
            UART_RHR_THR => {
                cvar.notify_one();
                regs[UART_LSR] &= !UART_LSR_RX;
                regs[UART_RHR_THR]
            }
            _ => regs[address]
        }
    }

    pub fn write(&mut self, address: usize, val: u8) {
        let (regs, _cvar) = &*self.regs;
        let mut regs = regs.lock().expect("Mutex envenendado");
        match address {
            UART_RHR_THR => {
                print!("{}", val as char);
                io::stdout().flush().expect("Fallo al limpiar stdout");
            },
            _ => regs[address] = val
        }
    }
}
