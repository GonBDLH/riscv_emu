use bitfield::bitfield;

use crate::interpreter::{
    riscv_core::PrivilegeLevel, virtual_memory::sv32::{AccessType, PhysicalAddress},
};

const PMPCFG_MASK: u32 = 0x9F;

pub struct PmpCsrs {
    pmp_cfg: [u32; 16],
    pmp_addr: [u32; 64],
}

impl Default for PmpCsrs {
    fn default() -> Self {
        Self {
            pmp_cfg: [0; 16],
            pmp_addr: [0; 64],
        }
    }
}

impl PmpCsrs {
    pub fn get_pmp_cfg(&self, pmp_csr: usize) -> u32 {
        assert!(pmp_csr < 16);
        self.pmp_cfg[pmp_csr]
    }

    pub fn get_pmp_addr(&self, pmp_csr: usize) -> u32 {
        assert!(pmp_csr < 64);
        self.pmp_addr[pmp_csr]
    }

    pub fn set_pmp_cfg(&mut self, pmp_csr: usize, val: u32) {
        assert!(pmp_csr < 16);
        let mut new_val = 0;

        for i in 0..4 {
            let val_sub = (val >> (i * 8)) & 0xFF;
            let pmpcfg_entry = self.get_pmp_cfg_entry(4 * pmp_csr + i);
            if pmpcfg_entry.get_l() {
                // lock bit set
                new_val |= (pmpcfg_entry.0 as u32) << (i * 8);
            } else {
                new_val |= (val_sub & PMPCFG_MASK) << (i * 8); 
            }
        }
        self.pmp_cfg[pmp_csr] = new_val;
    }

    pub fn set_pmp_addr(&mut self, pmp_csr: usize, val: u32) {
        assert!(pmp_csr < 64);
        let pmpcfg = self.get_pmp_cfg_entry(pmp_csr);

        let mut can_write = !pmpcfg.get_l();
        if pmp_csr < 63 {
            let next_pmp = self.get_pmp_cfg_entry(pmp_csr + 1);
            can_write |= !next_pmp.get_l() && (next_pmp.get_a() != 0x01); 
        }

        if can_write {
            self.pmp_addr[pmp_csr] = val;
        }
    }

    fn get_pmp_cfg_entry(&self, pmp_idx: usize) -> PmpCfgEntry {
        assert!(pmp_idx < 64);
        let csr = pmp_idx / 4;
        let idx = pmp_idx % 4;

        let cfg = self.pmp_cfg[csr as usize];
        let cfg_entry = (cfg >> (idx * 8)) & 0xFF;

        PmpCfgEntry(cfg_entry as u8)
    }

    pub fn has_access(
        &self,
        priv_level: PrivilegeLevel,
        access_type: AccessType,
        address: &PhysicalAddress,
        access_length: u64,
    ) -> bool {
        for (addr_idx, addr_entry) in self.pmp_addr.iter().enumerate() {
            let pmp_cfg_entry = self.get_pmp_cfg_entry(addr_idx);
            match pmp_cfg_entry.get_a() {
                0b00 => continue,
                0b01 => {
                    // TOR
                    let addr_start = if addr_idx == 0 {
                        0
                    } else {
                        (self.pmp_addr[addr_idx - 1] as u64) << 2
                    };
                    let addr_end = (*addr_entry as u64) << 2;
                    let access_end = address.0.wrapping_add(access_length).wrapping_sub(1);

                    if address.0 >= addr_start && access_end <= addr_end {
                        return pmp_cfg_entry.check_access(priv_level, access_type);
                    }
                }
                0b10 => {
                    // NA4
                    let addr_start = (*addr_entry as u64) << 2;
                    let addr_end = addr_start.wrapping_add(4);
                    let access_end = address.0.wrapping_add(access_length).wrapping_sub(1);

                    if address.0 >= addr_start && access_end <= addr_end {
                        return pmp_cfg_entry.check_access(priv_level, access_type);
                    }
                }
                0b11 => {
                    // NAPOT
                    let addr_range = 1u64 << (addr_entry.trailing_ones() + 3);
                    let addr_start = ((*addr_entry as u64) << 2) & !(addr_range - 1);
                    let addr_end = addr_start.wrapping_add(addr_range);
                    let access_end = address.0.wrapping_add(access_length).wrapping_sub(1);

                    if address.0 >= addr_start && access_end <= addr_end {
                        return pmp_cfg_entry.check_access(priv_level, access_type);
                    }
                }
                _ => unreachable!(),
            }
        }

        match priv_level {
            PrivilegeLevel::Machine => true,
            _ => false,
        }
    }
}

bitfield! {
    pub struct PmpCfgEntry(u8);
    pub get_r, _: 0; // WPRI 0
    pub get_w, _: 1;
    pub get_x, _: 2; // WPRI 2
    pub get_a, _: 4, 3;
    _, _: 6, 5;
    pub get_l, _: 7;
}

impl PmpCfgEntry {
    fn check_rwx(priv_level: PrivilegeLevel, perm_bit: bool, l: bool) -> bool {
        match priv_level {
            PrivilegeLevel::Machine if !l => true,
            _ => perm_bit,
            // PrivilegeLevel::Machine => {
            //     if !l {
            //         true
            //     } else {
            //         perm_bit
            //     }
            // }
            // _ => perm_bit
        }
    }

    fn check_access(&self, priv_level: PrivilegeLevel, access_type: AccessType) -> bool {
        match access_type {
            AccessType::Load => Self::check_rwx(priv_level, self.get_r(), self.get_l()),
            AccessType::StoreAmo => Self::check_rwx(priv_level, self.get_w(), self.get_l()),
            AccessType::Execute => Self::check_rwx(priv_level, self.get_x(), self.get_l()),
        }
    }
}
