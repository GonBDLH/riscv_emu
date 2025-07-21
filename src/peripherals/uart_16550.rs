use std::{
    collections::VecDeque,
    io::{Read, stdin},
    sync::{
        Arc, Condvar, Mutex,
        atomic::AtomicBool,
        mpsc::{Receiver, channel},
    },
    thread,
};

pub struct Uart16550 {
    rx_tx_reg: Arc<(Mutex<u8>, Condvar)>,

    interrupt: Arc<AtomicBool>,
}

impl Uart16550 {
    pub fn new() -> Self {
        let rx_tx_reg = Arc::new((Mutex::new(0), Condvar::new()));
        let rx_tx_reg_thread = rx_tx_reg.clone();

        let interrupt = Arc::new(AtomicBool::new(false));

        let _stdin_read_thread = thread::spawn(move || {
            loop {
                let mut buf = [0u8];

                if stdin().read_exact(&mut buf).is_ok() {
                    let (lock, cvar) = &*rx_tx_reg_thread;
                    let mut rx = lock.lock().expect("MUTEX ENVENENDADO");
                }
            }
        });

        Self {
            rx_tx_reg,
            interrupt,
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        0
    }

    pub fn write(&mut self, address: usize, val: u8) {
        if address == 0 {
            println!("{val}");
        }
    }
}
