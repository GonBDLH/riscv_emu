use crate::interpreter::{
    bus::Bus,
    csr::Satp32,
    riscv_core::{Exception, ExceptionType},
    virtual_memory::sv32::{
        LEVELS, PAGESIZE, PTESIZE, PageTableEntry, PhysicalAddress, VirtAddress,
    },
};

#[derive(Default)]
pub struct HitfState {
    pub tohost: usize,
    pub fromhost: usize,

    pub hitf_size: u8,

    tohost_val: [u8; 8],
    tohost_cnt: u8,
    fromhost_val: [u8; 8],
    fromhost_cnt: u8,
}

pub struct Hitf {
    device: u8,
    command: u8,
    data: u64,
}

impl HitfState {
    pub fn new(tohost: usize, fromhost: usize) -> Self {
        Self {
            tohost,
            fromhost,
            tohost_val: [0; 8],
            tohost_cnt: 0,
            fromhost_val: [0; 8],
            fromhost_cnt: 0,
            hitf_size: 4,
        }
    }

    pub fn write_tohost_byte(&mut self, address: usize, val: u8) -> Option<Hitf> {
        let address = address - self.tohost;

        self.tohost_val[address] = val;
        self.tohost_cnt += 1;

        if self.tohost_cnt == self.hitf_size {
            self.tohost_cnt = 0;

            let hitf = Hitf {
                device: self.tohost_val[7],
                command: self.tohost_val[6],
                data: (u64::from_le_bytes(self.tohost_val) & 0x0000FFFFFFFFFFFF),
            };

            Some(hitf)
        } else {
            None
        }
    }

    pub fn read_tohost_byte(&self, _address: usize) -> u8 {
        0
    }

    pub fn write_fromhost_byte(&mut self, address: usize, val: u8) {
        let address = address - self.fromhost;

        self.fromhost_val[address] = val;
        self.fromhost_cnt += 1;

        if self.fromhost_cnt == 8 {
            // TODO

            self.fromhost_cnt = 0;
        }
    }

    pub fn read_fromhost_byte(&self, address: usize) -> u8 {
        let address = address - self.fromhost;

        self.fromhost_val[address]
    }

    pub fn translate_sv32(satp: Satp32, bus: &Bus, virt_address: u32) -> Option<PhysicalAddress> {
        if !satp.get_mode() {
            return Some(PhysicalAddress(virt_address as u64));
        }
        let va = VirtAddress(virt_address);

        let mut a = satp.get_ppn() * PAGESIZE;
        let mut i = LEVELS as i32 - 1;

        while i >= 0 {
            let pte_addr = if i == 1 {
                a + va.get_vpn1() * PTESIZE
            } else {
                a + va.get_vpn0() * PTESIZE
            };

            let pte = PageTableEntry(bus.read_word(&PhysicalAddress(pte_addr as u64)).unwrap());

            if !pte.get_v() || (!pte.get_r() && pte.get_w()) {
                return None;
            }

            if pte.get_r() || pte.get_x() {
                if i > 0 && (pte.get_ppn0() != 0) {
                    return None;
                }

                let mut phys_addres = PhysicalAddress(0);
                phys_addres.set_page_offset(va.get_page_offset() as u64);
                if i > 0 {
                    phys_addres.set_ppn0(va.get_vpn0() as u64);
                    phys_addres.set_ppn1(pte.get_ppn1() as u64);
                } else {
                    phys_addres.set_ppn(pte.get_ppn() as u64);
                }

                return Some(phys_addres);
            }

            i -= 1;
            a = pte.get_ppn() * PAGESIZE;
        }

        None
    }
}

impl Hitf {
    pub fn run(&self) -> Result<(), Exception> {
        match self.device {
            0 => {
                // SYSCALL DEVICE
                if self.data & 0b1 > 0 {
                    // EXIT CODE

                    return Err(Exception::new(ExceptionType::HitfExit, self.data as u32));
                } else {
                    // SYSCALL
                    let syscall = self.data;

                    // println!("SYSCALL {:08X}", addr.0);

                    return Err(Exception::new(ExceptionType::HitfSyscall, syscall as u32));
                }
            }
            1 => {
                // BLOCKING CHARACTER DEVICE
                match self.command {
                    0 => {
                        // READ CHARACTER
                        todo!()
                    }
                    1 => {
                        // WRITE CHARACTER
                        todo!()
                    }
                    _ => Ok(()),
                }
            }
            _ => Ok(()),
        }
    }
}
